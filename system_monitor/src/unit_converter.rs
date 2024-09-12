pub struct UnitConverter;

impl UnitConverter {
    pub fn bytes_to_human_readable(bytes: u64) -> String {
        const GIGABYTE: u64 = 1024 * 1024 * 1024;
        const MEGABYTE: u64 = 1024 * 1024;

        if bytes >= GIGABYTE {
            format!("{:.2} GB", bytes as f64 / GIGABYTE as f64)
        } else if bytes >= MEGABYTE {
            format!("{:.2} MB", bytes as f64 / MEGABYTE as f64)
        } else {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        }
    }
    pub fn bits_to_human_readable(bits: u64) -> String {
        const GIGABIT: u64 = 1_000_000_000;
        const MEGABIT: u64 = 1_000_000;
        const KILOBIT: u64 = 1_000;

        if bits >= GIGABIT {
            format!("{:.2} Gbps", bits as f64 / GIGABIT as f64)
        } else if bits >= MEGABIT {
            format!("{:.2} Mbps", bits as f64 / MEGABIT as f64)
        } else if bits >= KILOBIT {
            format!("{:.2} Kbps", bits as f64 / KILOBIT as f64)
        } else {
            format!("{} bps", bits)
        }
    }
}


