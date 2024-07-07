use crate::main_windows::MainWindows;

pub trait AppSection {
    fn sections(&self) -> Vec<Section>;
    fn current_active(&self) -> Section;
}

pub type Section = &'static str;
pub struct SubSelection {
    pub sections: Vec<Section>,
    pub idx: usize,
}
pub struct StateItem {
    main_windows: MainWindows,
    sub: 
}
impl StateItem {
    pub fn new(main_windows: MainWindows, sub: Vec<Section>) -> Self {
        StateItem { main_windows, sub }
    }
    pub fn main_windows(&self) -> &MainWindows {
        &self.main_windows
    }
    pub fn main_windows_clone(&self) -> MainWindows {
        self.main_windows.clone()
    }
    pub fn sub(&self) -> Vec<Section> {
        self.sub
    }
}

pub struct State(Vec<StateItem>);
impl State {
    pub fn default() -> Self {
        State(vec![StateItem {
            main_windows: MainWindows::Main,
            sub: vec![REQUESTS],
        }])
    }
    pub fn last(&self) -> &StateItem {
        &self.0.last().unwrap()
    }
    pub fn push(&mut self, item: StateItem) {
        self.0.push(item);
    }
}

pub const REQUESTS: Section = "requests";
pub const ENVIRONMENTS: Section = "environments";
pub const COLLECTIONS: Section = "collections";
