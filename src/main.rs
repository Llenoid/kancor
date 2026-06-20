use std::{io::{self, stdout}, fs, fmt::Debug};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode, Clear, ClearType, size},
    event::{
        read, Event, KeyCode
    },
    cursor::{MoveTo},
    style::{Print}
};
use serde::{Serialize, Deserialize};

struct Terminal;
impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    body: String,
    is_completed: bool
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)] 
enum ColumnType {
    Unassigned,
    New,
    Backlog,
    Pending,
    Done,
}

impl ColumnType {
    const VARIANTS: [ColumnType; 5] = [
        ColumnType::Unassigned,
        ColumnType::New,
        ColumnType::Backlog,
        ColumnType::Pending,
        ColumnType::Done,
    ];
}

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "-- NORMAL --"),
            Mode::Insert => write!(f, "-- INSERT --"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Column {
    column_type: ColumnType,
    name: String,
    todos: Vec<Todo>,
    selected: usize,
}

#[derive(Serialize, Deserialize)]
struct AppState {
    columns: Vec<Column>,
    selected_column: usize,
    mode: Mode,
    input_buffer: String,
}

pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for ColumnType {
    fn to_snake_case(&self) -> String {
        let input = format!("{:?}", self);
        let mut snake = String::with_capacity(input.len() * 2);

        for (i, ch) in input.chars().enumerate() {
            if ch.is_uppercase() {
                if i > 0 {
                    snake.push('_');
                }
                snake.extend(ch.to_lowercase());
            } else {
                snake.push(ch);
            }
        }
        snake
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let _terminal = Terminal;
    let _ = execute!(stdout(), EnterAlternateScreen)?;

    let mut cols = Vec::new();
    for col in ColumnType::VARIANTS.iter() {
        let column = Column { column_type: *col, name: col.to_snake_case(), todos: Vec::new(), selected: 0 };
        cols.push(column)
    }

    let mut app_state = load().unwrap_or_else(|| AppState {
        columns: cols,
        selected_column: 0,
        mode: Mode::Normal,
        input_buffer: String::new(),
    });

    loop {
        render(&app_state)?;
        if let Event::Key(key_event) = read()? {
            match app_state.mode {
                Mode::Normal => {
                    match key_event.code {
                        KeyCode::Char('a') => {
                            app_state.mode = Mode::Insert;
                        }
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
                            let _ = save(&app_state);
                            break;
                        }
                        KeyCode::Char('d') => {
                            if !app_state.columns[app_state.selected_column].todos.is_empty() {
                                let current_col = app_state.selected_column;
                                let todo_index = app_state.columns[current_col].selected;
                                app_state.columns[current_col].todos.remove(todo_index);
                                let len = app_state.columns[current_col].todos.len();
                                if len == 0 {
                                    app_state.columns[current_col].selected = 0;
                                } else if app_state.columns[current_col].selected >=len {
                                    app_state.columns[current_col].selected = len - 1;
                                }
                            }
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
                                    // Follow the todo when moving across columns
                                    app_state.selected_column = current_col + 1;
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
                                    // Follow the todo when moving across columns
                                    app_state.selected_column = current_col - 1;
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
                Mode::Insert => {
                    match key_event.code {
                        KeyCode::Esc => {
                            app_state.mode = Mode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app_state.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.input_buffer.pop();
                        }
                        KeyCode::Enter => {
                            if !app_state.input_buffer.is_empty() {
                                let todo = Todo {
                                    title: app_state.input_buffer.clone(),
                                    body: String::new(),
                                    is_completed: false
                                };
                                app_state.input_buffer.clear();
                                let current_col = app_state.selected_column;
                                app_state.columns[current_col].todos.push(todo);
                                let dest_len = app_state.columns[current_col].todos.len();
                                app_state.columns[current_col].selected = dest_len - 1;
                                app_state.mode = Mode::Normal;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn render(app_state: &AppState) -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    let (_, y) = size()?;
    execute!(stdout(), MoveTo(0, y - 1), Print(&app_state.mode))?;
    execute!(stdout(), MoveTo(0, y - 4), Print(&app_state.input_buffer))?;
    for (i, cols) in app_state.columns.iter().enumerate() {
        let x = 2 + (i as u16 * 30);
        execute!(stdout(), MoveTo(x, 0), Print(&cols.name))?;
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

fn save(app_state: &AppState) -> io::Result<()> {
    let json = serde_json::to_string_pretty(app_state).unwrap();
    fs::write("kancor.json", &json)?;
    Ok(())
}

fn load() -> Option<AppState> {
    let message: String = fs::read_to_string("kancor.json").ok()?;
    let app_state: AppState = serde_json::from_str::<AppState>(&message).ok()?;
    Some(app_state)
}
