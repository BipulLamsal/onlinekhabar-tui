use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::ui::CITYLIGHT_WHITE;
use crate::ui::PRIMARY_BLUE;

pub struct TopLayout {
    heading: String,
    time: String,
}
impl TopLayout {
    pub fn new(heading: String, time: String) -> Self {
        TopLayout { heading, time }
    }
    pub fn render(&self, outer_layout: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(outer_layout);

        let left_title = Paragraph::new(self.heading.as_str())
            .style(Style::default().fg(CITYLIGHT_WHITE).bg(PRIMARY_BLUE))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0)),
            );
        let right_date = Paragraph::new(self.time.as_str())
            .right_aligned()
            .style(Style::default().fg(PRIMARY_BLUE))
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

//
// pub fn ui(f: &mut Frame, app: &App) {
//     match app.current_screen {
//         CurrentScreen::Loading => loading::render_screen(f, app),
//         CurrentScreen::Reading => println!("I am gay!"),
//         CurrentScreen::Main => println!("I am not gay!"),
//         CurrentScreen::Exit => println!("Exiting the bound"),
//     }
// }
