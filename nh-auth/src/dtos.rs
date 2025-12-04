use sqlx::FromRow;


// Cuentas
#[derive(Debug, FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub ip: Option<String>,
    pub mac: Option<String>,
    pub role: String,
}