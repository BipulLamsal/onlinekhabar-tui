use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2)])
        .split(f.size());

    let p = Paragraph::new("online khabar english [en]")
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb((56), (103), (214))),
        )
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );

    f.render_widget(p, chunks[0]);
}
