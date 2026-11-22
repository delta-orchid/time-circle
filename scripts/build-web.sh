#!/usr/bin/env bash
set -euo pipefail

# Ensure rustup and cargo are in PATH
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "Building Time Circle WebAssembly bundle with Trunk..."
trunk build --release

echo ""
echo "Done! Production assets generated in dist/:"
ls -lh dist/
echo ""
echo "To test locally: trunk serve --release"
echo "To deploy to Vercel: npx vercel deploy dist --prod"
