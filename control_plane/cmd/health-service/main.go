package main

import (
	"context"
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/exile-platform/exile/control_plane/internal/health"
	"github.com/exile-platform/exile/control_plane/internal/storage"
	"github.com/nats-io/nats.go"
)

func main() {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "postgres://postgres:password@localhost:5432/exile?sslmode=disable"
	}

	natsURL := os.Getenv("NATS_URL")
	if natsURL == "" {
		natsURL = "nats://localhost:4222"
	}

	store, err := storage.NewStore(dbURL)
	if err != nil {
		log.Fatalf("failed to connect to storage: %v", err)
	}

	healthSvc := health.NewService(store)

	nc, err := nats.Connect(natsURL)
	if err != nil {
		log.Fatalf("failed to connect to NATS: %v", err)
	}
	defer nc.Close()

	// Subscribe to heartbeat events
	// platform.nodes.*.heartbeat (Subject Pattern)
	_, err = nc.Subscribe("platform.nodes.*.heartbeat", func(m *nats.Msg) {
		nodeID := string(m.Data) // assuming data is just the node UUID (THis is the Simplified Logic )
		if err := healthSvc.ProcessHeartbeat(context.Background(), nodeID); err != nil {
			log.Printf("failed to process heartbeat for node %s: %v", nodeID, err)
		}
	})
	if err != nil {
		log.Fatalf("failed to subscribe to heartbeats: %v", err)
	}

	log.Println("Health Service starting, listening for heartbeats on NATS...")

	// Wait for termination
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	<-sigCh
}
