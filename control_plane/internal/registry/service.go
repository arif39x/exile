package registry

import (
	"context"
	"fmt"

	"github.com/google/uuid"
)

type Storage interface {
	RegisterNode(ctx context.Context, node *Node) error
	GetNode(ctx context.Context, id string) (*Node, error)
}

type Messaging interface {
	PublishNodeEvent(ctx context.Context, eventType string, node *Node) error
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

func (s *Service) Register(ctx context.Context, req RegistrationRequest) (*Node, error) {
	node := &Node{
		ID:        uuid.New(),
		Hostname:  req.Hostname,
		IPAddress: req.IPAddress,
		OSFamily:  req.OSFamily,
		OSArch:    req.OSArch,
		State:     StateRegistered,
	}

	if err := s.storage.RegisterNode(ctx, node); err != nil {
		return nil, fmt.Errorf("failed to store node: %w", err)
	}

	if err := s.messaging.PublishNodeEvent(ctx, "node.registered", node); err != nil {
		fmt.Printf("failed to publish registration event: %v\n", err) //log the error but don't fail registration as the state is persisted
	}

	return node, nil
}
