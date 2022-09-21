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

#[derive(PartialEq, Eq)]
pub enum AppState<'a> {
    Idle,
    Inspecting,
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
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
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
                KeyCode::Char('c') => self.history.clear(),
                KeyCode::Char('s') => self.history.save_csv(HISTORY_FILE_PATH),
                KeyCode::Char('r') => self.scramble = Scramble::new_rand(SCRAMBLE_LEN),
                KeyCode::Char('x') => self.state = AppState::Confirm("pop"), // self.history.pop(),
                KeyCode::Char('u') => self.history.undo_pop(),
                KeyCode::Char(' ') => {
                    self.state = AppState::Inspecting;
                    self.countdown.start();
                }
                KeyCode::Char('q') => {
                    self.state = AppState::ShouldQuit;
                    self.history.save_csv(HISTORY_FILE_PATH);
                }
                _ => {}
            },
            AppState::Inspecting => {
                if key.code == KeyCode::Char(' ') {
                    self.state = AppState::Timer;
                    self.countdown.stop();
                    self.timer.start();
                }
            }
            AppState::Timer => {
                self.state = AppState::Idle;
                self.timer.stop();
                self.history.push(&self.timer, &self.scramble, Penalty::No);
                self.scramble = Scramble::new_rand(SCRAMBLE_LEN);
            }
            AppState::Confirm("pop") => {
                match key.code {
                    KeyCode::Char('y') => {
                        self.history.pop();
                    }
                    _ => {}
                };
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
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.on_key(key)
                }
            }
            if self.state == AppState::Inspecting && self.countdown.done() {
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
