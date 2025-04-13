#!/bin/bash

set -e

INSTALL_DIR="/usr/local/bin"
FRONTEND_DIR="$INSTALL_DIR/frontend"
BACKEND_DIR="$INSTALL_DIR/backend"

echo "📁 Creando directorio de instalación en $INSTALL_DIR..."
sudo mkdir -p $INSTALL_DIR
sudo cp NetHound/target/release/NetHound $INSTALL_DIR
sudo cp Firewall/target/release/firewall $INSTALL_DIR/nethound/
sudo cp PacketAnalyzer/target/release/PacketAnalyzer $INSTALL_DIR/nethound/

echo "📁 Copiando backend..."
sudo mkdir -p $BACKEND_DIR
sudo cp -r backend/dist $BACKEND_DIR/
sudo cp backend/package.json backend/package-lock.json $BACKEND_DIR/

echo "📁 Copiando frontend..."
sudo mkdir -p $FRONTEND_DIR
sudo cp -r frontend/dist $FRONTEND_DIR/

echo "✅ Instalación completada en $INSTALL_DIR"
echo "Puedes iniciar con: sudo $INSTALL_DIR/nethound start"
