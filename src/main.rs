use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    terminal.draw(|f| ui(f, app))?;
    let fetched_news = App::news_fetch(20);
    match fetched_news {
        None => app.current_screen = CurrentScreen::Warning,
        Some(i) => app.set_data(Some(i)),
    }
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                //loading screen
                CurrentScreen::Loading => match key.code {
                    _ => {}
                },

                //mainscreen key combinations
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('o') => {
                        app.current_screen = CurrentScreen::Reading;
                    }
                    KeyCode::Char('O') => {
                        //to do - Opens Link directly to browser
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    KeyCode::Up | KeyCode::Char('h') => {
                        app.previous();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.next();
                    }
                    KeyCode::Enter => {}
                    _ => {}
                },

                //exit screen key combinations
                CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main
                    }
                    _ => {}
                },
                //warnign screen quit method
                CurrentScreen::Warning => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {}
                },

                //reading screen key combinations
                CurrentScreen::Reading => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Up | KeyCode::Char('h') => {
                        app.scroll = app.scroll.saturating_add(1);
                        app.scroll_state = app.scroll_state.position(app.scroll);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.scroll = app.scroll.saturating_sub(1);
                        app.scroll_state = app.scroll_state.position(app.scroll);
                    }
                    _ => {}
                },
            }
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let stdout = io::stdout;
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    //restore Terminal

    disable_raw_mode()?;
    // execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
