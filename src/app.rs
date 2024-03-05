pub enum CurrentScreen {
    Main,
    Reading,
    Exit,
}
pub struct App{
    pub current_screen:CurrentScreen,
}
impl App{
    pub fn new()->App{
        current_screen::Main,
    }
}


