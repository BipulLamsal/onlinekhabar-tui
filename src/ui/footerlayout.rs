use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::Style,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Borders,
    },
    Frame,
};

use super::PRIMARY_BLACK;
use super::PRIMARY_BLUE;

pub struct FooterLayout<'a> {
    // helpkeys: HashMap<&'a str, &'a str>,
    helpkeys: Vec<(&'a str, &'a str)>,
}
impl<'a> FooterLayout<'a> {
    pub fn new(helpkeys: Vec<(&'a str, &'a str)>) -> Self {
        FooterLayout { helpkeys }
    }
    pub fn render(&self, outer_layout: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1)])
            .split(outer_layout);
        //converting hashmap to vector in series all of str type
        let key_style = Style::new().bold().fg(PRIMARY_BLUE);
        let value_style = Style::new().fg(PRIMARY_BLACK);
        let mut help_binding_vector: Vec<Span> = Vec::new();

        for (key, value) in self.helpkeys.iter() {
            help_binding_vector.push(Span::styled(key.to_string(), key_style));
            help_binding_vector.push(Span::styled(value.to_string(), value_style));
        }

        let instructional_footer = Title::from(Line::from(help_binding_vector));
        let footer_block = Block::new()
            .title(
                instructional_footer
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .style(Style::default().fg(PRIMARY_BLACK))
            .borders(Borders::TOP)
            .border_set(border::PLAIN);

        f.render_widget(footer_block, chunks[0]);
    }
}
