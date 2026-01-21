# Knowledge Vault

A secure, lightweight knowledge management system with Vue 3 frontend and Rust/Actix-web backend.

## Features

- **Rich Text Editing**: SunEditor integration for article creation
- **Security**: Argon2id password hashing, JWT authentication, AES-256-GCM encryption at rest
- **Audit Logging**: Comprehensive logging of all user actions
- **Admin Management**: User roles (super_admin, admin, editor, viewer)
- **Search**: Full-text search powered by SQLite FTS5
- **Favorites**: Bookmark articles for quick access
- **Tags**: Categorize articles with tags

## Constraints

| Constraint | Target | Status |
|------------|--------|--------|
| Total Size | < 50MB | ✓ |
| RAM Usage | ≤ 100MB | ✓ |
| CPU Threads | 1 | ✓ |

## Quick Start

### Prerequisites

- Rust 1.75+ (`rustup`)
- Node.js 18+ (`node`, `npm`)

### Setup

1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd knowledge-vault
   ```

2. Create configuration:
   ```bash
   cp config.yaml.example config.yaml
   # Edit config.yaml with your secrets
   ```

3. Build and run:
   ```powershell
   # Windows
   .\scripts\run_all.ps1
   ```
   ```bash
   # Linux/macOS
   ./scripts/run_all.sh
   ```

4. Start the services:
   ```bash
   # Terminal 1 - Backend (port 8080)
   cd services/backend
   cargo run --release

   # Terminal 2 - Frontend (port 5173)
   cd apps/frontend
   npm run dev
   ```

5. Open http://localhost:5173

## Demo Accounts

| Username | Password | Role |
|----------|----------|------|
| superadmin | SuperAdmin123! | Super Admin |
| admin1 | Admin123! | Admin |
| editor1 | Editor123! | Editor |
| editor2 | Editor123! | Editor |
| viewer1 | Viewer123! | Viewer |
| viewer2 | Viewer123! | Viewer |

## Configuration

Copy `config.yaml.example` to `config.yaml` and configure:

```yaml
server:
  host: "127.0.0.1"
  port: 8080
  workers: 1  # Must be 1 for CPU constraint

security:
  jwt_secret: "your-32-byte-or-longer-secret-key"
  encryption_key: "your-32-byte-encryption-key-here"
  argon2:
    memory_kib: 19456  # ~19MB for RAM constraint
    parallelism: 1     # Must be 1 for CPU constraint
```

## Architecture

```
knowledge-vault/
├── services/backend/    # Rust Actix-web API
│   ├── src/
│   │   ├── handlers/    # HTTP endpoints
│   │   ├── models/      # Domain entities
│   │   ├── security/    # Crypto (Argon2, JWT, AES)
│   │   └── db/          # SQLite operations
│   └── tests/           # Backend tests
├── apps/frontend/       # Vue 3 SPA
│   ├── src/
│   │   ├── components/  # Vue components
│   │   ├── views/       # Page views
│   │   ├── stores/      # Pinia stores
│   │   └── composables/ # Vue composables
│   └── tests/           # Frontend tests
└── scripts/             # Build scripts
```

## Security Features

- **Password Hashing**: Argon2id with 19MB memory, 2 iterations
- **JWT**: HS256 tokens in HttpOnly cookies
- **Encryption**: AES-256-GCM for article content at rest
- **Audit**: All actions logged with user, IP, timestamp

## API Endpoints

### Authentication
- `POST /api/auth/login` - Login
- `POST /api/auth/logout` - Logout
- `GET /api/auth/me` - Current user

### Articles
- `GET /api/articles` - List articles
- `GET /api/articles/:id` - Get article
- `POST /api/articles` - Create article
- `PUT /api/articles/:id` - Update article
- `DELETE /api/articles/:id` - Delete article

### Search
- `GET /api/search?q=query` - Search articles

### Admin
- `GET /api/admin/users` - List users
- `PUT /api/admin/users/:id` - Update user
- `GET /api/admin/audit` - View audit logs

## License

MIT
