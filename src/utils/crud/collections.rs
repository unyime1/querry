use std::{error::Error, rc::Rc};

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

pub fn get_all_collections(db_connection: Rc<Connection>) -> RuResult<Vec<CollectionData>> {
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

/// Update a collection item.
pub fn update_collection_item(
    id: &str,
    name: &str,
    icon: &str,
    db_connection: Rc<Connection>,
) -> Result<CollectionData, Box<dyn Error>> {
    let mut stmt = db_connection
        .prepare("UPDATE collection SET name=?1, icon=?2 WHERE id = ?3 RETURNING id, name, icon")?;
    let mut rows = stmt.query([name, icon, id])?;

    let mut collections: Vec<CollectionData> = Vec::new();
    while let Some(row) = rows.next()? {
        collections.push(CollectionData {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
        });
    }

    let collection_item = collections.first().ok_or("Could not save collection.")?;
    Ok(collection_item.clone())
}

pub fn create_collection(
    name: String,
    db_connection: Rc<Connection>,
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
    Ok(collection_item.clone())
}

pub fn delete_collection(
    collection_id: String,
    db_connection: Rc<Connection>,
) -> Result<(), Box<dyn Error>> {
    let mut stmt = db_connection.prepare("DELETE FROM collection WHERE id=?1")?;

    stmt.execute([collection_id])?;

    Ok(())
}

pub fn get_single_collection(
    id: String,
    db_connection: Rc<Connection>,
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
        let _collection = create_collection("Test collection".to_string(), db.clone())
            .expect("Cant get collections");
        let existing_collection = get_all_collections(db.clone()).expect("cant get collections");
        assert!(existing_collection.len() == 1)
    }

    #[test]
    fn test_delete_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), db.clone())
            .expect("Cant get collections");

        delete_collection(collection.id, db.clone()).expect("Can't delete");

        let existing_collection = get_all_collections(db.clone()).expect("cant get collections");
        assert!(existing_collection.len() == 0)
    }

    #[test]
    fn test_single_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), db.clone())
            .expect("Cant get collections");

        let single_collection =
            get_single_collection(collection.id.clone(), db.clone()).expect("cant get collections");
        assert!(single_collection.id == collection.id);
        assert!(single_collection.name == collection.name);
        assert!(!single_collection.icon.is_empty());
    }

    #[test]
    fn test_update_collection() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), db.clone())
            .expect("Cant get collections");

        let single_collection =
            update_collection_item(&collection.id, "hey", "icon.png", db.clone())
                .expect("cant get collections");

        assert!(single_collection.id == collection.id);
        assert!(single_collection.name == "hey".to_string());
        assert!(single_collection.icon == "icon.png".to_string());
    }
}
