use tui::text::Text;
use tui_tree_widget::{TreeItem, TreeState};

#[derive(Debug, Clone)]
pub struct Node {
    pub path: String,
    pub name: String,
}
impl Node {
    pub fn new(path: String, name: String) -> Self {
        Node { path, name }
    }
    pub fn push_str(&mut self, s: String) {
        self.path.push_str(&s);
    }
    pub fn push(&mut self, c: char) {
        self.push(c);
    }
}
impl<'a> Into<Text<'a>> for Node {
    fn into(self) -> Text<'a> {
        Text::raw(self.name.clone())
    }
}

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }
    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }
    pub fn first(&mut self) {
        self.state.select_first();
    }
    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }
    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }
    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }
    pub fn left(&mut self) {
        self.state.key_left();
    }
    pub fn right(&mut self) {
        self.state.key_right();
    }
    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
    pub fn get_path(&self) -> String {
        let mut first = &self.items[self.state.selected()[0]];
        for idx in &self.state.selected()[1..] {
            match first.child(*idx) {
                Some(f) => first = f,
                None => (),
            }
        };
        first.path.clone()
    }
}
