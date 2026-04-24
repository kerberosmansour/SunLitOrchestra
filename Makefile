.PHONY: dev build test test-backend test-frontend setup clean

# Launch the Tauri desktop app in development mode (hot-reload for both Rust + React)
dev:
	cd crates/sldo-tauri && cargo tauri dev

# Build the production desktop app bundle
build:
	cd crates/sldo-tauri/ui && npm run build
	cd crates/sldo-tauri && cargo tauri build

# Run all tests (backend + frontend)
test: test-backend test-frontend

# Run all Rust workspace tests (unit + E2E, 200 tests)
test-backend:
	cargo test --workspace

# Run frontend component and E2E tests (90 tests)
test-frontend:
	cd crates/sldo-tauri/ui && npm test

# First-time setup: install frontend deps + Tauri CLI
setup:
	cd crates/sldo-tauri/ui && npm install
	cargo install tauri-cli --version '^2'

# Build workspace without Tauri bundling (faster iteration)
check:
	cargo build --workspace

# Clean build artifacts
clean:
	cargo clean
	rm -rf crates/sldo-tauri/ui/dist
