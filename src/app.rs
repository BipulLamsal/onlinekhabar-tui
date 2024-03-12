use ratatui::widgets::{ListState, ScrollbarState};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum CurrentScreen {
    Loading,
    Main,
    Reading,
    Exit,
    Warning,
}

//news json formating
#[derive(Debug, Clone)]
pub struct News {
    pub id: String,
    pub title: String,
    pub link: String,
    pub content: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub news_data: Option<Vec<News>>,
    pub state: ListState,
    pub scroll: usize,
    pub scroll_state: ScrollbarState,
}
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Loading,
            news_data: None,
            state: ListState::default().with_selected(Some(0)),
            scroll: 0,
            scroll_state: ScrollbarState::default(),
        }
    }
    pub fn set_data(&mut self, news_data: Option<Vec<News>>) {
        self.news_data = news_data;
        self.current_screen = CurrentScreen::Main;
    }
    pub fn news_fetch(number: i32) -> Option<Vec<News>> {
        let api_url = format!(
            "https://english.onlinekhabar.com/wp-json/wp/v2/posts?per_page={}",
            number
        );
        let res = reqwest::blocking::get(api_url).ok();
        let mut news_collection: Vec<News> = Vec::new();
        let mut counter = 1;
        match res {
            Some(res) => {
                if res.status().is_success() {
                    let data: Value = res.json().expect("Unable to parse the json form");
                    let new_data = data.as_array()?;
                    for item in new_data {
                        let object = item.as_object()?;
                        let title = object.get("title")?.get("rendered")?.as_str()?;
                        let date = object.get("date")?.as_str()?;
                        let content = object.get("content")?.get("rendered")?.as_str()?;
                        let link = object.get("link")?.as_str()?;
                        let news_struct = News {
                            id: counter.to_string(),
                            title: title.to_string(),
                            link: link.to_string(),
                            date: date.to_string(),
                            content: content.to_string(),
                        };
                        news_collection.push(news_struct);
                        counter += 1;
                    }
                }
                Some(news_collection)
            }
            None => None,
        }
    }

    //here goes the operation of the stateful list present in the list
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.news_data.clone().unwrap().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.news_data.clone().unwrap().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    //
    // pub fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}
