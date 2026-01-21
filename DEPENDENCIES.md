# Dependencies

## Backend (Rust)

### Core
| Crate | Version | Purpose |
|-------|---------|---------|
| actix-web | 4.4 | Web framework |
| actix-cors | 0.7 | CORS handling |
| tokio | 1.35 | Async runtime |

### Database
| Crate | Version | Purpose |
|-------|---------|---------|
| rusqlite | 0.30 | SQLite bindings |
| r2d2 | 0.8 | Connection pooling |
| r2d2_sqlite | 0.24 | SQLite pool adapter |

### Security
| Crate | Version | Purpose |
|-------|---------|---------|
| argon2 | 0.5 | Password hashing |
| jsonwebtoken | 9.2 | JWT tokens |
| aes-gcm | 0.10 | AES-256-GCM encryption |
| hkdf | 0.12 | Key derivation |
| sha2 | 0.10 | SHA-256 hashing |

### Serialization
| Crate | Version | Purpose |
|-------|---------|---------|
| serde | 1.0 | Serialization framework |
| serde_json | 1.0 | JSON support |
| serde_yaml | 0.9 | YAML config parsing |

### Utilities
| Crate | Version | Purpose |
|-------|---------|---------|
| uuid | 1.6 | UUID generation |
| chrono | 0.4 | Date/time handling |
| rand | 0.8 | Random generation |
| tracing | 0.1 | Logging |
| thiserror | 1.0 | Error types |
| validator | 0.16 | Input validation |

### Install

```bash
cd services/backend
cargo build --release
```

## Frontend (Node.js)

### Core
| Package | Version | Purpose |
|---------|---------|---------|
| vue | ^3.4.0 | UI framework |
| vue-router | ^4.2.5 | Routing |
| pinia | ^2.1.7 | State management |
| suneditor | ^2.45.1 | Rich text editor |
| @vueuse/core | ^10.7.0 | Vue utilities |

### Development
| Package | Version | Purpose |
|---------|---------|---------|
| vite | ^5.0.10 | Build tool |
| typescript | ^5.3.3 | Type checking |
| vitest | ^1.1.0 | Testing |
| @vue/test-utils | ^2.4.3 | Vue testing |

### Install

```bash
cd apps/frontend
npm install
```

Or with pnpm:
```bash
pnpm install
```

## System Requirements

- **Rust**: 1.75 or later
- **Node.js**: 18 or later
- **npm**: 9 or later (or pnpm 8+)
- **SQLite**: Bundled with rusqlite

## Security Notes

- All crypto libraries use well-audited implementations
- Argon2id is the recommended password hashing algorithm
- AES-256-GCM provides authenticated encryption
- JWT tokens use HS256 (symmetric) for simplicity
