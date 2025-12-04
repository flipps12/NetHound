use argon2::{Argon2, PasswordHasher, PasswordVerifier}; // ¡PasswordVerifier IMPORTADO!
use password_hash::{SaltString, PasswordHash};
use rand_core::OsRng;
use sha2::{Sha256, Digest};
use anyhow; // Ya no necesitas el trait std::error::Error


pub fn sha256(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// El mapeo de error manual ya no es necesario si usamos las versiones 0.5+
// pero lo mantendremos simple para que funcione con anyhow.
pub fn _hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    // El error de password_hash::Error ya se puede convertir con '?'
    // en versiones recientes o se mapea de forma sencilla.
    let hash = argon2.hash_password(password.as_bytes(), &salt)
        // Mapeamos el error de Argon2 a anyhow
        .map_err(|e| anyhow::anyhow!("Error al hashear: {}", e))? 
        .to_string();

    Ok(hash)
}

pub fn _verify_password(password: &str, stored_hash: &str) -> bool {
    // 1. Parsear el hash almacenado (puede fallar si el formato es malo)
    let parsed = match PasswordHash::new(stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false, // Si no se parseó, es inválido
    };
    
    // 2. Verificar la contraseña usando el trait PasswordVerifier
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}