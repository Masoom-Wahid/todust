use ncurses::*;

pub type Id = usize;
#[derive(Default)]
pub struct Ui{
    list_curr : Option<Id>,
    row : usize,
    col : usize,
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

    pub fn list_element(&mut self,label : &str,_id : Id,pair : i16){
        // let currr_id = self.list_curr.expect("not allowed to create list elements outside of list");
        
        self.label(
            label,
            pair);
    }

    pub fn label(&mut self,text: &str,pair : i16){
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text).unwrap();
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    pub fn end_list(&mut self){
        self.list_curr = None;
    }
    pub fn end(&mut self) {
        
    }
}
