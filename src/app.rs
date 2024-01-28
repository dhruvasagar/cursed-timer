use crate::{
    countdown::Countdown,
    history::{History, Penalty},
    scramble::Scramble,
    timer::Timer,
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use std::time::{Duration, Instant};
use tui::{backend::Backend, Terminal};

const SCRAMBLE_LEN: usize = 25;
const HISTORY_FILE_PATH: &str = "history.csv";
const WCA_INSPECTION: u64 = 16;
const KEY_HOLD: u64 = 2;

#[derive(PartialEq, Eq)]
pub enum AppState<'a> {
    Idle,
    Inspecting,
    KeyHold,
    Timer,
    ShowHelp,
    ShouldQuit,
    Confirm(&'a str),
}

pub struct App<'a> {
    pub title: &'a str,
    pub tick_rate: Duration,
    pub timer: Timer,
    pub history: History,
    pub scramble: Scramble,
    pub state: AppState<'a>,
    pub countdown: Countdown,
    pub key_hold: Countdown,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            timer: Timer::new(),
            state: AppState::Idle,
            tick_rate: Duration::from_millis(100),
            scramble: Scramble::new_rand(SCRAMBLE_LEN),
            history: History::from_csv(HISTORY_FILE_PATH),
            countdown: Countdown::new(Duration::from_secs(WCA_INSPECTION)),
            key_hold: Countdown::new(Duration::from_secs(KEY_HOLD)),
        }
    }

    pub fn on_key<B: Backend>(&mut self, key: KeyEvent, terminal: &mut Terminal<B>) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.state = AppState::ShouldQuit;
            return;
        }
        match self.state {
            AppState::ShowHelp => {
                if key.code == KeyCode::Char('q') {
                    self.state = AppState::Idle;
                }
            }
            AppState::Idle => match key.code {
                KeyCode::F(1) | KeyCode::Char('?') | KeyCode::Char('h') => {
                    self.state = AppState::ShowHelp
                }
                KeyCode::Char('q') => {
                    self.state = AppState::ShouldQuit;
                    self.history.save_csv(HISTORY_FILE_PATH);
                }
                KeyCode::Char('c') => self.state = AppState::Confirm("clear"),
                KeyCode::Char('s') => self.history.save_csv(HISTORY_FILE_PATH),
                KeyCode::Char('r') => self.scramble = Scramble::new_rand(SCRAMBLE_LEN),
                KeyCode::Char('x') => self.state = AppState::Confirm("pop"),
                KeyCode::Char('u') => self.history.undo_pop(),
                KeyCode::Char('d') => self.state = AppState::Confirm("dnf"),
                KeyCode::Char('t') => self.state = AppState::Confirm("time"),
                KeyCode::Char(' ') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        self.timer.start();
                        self.state = AppState::Timer;
                    } else {
                        #[cfg(feature = "debug")]
                        tracing::info!("Starting Inspection");
                        self.state = AppState::Inspecting;
                        self.countdown.start();
                    }
                }
                _ => {}
            },
            AppState::Inspecting => {
                if key.code == KeyCode::Char(' ') {
                    #[cfg(feature = "debug")]
                    tracing::info!("Starting KeyHold");
                    self.key_hold.start();
                    self.state = AppState::KeyHold;
                    // self.state = AppState::Timer;
                    // self.countdown.stop();
                    // self.timer.start();
                }
            }
            AppState::KeyHold => {
                #[cfg(feature = "debug")]
                tracing::info!("Polling for space");
                while let Ok(true) = crossterm::event::poll(Duration::from_millis(100)) {
                    if self.countdown.done() {
                        return;
                    }

                    terminal.draw(|f| ui::draw(f, self)).unwrap();
                    if let Ok(Event::Key(k)) = event::read() {
                        if k.code != KeyCode::Char(' ') {
                            break;
                        }
                    }
                }

                if !self.key_hold.done() {
                    if !self.countdown.done() {
                        // go back to inspecting
                        self.state = AppState::Inspecting;
                    } else {
                        self.state = AppState::Idle;
                    }
                    // did not hold long enough
                    self.key_hold.stop();
                    return;
                }

                self.state = AppState::Timer;
                self.key_hold.stop();
                self.countdown.stop();
                self.timer.start();
            }
            AppState::Timer => {
                self.state = AppState::Idle;
                self.timer.stop();
                self.history.push(&self.timer, &self.scramble, Penalty::No);
                self.scramble = Scramble::new_rand(SCRAMBLE_LEN);
            }
            AppState::Confirm(s) => {
                match s {
                    "pop" => {
                        if key.code == KeyCode::Char('y') {
                            self.history.pop();
                        }
                    }
                    "dnf" => {
                        if key.code == KeyCode::Char('y') {
                            self.history.penalize_last(Penalty::DNF);
                        }
                    }
                    "time" => {
                        if key.code == KeyCode::Char('y') {
                            self.history.penalize_last(Penalty::Time);
                        }
                    }
                    "clear" => {
                        if key.code == KeyCode::Char('y') {
                            self.timer.reset();
                            self.history.clear();
                        }
                    }
                    _ => {}
                }
                self.state = AppState::Idle;
            }
            _ => {}
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            let timeout = self
                .tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_millis(100));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.on_key(key, terminal)
                }
            }
            if (self.state == AppState::Inspecting || self.state == AppState::KeyHold)
                && self.countdown.done()
            {
                // consume extra space press
                while let Ok(true) = crossterm::event::poll(Duration::from_millis(100)) {
                    if let Ok(Event::Key(k)) = event::read() {
                        if k.code != KeyCode::Char(' ') {
                            break;
                        }
                    }
                }
                self.state = AppState::Idle;
                self.countdown.stop();
                self.history.push(&self.timer, &self.scramble, Penalty::DNS);
                self.scramble = Scramble::new_rand(SCRAMBLE_LEN);
            }
            if last_tick.elapsed() >= self.tick_rate {
                last_tick = Instant::now();
            }
            if self.state == AppState::ShouldQuit {
                return Ok(());
            }
        }
    }
}
