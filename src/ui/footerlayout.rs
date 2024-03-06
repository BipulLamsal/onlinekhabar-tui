use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use std::collections::HashMap;
pub struct FooterLayout<'a> {
    helpkeys: HashMap<char, &'a str>,
}
impl<'a> FooterLayout<'a> {
    pub fn new(helpkeys: HashMap<char, &'a str>) -> Self {
        FooterLayout { helpkeys }
    }
    pub fn render(&self, outer_layout: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(outer_layout);

        let left_title = Paragraph::new("hello")
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(87, 88, 187)),
            )
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0)),
            );
        let right_date = Paragraph::new("hola")
            .right_aligned()
            .style(Style::default().fg(Color::Rgb(87, 88, 187)))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0)),
            );
        f.render_widget(left_title, chunks[0]);
        f.render_widget(right_date, chunks[1]);
    }
}
