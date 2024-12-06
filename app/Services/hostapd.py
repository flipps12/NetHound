import subprocess

def get_connected_users():
    try:
        output = subprocess.check_output(['hostapd_cli', 'all_sta']).decode()
        clients = []
        for line in output.split("\n"):
            if line.startswith("Station"):
                clients.append(line.split()[1])  # Extrae la MAC
        return clients
    except Exception as e:
        print(f"Error: {e}")
        return []

connected_users = get_connected_users()
print("Usuarios conectados:", connected_users)
