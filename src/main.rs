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

enum ColumnType {
    Unassigned,
    New,
    Backlog,
    Pending,
    Done,
}

struct Column {
    column_type: ColumnType,
    name: String,
    todos: Vec<Todo>,
    selected: usize,
}

struct AppState {
    columns: Vec<Column>,
    selected_column: usize
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let _terminal = Terminal;
    let _ = execute!(stdout(), EnterAlternateScreen)?;
    // render(&message)?;
    let mut todos_1 = Vec::new();
    let mut todos_2 = Vec::new();
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

    todos_1.push(todo_1);
    todos_2.push(todo_2);
    let unassigned_header = "Unassigned".to_string();
    let done_header = "Done".to_string();
    let unassigned_col = Column { column_type: ColumnType::Unassigned, name: unassigned_header, todos: todos_1, selected: 0 };
    let done_col = Column { column_type: ColumnType::Done, name: done_header, todos: todos_2, selected: 0 };
    let mut cols = Vec::new();

    cols.push(unassigned_col);
    cols.push(done_col);
    let mut app_state = AppState { columns: cols, selected_column: 1 };

    loop {
        render(&app_state)?;
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('h') => {
                    if app_state.selected_column > 0 {
                        app_state.selected_column -= 1;
                    }
                }
                KeyCode::Char('l') => {
                    if app_state.selected_column < app_state.columns.len() - 1 {
                        app_state.selected_column += 1;
                    }
                }
                KeyCode::Char('j') => {
                    let col = &mut app_state.columns[app_state.selected_column];
                    if !col.todos.is_empty() && col.selected < col.todos.len() - 1 {
                        col.selected += 1;
                    }
                }
                KeyCode::Char('k') => {
                    let col = &mut app_state.columns[app_state.selected_column];
                    if !col.todos.is_empty() && col.selected > 0 {
                        col.selected -= 1;
                    }
                }
                KeyCode::Char('q') => {
                    let distance = 2;
                    execute!(stdout(), MoveTo(2, distance + 2), Print(format!("Stopping program: {:?}", key_event)))?;
                    // render_key_event(key_event, distance + 2)?;
                    break;
                }
                KeyCode::Enter => {
                    let current_col = app_state.selected_column;
                    // if the current_col is not the last col
                    if current_col < app_state.columns.len() - 1 {
                        let todo_index = app_state.columns[current_col].selected;
                        // if the current column has any todos
                        if !app_state.columns[current_col].todos.is_empty() {
                            let todo = app_state.columns[current_col].todos.remove(todo_index);
                            // Go to the next column Vec and then push
                            app_state.columns[current_col + 1].todos.push(todo);
                            let dest_len = app_state.columns[current_col + 1].todos.len();
                            app_state.columns[current_col + 1].selected = dest_len - 1;
                            // fix selected todo if it's out of bounds
                            let len = app_state.columns[current_col].todos.len();
                            if len == 0 {
                                app_state.columns[current_col].selected = 0;
                            } else if  app_state.columns[current_col].selected >= len {
                                app_state.columns[current_col].selected = len - 1;
                            } 
                        }
                    }
                }
                KeyCode::Backspace => {
                    let current_col = app_state.selected_column;
                    // if the current_col is not the first col
                    if current_col > 0 {
                        let todo_index = app_state.columns[current_col].selected;
                        // if the current column has any todos
                        if !app_state.columns[current_col].todos.is_empty() {
                            let todo = app_state.columns[current_col].todos.remove(todo_index);
                            // Go to the next column Vec and then push
                            app_state.columns[current_col - 1].todos.push(todo);
                            let dest_len = app_state.columns[current_col - 1].todos.len();
                            app_state.columns[current_col - 1].selected = dest_len - 1;
                            // fix selected todo if it's out of bounds
                            let len = app_state.columns[current_col].todos.len();
                            if len == 0 {
                                app_state.columns[current_col].selected = 0;
                            } else if  app_state.columns[current_col].selected >= len {
                                app_state.columns[current_col].selected = len - 1;
                            } 
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(app_state: &AppState) -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    for (i, cols) in app_state.columns.iter().enumerate() {
        let x = 2 + (i as u16 * 30);
        let header = &cols.name;
        execute!(stdout(), MoveTo(x, 0), Print(header))?;
        if cols.todos.is_empty() {
            let marker = if app_state.selected_column == i { ">(empty)" } else { " (empty)" };
            execute!(stdout(), MoveTo(x, 2), Print(marker))?;
        } else {
            for (j, todo) in cols.todos.iter().enumerate() {
                let mut message = format!("{}", &todo.title);
                if app_state.selected_column == i && cols.selected == j {
                    message = format!(">{}", &todo.title);
                }
                let x = 2 + (i as u16 * 30);
                execute!(stdout(), MoveTo(x, 2 + j as u16), Print(message))?;
            }
        }
    }
    Ok(())
}
