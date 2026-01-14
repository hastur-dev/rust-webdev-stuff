# Knowledge Vault Frontend

Vue 3 SPA for Knowledge Vault.

## Features

- Vue 3 with Composition API
- TypeScript for type safety
- Pinia for state management
- SunEditor for rich text editing
- Responsive design

## Setup

1. Install dependencies:
   ```bash
   npm install
   ```

2. Development server:
   ```bash
   npm run dev
   ```

3. Production build:
   ```bash
   npm run build
   ```

## Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start dev server (port 5173) |
| `npm run build` | Production build |
| `npm run preview` | Preview production build |
| `npm run test` | Run tests |
| `npm run type-check` | TypeScript check |

## Structure

```
src/
├── components/    # Reusable components
│   ├── articles/  # Article components
│   ├── layout/    # Layout components
│   ├── search/    # Search components
│   └── common/    # Shared components
├── views/         # Page views
├── stores/        # Pinia stores
├── composables/   # Vue composables
├── router/        # Vue Router config
└── types/         # TypeScript types
```

## Configuration

The frontend proxies `/api` requests to `http://localhost:8080` in development.

For production, configure your web server to proxy API requests.
