pub mod migrator;

use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::utils::sys_dir::get_db_path;

pub async fn get_database() -> Result<DatabaseConnection, DbErr> {
    let db_path = get_db_path();
    let db_url: &str = &format!("sqlite://{}", db_path);
    let db = Database::connect(db_url).await?;
    Ok(db)
}
