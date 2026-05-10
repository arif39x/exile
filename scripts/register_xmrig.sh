#!/bin/bash

curl -X POST http://localhost:8081/workloads/define \
  -H "Content-Type: application/json" \
  -d '{
    "name": "xmrig",
    "version": "6.21.0",
    "artifact_ref": "https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-static-x64.tar.gz",
    "artifact_hash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    "resource_requirements": {
      "cpu_cores": 1.0,
      "memory_mb": 1024,
      "os": "linux",
      "arch": "x86_64"
    },
    "health_check": {
      "type": "process",
      "interval_seconds": 30,
      "failure_threshold": 3
    },
    "log_format": "text"
  }'
