import React, {useState} from 'react';
import {Badge, Card, Input, Table, TableColumnsType} from 'antd';
import {Device} from "../interfaces/Device.ts";
import {SearchOutlined} from "@ant-design/icons";
import { Empty } from "antd";
import {DeviceWithKey} from "../interfaces/DeviceWithKey.ts";
interface DeviceTableProps {
    devices: Device[];
}


const generateColumns = (): TableColumnsType<Device> => {
    return [{
        title: 'Vendor',
        dataIndex: 'vendor',
        key: 'vendor',
        ellipsis: true,
    }, {
        title: "Status",
        dataIndex: 'status',
        key: 'status',
        render: (status: string) => (
            <Badge status={status === 'Online' ? 'success' : 'error'} text={status}/>
        )
    }, {
        title: "Last Connection",
        dataIndex: 'lastConnection',
        key: 'lastConnection',
    }, {
        title: 'IP',
        dataIndex: 'ip',
        key: 'ip',
        ellipsis: true,
    }, {
        title: "Mac Address",
        dataIndex: 'mac',
        key: 'mac',
        ellipsis: true,
        render: (value: string) => (value?.toUpperCase())
    }];
}

const DeviceTable: React.FC<DeviceTableProps> = ({devices}) => {
    const [searchTerm, setSearchTerm] = useState<string>('');

    const filteredDevices = devices.filter(device =>
        device.vendor.toLowerCase().includes(searchTerm.toLowerCase())
    ).map(device => {
        const mappedDevice: DeviceWithKey = {...device, key: device.mac};
        return mappedDevice
    });

    return <>
        <Card title="Devices" bordered={true}>
            <Input
                placeholder="Search by vendor name..."
                prefix={<SearchOutlined/>}
                style={{width: '200px', marginRight: '20px'}}
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
            />
            {devices.length > 0 && <Table key={"Table"} dataSource={filteredDevices} columns={generateColumns()} scroll={{x: true, y: "true"}}/>}
            {devices.length === 0 && <Empty/>}
        </Card>
    </>;
};

export default DeviceTable;
