use serde::Deserialize;
use std::fs;

// Asegúrate de que las structs GlobalConfig y DatabaseConfig están definidas
// como en el paso anterior.

const CONFIG_PATH: &str = "/etc/NetHound.toml";

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub database: DatabaseConfig,
}

pub fn load_global_config() -> Result<GlobalConfig, String> {
    println!("Intentando cargar configuración desde: {}", CONFIG_PATH);

    // 1. Leer el contenido del archivo
    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error al leer el archivo {}: {}", CONFIG_PATH, e)),
    };

    // 2. Deserializar (parsear) el contenido TOML
    match toml::from_str(&contents) {
        Ok(config) => {
            println!("Configuración cargada exitosamente.");
            Ok(config)
        }
        Err(e) => Err(format!("Error al parsear TOML: {}", e)),
    }
}

// Ejemplo
// fn main() {
//     match load_global_config() {
//         Ok(config) => {
//             println!("Configuración de servicio: {}", config.service_name);
//             println!("Hilos de trabajo: {}", config.threads);
//             println!("Host de DB: {}", config.database.host);
//         }
//         Err(e) => {
//             eprintln!("Fallo crítico al cargar la configuración: {}", e);
//             // Manejo de errores: el servicio no puede iniciar sin configuración
//             std::process::exit(1);
//         }
//     }
// }