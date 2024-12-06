from flask import Blueprint, jsonify

from app.Services.hostapd import get_hostnames_from_dhcp_lease

main = Blueprint('main', __name__)

@main.route('/')
def home():
    return jsonify({"message": "¡Hola, Flask!"})

@main.route('/status')
def status():
    return jsonify({"status": "OK", "version": "1.0"})

@main.route('/connected-users')
def users():
    return jsonify({"users": get_hostnames_from_dhcp_lease()})
