use std::fs;

use crate::{
    keys::keys::{Event, Key},
    request::Request,
};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Scrollbar, ScrollbarOrientation},
    Frame,
};
use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Clone, Debug)]
struct Node {
    file_path: String,
    file_name: String,
}
impl Node {
    fn new(file_path: String) -> Self {
        let file_name = file_path.split('/').last().unwrap().to_string();
        Self {
            file_path,
            file_name,
        }
    }
}

pub struct Collection<'a> {
    state: TreeState<String>,
    items: TreeItem<'a, String>,
}

impl<'a> Collection<'a> {
    pub fn default(path: String) -> Self {
        let items = Self::create_tree(Node::new(path), 0).unwrap();
        Self {
            state: TreeState::default(),
            items,
        }
    }
    fn create_tree(node: Node, mut depth: usize) -> Option<TreeItem<'a, String>> {
        let mut result = TreeItem::new_leaf(node.file_path.clone(), node.file_name.clone());
        if depth > 10 || !fs::metadata(node.file_path.clone()).unwrap().is_dir() {
            if !node.file_name.ends_with(".rph") {
                return None;
            };
            return Some(result);
        }
        for entry in fs::read_dir(node.file_path.clone()).unwrap() {
            let ent = entry.unwrap();
            let f_path = ent.path().to_string_lossy().to_string();
            let new_path = Node::new(f_path);
            depth += 1;
            if let Some(r) = Self::create_tree(new_path, depth) {
                result.add_child(r).unwrap();
            }
        }
        Some(result)
    }
    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.size();
        let all_items = &[self.items.clone()];
        let widget = Tree::new(all_items)
            .expect("all item identifiers are unique")
            .block(
                Block::bordered()
                    .title("Tree Widget")
                    .title_bottom(format!("{:?}", self.state)),
            )
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        frame.render_widget(Clear, area);
        frame.render_stateful_widget(widget, area, &mut self.state) //, &mut self.state)
    }
    pub fn get_selected(&self) -> Vec<String> {
        self.state
            .selected()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    pub fn update(&mut self, event: &Event) -> Option<Vec<String>> {
        match event.key {
            Key::Enter => {
                let selected = self.get_selected();
                Some(selected)
            }
            Key::Char('\n' | ' ') => {
                self.state.toggle_selected();
                None
            }
            Key::Left => {
                self.state.key_left();
                None
            }
            Key::Right => {
                self.state.key_right();
                None
            }
            Key::Down => {
                self.state.key_down();
                None
            }
            Key::Up => {
                self.state.key_up();
                None
            }
            _ => None,
        }
    }
}
