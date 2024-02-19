use crate::data_layer::contact::{Contact, ContactForUpdate};
use crate::error::AppResult;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct Db {
    conn: Arc<Mutex<Option<Connection>>>,
}

impl Db {
    pub fn new(conn: Arc<Mutex<Option<Connection>>>) -> Self {
        Self { conn }
    }

    pub fn insert(&self, contact: ContactForUpdate) -> AppResult<usize> {
        let mut guard = self.conn.lock().unwrap();
        if let Some(ref mut conn) = *guard {
            let sql = "INSERT INTO contacts (first_name, last_name, phone_number, company_name) VALUES (?, ?, ?, ?)";
            let params = params![
                    contact.first_name,
                    contact.last_name,
                    contact.phone_number,
                    contact.company_name
            ];

            let affected_rows = conn.execute(
                sql,
                params,
            )?;

            return Ok(affected_rows);
        }

        Ok(0)
    }

    pub fn update() {}
    pub fn delete(&self, id: u64) -> AppResult<usize> {
        let mut guard = self.conn.lock().unwrap();
        if let Some(ref mut conn) = *guard {
            let sql = "DELETE FROM contacts WHERE ID = ?";
            let affected_rows = conn.execute(sql, params![id])?;
            return Ok(affected_rows);
        }

        Ok(0)
    }
    pub fn get() {}
    pub fn list(&self, filter: impl Into<String>) -> AppResult<Vec<Contact>> {
        info!("Listing contacts");
        let mut guard = self.conn.lock().unwrap();

        if let Some(ref mut conn) = *guard {
            let mut sql = conn.prepare(
                "
                SELECT * FROM contacts 
                WHERE first_name LIKE '%' || ?1 || '%' 
                OR last_name LIKE '%' || ?1 || '%' 
                OR company_name LIKE '%' || ?1 || '%' 
                OR phone_number LIKE '%' || ?1 || '%'
            ",
            )?;

            let contact_iter = sql
                .query_map(params![filter.into()], |row| {
                    Ok(Contact {
                        id: row.get(0)?,
                        first_name: row.get(1)?,
                        last_name: row.get(2)?,
                        phone_number: row.get(3)?,
                        company_name: row.get(4)?,
                    })
                })?
                .filter_map(Result::ok)
                .collect();

            return Ok(contact_iter);
        }
        Ok(vec![])
    }
}
