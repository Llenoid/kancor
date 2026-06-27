use std::{io::{self, stdout}, fs};
use crossterm::{
    cursor,
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
enum PopupMode {
    Normal,
    Insert,
}

#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
enum Mode {
    Normal,
    Popup(PopupMode),
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "-- NORMAL --"),
            Mode::Popup(PopupMode::Normal) => write!(f, "-- POPUP Normal --"),
            Mode::Popup(PopupMode::Insert) => write!(f, "-- POPUP Insert --"),
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

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let _terminal = Terminal;
    let _ = execute!(stdout(), EnterAlternateScreen)?;

    let mut cols = Vec::new();
    for col in ColumnType::VARIANTS.iter() {
        let column = Column { column_type: *col, name: format!("{:?}", col), todos: Vec::new(), selected: 0 };
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
                        KeyCode::Char('n') => {
                            app_state.mode = Mode::Popup(PopupMode::Normal);
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
                Mode::Popup(PopupMode::Normal) => {
                    match key_event.code {
                        KeyCode::Esc => {
                            app_state.input_buffer.clear();
                            app_state.mode = Mode::Normal;
                        }
                        KeyCode::Char('i') => {
                            app_state.mode = Mode::Popup(PopupMode::Insert);
                        }
                        _ => {}
                    }
                }
                Mode::Popup(PopupMode::Insert) => {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            app_state.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.input_buffer.pop();
                        }
                        KeyCode::Esc => {
                            app_state.mode = Mode::Popup(PopupMode::Normal);
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
        let _ = save(&app_state);
    }
    Ok(())
}

fn render(app_state: &AppState) -> io::Result<()> {
    execute!(stdout(), cursor::Hide, Clear(ClearType::All))?;
    let (x, y) = size()?;
    for (i, cols) in app_state.columns.iter().enumerate() {
        let x = 2 + (i as u16 * 30);
        let col_name = &cols.name;
        execute!(stdout(), MoveTo(x, 0), Print(col_name))?;
        execute!(stdout(), MoveTo(x + col_name.len() as u16 + 2, 0), Print(&cols.todos.len()))?;
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
    if matches!(app_state.mode, Mode::Popup(_)) {
        render_rect(x, y, &app_state.input_buffer)?;
    }
    let shortcuts = "n: new  d: delete  h/l: column  j/k: todo  q: quit";
    execute!(stdout(), MoveTo(x - shortcuts.len() as u16, y - 1), Print(shortcuts))?;
    execute!(stdout(), MoveTo(0, y - 1), Print(&app_state.mode), cursor::Show)?;
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

fn render_rect(w: u16, h: u16, input: &str) -> io::Result<()> {
    let float_width = 50;
    let float_height = 20;
    let x = (w / 2) - (float_width / 2);
    let y = (h / 2) - (float_height / 2);
    execute!(stdout(), MoveTo(x, y), Print("┌"))?;
    execute!(stdout(), MoveTo(x + 1, y), Print("─".repeat(float_width as usize - 1)))?;
    execute!(stdout(), MoveTo(x + float_width, y), Print("┐"))?;
    execute!(stdout(), MoveTo(x, y + float_height), Print("└"))?;
    execute!(stdout(), MoveTo(x + 1, y + float_height), Print("─".repeat(float_width as usize - 1)))?;
    execute!(stdout(), MoveTo(x + float_width, y + float_height), Print("┘"))?;
    for row in 0..19 {
        execute!(stdout(), MoveTo(x + 1, y + 1 + row), Print(" ".repeat(float_width as usize - 1)))?;
        execute!(stdout(), MoveTo(x, y + 1 + row), Print("│"))?;
        execute!(stdout(), MoveTo(x + float_width, y + 1 + row), Print("│"))?;
    }
    execute!(stdout(), MoveTo(x + 2, y + 2), Print(format!("Title: {}", input)))?;
    execute!(stdout(), MoveTo(x + 2, y + 4), Print("Body: "))?;
    execute!(stdout(), MoveTo(x + 2, y + 6), Print("Is Completed?: "))?;
    Ok(())
}
