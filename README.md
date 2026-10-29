# Swarm Builder V2
Bun + Tauri 2 + Svelte 5 + ZMQ ROUTER p2p + whisper.cpp + blake3

bun install
bunx tauri dev

Models cache: ~/Library/Application Support/swarm-builder/models or ~/.local/share/swarm-builder/models
ZeroMQ: ROUTER on 5556, connect to peers via tcp://IP:PORT
blake3: every mesh payload hashed
