from app import create_app
from time import sleep
import netifaces
import socket
import json
import os
from app.Services.hostapd import get_hostnames_from_dhcp_lease

interface = netifaces.interfaces()[1]
ip_red = "192.168.0.1/24"  # Ejemplo: Red 192.168.1.x


# Start
print("Noseque CLI V0.0.1-Alpha")
print(f"Interface: {interface}  \nHostname: {socket.gethostname()} \nHost: {socket.gethostbyname(socket.gethostname())}")

app = create_app()

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=80)

if not os.path.exists("data.json"):
    with open("data.example.json", "r") as f:
        data = json.load(f)
    with open("data.json", "w") as f:
        json.dump(data, f, indent=4)
    print("Nuevo archivo de datos.")

while True:
    user = get_hostnames_from_dhcp_lease()
    with open("data.example.json", "r") as f:
        data = json.load(f)
    claves_comunes = set(data.roles.blacklist.keys()) & set(user.keys())
    items_comunes = {clave: None for clave in claves_comunes if clave in data.roles.blacklist and clave in user}

    print(items_comunes)

    sleep(20)