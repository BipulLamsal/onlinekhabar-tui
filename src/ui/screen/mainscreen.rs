use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

use crate::ui::CITYLIGHT_WHITE;
use crate::{
    app::App,
    ui::{FooterLayout, TopLayout},
};

pub fn render_screen(f: &mut Frame, app: &App) {
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

    //main layout

    // footer layout description
    let helpkeybindings: HashMap<&str, &str> =
        HashMap::from([(" <q> ", "Quit"), (" <o> ", "Open")]);
    let uifooterlayout = FooterLayout::new(helpkeybindings);

    // rendering layout
    uitoplayout.render(chunks[0], f);
    uifooterlayout.render(chunks[2], f);
}
