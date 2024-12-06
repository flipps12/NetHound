import subprocess
import socket


def get_hostnames_from_dhcp_lease():
    lease_file = "/var/lib/misc/dnsmasq.leases"
    hostnames = {}
    try:
        with open(lease_file, "r") as f:
            for line in f:
                parts = line.split()
                if len(parts) >= 4:
                    mac, hostname = parts[1], parts[3]
                    hostnames[mac] = hostname
        return hostnames
    except FileNotFoundError:
        print(f"No se encontró el archivo {lease_file}.")
        return {}


def get_hostname_from_ip(ip_address):
    try:
        hostname, _, _ = socket.gethostbyaddr(ip_address)
        return hostname
    except socket.herror:
        return None



def get_connected_users():
    try:
        # Ejecutar el comando hostapd_cli all_sta
        output = subprocess.check_output(['sudo', 'hostapd_cli', 'all_sta']).decode()
        clients = []
        for line in output.splitlines():
            # Las MACs son las únicas líneas formateadas como direcciones
            if line and ':' in line and len(line.split(':')) == 6:
                clients.append(line.strip())  # Extrae y limpia la MAC
                # get_hostname_from_ip(line.strip())
                # get_hostnames_from_dhcp_lease()
        return clients
    except Exception as e:
        print(f"Error: {e}")
        return []

# Llama a la función y muestra los resultados
connected_users = get_connected_users()
print("Usuarios conectados:", connected_users)


