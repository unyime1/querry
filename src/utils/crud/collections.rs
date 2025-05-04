use std::error::Error;

use crate::utils::get_icon_pack_names;
use rand::{rng, seq::IndexedRandom};
use rusqlite::{params, Connection, Result as RuResult};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
    pub icon: String,
}

pub fn get_all_collections(db_connection: &Connection) -> RuResult<Vec<CollectionData>> {
    let mut stmt =
        db_connection.prepare("SELECT id, name, icon FROM collection ORDER BY created_at DESC")?;
    let rows = stmt.query_map(params![], |row| {
        Ok(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        })
    })?;

    let collections: RuResult<Vec<CollectionData>> = rows.into_iter().collect();
    collections
}

pub fn create_collection(
    name: String,
    db_connection: &Connection,
) -> Result<CollectionData, Box<dyn Error>> {
    let icon_items = get_icon_pack_names()?;
    let mut rng = rng();
    let random_icon = icon_items
        .choose(&mut rng)
        .map(|s| s.to_string())
        .unwrap_or("1F4A6.svg".to_string());
    let mut stmt = db_connection.prepare(
        "INSERT INTO collection (id, name, icon) VALUES (?1, ?2, ?3) RETURNING id, name, icon",
    )?;

    let mut result_rows = stmt.query([Uuid::new_v4().to_string(), name, random_icon])?;

    let mut collections: Vec<CollectionData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        collections.push(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        });
    }

    let collection_item = collections.first().ok_or("Could not save collection.")?;
    println!("Created new collection");
    Ok(collection_item.clone())
}

pub fn delete_collection(
    collection_id: String,
    db_connection: &Connection,
) -> Result<(), Box<dyn Error>> {
    let mut stmt = db_connection.prepare("DELETE FROM collection WHERE id=?1")?;

    stmt.execute([collection_id])?;

    Ok(())
}

pub fn get_single_collection(
    id: String,
    db_connection: &Connection,
) -> Result<CollectionData, Box<dyn Error>> {
    let mut stmt = db_connection.prepare("SELECT id, name, icon FROM collection WHERE id=?1")?;

    let mut result_rows = stmt.query([id])?;

    let mut collections: Vec<CollectionData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        collections.push(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        });
    }

    let collection_item = collections.first().ok_or("Collection does not exist.")?;
    Ok(collection_item.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::setup_test_db;

    #[test]
    fn test_create_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let _collection =
            create_collection("Test collection".to_string(), &db).expect("Cant get collections");
        let existing_collection = get_all_collections(&db).expect("cant get collections");
        assert!(existing_collection.len() == 1)
    }

    #[test]
    fn test_delete_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection =
            create_collection("Test collection".to_string(), &db).expect("Cant get collections");

        delete_collection(collection.id, &db).expect("Can't delete");

        let existing_collection = get_all_collections(&db).expect("cant get collections");
        assert!(existing_collection.len() == 0)
    }

    #[test]
    fn test_single_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection =
            create_collection("Test collection".to_string(), &db).expect("Cant get collections");

        let single_collection =
            get_single_collection(collection.id.clone(), &db).expect("cant get collections");
        assert!(single_collection.id == collection.id);
        assert!(single_collection.name == collection.name);
        assert!(!single_collection.icon.is_empty());
    }
}
