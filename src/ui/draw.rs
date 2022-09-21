use crate::app::{App, AppState};
use tui::{backend::Backend, Frame};

use super::{draw_confirm_dialog, draw_help, draw_idle, draw_inspecting, draw_timer};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.state {
        AppState::ShowHelp => draw_help(f),
        AppState::Idle => draw_idle(f, app),
        AppState::Inspecting => draw_inspecting(f, app),
        AppState::Timer => draw_timer(f, app),
        AppState::Confirm(_) => draw_confirm_dialog(f, app),
        _ => {}
    }
}
