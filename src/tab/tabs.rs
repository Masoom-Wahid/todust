pub enum Tabs{
    Todos,
    Progress,
    Dones,
}

impl Tabs{
    pub fn new() -> Self{
        Tabs::Todos
    }
    
    pub fn toggle(&mut self) -> Self{
        match self{
            Tabs::Todos => Tabs::Progress,
            Tabs::Progress => Tabs::Dones,
            Tabs::Dones => Tabs::Todos
        }
    }
}