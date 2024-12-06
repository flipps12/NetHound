import subprocess

def get_connected_users():
    try:
        # Ejecutar el comando hostapd_cli all_sta
        output = subprocess.check_output(['sudo', 'hostapd_cli', 'all_sta']).decode()
        clients = []
        for line in output.splitlines():
            # Las MACs son las únicas líneas formateadas como direcciones
            if line and ':' in line and len(line.split(':')) == 6:
                clients.append(line.strip())  # Extrae y limpia la MAC
        return clients
    except Exception as e:
        print(f"Error: {e}")
        return []

# Llama a la función y muestra los resultados
connected_users = get_connected_users()
print("Usuarios conectados:", connected_users)
