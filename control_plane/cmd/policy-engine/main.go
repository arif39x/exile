package main

import (
	"log"
	"os"
	"os/signal"
	"syscall"

	"github.com/nats-io/nats.go"
)

func main() {
	natsURL := os.Getenv("NATS_URL")
	if natsURL == "" {
		natsURL = "nats://localhost:4222"
	}

	nc, err := nats.Connect(natsURL)
	if err != nil {
		log.Fatalf("failed to connect to NATS: %v", err)
	}
	defer nc.Close()

	log.Println("Policy Engine starting, listening for events...")

	// Listen for security events and trigger quarantine (Example)
	_, err = nc.Subscribe("platform.security.alerts", func(m *nats.Msg) {
		log.Printf("Received security alert: %s. Evaluating quarantine policy...", string(m.Data))
		// Logic to update node state to 'quarantined' With  Registry API
	})

	if err != nil {
		log.Fatalf("failed to subscribe: %v", err)
	}

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)
	<-sigCh
}
