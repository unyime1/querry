use crate::database::get_database;
use rusqlite::{params, Result};

#[derive(Clone, Debug)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
}

pub fn get_all_collections() -> Result<Vec<CollectionData>> {
    let db_connection = get_database().expect("Could not get database");

    let mut stmt = db_connection.prepare("SELECT id, name FROM collection")?;
    let rows = stmt.query_map(params![], |row| {
        Ok(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let collections: Result<Vec<CollectionData>> = rows.into_iter().collect();
    collections
}
