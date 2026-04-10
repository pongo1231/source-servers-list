# source-servers-list

Rust backend & frontend for aggregating the status output of source & goldsource servers and serving them in a list.
Written for DuckyServers https://servers.ecmec.eu/.

Only tested with Team Fortress 2 and Sven Coop.

## Setup

1. Copy the example configuration files:
   - `.env.example` → `.env`
   - `servers.yaml.example` → `servers.yaml`

2. Update files with your values.

3. Run.

## Development Environment

This project includes a **nix-direnv** setup for reproducible development.

```bash
direnv allow
```

All required dependencies will be provided automatically via Nix.

## Build & Run

### Standard build (debug or release)

```bash
cargo b
```

This is sufficient for both development and normal usage.

### Compressed release build

```bash
just release [target]
```

Builds a release binary and compresses it (via UPX).
