use crate::{app::App, time::State};
use cfonts::{render, Fonts, Options};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.timer.state {
        State::Active => draw_timer_active(f, app),
        _ => draw_timer_inactive(f, app),
    }
}

pub fn draw_timer_active<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());
    let timer_block = Block::default()
        .title("Timer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let timer_font = render(Options {
        text: app.timer.to_string(),
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

pub fn draw_timer_inactive<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);

    let left_top_pane = Block::default()
        .title("History")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let left_bottom_pane = Block::default()
        .title("Stats")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let right_top_pane = Block::default()
        .title("Scramble")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let right_bottom_pane = Block::default()
        .title("Timer")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);

    let summary = app.history.summarize(app.history.len());
    let items: Vec<ListItem> = summary
        .iter()
        .enumerate()
        .map(|(i, h)| ListItem::new(format!("{}: {}", i + 1, h)))
        .collect();
    let list = List::new(items).block(left_top_pane);
    f.render_widget(list, left_chunks[0]);

    let stats = app.history.stats();
    let mut rows: Vec<Row> = vec![];
    for stat in stats.iter() {
        let mut row: Vec<Cell> = vec![];
        for val in stat.iter() {
            row.push(Cell::from(val.as_str()));
        }
        rows.push(Row::new(row));
    }
    let table = Table::new(rows)
        .header(Row::new(vec!["", "current", "best"]))
        .block(left_bottom_pane)
        .widths(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .column_spacing(1);
    f.render_widget(table, left_chunks[1]);

    let scramble_text = Spans::from(vec![Span::styled(
        format!("{}", app.scramble.to_string().as_str()),
        Style::default()
            .fg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
    )]);
    let paragraph = Paragraph::new(scramble_text)
        .block(right_top_pane)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, right_chunks[0]);

    let timer_font = render(Options {
        text: app.timer.to_string(),
        font: Fonts::FontHuge,
        ..Options::default()
    });
    let timer_text = Text::styled(
        format!("{}", timer_font.text),
        Style::default().fg(Color::LightGreen),
    );
    let paragraph = Paragraph::new(timer_text)
        .block(right_bottom_pane)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, right_chunks[1]);
}
