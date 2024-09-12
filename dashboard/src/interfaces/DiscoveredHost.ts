export interface DiscoveredHost {
    id: string | null;
    ip: string;
    mac_address: string;
    vendor_name: string;
    discovered_at: string;
    open_ports: Array<{
        id: string | null;
        port: number;
        name: string;
    }>;
}
