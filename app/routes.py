from flask import Blueprint, jsonify

from app.Services.hostapd import get_connected_users

main = Blueprint('main', __name__)

@main.route('/')
def home():
    return jsonify({"message": "¡Hola, Flask!"})

@main.route('/status')
def status():
    return jsonify({"status": "OK", "version": "1.0"})

@main.route('/users')
def status():
    return jsonify({"users": get_connected_users()})