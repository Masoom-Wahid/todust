# todust
A Simple Todolist app written in Rust using ncurses

## How to run
```shell
cargo run file=state.txt
```

## add todolist form shell by
```shell
cargo run file=state.txt action=add data="Studing DSA"
```

## list todos by
```shell
cargo run file=state.txt action=list
```

## del todos by their index (You Can Find their index when running list command)
```shell
cargo run file=state.txt action=del data=0
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

### You can also build it to use from anywhere in the terminal
#### first build the release
```bash
cargo build --release
```

#### add this function into your .bashrc file
### add your release path and state path
```bash
function todust(){
    local action="$1"
    local data="$2"
    relese_file_path/todust file=state_txt_file_path action="$action" data="$data"
}
```
