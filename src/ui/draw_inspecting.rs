use crate::{app::App, countdown::CountdownState};
use cfonts::{render, Fonts, Options};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_inspecting<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
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
    let mut timer_text = Text::styled(
        format!("{}", timer_font.text),
        Style::default().fg(Color::White),
    );
    if app.key_hold.state == CountdownState::Start {
        timer_text.patch_style(Style::default().fg(Color::Green));
    }
    if app.countdown.warn() {
        timer_text.patch_style(Style::default().fg(Color::LightRed));
    }
    let paragraph = Paragraph::new(timer_text)
        .block(timer_block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    let help_block = Block::default().borders(Borders::ALL);
    let text = vec![Spans::from("Press <Space> for 3 seconds to Start Timer")];
    let paragraph = Paragraph::new(text)
        .block(help_block)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[1]);
}
