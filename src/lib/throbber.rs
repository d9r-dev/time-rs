#[derive(Debug, Clone, Copy)]
enum ThrobberState {
    One = 0,
    Two,
    Three,
    Four,
}

impl ThrobberState {
    fn next(&self) -> ThrobberState {
        use ThrobberState::*;
        match *self {
            One => Two,
            Two => Three,
            Three => Four,
            Four => One,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Throbber {
    state: ThrobberState,
}

impl Throbber {
    pub fn new() -> Throbber {
        Throbber {
            state: ThrobberState::One,
        }
    }

    pub fn tick(&mut self) {
        self.state = self.state.next()
    }

    pub fn get_state_string(&self) -> &str {
        match self.state {
            ThrobberState::One => "-",
            ThrobberState::Two => "\\",
            ThrobberState::Three => "|",
            ThrobberState::Four => "/",
        }
    }
}
