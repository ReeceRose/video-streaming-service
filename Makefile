# Variables
DOCKER_COMPOSE_PROD_FILE := compose-dev.yml
DOCKER_COMPOSE_DEV_FILE := compose-dev.yml
DOCKER_COMPOSE_PROD := docker compose -f $(DOCKER_COMPOSE_PROD_FILE)
DOCKER_COMPOSE__DEV := docker compose -f $(DOCKER_COMPOSE_DEV_FILE)


# Targets
.PHONY: all build test dev-build dev dev-down prod prod-down clean

all: build

# Build all services
build:
	@echo "Building all services..."
	cargo build --release

# Run all tests
test:
	@echo "Running tests..."
	cargo test

dev-build:
	@echo "Starting services in development mode..."
	$(DOCKER_COMPOSE_DEV) up --build

dev:
	@echo "Starting services in development mode..."
	$(DOCKER_COMPOSE_DEV) up

dev-down:
	@echo "Stopping services in development mode..."
	$(DOCKER_COMPOSE_DEV) down

prod:
	@echo "Starting services in production mode..."
	$(DOCKER_COMPOSE_PROD) up --build

prod-down:
	@echo "Stopping services in production mode..."
	$(DOCKER_COMPOSE_PROD) down

clean:
	@echo "Cleaning up build artifacts..."
	cargo clean
	@echo "Removing Docker volumes..."
	docker volume prune -f
	@echo "Removing Docker images..."
	docker image prune -a -f
