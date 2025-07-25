use tempfile::TempDir;
use timers::lib::app::{App, Timer, CurrentScreen};
use timers::lib::db::Db;
use timers::lib::throbber::Throbber;
use ratatui::widgets::TableState;

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
        let db_path = temp_dir.path().join("test.db");
        let db = Db::new(db_path.to_str().unwrap());

        let app = App {
            state: TableState::default().with_selected(1),
            timers: Vec::new(),
            current_screen: CurrentScreen::Main,
            name_input: String::new(),
            description_input: String::new(),
            currently_editing: None,
            selectable_rows: Vec::new(),
            db,
            throbber: Throbber::new(),
        };

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

#[test]
fn test_edit_timer() {
    let fixture = DBTestFixture::new();
    let mut timer = Timer::new("test".to_string(), "test".to_string());
    let mut timer2 = Timer::new("test2".to_string(), "test2".to_string());

    fixture.db.add_timer_to_db(&mut timer).unwrap();
    fixture.db.add_timer_to_db(&mut timer2).unwrap();

    timer.name = "test edited".to_string();
    timer.description = "test edited".to_string();
    fixture
        .db
        .edit_timer(&timer, "test edited", "test edited")
        .unwrap();

    let timers = fixture.db.get_timers_from_db().unwrap();
    let count = timers.len();
    assert_eq!(count, 2);
    let timer = timers.first().unwrap();
    let timer2 = timers.last().unwrap();
    assert_eq!(timer.name.as_str(), "test edited");
    assert_eq!(timer.description.as_str(), "test edited");
    assert_eq!(timer2.name.as_str(), "test2");
    assert_eq!(timer2.description.as_str(), "test2");
}
