use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

#[cfg(feature = "debug")]
use std::fs::OpenOptions;

mod app;
mod config;
mod countdown;
mod history;
mod scramble;
mod stats;
mod timer;
mod ui;

fn main() -> io::Result<()> {
    #[cfg(feature = "debug")]
    tracing_subscriber::fmt()
        .with_writer(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open("cursed-timer.log")?,
        )
        .init();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture,)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new("Rubik Cube Timer");
    let res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
