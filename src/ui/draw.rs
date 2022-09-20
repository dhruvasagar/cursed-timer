use crate::{app::App, timer::State};
use tui::{backend::Backend, Frame};

use super::{draw_help, draw_timer_active, draw_timer_inactive};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    if app.show_help {
        return draw_help(f);
    }
    match app.timer.state {
        State::Active => draw_timer_active(f, app),
        _ => draw_timer_inactive(f, app),
    }
}
