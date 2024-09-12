import {Device} from '../interfaces/Device';

export const initialDevices: Device[] = [
    {
        status: 'Online',
        lastConnection: '2023-08-19 10:45 AM',
        vendor: 'Acme Inc.',
        ip: '192.168.1.100',
        mac: '00:11:22:33:44:55',
        updatedAt: new Date(),
    },
    {
        status: 'Online',
        lastConnection: '2023-08-18 11:30 AM',
        vendor: 'Cisco',
        ip: '192.168.1.1',
        mac: '66:77:88:99:AA:BB',
        updatedAt: new Date(),
    },
    {
        status: 'Offline',
        lastConnection: '2023-08-17 3:15 PM',
        vendor: 'Acme Inc.',
        ip: '192.168.1.101',
        mac: 'CC:DD:EE:FF:00:11',
        updatedAt: new Date(),
    },
    {
        status: 'Online',
        lastConnection: '2023-08-20 9:00 AM',
        vendor: 'Netgear',
        ip: '192.168.1.50',
        mac: '22:33:44:55:66:77',
        updatedAt: new Date(),
    },
];
