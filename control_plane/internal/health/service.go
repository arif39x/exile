package health

import (
	"context"
	"fmt"
)

type Storage interface {
	UpdateHeartbeat(ctx context.Context, id string) error
}

type Service struct {
	storage Storage
}

func NewService(storage Storage) *Service {
	return &Service{
		storage: storage,
	}
}

func (s *Service) ProcessHeartbeat(ctx context.Context, nodeID string) error {
	if err := s.storage.UpdateHeartbeat(ctx, nodeID); err != nil {
		return fmt.Errorf("failed to update heartbeat: %w", err)
	}
	return nil
}
