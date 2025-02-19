# Check to see if we can use ash, in Alpine images, or default to BASH.
SHELL_PATH = /bin/ash
SHELL = $(if $(wildcard $(SHELL_PATH)),/bin/ash,/bin/bash)

# ==============================================================================
# Define dependencies

RUST            := rust:1.84.1-alpine3.21
POSTGRES        := postgres:17.3

SERVER_APP      := server
BASE_IMAGE_NAME := localhost/charlieroth
VERSION         := 0.1.0
SERVER_IMAGE    := $(BASE_IMAGE_NAME)/$(SERVER_APP):$(VERSION)

# ==============================================================================
# Install dependencies

dev-rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

dev-brew:
	brew update
	brew list pgcli || brew install pgcli
	brew list watch || brew install watch

dev-docker:
	docker pull $(RUST) & \
	docker pull $(POSTGRES) & \
	wait;

# ==============================================================================
# Building containers

build: server

server:
	docker build \
		-f zarf/docker/Dockerfile.server \
		-t $(SERVER_IMAGE) \
		--build-arg BUILD_REF=$(VERSION) \
		--build-arg BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ") \
		.

# ==============================================================================
# Docker Compose

compose-db-up:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile db up -d

compose-db-down:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile db down

compose-db-logs:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile db logs

compose-db-build-up: build compose-db-up

compose-server-up:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile server up -d

compose-server-build-up: build compose-server-up

compose-server-down:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile server down

compose-server-logs:
	cd ./zarf/compose/ && docker compose -f docker-compose.yaml --profile server logs

# ==============================================================================
# Administration

pgcli:
	pgcli postgresql://postgres:postgres@localhost

liveness:
	curl -i http://localhost:8080/liveness

readiness:
	curl -i http://localhost:8080/readiness

# ==============================================================================
# Local Development

run:
	cargo run --bin server	

ready:
	curl -i http://localhost:8080/readiness

live:
	curl -i http://localhost:8080/liveness