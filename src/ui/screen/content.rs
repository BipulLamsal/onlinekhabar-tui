use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::{
    app::App,
    ui::{FooterLayout, TopLayout, CITYLIGHT_WHITE, PRIMARY_BLACK},
};

pub fn render_screen(f: &mut Frame, app: &mut App) {
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

    let create_block = || {
        Block::default()
            .borders(Borders::NONE)
            .style(Style::default().fg(PRIMARY_BLACK))
    };

    let selected_index = app.state.selected().unwrap();
    let text = app.clone().news_data.unwrap()[selected_index]
        .clone()
        .content;
    app.scroll_state = app.scroll_state.content_length(30);
    let paragraph = Paragraph::new(text.clone())
        .block(create_block())
        .wrap(Wrap { trim: true })
        .scroll((app.scroll as u16, 0));
    f.render_widget(paragraph, chunks[1]);
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.scroll_state,
    );
    let helpkeybindings: Vec<(&str, &str)> =
        vec![(" <q> ", "Quit"), (" <h/↑> ", "Up"), (" <j/↓> ", "Down ")];
    let uifooterlayout = FooterLayout::new(helpkeybindings);
    let uitoplayout = TopLayout::new(
        app.clone().news_data.unwrap()[selected_index].clone().title,
        "Reading Mode".to_string(),
    );
    uitoplayout.render(chunks[0], f);
    uifooterlayout.render(chunks[2], f);
}
