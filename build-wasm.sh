#!/bin/bash
# Build script for WASM package

set -e

echo "Building Damdara WASM package..."

# Development build (faster, larger)
if [ "$1" == "dev" ]; then
    echo "Building in development mode..."
    wasm-pack build --target web --out-dir pkg --dev
else
    # Production build (optimized for size)
    echo "Building in release mode (optimized for size)..."
    wasm-pack build --target web --out-dir pkg
fi

echo "✅ WASM build complete!"
echo "📦 Package output: ./pkg/"
echo ""
echo "To test the package:"
echo "  cd examples/web && python -m http.server 8080"
