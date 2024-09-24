import { describe, it, expect } from 'vitest';
import UnitConverter from './UnitConverter';

describe('UnitConverter', () => {
    describe('bytesToHumanReadable', () => {
        it('should convert bytes to gigabytes when input is >= 1GB', () => {
            const result = UnitConverter.bytesToHumanReadable(2 * 1024 * 1024 * 1024); // 2GB
            expect(result).toEqual({ value: 2, label: 'GB' });
        });

        it('should convert bytes to megabytes when input is >= 1MB and < 1GB', () => {
            const result = UnitConverter.bytesToHumanReadable(2 * 1024 * 1024); // 2MB
            expect(result).toEqual({ value: 2, label: 'MB' });
        });

        it('should convert bytes to kilobytes when input is < 1MB', () => {
            const result = UnitConverter.bytesToHumanReadable(512 * 1024); // 512KB
            expect(result).toEqual({ value: 512, label: 'KB' });
        });
    });

    describe('bitsToHumanReadable', () => {
        it('should convert bits to gigabits when input is >= 1Gbps', () => {
            const result = UnitConverter.bitsToHumanReadable(2_000_000_000); // 2Gbps
            expect(result).toEqual({ value: 2, label: 'Gbps' });
        });

        it('should convert bits to megabits when input is >= 1Mbps and < 1Gbps', () => {
            const result = UnitConverter.bitsToHumanReadable(2_000_000); // 2Mbps
            expect(result).toEqual({ value: 2, label: 'Mbps' });
        });

        it('should convert bits to kilobits when input is >= 1Kbps and < 1Mbps', () => {
            const result = UnitConverter.bitsToHumanReadable(2_000); // 2Kbps
            expect(result).toEqual({ value: 2, label: 'Kbps' });
        });

        it('should return bits when input is < 1Kbps', () => {
            const result = UnitConverter.bitsToHumanReadable(500); // 500bps
            expect(result).toEqual({ value: 500, label: 'bps' });
        });
    });
});
