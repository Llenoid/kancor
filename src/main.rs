use std::{io::{self, stdout}};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, Clear, ClearType, size},
    event::{
        read, Event, KeyCode, KeyEvent
    },
    cursor::{MoveTo},
    style::{Print}
};

struct Terminal;
impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn main() -> io::Result<()> {
    let message = "Welcome to the alternate screen!".to_string();
    enable_raw_mode()?;
    let _terminal = Terminal;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    render(&message)?;
    loop {
        if let Event::Key(key_event) = read()? {
            let distance = 2;
            render(&message)?;
            render_key_event(key_event, distance)?;
            if key_event.code == KeyCode::Char('q') {
                // execute!(stdout(), MoveTo(2, distance + 2), Print(format!("Stopping program: {:?}", key_event)))?;
                render_key_event(key_event, distance + 2)?;
                break;
            }
        }
    }
    Ok(())
}

fn render(message: &str) -> io::Result<()> {
    let (w, h) = size()?;
    let mes_len = (&message.len() / 2) as u16;
    let centered_height = h / 2;
    let centered_width = w / 2;
    let centered_text_width = centered_width - mes_len;
    execute!(stdout(), Clear(ClearType::All), MoveTo(centered_text_width, centered_height), Print(&message))?;
    Ok(())
}

fn render_key_event(key_event: KeyEvent, distance: u16) -> io::Result<()> {
    execute!(stdout(), MoveTo(2, distance), Print(format!("{:?}", key_event)))?;
    Ok(())
}
