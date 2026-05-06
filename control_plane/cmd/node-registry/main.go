package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/exile-platform/exile/control_plane/internal/messaging"
	"github.com/exile-platform/exile/control_plane/internal/registry"
	"github.com/exile-platform/exile/control_plane/internal/storage"
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

	natsClient, err := messaging.NewNATSClient(natsURL)
	if err != nil {
		log.Fatalf("failed to connect to NATS: %v", err)
	}
	defer natsClient.Close()

	registrySvc := registry.NewService(store, natsClient)

	http.HandleFunc("/register", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req registry.RegistrationRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		node, err := registrySvc.Register(r.Context(), req)
		if err != nil {
			http.Error(w, fmt.Sprintf("Failed to register: %v", err), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(node)
	})

	log.Println("Node Registry Service starting on :8080...")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
