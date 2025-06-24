use tempfile::TempDir;
use timers::app::{App, Timer};
use timers::db::Db;

pub struct DBTestFixture {
    pub db: Db,
    pub temp_dir: TempDir,
}

impl DBTestFixture {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let db = Db::new(temp_dir.path().join("test.db").to_str().unwrap());
        Self { db, temp_dir }
    }
}

pub struct AppTestFixture {
    pub app: App,
    pub temp_dir: TempDir,
}

impl AppTestFixture {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let app = App::new(temp_dir.path().join("test.db").to_str().unwrap());
        Self { app, temp_dir }
    }
}

#[test]
fn test_add_timer_to_app() {
    let mut fixture = AppTestFixture::new();
    fixture.app.name_input = "test".to_string();
    fixture.app.description_input = "test".to_string();
    fixture.app.add_timer();
    assert_eq!(fixture.app.timers.len(), 1);
}

#[test]
fn test_init_db() {
    let fixture = DBTestFixture::new();
    let timers = fixture.db.get_timers_from_db().unwrap();
    let count = timers.len();
    assert_eq!(count, 0);
}

#[test]
fn test_add_timer_to_db() {
    let fixture = DBTestFixture::new();
    let mut timer = Timer::new("test".to_string(), "test".to_string());
    fixture.db.add_timer_to_db(&mut timer).unwrap();
    let timers = fixture.db.get_timers_from_db().unwrap();
    let count = timers.len();
    assert_eq!(count, 1);
}
