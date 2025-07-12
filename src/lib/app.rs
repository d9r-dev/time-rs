use crate::lib::db::Db;
use crate::lib::throbber::Throbber;
use chrono::{DateTime, Duration, Utc};
use ratatui::widgets::TableState;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Edit,
    Add,
    Exit,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Name,
    Description,
}

#[derive(Debug)]
pub struct App {
    pub timers: Vec<Timer>,
    pub name_input: String,
    pub description_input: String,
    pub currently_editing: Option<CurrentlyEditing>,
    pub current_screen: CurrentScreen,
    pub state: TableState,
    pub selectable_rows: Vec<bool>,
    pub db: Db,
    pub throbber: Throbber,
}

impl App {
    pub fn edit_timer(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.timers[selected - 1].name = self.name_input.clone();
            self.timers[selected - 1].description = self.description_input.clone();
            self.db
                .edit_timer(
                    &self.timers[selected - 1],
                    &self.name_input,
                    &self.description_input,
                )
                .expect("Unable to edit timer");
            self.currently_editing = None;
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    pub start_time: DateTime<Utc>,
    pub name: String,
    pub(crate) duration: Duration,
    pub description: String,
    pub id: usize,
    pub running: bool,
}

impl App {
    pub fn new(path: &str) -> Self {
        App {
            state: TableState::default().with_selected(1),
            timers: Vec::new(),
            current_screen: CurrentScreen::Main,
            name_input: String::new(),
            description_input: String::new(),
            currently_editing: None,
            selectable_rows: Vec::new(),
            db: Db::new(path),
            throbber: Throbber::new(),
        }
    }

    pub fn next_row(&mut self) {
        if self.selectable_rows.is_empty() {
            return;
        }

        let current = self.state.selected().unwrap_or(0);
        let mut next = current;

        loop {
            next = (next + 1) % self.selectable_rows.len();
            if self.selectable_rows[next] || next == current {
                self.state.select(Some(next));
                break;
            }
        }
        self.state.select(Some(next));
    }

    pub fn previous_row(&mut self) {
        if self.selectable_rows.is_empty() {
            return;
        }

        let current = self.state.selected().unwrap_or(0);
        let mut prev = current;
        loop {
            prev = (prev + self.selectable_rows.len() - 1) % self.selectable_rows.len();
            if self.selectable_rows[prev] || prev == current {
                self.state.select(Some(prev));
                break;
            }
        }

        self.state.select(Some(prev));
    }

    pub fn add_timer(&mut self) {
        let mut timer = Timer::new(self.name_input.clone(), self.description_input.clone());
        match self.timers.last_mut() {
            Some(t) => t.stop(),
            None => (),
        }
        self.db
            .add_timer_to_db(&mut timer)
            .expect("TODO: panic message");
        self.timers.push(timer);
        self.name_input = String::new();
        self.description_input = String::new();
    }

    pub fn delete_selected_timer(&mut self) -> Result<(), rusqlite::Error> {
        let selected = self.state.selected();
        if let Some(selected) = selected {
            self.db.delete_timer(self.timers[selected - 1].id)?;
        }
        self.timers.remove(selected.unwrap_or(0) - 1);
        Ok(())
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Name => {
                    self.currently_editing = Some(CurrentlyEditing::Description)
                }
                CurrentlyEditing::Description => {
                    self.currently_editing = Some(CurrentlyEditing::Name)
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }
    pub fn toggle_timer(&mut self) {
        if let Some(timer) = self.timers.last_mut() {
            if timer.running {
                timer.stop();
            } else {
                timer.start();
            }
        }
    }
}

impl Timer {
    pub fn new(name: String, description: String) -> Timer {
        Timer {
            start_time: Utc::now(),
            duration: Duration::zero(),
            name,
            description,
            id: 0,
            running: true,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.running = true;
    }
    pub fn tick(&mut self) {
        self.duration += Duration::seconds(1);
    }

    pub fn formatted_duration(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.duration.num_hours(),
            self.duration.num_minutes() % 60,
            self.duration.num_seconds() % 60
        )
    }

    pub fn formatted_date(&self) -> String {
        self.start_time.format("%d-%m-%Y").to_string()
    }
}
