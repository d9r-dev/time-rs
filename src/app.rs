use chrono::{DateTime, Duration, Utc};
use ratatui::widgets::TableState;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
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
    pub(crate) state: TableState,
}

#[derive(Debug)]
pub struct Timer {
    pub start_time: DateTime<Utc>,
    pub name: String,
    duration: Duration,
    pub description: String,
    pub id: usize,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            state: TableState::default().with_selected(0),
            timers: Vec::new(),
            current_screen: CurrentScreen::Main,
            name_input: String::new(),
            description_input: String::new(),
            currently_editing: None,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.timers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.timers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn add_timer(&mut self) {
        let timer = Timer::new(self.name_input.clone(), self.description_input.clone());
        match self.timers.last_mut() {
            Some(t) => t.stop(),
            None => (),
        }
        self.timers.push(timer);
        self.name_input = String::new();
        self.description_input = String::new();
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
}

impl Timer {
    pub fn new(name: String, description: String) -> Timer {
        Timer {
            start_time: Utc::now(),
            duration: Duration::zero(),
            name,
            description,
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            running: true,
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
        self.duration = self.get_duration()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn get_duration(&self) -> Duration {
        let now = Utc::now();
        now.signed_duration_since(self.start_time)
    }

    pub fn set_duration(&mut self) {
        if self.running {
            self.duration = self.get_duration();
        }
    }

    pub fn duration_seconds(&self) -> i64 {
        self.get_duration().num_seconds()
    }

    pub fn formatted_duration(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.duration.num_hours(),
            self.duration.num_minutes() % 60,
            self.duration.num_seconds() % 60
        )
    }
}
