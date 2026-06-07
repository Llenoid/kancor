use std::{io::{self, stdout}};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
    event::{
        read, Event, KeyCode,
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
    let _ = execute!(stdout(), EnterAlternateScreen, MoveTo(2, 2), Print(&message))?;
    enable_raw_mode()?;
    let _terminal = Terminal;
    let mut i: u16 = 0;
    loop {
        if let Event::Key(key_event) = read()? {
            i += 2;
            let distance = 2 + i;
            let _ = execute!(stdout(), MoveTo(2, distance), Print(format!("{:?}", key_event)));
            if key_event.code == KeyCode::Char('q') {
                let _ = execute!(stdout(), MoveTo(2, distance + 2), Print(format!("Stopping program: {:?}", key_event)));
                break;
            }
        }
    }
    Ok(())
}
