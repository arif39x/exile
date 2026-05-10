package storage

import (
	"context"
	"database/sql"
	"fmt"

	_ "github.com/lib/pq"
	"github.com/exile-platform/exile/control_plane/internal/registry"
	"github.com/exile-platform/exile/control_plane/internal/workload"
    "encoding/json"
    "github.com/google/uuid"
)

type Store struct {
	db *sql.DB
}

func (s *Store) SaveWorkloadDefinition(ctx context.Context, wd *workload.WorkloadDefinition) error {
	definition, err := json.Marshal(wd)
	if err != nil {
		return fmt.Errorf("failed to marshal workload definition: %w", err)
	}

	query := `
		INSERT INTO workloads (id, name, version, definition)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (id) DO UPDATE SET
			name = EXCLUDED.name,
			version = EXCLUDED.version,
			definition = EXCLUDED.definition
	`
	_, err = s.db.ExecContext(ctx, query, wd.ID, wd.Name, wd.Version, definition)
	return err
}

func (s *Store) GetWorkloadDefinition(ctx context.Context, id uuid.UUID) (*workload.WorkloadDefinition, error) {
	var wd workload.WorkloadDefinition
	var definition []byte
	query := `SELECT id, name, version, definition, created_at FROM workloads WHERE id = $1`
	err := s.db.QueryRowContext(ctx, query, id).Scan(&wd.ID, &wd.Name, &wd.Version, &definition, &wd.CreatedAt)
	if err != nil {
		return nil, err
	}

	if err := json.Unmarshal(definition, &wd); err != nil {
		return nil, fmt.Errorf("failed to unmarshal workload definition: %w", err)
	}

	return &wd, nil
}

func (s *Store) CreatePlacement(ctx context.Context, nodeID, workloadID uuid.UUID) error {
	query := `
		INSERT INTO placements (node_id, workload_id, state)
		VALUES ($1, $2, $3)
		ON CONFLICT (node_id, workload_id) DO NOTHING
	`
	_, err := s.db.ExecContext(ctx, query, nodeID, workloadID, "pending")
	return err
}

func (s *Store) UpdatePlacementState(ctx context.Context, nodeID, workloadID uuid.UUID, state string) error {
	query := `UPDATE placements SET state = $3, last_updated = CURRENT_TIMESTAMP WHERE node_id = $1 AND workload_id = $2`
	_, err := s.db.ExecContext(ctx, query, nodeID, workloadID, state)
	return err
}


func NewStore(connStr string) (*Store, error) {
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}

	if err := db.Ping(); err != nil {
		return nil, fmt.Errorf("failed to ping database: %w", err)
	}

	return &Store{db: db}, nil
}

func (s *Store) RegisterNode(ctx context.Context, node *registry.Node) error {
	query := `
		INSERT INTO nodes (id, hostname, ip_address, os_family, os_arch, state, certificate_serial)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			hostname = EXCLUDED.hostname,
			ip_address = EXCLUDED.ip_address,
			state = EXCLUDED.state,
			updated_at = CURRENT_TIMESTAMP
	`
	_, err := s.db.ExecContext(ctx, query,
		node.ID, node.Hostname, node.IPAddress, node.OSFamily, node.OSArch, node.State, node.CertificateSerial)
	return err
}

func (s *Store) GetNode(ctx context.Context, id string) (*registry.Node, error) {
	var node registry.Node
	query := `SELECT id, hostname, ip_address, os_family, os_arch, state, certificate_serial, last_heartbeat FROM nodes WHERE id = $1`
	err := s.db.QueryRowContext(ctx, query, id).Scan(
		&node.ID, &node.Hostname, &node.IPAddress, &node.OSFamily, &node.OSArch, &node.State, &node.CertificateSerial, &node.LastHeartbeat)
	if err != nil {
		return nil, err
	}
	return &node, nil
}

func (s *Store) UpdateHeartbeat(ctx context.Context, id string) error {
	query := `UPDATE nodes SET last_heartbeat = CURRENT_TIMESTAMP, state = 'healthy' WHERE id = $1`
	_, err := s.db.ExecContext(ctx, query, id)
	return err
}
