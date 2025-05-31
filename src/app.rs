use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Exit,
}

#[derive(Debug)]
pub struct App {
    pub timers: Vec<Timer>,
    pub current_screen: CurrentScreen,
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
    pub fn new() -> App {
        App {
            timers: Vec::new(),
            current_screen: CurrentScreen::Main,
        }
    }
}

impl Timer {
    pub fn new(name: &str, description: &str, id: usize) -> Timer {
        Timer {
            start_time: Utc::now(),
            duration: Duration::zero(),
            name: String::from(name),
            description: String::from(description),
            id,
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
