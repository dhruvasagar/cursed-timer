use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use super::draw_idle;

pub fn draw_confirm_dialog<B: Backend>(f: &mut Frame<B>, app: &App) {
    draw_idle(f, app);

    let area = centered_rect(30, 20, f.size());

    let confirm_block = Block::default().borders(Borders::ALL);
    let text = Spans::from(vec![Span::styled(
        "Are you sure (y/n) ?",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )]);
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(confirm_block);
    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
