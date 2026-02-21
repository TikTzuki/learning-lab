```
APP__HTTP__ADDRESS='0.0.0.0:8081' APP__WEBSOCKET__PORT=3001 RUST_LOG=info cargo run

docker build -t tiktuzki/mdns_rr -f mdns_rr/Dockerfile .

docker compose -f mdns_rr/docker-compose.yaml up 

docker compose -f mdns_rr/docker-compose.yaml stop mdns-rr-node1

docker compose -f mdns_rr/docker-compose.yaml start mdns-rr-node1
```
