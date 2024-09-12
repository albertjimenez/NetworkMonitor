export default class UnitConverter {
    static bytesToHumanReadable(bytes: number): { value: number; label: string } {
        const GIGABYTE = 1024 * 1024 * 1024;
        const MEGABYTE = 1024 * 1024;

        if (bytes >= GIGABYTE) {
            return {value: parseFloat((bytes / GIGABYTE).toFixed(2)), label: 'GB'};
        } else if (bytes >= MEGABYTE) {
            return {value: parseFloat((bytes / MEGABYTE).toFixed(2)), label: 'MB'};
        } else {
            return {value: parseFloat((bytes / 1024).toFixed(2)), label: 'KB'};
        }
    }

    static bitsToHumanReadable(bits: number): { value: number; label: string } {
        const GIGABIT = 1_000_000_000;
        const MEGABIT = 1_000_000;
        const KILOBIT = 1_000;

        if (bits >= GIGABIT) {
            return {value: parseFloat((bits / GIGABIT).toFixed(2)), label: 'Gbps'};
        } else if (bits >= MEGABIT) {
            return {value: parseFloat((bits / MEGABIT).toFixed(2)), label: 'Mbps'};
        } else if (bits >= KILOBIT) {
            return {value: parseFloat((bits / KILOBIT).toFixed(2)), label: 'Kbps'};
        } else {
            return {value: bits, label: 'bps'};
        }
    }
}
