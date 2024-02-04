use std::{cell::RefCell, error::Error};

use rusqlite::Connection;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum ProtocolTypes {
    Http,
    Websocket,
    Grpc,
    GraphQL,
}

impl ToString for ProtocolTypes {
    fn to_string(&self) -> String {
        match self {
            ProtocolTypes::Http => String::from("HTTP"),
            ProtocolTypes::Websocket => String::from("WS"),
            ProtocolTypes::Grpc => String::from("GRPC"),
            ProtocolTypes::GraphQL => String::from("GQL"),
        }
    }
}

impl ProtocolTypes {
    pub fn from_string(s: &str) -> Option<ProtocolTypes> {
        match s {
            "HTTP" => Some(ProtocolTypes::Http),
            "WS" => Some(ProtocolTypes::Websocket),
            "GRPC" => Some(ProtocolTypes::Grpc),
            "GQL" => Some(ProtocolTypes::GraphQL),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HTTPMethods {
    Post,
    Get,
    Put,
    Delete,
}

impl ToString for HTTPMethods {
    fn to_string(&self) -> String {
        match self {
            HTTPMethods::Post => String::from("POST"),
            HTTPMethods::Get => String::from("GET"),
            HTTPMethods::Put => String::from("PUT"),
            HTTPMethods::Delete => String::from("DEL"),
        }
    }
}

impl HTTPMethods {
    pub fn from_string(s: &str) -> Option<HTTPMethods> {
        match s {
            "POST" => Some(HTTPMethods::Post),
            "GET" => Some(HTTPMethods::Get),
            "PUT" => Some(HTTPMethods::Put),
            "DEL" => Some(HTTPMethods::Delete),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestData {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub protocol: String,
    pub collection_id: String,
    pub http_method: Option<String>,
}

pub fn get_collection_requests(
    db_connection: &Connection,
    collection_id: &str,
) -> Result<Vec<RequestData>, Box<dyn Error>> {
    let mut stmt = db_connection
        .prepare("SELECT id, name, url, protocol, http_method, collection_id FROM requestitem WHERE collection_id=?1 ORDER BY created_at DESC")?;

    let mut result_rows = stmt.query([collection_id])?;

    let mut requests: Vec<RequestData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        requests.push(RequestData {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            protocol: row.get(3)?,
            http_method: row.get(4)?,
            collection_id: row.get(5)?,
        });
    }

    Ok(requests)
}

pub fn create_request(
    protocol: ProtocolTypes,
    collection_id: String,
    db_connection: &Connection,
) -> Result<RequestData, Box<dyn Error>> {
    let mut stmt = db_connection
        .prepare("INSERT INTO requestitem (id, name, protocol, http_method, collection_id) VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id, name, url, protocol, http_method, collection_id")?;

    let mut result_rows = stmt.query([
        Uuid::new_v4().to_string(),
        "New Request".to_string(),
        protocol.to_string(),
        HTTPMethods::Get.to_string(),
        collection_id,
    ])?;

    let mut requests: Vec<RequestData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        requests.push(RequestData {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            protocol: row.get(3)?,
            http_method: row.get(4)?,
            collection_id: row.get(5)?,
        });
    }

    let request_item = requests.first().ok_or("Could not save request.")?;
    Ok(request_item.clone())
}

pub fn delete_request(
    request_id: String,
    db_connection: &Connection,
) -> Result<(), Box<dyn Error>> {
    let mut stmt = db_connection.prepare("DELETE FROM requestitem WHERE id=?1")?;

    stmt.execute([request_id])?;

    Ok(())
}

pub fn get_single_request(
    id: String,
    db_connection: &Connection,
) -> Result<RequestData, Box<dyn Error>> {
    let mut stmt = db_connection.prepare(
        "SELECT id, name, url, protocol, http_method, collection_id FROM requestitem WHERE id=?1",
    )?;

    let mut result_rows = stmt.query([id])?;

    let mut requests: Vec<RequestData> = Vec::new();
    while let Some(row) = result_rows.next()? {
        requests.push(RequestData {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            protocol: row.get(3)?,
            http_method: row.get(4)?,
            collection_id: row.get(5)?,
        });
    }

    let request_item = requests.first().ok_or("Cannot find request item.")?;
    Ok(request_item.clone())
}

/// Update a request item.
pub fn update_request_item(
    id: String,
    name: Option<String>,
    protocol: Option<ProtocolTypes>,
    http_method: Option<HTTPMethods>,
    url: Option<String>,
    db_connection: &Connection,
) -> Result<RequestData, Box<dyn Error>> {
    let id_cell = RefCell::new(id);

    if let Some(name) = name {
        let mut stmt = db_connection.prepare("UPDATE requestitem SET name=?1 WHERE id = ?2")?;
        let _ = stmt.execute([name, id_cell.borrow().to_string()])?;
    }

    if let Some(protocol) = protocol {
        let mut stmt = db_connection.prepare("UPDATE requestitem SET protocol=?1 WHERE id = ?2")?;
        let _ = stmt.execute([protocol.to_string(), id_cell.borrow().to_string()])?;
    }
    if let Some(http_method) = http_method {
        let mut stmt =
            db_connection.prepare("UPDATE requestitem SET http_method=?1 WHERE id = ?2")?;
        let _ = stmt.execute([http_method.to_string(), id_cell.borrow().to_string()])?;
    }

    if let Some(url) = url {
        let mut stmt = db_connection.prepare("UPDATE requestitem SET url=?1 WHERE id = ?2")?;
        let _ = stmt.execute([url, id_cell.borrow().to_string()])?;
    }

    // Retrieve the updated values
    let updated_request = get_single_request(id_cell.borrow().to_string(), db_connection)?;

    Ok(updated_request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::setup_test_db;
    use crate::utils::crud::collections::create_collection;

    #[test]
    fn test_create_request() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let _request = create_request(ProtocolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let requests = get_collection_requests(&db, &collection.id).unwrap();
        assert!(requests.len() == 1);
        assert!(requests[0].name == "New Request");
        assert!(requests[0].protocol == "HTTP");
        assert!(requests[0].http_method == Some("GET".to_string()));
        assert!(requests[0].url.is_none());
    }

    #[test]
    fn test_get_collection_requests() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let _request = create_request(ProtocolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let requests = get_collection_requests(&db, &collection.id).unwrap();
        assert!(requests.len() == 1);
        assert!(requests[0].name == "New Request");
        assert!(requests[0].protocol == "HTTP");
        assert!(requests[0].http_method == Some("GET".to_string()));
        assert!(requests[0].url.is_none());
    }

    #[test]
    fn test_get_single_request() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let request = create_request(ProtocolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let fetched_request = get_single_request(request.id, &db).unwrap();

        assert!(fetched_request.name == "New Request");
        assert!(fetched_request.protocol == "HTTP");
        assert!(fetched_request.http_method == Some("GET".to_string()));
        assert!(fetched_request.url.is_none());
    }

    #[test]
    fn test_delete_request() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let request = create_request(ProtocolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let fetched_request = get_single_request(request.id, &db).unwrap();

        assert!(fetched_request.name == "New Request");
        assert!(fetched_request.protocol == "HTTP");
        assert!(fetched_request.http_method == Some("GET".to_string()));
        assert!(fetched_request.url.is_none());

        delete_request(fetched_request.id.clone(), &db).unwrap();
        let fetched_request_new = get_single_request(fetched_request.id, &db);
        assert!(fetched_request_new.is_err())
    }

    #[test]
    fn test_update_single_request() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let request = create_request(ProtocolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let fetched_request = get_single_request(request.id, &db).unwrap();

        assert!(fetched_request.name == "New Request");
        assert!(fetched_request.protocol == "HTTP");
        assert!(fetched_request.http_method == Some("GET".to_string()));
        assert!(fetched_request.url.is_none());

        let updated_request = update_request_item(
            fetched_request.id,
            Some("Hello Request".to_string()),
            None,
            None,
            Some("https://bbc.co.uk".to_string()),
            &db,
        )
        .unwrap();

        println!("{:?}", updated_request);

        assert!(updated_request.name == "Hello Request".to_string());
        assert!(updated_request.url == Some("https://bbc.co.uk".to_string()))
    }
}
