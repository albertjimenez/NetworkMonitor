import {render, screen} from '@testing-library/react';
import {beforeAll, describe, expect, it, vi} from 'vitest';
import ServerStatistics from './ServerStatistics';
import '@testing-library/jest-dom';

import Metrics from "../interfaces/Metrics.ts";
const metrics = {
    cpu_usage: {usage: 75},
    disk_usage: {total: 1024 * 1024 * 1024, available: 512 * 1024 * 1024},
    mem_usage: {used: 2048 * 1024, total: 4096 * 1024},
    network_usage: [{name: 'eth0', tx: 100, rx: 200}],
    temperature_info: [{name: 'CPU', temperature: 70}],
} as unknown as Metrics;

beforeAll(() => {
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

describe('ServerStatistics component', () => {
    it('renders loading skeleton when isLoading is true', () => {
        render(<ServerStatistics isLoading={true}/>);

        // Check if the skeleton is displayed
        expect(screen.getByText('Server Monitoring')).toBeInTheDocument();
    });

    it('renders metrics correctly when data is provided', () => {
        render(<ServerStatistics metrics={metrics} isLoading={false}/>);

        // Check if the metrics are rendered
        expect(screen.getByText('Server Monitoring')).toBeInTheDocument();
        expect(screen.getByText('CPU Usage')).toBeInTheDocument();
        expect(screen.getByText(/Disk Available - Total 1 GB/i)).toBeInTheDocument();
    });

    it('renders the correct titles', () => {
        render(<ServerStatistics isLoading={false}/>);

        // Check titles for sections
        expect(screen.getByText('Hardware Usages')).toBeInTheDocument();
        expect(screen.getByText('Network Usage')).toBeInTheDocument();
        expect(screen.getByText('Temperatures')).toBeInTheDocument();
    });
    it('ServerStatistics matches snapshot', () => {
        const server = render(<ServerStatistics metrics={metrics} isLoading={false}/>);
        expect(server).toMatchSnapshot();

    })
});
