import {render, screen} from '@testing-library/react';
import {beforeAll, describe, expect, it, vi} from 'vitest';
import Settings from './Settings';
import '@testing-library/jest-dom';
import {BrowserRouter} from "react-router-dom";

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
// Mock useNats
vi.mock('../hooks/useNats.tsx', () => ({
    useNats: () => ({mockNatsConnection: true}),  // Return a mock NATS object
}));

// Mock useReadFromKV
vi.mock('../hooks/useReadFromKV.tsx', () => ({
    default: (key: string) => ({
        value: key === 'SYSTEM_MONITOR_REFRESH_TIME' ? '30' : '60',  // Mock values for KV keys
    }),
}));

// Mock sendToKV directly within vi.mock
vi.mock('../hooks/sendToKV.tsx', () => ({
    sendToKV: vi.fn(async () => Promise.resolve()),  // Mock sendToKV function inside vi.mock
}));

describe('Settings component', () => {
    it('renders correctly and matches snapshot', () => {
        const settings = render(<BrowserRouter>
            <Settings/>
        </BrowserRouter>);
        expect(settings).toMatchSnapshot();
    });
    it('renders the settings form with prefilled values from KV store', () => {
        render(<Settings/>);

        // Check if the input fields are populated with mocked KV values
        const refreshTimeInput = screen.getByPlaceholderText(/enter refresh time/i);
        const hostIntervalInput = screen.getByPlaceholderText(/enter host scan interval/i);

        expect(refreshTimeInput).toHaveValue(60);  // Mocked value from useReadFromKV
        expect(hostIntervalInput).toHaveValue(60);  // Mocked value from useReadFromKV
    });
});

