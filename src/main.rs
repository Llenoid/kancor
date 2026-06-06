use std::io::{self, stdout, Write};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};

struct OutputMessage {
    message: String,
}

impl OutputMessage {
    fn say_hello(&self) {
        println!("{}", self.message);
    }
}

impl Drop for OutputMessage {
    fn drop(&mut self) {
        print!("> Dropping {}\r\n", self.message);
        let _ = std::io::stdout().flush();
    }
}

fn main() -> io::Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    let guard = OutputMessage {
        message: "Welcome to the alternate screen!".to_string(),
    };
    guard.say_hello();
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(guard);
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
