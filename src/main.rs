use std::{io::{self, stdout}};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    event::{
        read, Event, KeyCode
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

struct Todo {
    title: String,
    body: String,
    is_completed: bool
}

struct AppState {
    todos: Vec<Todo>,
    selected: usize,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let _terminal = Terminal;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    // render(&message)?;
    let mut todos = Vec::new();
    let todo_1 = Todo {
        title: String::from("This is a title"),
        body: String::from("This is a body"),
        is_completed: false,
    };

    let todo_2 = Todo {
        title: String::from("This is another title"),
        body: String::from("This is another body"),
        is_completed: false,
    };
    todos.push(todo_1);
    todos.push(todo_2);
    let mut app_state = AppState { todos: todos, selected: 0 };
    loop {
        render(&app_state)?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('j') => {
                    if app_state.selected < app_state.todos.len() - 1 {
                        app_state.selected += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if app_state.selected > 0 {
                        app_state.selected -= 1;
                    }
                }
                KeyCode::Char('q') => {
                    let distance = 2;
                    execute!(stdout(), MoveTo(2, distance + 2), Print(format!("Stopping program: {:?}", key_event)))?;
                    // render_key_event(key_event, distance + 2)?;
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(app_state: &AppState) -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    for (i, todo) in app_state.todos.iter().enumerate() {
        let mut message = format!("{}", &todo.title);
        if app_state.selected == i {
            message = format!(">{}", &todo.title);
        }
        execute!(stdout(), MoveTo(2, 2 + i as u16), Print(message))?;
    }
    Ok(())
}
