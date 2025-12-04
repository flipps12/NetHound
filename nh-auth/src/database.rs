use sqlx::SqlitePool;
use std::path::Path;
use crate::{GlobalConfig, dtos::Account};



pub async fn initialize_db(config: &GlobalConfig) -> Result<SqlitePool, sqlx::Error> {
    let db_url = format!("sqlite:{}", config.database.path);
    
    // Crear el directorio si no existe (importante para SQLite)
    if let Some(parent_dir) = Path::new(&config.database.path).parent() {
        // Ignoramos el error si el directorio ya existe
        let _ = tokio::fs::create_dir_all(parent_dir).await; 
    }

    // 1. Establecer el Pool de Conexiones
    let pool = SqlitePool::connect(&db_url).await?;

    // 2. Ejecutar la migración (crear la tabla si no existe)
    // Para simplificar, aquí se ejecuta la creación de tabla directamente.
    // En producción, usarías un sistema de migraciones más formal (sqlx-cli).
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL,
            ip TEXT,
            mac TEXT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// C: Create
pub async fn create_account(pool: &SqlitePool, name: &str, password: &str, role: &str) -> Result<Account, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO accounts (name, password, role) 
        VALUES ($1, $2, $3)
        RETURNING id, name, password, ip, mac, role
        "#,
        name,
        password,
        role
    )
    .fetch_one(pool)
    .await?;

    // Mapear el resultado de la macro query! a tu struct Item
    Ok(Account {
        id: result.id,
        name: result.name,
        password: result.password,
        ip: result.ip,
        mac: result.mac,
        role: result.role
    })
}

// R: Read (Leer todos)
pub async fn read_all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as!(Account, "SELECT id, name, password, role, ip, mac FROM accounts")
        .fetch_all(pool)
        .await
}

pub async fn read_account_by_name(pool: &SqlitePool, username: &str) -> Result<String, sqlx::Error> {
    
    let result = sqlx::query!(
        r#"
        -- Seleccionar solo la columna 'password'
        SELECT password
        FROM accounts 
        WHERE name = $1
        "#,
        username
    )
    .fetch_optional(pool) // Devuelve Option<Record>
    .await?;
    match result {
        Some(record) => Ok(record.password),
        _none => Err(sqlx::Error::RowNotFound),
    }
}

// U: Update
pub async fn update_account_ip_mac(pool: &SqlitePool, id: i64, ip: &str, mac: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE accounts SET ip = $1, mac = $2 WHERE id = $3",
        ip,
        mac,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected()) // Devuelve el número de filas afectadas
}

// U: Update
pub async fn update_account_role(pool: &SqlitePool, id: i64, role: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE accounts SET role = $1 WHERE id = $2",
        role,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected()) // Devuelve el número de filas afectadas
}

// D: Delete
pub async fn delete_item(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM accounts WHERE id = $1", id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}