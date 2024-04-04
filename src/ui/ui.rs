use ncurses::*;

use crate::tab::tabs::Tabs;

pub type Id = usize;

pub enum Screen{
    Home,
    Main,
    Input,
}

impl Screen{
    fn toggle(&mut self) -> Self{
        match self {
            Screen::Home => Screen::Main,
            Screen::Main => Screen::Home,
            Screen::Input => Screen::Main,
        }
    }
}
impl Default for Screen{
    fn default() -> Self{
        Screen::Home
    }
}



#[derive(Default)]
pub struct Ui{
    list_curr : Option<Id>,
    row : usize,
    col : usize,
    pub screen : Screen
}
impl Ui {
    pub fn begin(&mut self, row : usize , col : usize) {
        self.row = row;
        self.col = col;
    }

    pub fn begin_list(&mut self, id : Id) {
        assert!(self.list_curr.is_none(),"Nested lists are not allowed");
        self.list_curr = Some(id);
    }

    pub fn toggle_screen(&mut self){
        self.screen = self.screen.toggle();
        let mut x = 0;
        let mut y = 0;
        getmaxyx(stdscr(), &mut y, &mut x);
        self.row = y as usize;
        self.col = x as usize;
    }

    pub fn to_screen(&mut self,screen : Screen){
        self.screen = screen;
    }

    pub fn list_element(&mut self,label : &str,_id : Id,pair : i16){
        // let currr_id = self.list_curr.expect("not allowed to create list elements outside of list");
        self.label(
            label,
            pair);
    }

    pub fn render_notifs(&mut self,notif : &str){
        mv(1,2);
        attron(A_BOLD | COLOR_PAIR(0));
        addstr(&notif).unwrap();
        attroff(A_BOLD | COLOR_PAIR(0));
    }

    pub fn render_home(&mut self) {
        self.label("              Welcome          ",2);
        self.label("",0);
        self.label("    => Add Todos By Pressing 'i'",0);
        self.label("",0);
        self.label("    => Change Tab by Pressing 'Tab'",0);
        self.label("",0);
        self.label("    => Select A Tab", 0);
        self.label("",0);
        self.label("    -> Todo : '1'", 0);
        self.label("    -> Doing: '2'", 0);
        self.label("    -> Done : '3'", 0);
    }

    pub fn render_input(&mut self, todos: &mut Vec<String>) {
        let mut input_str = String::new();

        let mut rows = 0;
        let mut cols = 0;
        getmaxyx(stdscr(), &mut rows, &mut cols);
    
        let input_box_width = cols - 4;
        let input_box_height = 3;
        let input_box_offset_x = (cols - input_box_width) / 2;
        let input_box_offset_y = (rows - input_box_height) / 2;
        let input_box_win = newwin(input_box_height, input_box_width, input_box_offset_y, input_box_offset_x);
    
        box_(input_box_win, 0, 0); // Draw input box borders
    
        loop {
            mvprintw(input_box_offset_y + 1, input_box_offset_x + 1, &format!("Enter text: {}", input_str)).unwrap();
    
            wrefresh(input_box_win);
    
            let ch = getch();
            match ch {
                KEY_ENTER | 10 | 13 => {
                    todos.push(input_str);
                    break;
                }, // Enter key
                KEY_BACKSPACE | 127 => {
                    input_str.pop(); // Backspace key
                }
                KEY_DC => {
                    input_str.clear(); // Delete key
                },
                27 => break,
                32..=126 => {
                    input_str.push(ch as u8 as char); // Printable character
                }
                _ => continue,
            }
        }
    
        delwin(input_box_win);
    
    }
    #[allow(non_snake_case)]
    pub fn render_main(
        &mut self,
        tab : &Tabs,
        todos : &Vec<String>,
        dones : &Vec<String>,
        progress : &Vec<String>,
        curr_todo : &usize,
        curr_done : &usize,
        curr_prog : &usize,
        HIGHLIGHT_PAIR : i16,
        REGULAR_PAIR : i16,
        TODO_PAIR: i16,
        PROG_PAIR : i16,
        DONE_PAIR : i16
        ){
        match tab {
            Tabs::Todos => {
                self.label("Todo: ",TODO_PAIR);
                self.begin_list(*curr_todo);
                for (index,todo) in todos.iter().enumerate(){
                        let label = &format!("-- {} => {}",index+1,todo);
                        if index == *curr_todo{
                            self.list_element(label,index,HIGHLIGHT_PAIR);
                        }else{
                            self.list_element(label,index,REGULAR_PAIR);
                        }
                    }
                self.end_list();
            },
            Tabs::Dones => {
                self.label("Done: ",DONE_PAIR);
                self.begin_list(curr_done+1729);
                for (index,done) in dones.iter().enumerate(){
                        let label = &format!("|| {} => {}",index+1,done);
                        if index == *curr_done{
                            self.list_element(label,index,HIGHLIGHT_PAIR);
                        }else{
                            self.list_element(label,index,REGULAR_PAIR);
                        }
                }
                self.end_list();
            },
            Tabs::Progress => {
                self.label("In Progress: ",PROG_PAIR);
                self.begin_list(curr_done+4000);
                for (index,prog) in progress.iter().enumerate(){
                        let label = &format!("|- {} => {}",index+1,prog);
                        if index == *curr_prog{
                            self.list_element(label,index,HIGHLIGHT_PAIR);
                        }else{
                            self.list_element(label,index,REGULAR_PAIR);
                        }
                }
                self.end_list();
            }
        }   
    }
    

    pub fn label(&mut self,text: &str,pair : i16){
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair) | A_BOLD );
        addstr(text).unwrap();
        attroff(COLOR_PAIR(pair) | A_BOLD );
        self.row += 1;
    }

    pub fn end_list(&mut self){
        self.list_curr = None;
    }
    pub fn end(&mut self) {
        
    }
}
