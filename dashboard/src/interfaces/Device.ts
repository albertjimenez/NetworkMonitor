export interface Device {
    status: 'Online' | 'Offline';
    lastConnection: string;
    vendor: string;
    ip: string;
    mac: string;
    updatedAt: Date;
}

