package workload

import (
	"context"
	"fmt"

	"github.com/google/uuid"
)

type Storage interface {
	SaveWorkloadDefinition(ctx context.Context, wd *WorkloadDefinition) error
	GetWorkloadDefinition(ctx context.Context, id uuid.UUID) (*WorkloadDefinition, error)
	CreatePlacement(ctx context.Context, nodeID, workloadID uuid.UUID) error
	UpdatePlacementState(ctx context.Context, nodeID, workloadID uuid.UUID, state string) error
}

type Messaging interface {
	IssueWorkloadIntent(ctx context.Context, nodeID uuid.UUID, operation string, wd *WorkloadDefinition) error
}

type Service struct {
	storage   Storage
	messaging Messaging
}

func NewService(storage Storage, messaging Messaging) *Service {
	return &Service{
		storage:   storage,
		messaging: messaging,
	}
}

func (s *Service) CreateDefinition(ctx context.Context, wd *WorkloadDefinition) error {
	if wd.ID == uuid.Nil {
		wd.ID = uuid.New()
	}
	return s.storage.SaveWorkloadDefinition(ctx, wd)
}

func (s *Service) Deploy(ctx context.Context, nodeID, workloadID uuid.UUID) error {
	wd, err := s.storage.GetWorkloadDefinition(ctx, workloadID)
	if err != nil {
		return fmt.Errorf("failed to get workload definition: %w", err)
	}

	if err := s.storage.CreatePlacement(ctx, nodeID, workloadID); err != nil {
		return fmt.Errorf("failed to create placement: %w", err)
	}

	if err := s.messaging.IssueWorkloadIntent(ctx, nodeID, "deploy", wd); err != nil {
		return fmt.Errorf("failed to issue deploy intent: %w", err)
	}

	return s.storage.UpdatePlacementState(ctx, nodeID, workloadID, "deploying")
}

func (s *Service) Start(ctx context.Context, nodeID, workloadID uuid.UUID) error {
	wd, err := s.storage.GetWorkloadDefinition(ctx, workloadID)
	if err != nil {
		return fmt.Errorf("failed to get workload definition: %w", err)
	}

	if err := s.messaging.IssueWorkloadIntent(ctx, nodeID, "start", wd); err != nil {
		return fmt.Errorf("failed to issue start intent: %w", err)
	}

	return s.storage.UpdatePlacementState(ctx, nodeID, workloadID, "starting")
}

func (s *Service) Stop(ctx context.Context, nodeID, workloadID uuid.UUID) error {
	wd, err := s.storage.GetWorkloadDefinition(ctx, workloadID)
	if err != nil {
		return fmt.Errorf("failed to get workload definition: %w", err)
	}

	if err := s.messaging.IssueWorkloadIntent(ctx, nodeID, "stop", wd); err != nil {
		return fmt.Errorf("failed to issue stop intent: %w", err)
	}

	return s.storage.UpdatePlacementState(ctx, nodeID, workloadID, "stopping")
}

func (s *Service) Restart(ctx context.Context, nodeID, workloadID uuid.UUID) error {
	wd, err := s.storage.GetWorkloadDefinition(ctx, workloadID)
	if err != nil {
		return fmt.Errorf("failed to get workload definition: %w", err)
	}

	if err := s.messaging.IssueWorkloadIntent(ctx, nodeID, "restart", wd); err != nil {
		return fmt.Errorf("failed to issue restart intent: %w", err)
	}

	return s.storage.UpdatePlacementState(ctx, nodeID, workloadID, "restarting")
}

func (s *Service) Reconfigure(ctx context.Context, nodeID, workloadID uuid.UUID, configTemplate string) error {
	wd, err := s.storage.GetWorkloadDefinition(ctx, workloadID)
	if err != nil {
		return fmt.Errorf("failed to get workload definition: %w", err)
	}

	wd.ConfigTemplate = configTemplate

	if err := s.messaging.IssueWorkloadIntent(ctx, nodeID, "reconfigure", wd); err != nil {
		return fmt.Errorf("failed to issue reconfigure intent: %w", err)
	}

	return s.storage.UpdatePlacementState(ctx, nodeID, workloadID, "reconfiguring")
}
