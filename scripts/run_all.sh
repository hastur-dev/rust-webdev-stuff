#!/usr/bin/env bash
# Knowledge Vault - Run All Script (POSIX)
# Builds and runs all components

set -euo pipefail

echo "[run_all] Knowledge Vault Build & Test"
echo "========================================"

# Check toolchains
echo -e "\n[1/6] Verifying toolchains..."
cargo --version || { echo "Rust not found!"; exit 1; }
node -v || { echo "Node.js not found!"; exit 1; }

# Backend build and test
echo -e "\n[2/6] Building backend..."
pushd services/backend > /dev/null

# Create config for testing
if [ ! -f "config.yaml" ]; then
    cp config.yaml.example config.yaml
    echo "  Created config.yaml from example"
fi

export RUSTFLAGS="-D warnings"
cargo build --release
echo "  Backend built successfully"

echo -e "\n[3/6] Running backend tests..."
cargo test --all-features
echo "  Backend tests passed"

echo -e "\n[4/6] Running clippy..."
cargo clippy --all-targets -- -D warnings
echo "  Clippy passed"

popd > /dev/null

# Frontend build and test
echo -e "\n[5/6] Building frontend..."
pushd apps/frontend > /dev/null

if [ ! -d "node_modules" ]; then
    echo "  Installing dependencies..."
    npm install
fi

echo "  Running type check..."
npm run type-check

echo "  Running tests..."
npm run test -- --run
echo "  Frontend tests passed"

echo "  Building production bundle..."
npm run build
echo "  Frontend built successfully"

popd > /dev/null

# Size check
echo -e "\n[6/6] Checking sizes..."
BACKEND_SIZE=$(du -sh services/backend/target/release/knowledge-vault 2>/dev/null | cut -f1 || echo "N/A")
FRONTEND_SIZE=$(du -sh apps/frontend/dist 2>/dev/null | cut -f1 || echo "N/A")

echo "  Backend binary: $BACKEND_SIZE"
echo "  Frontend dist: $FRONTEND_SIZE"

echo -e "\n========================================"
echo "[run_all] All checks passed!"
echo -e "\nTo start the application:"
echo "  Backend:  cd services/backend && cargo run --release"
echo "  Frontend: cd apps/frontend && npm run dev"
