use crate::app::Timer;
use chrono::{DateTime, Duration};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

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
}
