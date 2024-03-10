use crate::{app::App, ui::PRIMARY_BLUE};
// use crate::ui::FooterLayout;
// use crate::ui::HashMap;
// use crate::ui::TopLayout;
use crate::ui::CITYLIGHT_WHITE;
use ratatui::layout::{Alignment, Flex};
use ratatui::widgets::{BorderType, Borders, Paragraph, Wrap};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Padding},
    Frame,
};

pub fn render_screen(f: &mut Frame) {
    let white_block = Block::default()
        .style(Style::default().bg(CITYLIGHT_WHITE))
        .padding(Padding::new(1, 1, 1, 1));

    let inner_area = white_block.inner(f.size());
    f.render_widget(white_block, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .flex(Flex::Center)
        .split(inner_area);

    let loading_block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, inner_area.height / 2, 0))
        .bg(PRIMARY_BLUE)
        .fg(CITYLIGHT_WHITE);
    let loading_paragraph = Paragraph::new("Cooking some latest news 🍲")
        .block(loading_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(loading_paragraph, chunks[0]);
    //rednering the layout

    // rendering layout
    // uitoplayout.render(chunks[0], f);
    // uifooterlayout.render(chunks[2], f);
}
