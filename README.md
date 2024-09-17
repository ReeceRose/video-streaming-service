# Video On Demand (VOD)

## Setup

### Developer Dependencies

1. [Rust](https://www.rust-lang.org/)
2. [Docker](https://docs.docker.com/desktop/)
3. [Docker Compose](https://docs.docker.com/compose/install/)
4. [Make](https://www.gnu.org/software/make/)

```bash
# First time
make dev-build
```

## Development

```bash
# Start all containers, local services are run in "watch" mode and will be recompiled when files change.
make dev
# Stop all containers
make dev-down
```
