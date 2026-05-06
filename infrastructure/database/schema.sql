-- Nodes identity and state
CREATE TYPE node_state AS ENUM (
    'unregistered',
    'registered',
    'healthy',
    'degraded',
    'quarantined',
    'decommissioned'
);

CREATE TABLE nodes (
    id UUID PRIMARY KEY,
    hostname TEXT NOT NULL,
    ip_address INET NOT NULL,
    os_family TEXT NOT NULL,
    os_arch TEXT NOT NULL,
    state node_state DEFAULT 'unregistered',
    certificate_serial TEXT UNIQUE,
    last_heartbeat TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Workload definitions
CREATE TABLE workloads (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    definition JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Current workload placement
CREATE TABLE placements (
    node_id UUID REFERENCES nodes(id),
    workload_id UUID REFERENCES workloads(id),
    state TEXT,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (node_id, workload_id)
);

-- Immutable event log
CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    source_id UUID NOT NULL, -- node_id or workload_id
    payload JSONB,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Aggregated node health metrics
CREATE TABLE health_history (
    id BIGSERIAL PRIMARY KEY,
    node_id UUID REFERENCES nodes(id),
    cpu_usage FLOAT,
    memory_usage FLOAT,
    thermal_state TEXT,
    reported_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Audit log
CREATE TABLE audit_log (
    id BIGSERIAL PRIMARY KEY,
    operator_id TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT,
    target_id TEXT,
    changes JSONB,
    occurred_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indices for performance
CREATE INDEX idx_nodes_state ON nodes(state);
CREATE INDEX idx_events_occurred_at ON events(occurred_at);
CREATE INDEX idx_health_history_node_reported ON health_history(node_id, reported_at);
