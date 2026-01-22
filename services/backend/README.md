# Knowledge Vault Backend

Rust/Actix-web API server for Knowledge Vault.

## Features

- Single-threaded for resource constraints (1 CPU, 100MB RAM)
- SQLite database with FTS5 full-text search
- AES-256-GCM encryption for article content
- Argon2id password hashing
- JWT authentication with HttpOnly cookies
- Comprehensive audit logging

## Setup

1. Copy configuration:
   ```bash
   cp config.yaml.example config.yaml
   ```

2. Edit `config.yaml` with secure secrets

3. Build:
   ```bash
   cargo build --release
   ```

4. Run:
   ```bash
   cargo run --release
   ```

## Configuration

Key settings in `config.yaml`:

- `server.workers`: Must be 1 for CPU constraint
- `security.argon2.memory_kib`: ~19MB for RAM constraint
- `security.argon2.parallelism`: Must be 1 for CPU constraint

## API

Server runs on `http://localhost:8080` by default.

See main README for endpoint documentation.

## Testing

```bash
cargo test --all-features
cargo clippy --all-targets -- -D warnings
```

## Security

- Passwords: Argon2id with configurable memory/iterations
- Tokens: JWT with HS256, HttpOnly cookies
- Content: AES-256-GCM encryption at rest
- Audit: All actions logged to database
