use ratatui::style::Color;
use ratatui::Frame;
//color schemes
pub const CITYLIGHT_WHITE: Color = Color::Rgb(223, 230, 233);
pub const PRIMARY_BLUE: Color = Color::Rgb(9, 132, 227);
pub const PRIMARY_BLACK: Color = Color::Rgb(45, 52, 54);
// pub const SECONDARY_WHITE: Color = Color::Rgb(178, 190, 195);
pub const SECONDARY_BLACK: Color = Color::Rgb(99, 110, 114);

use crate::app::App;
use crate::app::CurrentScreen;
use crate::ui::screen::{loading, mainscreen, warning};
mod footerlayout;
mod popup;
mod screen;
mod toplayout;
use footerlayout::FooterLayout;
use toplayout::TopLayout;

use self::screen::content;

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Loading => loading::render_screen(f),
        CurrentScreen::Reading => content::render_screen(f, app),
        CurrentScreen::Main => mainscreen::render_screen(f, app),
        CurrentScreen::Exit => popup::render_screen(f),
        CurrentScreen::Warning => warning::render_screen(f),
    }
}
