package storage

import (
	"context"
	"database/sql"
	"fmt"

	_ "github.com/lib/pq"
	"github.com/exile-platform/exile/control_plane/internal/registry"
)

type Store struct {
	db *sql.DB
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
