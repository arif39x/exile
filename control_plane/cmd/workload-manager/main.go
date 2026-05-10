package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/exile-platform/exile/control_plane/internal/messaging"
	"github.com/exile-platform/exile/control_plane/internal/storage"
	"github.com/exile-platform/exile/control_plane/internal/workload"
	"github.com/google/uuid"
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

	workloadSvc := workload.NewService(store, natsClient)

	http.HandleFunc("/workloads/define", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var wd workload.WorkloadDefinition
		if err := json.NewDecoder(r.Body).Decode(&wd); err != nil {
			http.Error(w, "Invalid request", http.StatusBadRequest)
			return
		}

		if err := workloadSvc.CreateDefinition(r.Context(), &wd); err != nil {
			http.Error(w, fmt.Sprintf("Failed to create workload: %v", err), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(wd)
	})

	http.HandleFunc("/workloads/deploy", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		nodeIDStr := r.URL.Query().Get("node_id")
		workloadIDStr := r.URL.Query().Get("workload_id")

		nodeID, err := uuid.Parse(nodeIDStr)
		if err != nil {
			http.Error(w, "Invalid node_id", http.StatusBadRequest)
			return
		}
		workloadID, err := uuid.Parse(workloadIDStr)
		if err != nil {
			http.Error(w, "Invalid workload_id", http.StatusBadRequest)
			return
		}

		if err := workloadSvc.Deploy(r.Context(), nodeID, workloadID); err != nil {
			http.Error(w, fmt.Sprintf("Failed to deploy: %v", err), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusAccepted)
	})

	http.HandleFunc("/workloads/start", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		nodeIDStr := r.URL.Query().Get("node_id")
		workloadIDStr := r.URL.Query().Get("workload_id")

		nodeID, _ := uuid.Parse(nodeIDStr)
		workloadID, _ := uuid.Parse(workloadIDStr)

		if err := workloadSvc.Start(r.Context(), nodeID, workloadID); err != nil {
			http.Error(w, fmt.Sprintf("Failed to start: %v", err), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusAccepted)
	})

	http.HandleFunc("/workloads/stop", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		nodeIDStr := r.URL.Query().Get("node_id")
		workloadIDStr := r.URL.Query().Get("workload_id")

		nodeID, _ := uuid.Parse(nodeIDStr)
		workloadID, _ := uuid.Parse(workloadIDStr)

		if err := workloadSvc.Stop(r.Context(), nodeID, workloadID); err != nil {
			http.Error(w, fmt.Sprintf("Failed to stop: %v", err), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusAccepted)
	})

	log.Println("Workload Manager Service starting on :8081...")
	if err := http.ListenAndServe(":8081", nil); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
