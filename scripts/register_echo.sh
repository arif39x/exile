#!/bin/bash

curl -X POST http://localhost:8081/workloads/define \
  -H "Content-Type: application/json" \
  -d '{
    "name": "echo-service",
    "version": "1.0.0",
    "artifact_ref": "local://echo",
    "artifact_hash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    "resource_requirements": {
      "cpu_cores": 0.1,
      "memory_mb": 64,
      "os": "linux",
      "arch": "x86_64"
    },
    "health_check": {
      "type": "process",
      "interval_seconds": 10,
      "failure_threshold": 2
    },
    "log_format": "text"
  }'
