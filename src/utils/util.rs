use std::fs::File;
use crate::tab::tabs::Tabs;
use anyhow::Result;
use std::io::BufReader;
use std::process;
use std::io::BufRead;
use std::io::Write;

pub fn list_up(curr : &mut usize){
    if *curr > 0  { 
        *curr -= 1;
    }
}

pub fn list_down(list : &mut Vec<String> , curr : &mut usize){
    if *curr < list.len() - 1 { 
        *curr += 1; 
    }   
}

pub fn list_transfer(
                        prev_list : &mut Vec<String> 
                        ,next_list : &mut Vec<String>
                        , curr : &mut usize)
{
    if prev_list.len() > 0 || *curr < prev_list.len(){
        next_list.push(prev_list.remove(*curr));
        *curr = 0;
    }
                    
}

pub fn remove_from_list(list : &mut Vec<String>,index: &mut usize){
    if list.len() > 0 || *index < list.len(){
        list.remove(*index);
        *index = 0;
    }
}


fn parse_item(line: &str) -> Option<(Tabs, &str)> {
    let todo_item = line
        .strip_prefix("TODO: ")
        .map(|title| (Tabs::Todos, title));
    let done_item = line
        .strip_prefix("DONE: ")
        .map(|title| (Tabs::Dones, title));
    let prog_item = line
        .strip_prefix("PROGRESS: ")
        .map(|title| (Tabs::Progress, title));

    todo_item.or(done_item).or(prog_item)
}

pub fn run_from_cl(_action : &str, data : Option<String>,file_path: &str){
    let mut todos : Vec<String> = Vec::new();
    let mut progress : Vec<String> = Vec::new();
    let mut dones : Vec<String> = Vec::new();
    match data {
        Some(todo) => {
            load_state(&mut todos, &mut dones, &mut progress, &file_path).unwrap();
            todos.push(todo.to_string());
            println!("Added Todo: {}", todo);
            save_state(&mut todos, &mut dones, &mut progress, file_path).unwrap();
            process::exit(1);
        }
        _ => {
            println!("Couldnt add a todo");
            process::exit(1);
        }
    }
    
}

pub fn load_state(todos: &mut Vec<String>, dones: &mut Vec<String>,progress : &mut Vec<String>,file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    for (index, line) in BufReader::new(file).lines().enumerate() {
        match parse_item(&line?) {
            Some((Tabs::Todos, title)) => todos.push(title.to_string()),
            Some((Tabs::Dones, title)) => dones.push(title.to_string()),
            Some((Tabs::Progress,title)) => progress.push(title.to_string()),
            None => {
                eprintln!("{}:{}: ERROR: ill-formed item line", file_path, index + 1);
                process::exit(1);
            }
        }
    }
    Ok(())
}


pub fn save_state(todos: &mut Vec<String>, dones: &mut Vec<String>,progress : &mut Vec<String>,file_path: &str) -> Result<()> {
    //TODO: create a new file if there isnt one
    let mut state_file = File::create(file_path)?;
    for todo in todos.iter(){
        writeln!(state_file,"TODO: {}",todo).unwrap();
    }
    for prog in progress.iter(){
        writeln!(state_file,"PROGRESS: {}",prog).unwrap();
    }
    for done in dones.iter(){
        writeln!(state_file,"DONE: {}",done).unwrap();
    }

    Ok(())
}