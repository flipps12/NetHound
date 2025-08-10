# 🐾 NetHound

**NetHound** es una herramienta de análisis y seguridad de red desarrollada en **Rust**, diseñada para capturar, inspeccionar y filtrar tráfico de red en tiempo real.  
Su objetivo principal es servir como plataforma de investigación y demostración tecnológica para entornos educativos, ferias de ciencias y pruebas controladas de ciberseguridad.

## 📌 Características principales

- **Captura de paquetes en tiempo real** con análisis detallado.
- **Firewall inteligente** para bloquear o permitir tráfico según reglas definidas.
- **Arquitectura modular** con servicios independientes:
  - `PacketAnalyzer` → Captura y analiza paquetes.
  - `Firewall` → Aplica reglas dinámicas para filtrar tráfico.
- **Orquestador en Rust** para coordinar todos los módulos.
- **Frontend en React** para visualización de datos y alertas.
- **Backend en Express (TypeScript)** para la comunicación entre el frontend y los servicios de red.
- Integración con `hostapd` e `iptables` para control de acceso por MAC/IP.
- Actualización periódica del estado de los dispositivos conectados.

## 📦 Instalación

```bash
./compile.sh
sudo ./install.sh
sudo /usr/local/bin/NetHound start
````


## 🛠️ Arquitectura

```
┌────────────────┐
│   Orquestador   │  ← Rust
└───────┬─────────┘
        │
        ├─ PacketAnalyzer (Rust)
        │   └─ Captura y analiza paquetes
        │
        ├─ Firewall (Rust)
        │   └─ Aplica reglas de filtrado
        │
        ├─ Backend (Express + TypeScript)
        │   └─ API REST para control y monitoreo
        │
        └─ Frontend (React + TypeScript)
            └─ Interfaz web para visualización y control
```

## 🚀 Objetivo

Proporcionar una herramienta educativa y práctica para:
- Analizar el comportamiento del tráfico en una red.
- Detectar y mitigar amenazas en tiempo real.
- Visualizar y comprender mejor cómo fluye la información a través de una red.

## ⚠️ Uso Responsable

NetHound **NO** está diseñado para usarse en redes ajenas sin autorización.  
Su uso debe limitarse a **entornos controlados**, como laboratorios de pruebas o redes propias, cumpliendo siempre las leyes locales sobre ciberseguridad.

## 📋 Estado del proyecto

✅ Arquitectura base definida  
✅ Servicios principales (PacketAnalyzer y Firewall) en desarrollo  
✅ Orquestador en Rust implementado parcialmente  
✅ Backend y Frontend en fase de integración  
🔄 Mejora de la interfaz web  
🔄 Optimización del sistema de eventos
