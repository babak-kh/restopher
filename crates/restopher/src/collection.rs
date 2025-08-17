use std::fs;
use std::path::Path;

use crate::{
    components::{PopUpComponent, YesNoPopupComponent},
    keys::keys::{Event, Key, Modifier as keyModifier},
    layout::centered_rect,
};
use ratatui::widgets::{Block, Clear, Scrollbar, ScrollbarOrientation};
use ratatui::{
    style::{Color, Modifier, Style},
    Frame,
};

use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Debug, Clone)]
pub enum Action {
    Delete,
    Create,
    AddRequest,
}

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
    caller: Option<String>,
    delete_pop_up: Option<YesNoPopupComponent<'a>>,
    create_pop_up: Option<PopUpComponent>,
}

impl<'a> Collection<'a> {
    pub fn default(path: String) -> Self {
        let items = Self::create_tree(Node::new(path), 0).unwrap();
        Self {
            state: TreeState::default(),
            items,
            caller: None,
            delete_pop_up: None,
            create_pop_up: None,
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
        let area = frame.area();
        let all_items = &[self.items.clone()];
        let widget = Tree::new(all_items)
            .expect("all item identifiers are unique")
            .block(Block::bordered().title_bottom(format!("{:?}", self.state)))
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
        frame.render_stateful_widget(widget, area, &mut self.state);
        if let Some(popup) = &self.delete_pop_up {
            popup.draw(frame);
        };
        if let Some(popup) = &self.create_pop_up {
            let rect = centered_rect(40, 20, frame.area());
            popup.draw(frame, rect);
        };
    }
    pub fn get_selected(&self) -> Vec<String> {
        self.state
            .selected()
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    pub fn set_parent(&mut self, parent: String) {
        self.caller = Some(parent);
    }
    pub fn update(&mut self, event: &Event) -> Option<(Option<String>, Action, Vec<String>)> {
        if let Some(popup) = &mut self.delete_pop_up {
            if let Some(result) = popup.update(event) {
                self.delete_pop_up = None;
                if result {
                    self.caller = None;
                    return Some((self.caller.clone(), Action::Delete, self.get_selected()));
                }
                return None;
            }
            return None;
        }
        if let Some(popup) = &mut self.create_pop_up {
            let (result, to_continue) = popup.update(event);
            if to_continue {
                return None;
            }
            if let Some(filename) = result {
                self.create_pop_up = None;
                let path = self.get_selected();
                let path = Path::new(path.last().unwrap());
                let new_path = if fs::metadata(path).unwrap().is_dir() {
                    path.join(filename)
                } else {
                    path.parent().unwrap().join(filename)
                };
                self.caller = None;
                return Some((
                    self.caller.clone(),
                    Action::Create,
                    vec![new_path.to_string_lossy().to_string()],
                ));
            } else {
                self.create_pop_up = None;
                return None;
            }
        }
        if let Some(modifier) = &event.modifier {
            match modifier {
                keyModifier::Control => match event.key {
                    Key::Char('d') => {
                        self.delete_pop_up = Some(YesNoPopupComponent::new("Delete?"));
                    }
                    Key::Char('n') => {
                        self.create_pop_up = Some(PopUpComponent::new(
                            String::from("new collection"),
                            String::from("Input the name of the new collection"),
                        ));
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        match event.key {
            Key::Enter => {
                let selected = self.get_selected();
                Some((self.caller.clone(), Action::AddRequest, selected))
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
