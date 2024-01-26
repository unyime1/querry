use std::error::Error;

use rusqlite::Connection;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum ProcolTypes {
    Http,
    Websocket,
    Grpc,
    GraphQL,
}

impl ToString for ProcolTypes {
    fn to_string(&self) -> String {
        match self {
            ProcolTypes::Http => String::from("HTTP"),
            ProcolTypes::Websocket => String::from("WS"),
            ProcolTypes::Grpc => String::from("GRPC"),
            ProcolTypes::GraphQL => String::from("GQL"),
        }
    }
}

impl ProcolTypes {
    pub fn from_string(s: &str) -> Option<ProcolTypes> {
        match s {
            "HTTP" => Some(ProcolTypes::Http),
            "WS" => Some(ProcolTypes::Websocket),
            "GRPC" => Some(ProcolTypes::Grpc),
            "GQL" => Some(ProcolTypes::GraphQL),
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
        .prepare("SELECT id, name, url, protocol, http_method, collection_id FROM requestitem WHERE collection_id=?1")?;

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
    protocol: ProcolTypes,
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
) -> Result<Option<RequestData>, Box<dyn Error>> {
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

    Ok(requests.first().cloned())
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
        let _request = create_request(ProcolTypes::Http, collection.id.clone(), &db)
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
        let _request = create_request(ProcolTypes::Http, collection.id.clone(), &db)
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
        let request = create_request(ProcolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let fetched_request = get_single_request(request.id, &db).unwrap();
        assert!(fetched_request.is_some());
        let fetched_request_some = fetched_request.unwrap();

        assert!(fetched_request_some.name == "New Request");
        assert!(fetched_request_some.protocol == "HTTP");
        assert!(fetched_request_some.http_method == Some("GET".to_string()));
        assert!(fetched_request_some.url.is_none());
    }

    #[test]
    fn test_delete_request() {
        let db = setup_test_db().expect("Cant setup db.");
        let collection = create_collection("Test collection".to_string(), &db).unwrap();
        let request = create_request(ProcolTypes::Http, collection.id.clone(), &db)
            .expect("Cant get collections");

        let fetched_request = get_single_request(request.id, &db).unwrap();
        assert!(fetched_request.is_some());
        let fetched_request_some = fetched_request.unwrap();

        assert!(fetched_request_some.name == "New Request");
        assert!(fetched_request_some.protocol == "HTTP");
        assert!(fetched_request_some.http_method == Some("GET".to_string()));
        assert!(fetched_request_some.url.is_none());

        delete_request(fetched_request_some.id.clone(), &db).unwrap();
        let fetched_request_new = get_single_request(fetched_request_some.id, &db).unwrap();
        assert!(fetched_request_new.is_none());
    }
}
