use crate::app::App;
use cfonts::{render, Fonts, Options};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_inspecting<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());
    let timer_block = Block::default()
        .title("Timer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let timer_font = render(Options {
        text: app.countdown.to_string(),
        font: Fonts::FontHuge,
        ..Options::default()
    });
    let timer_text = Text::styled(
        format!("{}", timer_font.text),
        Style::default().fg(Color::LightGreen),
    );
    let paragraph = Paragraph::new(timer_text)
        .block(timer_block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);
}
