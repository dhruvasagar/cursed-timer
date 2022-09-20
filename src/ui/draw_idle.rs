use crate::app::App;
use cfonts::{render, Fonts, Options};
use chrono::NaiveDateTime;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::Marker::Dot,
    text::{Span, Spans, Text},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, List, ListItem, ListState, Paragraph, Row,
        Table,
    },
    Frame,
};

pub fn draw_idle<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let left_top_pane = Block::default()
        .title("History")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let summary = app.history.summarize();
    let items: Vec<ListItem> = summary
        .iter()
        .enumerate()
        .map(|(i, h)| ListItem::new(format!("{}: {}", i + 1, h)))
        .collect();
    let list = List::new(items).block(left_top_pane);
    let mut state = ListState::default();
    state.select(Some(summary.len() as usize - 1));
    f.render_stateful_widget(list, left_chunks[0], &mut state);

    let left_middle_pane = Block::default()
        .title("Stats")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
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
        .block(left_middle_pane)
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

    let left_bottom_pane = Block::default()
        .title("Plot")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let (points, xbounds, ybounds) = app.history.points();
    let datasets = vec![Dataset::default()
        .name("Solve Times")
        .marker(Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&points)];
    let chart = Chart::new(datasets)
        .block(left_bottom_pane)
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::White))
                .bounds(xbounds)
                .labels(
                    xbounds
                        .iter()
                        .cloned()
                        .map(|x| {
                            Span::from(format!(
                                "{}",
                                NaiveDateTime::from_timestamp(x as i64, 0).format("%H:%M:%S")
                            ))
                        })
                        .collect(),
                ),
        )
        .y_axis(
            Axis::default()
                .title("Solve Times")
                .style(Style::default().fg(Color::White))
                .bounds(ybounds)
                .labels(
                    ybounds
                        .iter()
                        .cloned()
                        .map(|y| Span::from(format!("{}", y)))
                        .collect(),
                ),
        );
    f.render_widget(chart, left_chunks[2]);

    let right_top_pane = Block::default()
        .title("Scramble")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
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

    let right_middle_pane = Block::default()
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
        .block(right_middle_pane)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, right_chunks[1]);

    let right_bottom_pane = Block::default().borders(Borders::ALL);
    let table = Table::new(vec![Row::new(vec![
        "Press <Space> to Start Inspection",
        "Press ? to Show Help",
        "Press q to Quit Application",
    ])])
    .block(right_bottom_pane)
    .widths(
        [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]
        .as_ref(),
    )
    .column_spacing(1);
    f.render_widget(table, right_chunks[2]);
}
