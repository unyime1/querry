use sea_orm::*;
use uuid::Uuid;

use crate::database::get_database;
use crate::entities::{prelude::*, *};

#[derive(Clone)]
pub struct CollectionData {
    pub id: String,
    pub name: String,
}

pub async fn get_all_collections() -> Result<Vec<CollectionData>, DbErr> {
    let db = get_database().await?;
    let mut collections: Vec<collection::Model> = Collection::find().all(&db).await?;

    if collections.len() == 0 {
        for item in 0..5 {
            let collection_item = collection::ActiveModel {
                name: ActiveValue::Set(format!("Test collection_{}", item).to_owned()),
                id: ActiveValue::Set(Uuid::new_v4().to_string()),
                ..Default::default()
            };
            Collection::insert(collection_item).exec(&db).await?;
        }
        let new_collections: Vec<collection::Model> = Collection::find().all(&db).await?;
        collections.extend(new_collections);
    }

    // Map collections to CollectionData
    let collection_data: Vec<CollectionData> = collections
        .iter()
        .map(|c| CollectionData {
            id: c.id.to_string(),
            name: c.name.to_string(),
        })
        .collect();

    println!("Total collections - {}", collection_data.len());
    Ok(collection_data)
}
