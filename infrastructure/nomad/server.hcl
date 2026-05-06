# Nomad Server Configuration
data_dir = "/var/lib/nomad"
bind_addr = "0.0.0.0"

server {
    enabled          = true
    bootstrap_expect = 3
}

tls {
    http = true
    rpc  = true

    ca_file   = "/etc/nomad.d/certs/ca.crt"
    cert_file = "/etc/nomad.d/certs/server.crt"
    key_file  = "/etc/nomad.d/certs/server.key"

    verify_server_hostname = true
    verify_https_client    = true
}

vault {
    enabled = true
    address = "https://vault.service.consul:8200"
}

consul {
    address = "127.0.0.1:8500"
}
