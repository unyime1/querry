use sea_orm::*;

use crate::database::get_database;
use crate::entities::{prelude::*, *};

#[derive(Clone)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
}

pub async fn get_all_collections() -> Result<Vec<CollectionData>, DbErr> {
    let db = get_database().await?;
    let collections: Vec<collection::Model> = Collection::find().all(&db).await?;

    println!("{:?}", collections);

    // Map collections to CollectionData
    let collection_data: Vec<CollectionData> = collections
        .iter()
        .map(|c| CollectionData {
            id: c.id.to_string(),
            name: c.name.to_string(),
        })
        .collect();

    Ok(collection_data)
}
