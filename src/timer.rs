use std::time::{Duration, SystemTime};

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Active,
    Inactive,
}

#[derive(Copy, Clone)]
pub struct Timer {
    pub state: State,
    time: SystemTime,
    pub result: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            state: State::Inactive,
            time: SystemTime::now(),
            result: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.time = SystemTime::now();
        self.state = State::Active;
    }

    pub fn stop(&mut self) {
        self.result = self.time.elapsed().unwrap();
        self.state = State::Inactive;
    }
}

use std::fmt;

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current = if self.state == State::Active {
            self.time.elapsed().unwrap()
        } else {
            self.result
        }
        .as_millis();

        write!(f, "{:0>2}.{:0>3}", current / 1000, current % 1000)
    }
}
