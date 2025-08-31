use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: u32,
    pub name: Option<String>,
}
