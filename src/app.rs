use serde_json::Value;
pub enum CurrentScreen {
    Loading,
    Main,
    Reading,
    Exit,
}
//news json formating
#[derive(Debug)]
pub struct News {
    pub title: String,
    pub link: String,
    pub content: String,
    pub date: String,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub news_data: Option<Vec<News>>,
}
impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Loading,
            news_data: None,
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
                            title: title.to_string(),
                            link: link.to_string(),
                            date: date.to_string(),
                            content: content.to_string(),
                        };
                        news_collection.push(news_struct)
                    }
                }
                Some(news_collection)
            }
            None => None,
        }
    }
}
