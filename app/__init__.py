from flask import Flask

def create_app():
    app = Flask(__name__)

    # Cargar configuración
    # app.config.from_object('config.Config')

    # Registrar rutas
    from .routes import main
    app.register_blueprint(main)

    return app