import { render, screen } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import Header from './Header';
import { describe, it, expect } from 'vitest';
import '@testing-library/jest-dom';

describe('Header component', () => {
    it('renders the logo with "Network Monitor" text', () => {
        render(
            <BrowserRouter>
                <Header />
            </BrowserRouter>
        );

        // Check for logo text "Network Monitor"
        const logoText = screen.getByText(/Network Monitor/i);
        expect(logoText).toBeInTheDocument();
    });

    it('Header matches snapshot', () => {
        const header = render(
            <BrowserRouter>
                <Header />
            </BrowserRouter>
        );

        expect(header).toMatchSnapshot();
    });

});
