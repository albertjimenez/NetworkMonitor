export default interface Metrics {
    cpu_usage: CPUUsage;
    disk_usage: DiskUsage;
    mem_usage: MemUsage;
    network_usage: NetworkUsage;
    temperature_info: TemperatureInfo[];
}

export interface CPUUsage {
    usage: number;
}

export interface DiskUsage {
    available: number;
    total: number;
}

export interface MemUsage {
    used: number;
    total: number;
}

export interface NetworkUsage {
    name: string;
    tx: number;
    rx: number;
}

export interface TemperatureInfo {
    name: string;
    temperature: number;
}
