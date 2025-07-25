use crate::lib::app::Timer;
use chrono::{DateTime, Duration};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
use dirs;

#[derive(Debug)]
pub struct Db {
    conn: Arc<Mutex<Connection>>,
}

impl Db {
    pub fn new(path: &str) -> Self {
        Db {
            conn: Db::init_db(path).expect("Unable to init db"),
        }
    }

    /// Get the platform-appropriate database path
    pub fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let data_dir = dirs::data_dir()
            .ok_or("Unable to determine data directory")?;

        let app_dir = data_dir.join("timers");

        // Create the directory if it doesn't exist
        fs::create_dir_all(&app_dir)?;

        Ok(app_dir.join("timers.db"))
    }

    /// Create a new Db instance using the platform-appropriate path
    pub fn new_with_default_path() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        let path_str = db_path.to_str()
            .ok_or("Database path contains invalid UTF-8")?;

        Ok(Db {
            conn: Db::init_db(path_str).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?,
        })
    }

    fn init_db(path: &str) -> Result<Arc<Mutex<Connection>>, rusqlite::Error> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS timers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            start_time DATETIME NOT NULL,
            duration INTEGER NOT NULL,
            running BOOLEAN NOT NULL
        )",
            [],
        )?;

        Ok(Arc::new(Mutex::new(conn))) // Wrap the connection in an Arc<Mutex<Connection>>conn)
    }

    pub fn add_timer_to_db(&self, timer: &mut Timer) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().expect("Unable to lock connection");
        conn.execute(
            "INSERT INTO timers (name, description, start_time, duration, running) VALUES (?, ?, ?, ?, ?)",
            params![
            timer.name,
            timer.description,
            timer.start_time.to_rfc3339(),
            timer.duration.num_seconds(),
            timer.running
        ],
        )?;

        let id = conn.last_insert_rowid();
        timer.id = id as usize;
        Ok(())
    }

    pub fn get_timers_from_db(&self) -> Result<Vec<Timer>, rusqlite::Error> {
        let conn = self.conn.lock().expect("Unable to lock connection");
        let mut stmt = conn
            .prepare("SELECT id, name, description, start_time, duration, running FROM timers")?;
        let timers = stmt
            .query_map(params![], |row| {
                let timestamp: String = row.get(3)?;
                let date = DateTime::parse_from_rfc3339(&timestamp).unwrap().to_utc();
                Ok(Timer {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    start_time: date,
                    duration: Duration::seconds(row.get(4)?),
                    running: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<Timer>, rusqlite::Error>>()?;
        Ok(timers)
    }

    pub fn update_timers_in_db(&self, timers: &Vec<Timer>) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().expect("Unable to lock connection");
        for timer in timers {
            conn.execute(
                "UPDATE timers SET duration = ?, running = ? WHERE id = ?",
                params![timer.duration.num_seconds(), timer.running, timer.id],
            )?;
        }
        Ok(())
    }

    pub fn delete_timer(&self, id: usize) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().expect("Unable to lock connection");
        conn.execute("DELETE FROM timers WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn edit_timer(
        &self,
        timer: &Timer,
        name: &str,
        description: &str,
    ) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().expect("Unable to lock connection");
        conn.execute(
            "UPDATE timers SET name = ?, description = ? WHERE id = ?",
            params![name, description, timer.id],
        )?;
        Ok(())
    }
}
