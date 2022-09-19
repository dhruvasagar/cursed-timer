use crate::{
    history::History,
    scramble::Scramble,
    timer::{State, Timer},
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use std::time::{Duration, Instant};
use tui::{backend::Backend, Terminal};

const SCRAMBLE_LEN: usize = 25;
const HISTORY_FILE_PATH: &str = "history.csv";

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tick_rate: Duration,
    pub inspection: Duration,
    pub timer: Timer,
    pub history: History,
    pub scramble: Scramble,
    pub show_help: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            timer: Timer::new(),
            show_help: false,
            should_quit: false,
            tick_rate: Duration::from_millis(200),
            scramble: Scramble::new_rand(SCRAMBLE_LEN),
            history: History::from_csv(HISTORY_FILE_PATH),
            inspection: Duration::from_secs(15),
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
            self.should_quit = true
        }
        if self.show_help && key.code != KeyCode::Char('q') && self.show_help {
            return;
        }
        match key.code {
            KeyCode::Char('q') => {
                if self.show_help {
                    self.show_help = false;
                } else {
                    self.should_quit = true;
                    self.history.save_csv(HISTORY_FILE_PATH);
                }
            }
            KeyCode::F(1) | KeyCode::Char('?') | KeyCode::Char('h') => self.show_help = true,
            KeyCode::Char('c') => self.history.clear(),
            KeyCode::Char('s') => self.history.save_csv(HISTORY_FILE_PATH),
            KeyCode::Char('r') => self.scramble = Scramble::new_rand(SCRAMBLE_LEN),
            KeyCode::Char('x') => self.history.pop(),
            KeyCode::Char('u') => self.history.undo_pop(),
            KeyCode::Char(' ') => match self.timer.state {
                State::Active => {
                    self.timer.stop();
                    self.history.push(&self.timer, &self.scramble);
                    self.scramble = Scramble::new_rand(SCRAMBLE_LEN);
                }
                _ => self.timer.start(),
            },
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
            if last_tick.elapsed() >= self.tick_rate {
                last_tick = Instant::now();
            }
            if self.should_quit {
                return Ok(());
            }
        }
    }
}
