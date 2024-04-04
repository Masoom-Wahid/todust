#![warn(unused_variables)]



mod ui;
mod tab;
mod utils;
use ui::ui::Screen;
use ui::ui::Ui;
use utils::util::*;
use tab::tabs::Tabs;
use ncurses::*;
use anyhow::Result;
use std::env;

const REGULAR_PAIR : i16 = 0;
const HIGHLIGHT_PAIR : i16 = 1;
const TODO_PAIR : i16 = 2;
const DONE_PAIR : i16 = 3;
const PROG_PAIR : i16 = 4;


fn main() -> Result<()>{ 
    let args : Vec<String> = env::args().collect();
    let mut file_path :Option<String> = None;
    let mut action : Option<String> = None;
    let mut data : Option<String> = None;

    for arg in args.iter().skip(1){
        if arg.starts_with("file="){
            file_path = Some(arg[5..].to_owned());
        }else if arg.starts_with("action="){
            action = Some(arg[7..].to_owned());
        }else if arg.starts_with("data="){
            data = Some(arg[5..].to_owned());
        }
    }
    let file_path = file_path.unwrap();

    if let Some(action) = action {
        match action.as_str() {
            "add" => add_from_cl(data, &file_path)?,
            "list" => list_from_cl(&file_path)?,
            "del" => del_from_cl(data, &file_path)?,
            _ => {}
        }
    }

    
    let mut todos : Vec<String> = Vec::new();
    let mut progress : Vec<String> = Vec::new();
    let mut dones : Vec<String> = Vec::new();

    load_state(&mut todos, &mut dones,&mut progress, &file_path)?;

    let mut curr_done : usize = 0;
    let mut curr_todo : usize = 0;
    let mut curr_prog : usize = 0;



    let window = initscr();
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
    let mut notifs_delay : u16 = 0;

    let mut ui  : Ui = Ui::default();
    let mut tab : Tabs  = Tabs::new();
    let mut notifs : String = String::from("");
    

    while !quit{
        let mut x = 0;
        let mut y = 0;
        getmaxyx(window, &mut y, &mut x);
        ui.begin(((y/2) - 4) as usize,((x/2) - 20) as usize);
        {
            erase();
            box_(stdscr(), 0, 0);
            {
                if notifs_delay > 0 {
                    ui.render_notifs(&notifs);
                }    
            }
            
            match ui.screen {
                Screen::Home => ui.render_home(),
                Screen::Main => ui.render_main(&tab,&todos,&dones,&progress,&curr_todo,&curr_done,&curr_prog,HIGHLIGHT_PAIR,REGULAR_PAIR,TODO_PAIR,PROG_PAIR,DONE_PAIR),
                Screen::Input => {
                    ui.render_input(&mut todos);
                    ui.toggle_screen();
                },
            }
            

        }
        if notifs_delay > 0 {notifs_delay -= 1}
        ui.end();

        let key : i32 = getch();
        match key as u8{
            b'1'..=b'3' => {
                match ui.screen {
                    Screen::Home => {
                        match key as u8 {                            
                            b'1' => tab = Tabs::Todos,
                            b'2' => tab = Tabs::Progress,
                            b'3' => tab  = Tabs::Dones,
                            _ => {}
                        }
                        ui.toggle_screen();
                    },
                    _ => {}
                }
            },

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
                ui.screen = Screen::Input;
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
