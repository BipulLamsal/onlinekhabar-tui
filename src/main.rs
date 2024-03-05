use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode,enable_raw_mode, LeaveAlternateScreen,EnterAlternateScreen};
use std::io;

fn run_app<B:Backend>(
    terminal: &mut Terminal<B>,
    app:&mut App,
    ) -> io::Result<bool>{

    loop {
        terminal.draw(|f| ui(f,app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
        }
        match app.current_screen {
            CurrentScreen::Main => match key.code {
                KeyCode::Char('o') => {
                    app.current_screen = CurrentScreen::Reading;
                }
                KeyCode::Char('O' => {
                    //to do - Opens Link directly to browser
                    app.current_screen = CurrentScreen::Main;
                })
                KeyCode::Char('q') => {
                    app.current_screen = CurrentScreen::Exit;
                }
                _ =>{}
            },
            CurrentScreen::Exit => match key.code {
                KeyCode::Char('y') => {
                    return Ok(true);
                }
                KeyCode::Char('n') | KeyCode::Char('q') => {
                    return Ok(false);
                }
                _ => {}
            },
            CurrentScreen::Reading => match key.code {
                KeyCode::Char('q') => {
                    app.current_screen = CurrentScreen::Main;
                }
            },
        }
    

    }


}



fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout;
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    //restore Terminal
    
    disable_raw_mode()?;
    execute!(stdout,LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
