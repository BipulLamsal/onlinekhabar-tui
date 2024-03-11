use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use ratatui::Frame;

use super::{CITYLIGHT_WHITE, PRIMARY_BLUE};

pub fn render_screen(f: &mut Frame) {
    let area = centered_rect(60, 20, f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, area.height / 2 - 1, 0));
    let text = Paragraph::new("Are you sure you want to quit? [Y/N]")
        .block(block)
        .bg(PRIMARY_BLUE)
        .fg(CITYLIGHT_WHITE)
        .alignment(Alignment::Center);
    f.render_widget(text, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
