use std::{alloc::System, collections::HashMap};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
mod footerlayout;
mod toplayout;

use footerlayout::FooterLayout;
use toplayout::TopLayout;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(f.size());

    // top layout description
    let uitoplayout = TopLayout::new(
        String::from("online khabar [en]"),
        String::from("06 March,2024"),
    );

    // footer layout description
    let helpkeybindings: HashMap<char, &str> = HashMap::from([('q', "Quit"), ('o', "Open")]);
    let uifooterlayout = FooterLayout::new(helpkeybindings);

    // rendering layout
    uitoplayout.render(chunks[0], f);
    uifooterlayout.render(chunks[2], f);

    // let p = Paragraph::new("online khabar english [en]")
    //     .style(
    //         Style::default()
    //             .fg(Color::White)
    //             .bg(Color::Rgb((56), (103), (214))),
    //     )
    //     .block(
    //         Block::default()
    //             .borders(Borders::BOTTOM)
    //             .border_type(BorderType::Rounded)
    //             .padding(Padding::new(1, 1, 0, 0)),
    //     );
    //
    // f.render_widget(p, chunks[0]);
}
