#!/usr/sbin/nft -f

# Usar en sudo nano /etc/nftables.conf
# aplicar:
# sudo nft flush ruleset
# sudo nft -f /etc/nftables.conf
# iniciar nftables systemctl

# sudo nft add element inet nethound authorized { 192.168.1.23 }
# sudo nft delete element inet nethound authorized { 192.168.1.23 }
# sudo nft list set inet nethound authorized


# Variables
define WAN_IF = "eth0"
define AP_IF  = "wlan0"

define ROUTER_IP = 192.168.0.1
define LAN_NET   = 192.168.0.0/24
define AP_NET    = 192.168.1.0/24

# Crear tabla
table inet nethound {

    # Set de IPs autorizadas (reemplaza ipset)
    set authorized {
        type ipv4_addr
        flags interval
    }

    chain forward {
        type filter hook forward priority 0;

        # Política por defecto: DROP
        policy drop;

        # Aceptar conexiones establecidas
        ct state { established, related } accept

        # Aislar clientes del AP entre sí
        iifname $AP_IF oifname $AP_IF drop

        # Bloquear acceso de AP a la LAN interna
        iifname $AP_IF oifname $WAN_IF ip daddr $LAN_NET drop

        # Permitir acceso del AP al router (Backend/Autorizador)
        iifname $AP_IF ip daddr $ROUTER_IP accept

        # Permitir internet solo a IP autorizada
        iifname $AP_IF ip saddr @authorized accept
    }
}

# Tabla NAT
table ip nat {
    chain postrouting {
        type nat hook postrouting priority 100;
        oifname $WAN_IF masquerade
    }
}
