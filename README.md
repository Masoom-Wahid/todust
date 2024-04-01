# todust
A Simple Todolist app written in Rust using ncurses

## How to run
```shell
cargo run
```

## add todolists by
```shell
cargo run add "checking"
```

### list of commands
```json
    "j" : "goes down",
    "k" : "goes up",
    "d" : "delete the current highlighted text",
    "Tab" : "changes the tab",
    "Enter" : "change the task from todo -> progress -> done"
    "q" : "exits",
    "e" : "saves the state"

```