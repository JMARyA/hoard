use jobdispatcher::{JobDispatcher, JobOrder};
use rusqlite::{Connection, OptionalExtension};
use std::sync::{mpsc::Receiver, Arc};

pub struct DatabaseBackend {
    pub file: String,
    pub conn: Connection,
    pub dispatcher: Arc<JobDispatcher<Query, Out>>,
    pub recv: Receiver<JobOrder<Query, Out>>,
}

impl DatabaseBackend {
    pub fn new(file: &str) -> Self {
        let (dispatcher, recv) = jobdispatcher::JobDispatcher::<Query, Out>::new();
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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS item_log (
                id INTEGER PRIMARY KEY,
                module TEXT NOT NULL,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        let dispatcher = Arc::new(dispatcher);
        Self {
            file: file.to_string(),
            conn,
            dispatcher,
            recv,
        }
    }

    pub fn take_db(&self) -> Database {
        Database::new(self.dispatcher.clone())
    }

    pub fn run(&self) {
        while let Ok(job) = self.recv.recv() {
            match job.param {
                Query::InsertUrl(ref url) => {
                    let timestamp = chrono::Local::now().to_rfc3339();
                    self.conn
                        .execute(
                            "INSERT INTO urls (url, timestamp) VALUES (?, ?)",
                            [url, &timestamp],
                        )
                        .unwrap();
                    job.done(Out::Ok);
                }
                Query::CheckForUrl(ref url) => {
                    let mut stmt = self
                        .conn
                        .prepare("SELECT COUNT(*) FROM urls WHERE url = ?")
                        .unwrap();
                    let count: i64 = stmt.query_row([url], |row| row.get(0)).unwrap();
                    job.done(Out::Bool(count > 0));
                }
                Query::UpdateNewDownloads(ref module, ref name, ref url) => {
                    let timestamp = chrono::Local::now().to_rfc3339();

                    // Check if the entry exists
                    let existing_timestamp: Option<String> = self.conn.query_row(
                        "SELECT timestamp FROM item_log WHERE module = ? AND name = ? AND url = ?",
                        [module, name, url],
                        |row| row.get(0)
                    ).optional().unwrap();

                    if existing_timestamp.is_some() {
                        // Entry exists, update timestamp
                        self.conn.execute(
                            "UPDATE item_log SET timestamp = ? WHERE module = ? AND name = ? AND url = ?",
                            [&timestamp, module, name, url]
                        ).unwrap();
                    } else {
                        // Entry doesn't exist, insert new row
                        self.conn.execute(
                            "INSERT INTO item_log (module, name, url, timestamp) VALUES (?, ?, ?, ?)",
                            [module, name, url, &timestamp]
                        ).unwrap();
                    }

                    job.done(Out::Ok);
                }
            }
        }
    }
}

pub enum Query {
    InsertUrl(String),
    CheckForUrl(String),
    UpdateNewDownloads(String, String, String),
}

pub enum Out {
    Ok,
    Bool(bool),
    // Rows(Vec<String>),
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<JobDispatcher<Query, Out>>,
}

impl Database {
    pub fn new(conn: Arc<JobDispatcher<Query, Out>>) -> Self {
        Self { conn }
    }

    pub fn insert_url(&self, url: &str) {
        self.conn.send(Query::InsertUrl(url.to_string()));
    }

    pub fn check_for_url(&self, url: &str) -> bool {
        match self.conn.send(Query::CheckForUrl(url.to_string())) {
            Out::Ok => false,
            Out::Bool(b) => b,
        }
    }

    pub fn update_new_downloads(&self, module: &str, name: &str, url: &str) {
        self.conn.send(Query::UpdateNewDownloads(
            module.to_string(),
            name.to_string(),
            url.to_string(),
        ));
    }
}
