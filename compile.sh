#!/bin/bash

set -e

echo "🔧 Compilando binarios en Rust..."

cargo build --release --manifest-path NetHound/Cargo.toml
cargo build --release --manifest-path Firewall/Cargo.toml
cargo build --release --manifest-path PacketAnalyzer/Cargo.toml

echo "🛠️ Instalando dependencias de backend (Node.js)..."
cd backend
npm install
npm run build
cd ..

echo "🌐 Compilando frontend (React)..."
cd frontend
npm install
npm run build
cd ..

echo "✅ Ejecuta: sudo ./install.sh para instalar los binarios"