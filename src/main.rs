#![warn(unused_variables)]



mod ui;
mod tab;
mod utils;
use ui::ui::Ui;
use utils::util::*;
use tab::tabs::Tabs;
use ncurses::*;
use anyhow::Result;
use std::env;
use std::process;

const REGULAR_PAIR : i16 = 0;
const HIGHLIGHT_PAIR : i16 = 1;
const TODO_PAIR : i16 = 2;
const DONE_PAIR : i16 = 3;
const PROG_PAIR : i16 = 4;


fn main() -> Result<()>{
    let mut env = env::args();
    env.next().unwrap();

    let file_path = match env.next() {
        Some(path) => {
            if path == "add"{
                run_from_cl("add",env.next(),"state.txt");
            }
            path
        },
        _ => "state.txt".to_string()
    };

    
    let mut todos : Vec<String> = Vec::new();
    let mut progress : Vec<String> = Vec::new();
    let mut dones : Vec<String> = Vec::new();
    
    match env.next() {
        Some(action) => {
            match action.as_str() {
                "add" => {
                    match env.next() {
                        Some(data) => {
                            load_state(&mut todos, &mut dones, &mut progress, &file_path)?;
                            todos.push(data.clone());
                            println!("Added Todo: {}", data);
                        }
                        _ => {
                            println!("Invalid Data");
                            process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("Invalid Command your command was {}",action);
                    process::exit(1);
                }
            }
        }
        _ => {}
    }

    load_state(&mut todos, &mut dones,&mut progress, &file_path)?;

    let mut curr_done : usize = 0;
    let mut curr_todo : usize = 0;
    let mut curr_prog : usize = 0;



    initscr();
    noecho();
    timeout(16);
    raw();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();


    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(TODO_PAIR, COLOR_RED, COLOR_BLACK);
    init_pair(DONE_PAIR, COLOR_GREEN, COLOR_BLACK);
    init_pair(PROG_PAIR, COLOR_YELLOW, COLOR_BLACK);


    let mut quit : bool  = false;
    let mut editing : bool = false;
    let mut notifs_delay : u16 = 0;

    let mut ui = Ui::default();
    let mut tab  = Tabs::new();
    let mut notifs : String = String::from("");
    // let mut input : Vec<char> = Vec::new();
    

    while !quit{
        ui.begin(3,0);
        if editing{

        }
        {
            erase();
            {
                if notifs_delay > 0 {
                    mv(0,0);
                    attron(A_BOLD | COLOR_PAIR(REGULAR_PAIR));
                    addstr(&notifs).unwrap();
                    attroff(A_BOLD | COLOR_PAIR(REGULAR_PAIR));
                }    
            }
                
            match tab {
                Tabs::Todos => {
                    ui.label("Todo: ",TODO_PAIR);
                    ui.begin_list(curr_todo);
                    for (index,todo) in todos.iter().enumerate(){
                            let label = &format!("-- {} => {}",index+1,todo);
                            if index == curr_todo{
                                ui.list_element(label,index,HIGHLIGHT_PAIR);
                            }else{
                                ui.list_element(label,index,REGULAR_PAIR);
                            }
                        }
                    ui.end_list();
                },
                Tabs::Dones => {
                    ui.label("Done: ",DONE_PAIR);
                    ui.begin_list(curr_done+1729);
                    for (index,done) in dones.iter().enumerate(){
                            let label = &format!("|| {} => {}",index+1,done);
                            if index == curr_done{
                                ui.list_element(label,index,HIGHLIGHT_PAIR);
                            }else{
                                ui.list_element(label,index,REGULAR_PAIR);
                            }
                    }
                    ui.end_list();
                },
                Tabs::Progress => {
                    ui.label("In Progress: ",PROG_PAIR);
                    ui.begin_list(curr_done+4000);
                    for (index,prog) in progress.iter().enumerate(){
                            let label = &format!("|- {} => {}",index+1,prog);
                            if index == curr_prog{
                                ui.list_element(label,index,HIGHLIGHT_PAIR);
                            }else{
                                ui.list_element(label,index,REGULAR_PAIR);
                            }
                    }
                    ui.end_list();
                }
            }
        }
        if notifs_delay > 0 {notifs_delay -= 1}
        ui.end();
        refresh();

        let key : i32 = getch();
        match key as u8{
            b'q' => {
                quit = true;
                save_state(&mut todos, &mut dones, &mut progress, &file_path)?
            },
            b'e' => {
                save_state(&mut todos, &mut dones, &mut progress, &file_path)?;
                notifs = format!("State Saved To {}",file_path);
                notifs_delay = 50;
            },
            b'i' =>{
                notifs = "Add Todos!".to_string();
                notifs_delay = 50;
                // editing = true;
            },
            b'j' => {
                match tab{
                    Tabs::Dones => list_down(&mut dones, &mut curr_done),
                    Tabs::Todos => list_down(&mut todos, &mut curr_todo),
                    Tabs::Progress => list_down(&mut progress, &mut curr_prog)
                }
            },
            b'k' => 
             match tab {
                Tabs::Dones => list_up(&mut curr_done),
                Tabs::Todos => list_up(&mut curr_todo),
                Tabs::Progress => list_up(&mut curr_prog),
            },
            b'\n' => {
                match tab {
                    Tabs::Todos => list_transfer(&mut todos,&mut progress,& mut curr_todo),
                    Tabs::Dones => list_transfer(&mut dones,&mut todos, & mut curr_done),
                    Tabs::Progress => list_transfer(&mut progress,&mut dones,& mut curr_prog)
                }

            },
            b'd' => {
                match tab {
                    Tabs::Dones => remove_from_list(&mut dones,&mut curr_done),
                    Tabs::Todos => remove_from_list(&mut todos,&mut curr_todo),
                    Tabs::Progress => remove_from_list(&mut progress,&mut curr_prog),
                }
            }
            b'\t' => {
                tab = tab.toggle();
            },
            _ => {}
        }

    }
    endwin();
    Ok(())
}
