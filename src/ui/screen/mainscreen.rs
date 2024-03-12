use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, List, ListItem, Padding},
    Frame,
};

use crate::ui::CITYLIGHT_WHITE;
use crate::{
    app::App,
    ui::{FooterLayout, TopLayout, PRIMARY_BLACK, PRIMARY_BLUE, SECONDARY_BLACK},
};

pub fn render_screen(f: &mut Frame, app: &mut App) {
    // popup content
    render_inner_info(f, app)
}

fn render_inner_info(f: &mut Frame, app: &mut App) {
    f.render_widget(Clear, f.size());
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
        String::from("List Mode"),
    );

    //main Layout
    let mut event = app.clone();
    let items: Vec<ListItem> = event
        .news_data
        .clone()
        .unwrap()
        .iter()
        .map(|i| {
            let content_span = Span::styled(
                i.id.clone() + "." + &i.title.clone(),
                Style::new().fg(PRIMARY_BLACK).add_modifier(Modifier::BOLD),
            );
            let date_span = Span::styled(
                format!("{}", i.date.clone()) + " | ",
                Style::new().fg(SECONDARY_BLACK),
            );
            let link_span = Span::styled(i.link.clone(), Style::new().fg(PRIMARY_BLUE));
            let total_line = Line::from(vec![date_span.clone(), link_span.clone()]);

            let mut text = Text::default();
            text.extend([Line::from(content_span), total_line.clone()]);
            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().fg(PRIMARY_BLACK))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("🧲 ");

    // footer layout description
    let helpkeybindings: Vec<(&str, &str)> = vec![
        (" <q> ", "Quit"),
        (" <o> ", "Open"),
        (" <h/↑> ", "Up"),
        (" <j/↓> ", "Down "),
    ];
    let uifooterlayout = FooterLayout::new(helpkeybindings);

    // rendering layout
    f.render_stateful_widget(list.clone(), chunks[1], &mut event.state);
    // render_scrollbar(f, app, chunks[1]);
    uitoplayout.render(chunks[0], f);
    uifooterlayout.render(chunks[2], f);
}

//
// fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
//     f.render_stateful_widget(
//         Scrollbar::default()
//             .orientation(ScrollbarOrientation::VerticalRight)
//             .begin_symbol(None)
//             .end_symbol(None),
//         area.inner(&Margin {
//             vertical: 1,
//             horizontal: 1,
//         }),
//         &mut app.scroll_state,
//     );
// }
