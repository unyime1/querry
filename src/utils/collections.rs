use std::error::Error;

use crate::database::get_database;
use rusqlite::{params, Result as RuResult};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
}

pub fn get_all_collections() -> RuResult<Vec<CollectionData>> {
    let db_connection = get_database().expect("Could not get database");

    let mut stmt = db_connection.prepare("SELECT id, name FROM collection")?;
    let rows = stmt.query_map(params![], |row| {
        Ok(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let collections: RuResult<Vec<CollectionData>> = rows.into_iter().collect();
    collections
}

pub fn create_collection(name: String) -> Result<CollectionData, Box<dyn Error>> {
    let db_connection = get_database()?;
    let mut stmt = db_connection
        .prepare("INSERT INTO collection (id, name) VALUES (?1, ?2) RETURNING id, name")?;

    let mut result_rows = stmt.query([Uuid::new_v4().to_string(), name]).expect("2");

    let mut collections: Vec<CollectionData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        collections.push(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }

    let collection_item = collections.first().ok_or("Could not save collection.")?;
    Ok(collection_item.clone())
}
