use std::time::{Duration, SystemTime};

#[derive(Copy, Clone, PartialEq)]
pub enum CountdownState {
    Idle,
    Start,
    Stop,
    Done,
}

#[derive(Copy, Clone)]
pub struct Countdown {
    time: SystemTime,
    duration: Duration,
    pub state: CountdownState,
}

impl Countdown {
    pub fn new(duration: Duration) -> Self {
        Countdown {
            duration,
            time: SystemTime::now(),
            state: CountdownState::Idle,
        }
    }

    pub fn start(&mut self) {
        self.time = SystemTime::now();
        self.state = CountdownState::Start;
    }

    pub fn stop(&mut self) {
        self.state = CountdownState::Stop;
        if self.time.elapsed().unwrap() > self.duration {
            self.state = CountdownState::Done;
        }
    }
}

use std::fmt;

impl fmt::Display for Countdown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current = match self.state {
            CountdownState::Done => Duration::from_secs(0),
            CountdownState::Start | CountdownState::Stop => {
                self.duration - self.time.elapsed().unwrap()
            }
            _ => self.duration,
        }
        .as_secs();
        write!(f, "{:0>2}", current)
    }
}
