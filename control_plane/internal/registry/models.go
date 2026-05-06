package registry

import (
	"time"

	"github.com/google/uuid"
)

type NodeState string

const (
	StateUnregistered  NodeState = "unregistered"
	StateRegistered    NodeState = "registered"
	StateHealthy       NodeState = "healthy"
	StateDegraded      NodeState = "degraded"
	StateQuarantined   NodeState = "quarantined"
	StateDecommissioned NodeState = "decommissioned"
)

type Node struct {
	ID                uuid.UUID `json:"id"`
	Hostname          string    `json:"hostname"`
	IPAddress         string    `json:"ip_address"`
	OSFamily          string    `json:"os_family"`
	OSArch            string    `json:"os_arch"`
	State             NodeState `json:"state"`
	CertificateSerial string    `json:"certificate_serial"`
	LastHeartbeat     time.Time `json:"last_heartbeat"`
	CreatedAt         time.Time `json:"created_at"`
	UpdatedAt         time.Time `json:"updated_at"`
}

type RegistrationRequest struct {
	Hostname  string `json:"hostname"`
	IPAddress string `json:"ip_address"`
	OSFamily  string `json:"os_family"`
	OSArch    string `json:"os_arch"`
}
