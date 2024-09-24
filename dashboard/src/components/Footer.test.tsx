import { render, screen } from '@testing-library/react';
import Footer from './Footer';
import {describe, expect, it, vi} from 'vitest';
import '@testing-library/jest-dom';  // Import this to use toBeInTheDocument()

// Mock the getCurrentYear function
vi.mock('../utils/dateUtils.ts', () => ({
    getCurrentYear: () => 2024, // Mock it to return a fixed year
}));

describe('Footer component', () => {
    it('renders the footer with the correct year and text', () => {
        render(<Footer/>);

        // Check that the footer text includes the mocked current year
        const footerText = screen.getByText(/© 2024 Network Monitor. All rights reserved./i);
        expect(footerText).toBeInTheDocument();
    });
    it('Footer matches snapshot', () => {
        const footerComponent = render(<Footer/>);
        expect(footerComponent).toMatchSnapshot();
    });
});
