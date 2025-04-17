extern crate rusqlite;

use std::error::Error;

use crate::utils::sys_dir::get_db_path;
use rusqlite::Connection;

pub fn get_database() -> Result<Connection, Box<dyn Error>> {
    let db_path = get_db_path(Some(false))?;
    let db = Connection::open(db_path)?;
    Ok(db)
}

pub fn migrate_database(db_connection: &Connection) -> Result<(), Box<dyn Error>> {
    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS collection(
                id UUID NOT NULL PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                name TEXT NOT NULL
            )",
        (), // empty list of parameters.
    )?;

    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS collectionheader(
                id UUID NOT NULL PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                name TEXT,
                value TEXT,
                collection_id UUID NOT NULL REFERENCES collection(id) ON DELETE CASCADE
            )",
        (), // empty list of parameters.
    )?;

    db_connection.execute(
        "CREATE TABLE IF NOT EXISTS requestitem(
                id UUID NOT NULL PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                name TEXT,
                url TEXT,
                protocol TEXT,
                http_method TEXT,
                collection_id UUID NOT NULL REFERENCES collection(id) ON DELETE CASCADE
            )",
        (), // empty list of parameters.
    )?;

    Ok(())
}

/// Setup a clean database for tests.
pub fn setup_test_db() -> Result<Connection, Box<dyn Error>> {
    let db_path = get_db_path(Some(true))?;
    let db = Connection::open(db_path)?;

    match db.execute("DROP TABLE collection", ()) {
        Ok(_) => {}
        Err(error) => {
            let error_str = error.to_string();
            println!("{}", error_str)
        }
    };

    match db.execute("DROP TABLE collectionheader", ()) {
        Ok(_) => {}
        Err(error) => {
            let error_str = error.to_string();
            println!("{}", error_str)
        }
    };

    match db.execute("DROP TABLE requestitem", ()) {
        Ok(_) => {}
        Err(error) => {
            let error_str = error.to_string();
            println!("{}", error_str)
        }
    };

    let _data = migrate_database(&db)?;

    Ok(db)
}
