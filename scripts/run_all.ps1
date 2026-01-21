# Knowledge Vault - Run All Script (Windows PowerShell)
# Builds and runs all components

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Write-Host "[run_all] Knowledge Vault Build & Test" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Check toolchains
Write-Host "`n[1/6] Verifying toolchains..." -ForegroundColor Yellow

try { $rustVersion = cargo --version } catch { Write-Host "Rust not found!" -ForegroundColor Red; exit 1 }
Write-Host "  Rust: $rustVersion" -ForegroundColor Green

try { $nodeVersion = node -v } catch { Write-Host "Node.js not found!" -ForegroundColor Red; exit 1 }
Write-Host "  Node.js: $nodeVersion" -ForegroundColor Green

# Backend build and test
Write-Host "`n[2/6] Building backend..." -ForegroundColor Yellow
Push-Location services/backend

# Create config for testing
if (-not (Test-Path "config.yaml")) {
    Copy-Item "config.yaml.example" "config.yaml"
    Write-Host "  Created config.yaml from example" -ForegroundColor Gray
}

$env:RUSTFLAGS = "-D warnings"
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Host "Backend build failed!" -ForegroundColor Red; exit 1 }
Write-Host "  Backend built successfully" -ForegroundColor Green

Write-Host "`n[3/6] Running backend tests..." -ForegroundColor Yellow
cargo test --all-features
if ($LASTEXITCODE -ne 0) { Write-Host "Backend tests failed!" -ForegroundColor Red; exit 1 }
Write-Host "  Backend tests passed" -ForegroundColor Green

Write-Host "`n[4/6] Running clippy..." -ForegroundColor Yellow
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { Write-Host "Clippy warnings!" -ForegroundColor Red; exit 1 }
Write-Host "  Clippy passed" -ForegroundColor Green

Pop-Location

# Frontend build and test
Write-Host "`n[5/6] Building frontend..." -ForegroundColor Yellow
Push-Location apps/frontend

if (-not (Test-Path "node_modules")) {
    Write-Host "  Installing dependencies..." -ForegroundColor Gray
    npm install
}

Write-Host "  Running type check..." -ForegroundColor Gray
npm run type-check
if ($LASTEXITCODE -ne 0) { Write-Host "Type check failed!" -ForegroundColor Red; exit 1 }

Write-Host "  Running tests..." -ForegroundColor Gray
npm run test -- --run
if ($LASTEXITCODE -ne 0) { Write-Host "Frontend tests failed!" -ForegroundColor Red; exit 1 }
Write-Host "  Frontend tests passed" -ForegroundColor Green

Write-Host "  Building production bundle..." -ForegroundColor Gray
npm run build
if ($LASTEXITCODE -ne 0) { Write-Host "Frontend build failed!" -ForegroundColor Red; exit 1 }
Write-Host "  Frontend built successfully" -ForegroundColor Green

Pop-Location

# Size check
Write-Host "`n[6/6] Checking sizes..." -ForegroundColor Yellow
$backendSize = (Get-Item "services/backend/target/release/knowledge-vault.exe" -ErrorAction SilentlyContinue).Length / 1MB
$frontendSize = (Get-ChildItem "apps/frontend/dist" -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB

Write-Host "  Backend binary: $([math]::Round($backendSize, 2)) MB" -ForegroundColor Gray
Write-Host "  Frontend dist: $([math]::Round($frontendSize, 2)) MB" -ForegroundColor Gray

$totalSize = $backendSize + $frontendSize
if ($totalSize -gt 50) {
    Write-Host "  WARNING: Total size ($([math]::Round($totalSize, 2)) MB) exceeds 50MB limit!" -ForegroundColor Red
} else {
    Write-Host "  Total: $([math]::Round($totalSize, 2)) MB (under 50MB limit)" -ForegroundColor Green
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "[run_all] All checks passed!" -ForegroundColor Green
Write-Host "`nTo start the application:" -ForegroundColor Gray
Write-Host "  Backend:  cd services/backend && cargo run --release" -ForegroundColor Gray
Write-Host "  Frontend: cd apps/frontend && npm run dev" -ForegroundColor Gray
