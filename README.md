# todust
A Simple Todolist app written in Rust using ncurses

## How to run
```shell
cargo run
```

## add todolist form shell by
```shell
cargo run add "checking"
```

### check ur todos from shell by
```
cargo run list
```

### list of commands
```json
    "j" : "goes down",
    "k" : "goes up",
    "d" : "delete the current highlighted text",
    "Tab" : "changes the tab",
    "Enter" : "change the task from todo -> progress -> done"
    "q" : "exits",
    "e" : "saves the state",
    "i" : "enters input mode to add todos",
    "Esc(input mode)" : "leaves the input screen without adding the todo",
    "Enter(input mode)" : "saves the todos",

```