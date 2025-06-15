use crate::app;
use crate::app::Timer;
use chrono::{DateTime, Duration};
use rusqlite::{Connection, params};
use std::sync::{Arc, Mutex};

pub fn init_db() -> Result<Arc<Mutex<Connection>>, rusqlite::Error> {
    let conn = Connection::open("timers.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS timers (
            id INTEGER PRIMARY KEY,
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

pub fn add_timer_to_db(
    conn: Arc<Mutex<Connection>>,
    timer: &app::Timer,
) -> Result<(), rusqlite::Error> {
    let conn = conn.lock().expect("Unable to lock connection");
    conn.execute(
        "INSERT INTO timers (name, description, start_time, duration, running) VALUES (?, ?, ?, ?, ?)",
        params![
            timer.name,
            timer.description,
            timer.start_time.to_rfc3339(),
            timer.duration.as_seconds_f32(),
            timer.running
        ],
    )?;
    Ok(())
}

pub fn get_timers_from_db(
    conn: Arc<Mutex<Connection>>,
) -> Result<Vec<app::Timer>, rusqlite::Error> {
    let conn = conn.lock().expect("Unable to lock connection");
    let mut stmt =
        conn.prepare("SELECT id, name, description, start_time, duration, running FROM timers")?;
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

pub fn get_count_of_timers(conn: Arc<Mutex<Connection>>) -> Result<u32, rusqlite::Error> {
    let conn = conn.lock().expect("Unable to lock connection");
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM timers")?;
    let count: u32 = stmt.query_row(params![], |row| row.get(0))?;
    Ok(count)
}

pub fn update_timers_in_db(
    conn: Arc<Mutex<Connection>>,
    timers: &Vec<Timer>,
) -> Result<(), rusqlite::Error> {
    let conn = conn.lock().expect("Unable to lock connection");
    for timer in timers {
        conn.execute(
            "UPDATE timers SET name = ?, description = ?, start_time = ?, duration = ?, running = ? WHERE id = ?",
            params![
                timer.name,
                timer.description,
                timer.start_time.to_rfc3339(),
                timer.duration.num_seconds(),
                timer.running,
                timer.id
            ],
        )?;
    }
    Ok(())
}
