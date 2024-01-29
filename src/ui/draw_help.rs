use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_help<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());
    let help_block = Block::default()
        .title("Help")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    let text = vec![
        Spans::from(""),
        Spans::from("    ?: Show Help"),
        Spans::from("    q: Quit Help / Quit Application"),
        Spans::from("    c: Clear History"),
        Spans::from("    s: Save History to file"),
        Spans::from("    r: Refresh Scramble"),
        Spans::from("    x: Delete last recorded time from History"),
        Spans::from("    u: Undo delete from History"),
        Spans::from("    d: Mark last entry as DNF penalty"),
        Spans::from("    t: Mark last entry as Time penalty"),
        Spans::from("    <Space>: Start Inspection -> Start Timer -> Stop Timer"),
        Spans::from("    Ctrl <Space>: Start Timer Without Starting Inspection"),
    ];
    let paragraph = Paragraph::new(text).block(help_block);
    f.render_widget(paragraph, chunks[0]);
}
