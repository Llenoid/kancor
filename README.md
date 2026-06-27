Kancor:

Kanban Board in terminal

Features:
- Current selection highlighting
- Moving todos around
- Editing todos
- making new todos
- todo count
- modal editing

Installation:
git clone then cargo build --release

Shortcuts:
```
h/l     move between columns
j/k     move within column
n       new todo
r       rename selected todo
d       delete selected todo
Enter   move todo to next column
Backspace  move todo to previous column
q       quit and save
```

Todos are saved in current working directory in `kancor.json`
