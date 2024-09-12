CREATE TABLE IF NOT EXISTS service_ports (
    id SERIAL PRIMARY KEY,
    port INTEGER NOT NULL,
    name TEXT NOT NULL,
    UNIQUE (port, name)
);

CREATE TABLE IF NOT EXISTS discovered_hosts (
    id SERIAL PRIMARY KEY,
    ip TEXT NOT NULL,
    mac_address TEXT NOT NULL,
    vendor_name TEXT NOT NULL,
    discovered_at TIMESTAMP NOT NULL,
    UNIQUE (mac_address, ip, vendor_name)
);

CREATE TABLE IF NOT EXISTS services_discovered (
    service_port_id INTEGER NOT NULL REFERENCES service_ports(id),
    discovered_host_id INTEGER NOT NULL REFERENCES discovered_hosts(id)
);
