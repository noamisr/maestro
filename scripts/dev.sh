#!/bin/bash
# Maestro development launcher
# Starts the Python sidecar and Tauri dev server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Maestro Development Mode ==="
echo ""

# Start Python sidecar in background
echo "[1/2] Starting Python sidecar..."
cd "$PROJECT_DIR/sidecar"
python -m maestro_sidecar.main &
SIDECAR_PID=$!
echo "  Sidecar PID: $SIDECAR_PID"

# Wait for sidecar to be ready
echo "  Waiting for sidecar health..."
for i in $(seq 1 30); do
    if curl -s http://127.0.0.1:9400/health > /dev/null 2>&1; then
        echo "  Sidecar ready!"
        break
    fi
    sleep 1
done

# Start Tauri dev
echo "[2/2] Starting Tauri dev server..."
cd "$PROJECT_DIR"
npm run tauri dev

# Cleanup on exit
echo "Shutting down sidecar..."
kill $SIDECAR_PID 2>/dev/null || true
