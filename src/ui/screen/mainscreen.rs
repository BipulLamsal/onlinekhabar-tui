use std::collections::HashMap;

use crossterm::{event::Event, style::ContentStyle};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap,
    },
    Frame,
};

use crate::{
    app::App,
    ui::{FooterLayout, TopLayout, PRIMARY_BLACK, PRIMARY_BLUE, SECONDARY_BLACK},
};
use crate::{app::News, ui::CITYLIGHT_WHITE};

struct Events {
    items: Vec<News>,
    state: ListState,
}
impl Events {
    fn new(items: Vec<News>) -> Events {
        Events {
            items,
            state: ListState::default(),
        }
    }
    fn set_item(&mut self, items: Vec<News>) {
        self.items = items;
        self.state = ListState::default();
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

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

    //main Layout
    let news_vec = app.news_data.clone().unwrap();
    let mut events = Events::new(news_vec);
    let items: Vec<ListItem> = events
        .items
        .iter()
        .map(|i| {
            let content_span = Span::styled(
                i.title.clone(),
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
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    // footer layout description
    let helpkeybindings: HashMap<&str, &str> =
        HashMap::from([(" <q> ", "Quit"), (" <o> ", "Open")]);
    let uifooterlayout = FooterLayout::new(helpkeybindings);

    // rendering layout
    f.render_stateful_widget(list, chunks[1], &mut events.state);
    uitoplayout.render(chunks[0], f);
    uifooterlayout.render(chunks[2], f);
}
