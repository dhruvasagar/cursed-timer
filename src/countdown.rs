use std::time::{Duration, SystemTime};

const INSPECTING_THRESHOLD: u64 = 9;

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
        if self.time.elapsed().unwrap() >= self.duration {
            self.state = CountdownState::Done;
        }
    }

    pub fn remaining(&self) -> Duration {
        self.duration - self.time.elapsed().unwrap()
    }

    pub fn warn(&self) -> bool {
        self.remaining() < Duration::from_secs(INSPECTING_THRESHOLD)
    }

    pub fn done(&self) -> bool {
        self.duration <= self.time.elapsed().unwrap()
    }
}

use std::fmt;

impl fmt::Display for Countdown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current = match self.state {
            CountdownState::Done => Duration::from_secs(0),
            CountdownState::Start | CountdownState::Stop => self.remaining(),
            _ => self.duration,
        }
        .as_secs();
        write!(f, "{:0>2}", current)
    }
}
