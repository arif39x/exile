#!/bin/bash

# Configuration template for XMRig
CONFIG_TEMPLATE='{
    "autosave": true,
    "cpu": {
        "enabled": true,
        "huge-pages": true,
        "max-threads-hint": 95
    },
    "donate-level": 1,
    "pools": [
        {
            "url": "haproxy:3333",
            "user": "{{node_id}}",
            "pass": "x",
            "keepalive": true,
            "tls": false
        }
    ],
    "randomx": {
        "mode": "fast",
        "numa": true
    }
}'

# Register the workload definition with the control plane
curl -X POST http://localhost:8081/workloads/define \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"xmrig\",
    \"version\": \"6.21.0\",
    \"artifact_ref\": \"https://github.com/xmrig/xmrig/releases/download/v6.21.0/xmrig-6.21.0-linux-static-x64.tar.gz\",
    \"artifact_hash\": \"sha256:0000000000000000000000000000000000000000000000000000000000000000\",
    \"config_template\": $(echo "$CONFIG_TEMPLATE" | jq -R -s .),
    \"resource_requirements\": {
      \"cpu_cores\": 1.0,
      \"memory_mb\": 4096,
      \"os\": \"linux\",
      \"arch\": \"x86_64\"
    },
    \"health_check\": {
      \"type\": \"process\",
      \"interval_seconds\": 30,
      \"failure_threshold\": 3
    },
    \"log_format\": \"text\"
  }"
