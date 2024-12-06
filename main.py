import netifaces
import socket
import flask

interface = netifaces.interfaces()[1]
ip_red = "192.168.0.1/24"  # Ejemplo: Red 192.168.1.x


# Start
print("Noseque CLI V0.0.1-Alpha")
print(f"Interface: {interface}  \nHostname: {socket.gethostname()} \nHost: {socket.gethostbyname(socket.gethostname())}")


