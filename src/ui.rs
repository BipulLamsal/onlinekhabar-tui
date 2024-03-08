use std::collections::HashMap;

use crossterm::style;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

//color schemes
pub const CITYLIGHT_WHITE: Color = Color::Rgb(223, 230, 233);
pub const PRIMARY_BLUE: Color = Color::Rgb(9, 132, 227);
pub const PRIMARY_BLACK: Color = Color::Rgb(45, 52, 54);
pub const SECONDARY_WHITE: Color = Color::Rgb(178, 190, 195);
pub const SECONDARY_BLACK: Color = Color::Rgb(99, 110, 114);

use crate::app::App;
mod footerlayout;
mod toplayout;

use footerlayout::FooterLayout;
use toplayout::TopLayout;

pub fn ui(f: &mut Frame, app: &App) {
    let white_block = Block::default()
        .style(Style::default().bg(CITYLIGHT_WHITE))
        .padding(Padding::new(1, 1, 1, 1));

    let inner_area = white_block.inner(f.size());
    f.render_widget(white_block, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(4),
            Constraint::Length(1),
        ])
        .split(inner_area);

    // top layout description
    let uitoplayout = TopLayout::new(
        String::from("online khabar [en]"),
        String::from("06 March,2024"),
    );

    // footer layout description
    let helpkeybindings: HashMap<&str, &str> =
        HashMap::from([(" <q> ", "Quit"), (" <o> ", "Open")]);
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
