import { render, screen, fireEvent } from '@testing-library/react';
import {describe, it, expect, beforeAll, vi} from 'vitest';
import DeviceTable from './DeviceTable';
import '@testing-library/jest-dom';
import {Device} from "../interfaces/Device.ts";

beforeAll(() => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    window.getComputedStyle = vi.fn().mockImplementation((_) => {
        return {
            display: 'block',
            width: '100%', // Mock any necessary CSS properties here
            height: 'auto',
            // Add other properties as needed
        } as CSSStyleDeclaration;
    });
    window.matchMedia = vi.fn().mockImplementation((query) => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(), // Deprecated
        removeListener: vi.fn(), // Deprecated
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
    }));
});
describe('DeviceTable component', () => {
    const mockDevices: Device[] = [
        {
            status: 'Online',
            lastConnection: '2024-09-20T12:00:00Z',
            vendor: 'Vendor A',
            ip: '192.168.1.1',
            mac: 'AA:BB:CC:DD:EE:FF',
            updatedAt: new Date(),
        },
        {
            status: 'Offline',
            lastConnection: '2024-09-19T12:00:00Z',
            vendor: 'Vendor B',
            ip: '192.168.1.2',
            mac: 'FF:EE:DD:CC:BB:AA',
            updatedAt: new Date(),
        },
    ];

    it('renders the device table with devices', () => {
        render(<DeviceTable devices={mockDevices} />);

        // Check if the table is rendered
        expect(screen.getByText('Devices')).toBeInTheDocument();
        expect(screen.getByText('Vendor A')).toBeInTheDocument();
        expect(screen.getByText('Vendor B')).toBeInTheDocument();
    });

    it('filters devices based on search input', () => {
        render(<DeviceTable devices={mockDevices} />);

        // Check initial state
        expect(screen.getByText('Vendor A')).toBeInTheDocument();
        expect(screen.getByText('Vendor B')).toBeInTheDocument();

        // Type in the search input
        const searchInput = screen.getByPlaceholderText('Search by vendor name...');
        fireEvent.change(searchInput, { target: { value: 'Vendor A' } });

        // Verify that only the filtered device is displayed
        expect(screen.getByText('Vendor A')).toBeInTheDocument();
        expect(screen.queryByText('Vendor B')).not.toBeInTheDocument();
    });

    it('shows empty state when there are no devices', () => {
        render(<DeviceTable devices={[]} />);

        // Check if the Empty component is rendered
        expect(screen.getByText('No data')).toBeInTheDocument();
    });
    it('DeviceTable matches snapshot', () => {
        const deviceTable = render(<DeviceTable devices={[]} />);

        // Check if the Empty component is rendered
        expect(deviceTable).toMatchSnapshot();
    });
});
