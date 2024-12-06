from scapy.all import sniff
from main import interface

def packet_callback(packet):
    print(packet.summary())  # Muestra un resumen del paquete

# Cambia 'wlan0' a tu interfaz de red
sniff(iface=interface, prn=packet_callback, count=10)