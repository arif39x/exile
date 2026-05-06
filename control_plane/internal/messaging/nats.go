package messaging

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/nats-io/nats.go"
	"github.com/exile-platform/exile/control_plane/internal/registry"
)

type NATSClient struct {
	nc *nats.Conn
	js nats.JetStreamContext
}

func NewNATSClient(url string) (*NATSClient, error) {
	nc, err := nats.Connect(url)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to NATS: %w", err)
	}

	js, err := nc.JetStream()
	if err != nil {
		return nil, fmt.Errorf("failed to create JetStream context: %w", err)
	}

	return &NATSClient{nc: nc, js: js}, nil
}

func (n *NATSClient) PublishNodeEvent(ctx context.Context, eventType string, node *registry.Node) error {
	subject := fmt.Sprintf("platform.nodes.%s.%s", node.ID.String(), eventType)
	data, err := json.Marshal(node)
	if err != nil {
		return fmt.Errorf("failed to marshal node: %w", err)
	}

	_, err = n.js.Publish(subject, data)
	return err
}

func (n *NATSClient) Close() {
	n.nc.Close()
}
