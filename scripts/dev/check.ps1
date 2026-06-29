$ErrorActionPreference = "Stop"

cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace

if (Test-Path "apps/desktop/node_modules") {
  npm --prefix apps/desktop run typecheck
} else {
  Write-Host "Skipping desktop typecheck because apps/desktop/node_modules is not installed."
  Write-Host "Run: npm --prefix apps/desktop install"
}

