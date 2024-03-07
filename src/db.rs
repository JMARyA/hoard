use rusqlite::Connection;
use std::error::Error;

// todo : make db singleton

#[derive(Debug, Clone)]
pub struct Database {
    file: String,
}

impl Database {
    pub fn new(file: &str) -> Self {
        let conn = Connection::open(file).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS urls (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        Self {
            file: file.to_string(),
        }
    }

    pub fn insert_url(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let conn = Connection::open(&self.file)?;
        let timestamp = chrono::Local::now().to_rfc3339();
        conn.execute(
            "INSERT INTO urls (url, timestamp) VALUES (?, ?)",
            [url, &timestamp],
        )?;
        Ok(())
    }

    pub fn check_for_url(&self, url: &str) -> Result<bool, Box<dyn Error>> {
        let conn = Connection::open(&self.file)?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM urls WHERE url = ?")?;
        let count: i64 = stmt.query_row([url], |row| row.get(0))?;
        Ok(count > 0)
    }
}
