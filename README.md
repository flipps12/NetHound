# NetHound

NetHound es un módem WiFi configurable basado en Linux, que utiliza `hostapd`, `dnsmasq` e `ipset` combinado con microservicios escritos en Rust para ofrecer un portal cautivo eficiente, autenticación de usuarios y control avanzado de dispositivos.

## Objetivo del proyecto
Crear un módem accesible, modular y optimizado para hardware de bajos recursos, capaz de manejar usuarios, sesiones y control de red sin comprometer el rendimiento del tráfico WiFi.

---

## Hardware

**Hardware actual (desarrollo):**
- Raspberry Pi 5 (4 GB)

**Hardware objetivo (producción):**
- Raspberry Pi Zero / Zero 2W (512 MB)
  
Diseñado para funcionar incluso con recursos muy limitados.

---

## Software
El sistema utiliza una arquitectura de **microservicios**, donde cada componente cumple una tarea específica.  
La prioridad del sistema es preservar el rendimiento del **data-plane** (paso de paquetes) por encima del control administrativo.

### Servicios:

- **nh-captive-portal**   
  Servidor HTTP mínimo que redirige a los clientes al portal de inicio de sesión.

- **nh-auth**  
  Autentica usuarios y gestiona sesiones, dispositivos, MAC/IP.

- **nh-approval-queue**  
  Cola de aprobación que procesa en lote las IP/MAC autorizadas y las inserta en ipset.

- **nh-dashboard**  
  Panel de administración para configurar todos los servicios del sistema.

- **nh-cloud**  
  Sistema de archivos remoto integrado.

---

NetHound prioriza la estabilidad, el rendimiento y la escalabilidad ligera para convertir hardware económico en un módem completamente personalizable.
