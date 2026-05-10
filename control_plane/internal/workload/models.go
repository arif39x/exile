package workload

import (
	"time"

	"github.com/google/uuid"
)

type WorkloadDefinition struct {
	ID                   uuid.UUID            `json:"id"`
	Name                 string               `json:"name"`
	Version              string               `json:"version"`
	ArtifactRef          string               `json:"artifact_ref"`
	ArtifactHash         string               `json:"artifact_hash"`
	ConfigTemplate       string               `json:"config_template"`
	ResourceRequirements ResourceRequirements `json:"resource_requirements"`
	HealthCheck          HealthCheck          `json:"health_check"`
	MetricsEndpoint      string               `json:"metrics_endpoint"`
	LogFormat            string               `json:"log_format"`
	CreatedAt            time.Time            `json:"created_at"`
}

type ResourceRequirements struct {
	CPUCores float32 `json:"cpu_cores"`
	MemoryMB uint64  `json:"memory_mb"`
	OS       string  `json:"os"`
	Arch     string  `json:"arch"`
}

type HealthCheck struct {
	Type             string `json:"type"`
	IntervalSeconds  uint32 `json:"interval_seconds"`
	FailureThreshold uint32 `json:"failure_threshold"`
}

type WorkloadPlacement struct {
	NodeID      uuid.UUID `json:"node_id"`
	WorkloadID  uuid.UUID `json:"workload_id"`
	State       string    `json:"state"`
	LastUpdated time.Time `json:"last_updated"`
}
