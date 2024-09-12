import { useState, useEffect } from "react";
import { useNatsSubscription } from "../../hooks/useNats.tsx";
import { JSONCodec, Msg } from "nats.ws";
import { Device } from "../../interfaces/Device.ts";
import DeviceTable from "../DeviceTable.tsx";
import { DiscoveredHost } from "../../interfaces/DiscoveredHost.ts";

const mapFromDiscoveredHostToDevice = (host: DiscoveredHost): Device => {
    return {
        status: 'Online', // Default to Online; will update based on time
        lastConnection: new Date(host.discovered_at).toLocaleString(),
        vendor: host.vendor_name,
        ip: host.ip,
        mac: host.mac_address,
        updatedAt: new Date(host.discovered_at) // Track last update time
    };
};

const natsSubject = 'monitoring.data';
const TIME_OUT = 60 * 1000; // 60 seconds to declare offline status

const DeviceTableHandler = () => {
    const sc = JSONCodec();
    const [devicesMap, setDevicesMap] = useState<Map<string, Device>>(new Map());

    // Update status based on the time since the last update
    useEffect(() => {
        const interval = setInterval(() => {
            const now = new Date();
            setDevicesMap(prevDevicesMap => {
                const updatedMap = new Map(prevDevicesMap);

                updatedMap.forEach((device, mac) => {
                    const isOnline = (now.getTime() - device.updatedAt.getTime()) < 30000; // 30 seconds
                    updatedMap.set(mac, { ...device, status: isOnline ? 'Online' : 'Offline' });
                });

                return updatedMap;
            });
        }, TIME_OUT); // Update status every 30 seconds

        return () => clearInterval(interval); // Cleanup on unmount
    }, []);

    async function onMessage(msg: Msg) {
        const data = sc.decode(msg.data) as DiscoveredHost;

        setDevicesMap(prevDevicesMap => {
            const newDevice = mapFromDiscoveredHostToDevice(data);
            const updatedMap = new Map(prevDevicesMap);
            updatedMap.set(newDevice.mac, newDevice);
            return updatedMap;
        });
    }

    useNatsSubscription(natsSubject, onMessage);

    // Convert Map to array for rendering
    const devices = Array.from(devicesMap.values());

    return <DeviceTable devices={devices} />;
};

export default DeviceTableHandler;
