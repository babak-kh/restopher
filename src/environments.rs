use crate::components::{default_block, PopUpComponent, KV};
use crate::layout::centered_rect;
use ratatui::layout::Margin;
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListState, Row, Table, TableState},
    Frame,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::keys::keys::{Event, Key, Modifier::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub envs: HashMap<String, String>,
}
impl Environment {
    pub fn new(name: String) -> Self {
        Environment {
            name,
            envs: HashMap::new(),
        }
    }
    pub fn default() -> Self {
        Environment {
            name: "".to_string(),
            envs: HashMap::new(),
        }
    }
    fn into_kvs(&self) -> Vec<KV> {
        let mut result = Vec::new();
        for (k, v) in self.envs.iter() {
            result.push(KV::from(k.clone(), v.clone()))
        }
        result
    }
}

#[derive(Debug)]
pub enum EnvironmentSubSection {
    Name,
    KVs,
}
#[derive(Debug)]
pub struct TempEnv {
    all_envs: Vec<Environment>,
    changed: bool,
    selected: usize,
    selected_kv: usize,
    initial_idx: usize,
    current_kvs: Vec<KV>,
    environment_sub_selection: EnvironmentSubSection,
    popup: Option<PopUpComponent>,
}

impl Into<TempEnv> for Environment {
    fn into(self) -> TempEnv {
        TempEnv {
            all_envs: Vec::new(),
            current_kvs: into(self.envs),
            changed: false,
            initial_idx: 0,
            selected: 0,
            selected_kv: 0,
            environment_sub_selection: EnvironmentSubSection::Name,
            popup: None,
        }
    }
}

impl TempEnv {
    pub fn new(all_envs: Vec<Environment>, initial_idx: usize) -> Self {
        TempEnv {
            changed: false,
            selected: initial_idx,
            initial_idx,
            selected_kv: 0,
            current_kvs: into(all_envs[initial_idx].envs.clone()),
            all_envs,
            environment_sub_selection: EnvironmentSubSection::Name,
            popup: None,
        }
    }
    pub fn add_env(&mut self, name: String) {
        self.all_envs.push(Environment::new("".to_string()));
        self.selected = self.all_envs.len() - 1;
        self.current_kvs = Vec::new();
    }
    pub fn sync_envs(&mut self) {
        self.all_envs[self.selected].envs = self
            .current_kvs
            .iter()
            .map(|kv| (kv.get_key(), kv.get_value()))
            .collect();
    }
    pub fn update(&mut self, event: &Event) -> (Option<Vec<Environment>>, bool) {
        if matches!(event.key, Key::Tab) {
            self.environment_sub_selection = match self.environment_sub_selection {
                EnvironmentSubSection::Name => EnvironmentSubSection::KVs,
                EnvironmentSubSection::KVs => EnvironmentSubSection::Name,
            };
            return (None, true);
        }
        if let Some(popup) = &mut self.popup {
            let result = popup.update(event);
            if result.1 {
                return (None, true);
            };
            self.popup = None;
            if let Some(new_env) = result.0 {
                self.all_envs.push(Environment::new(new_env));
                self.selected = self.all_envs.len() - 1;
                self.current_kvs = Vec::new();
                return (None, true);
            };
            return (None, true);
        };
        if matches!(event.key, Key::Esc) {
            self.sync_envs();
            return (Some(self.all_envs.clone()), false);
        }
        match self.environment_sub_selection {
            EnvironmentSubSection::Name => {
                match event.modifier {
                    Some(Control) => match event.key {
                        Key::Char('n') => {
                            self.popup = Some(PopUpComponent::new(
                                "New Environment".to_string(),
                                "Enter the name of the new environment".to_string(),
                                Some("Cancel".to_string()),
                                Some("Create".to_string()),
                            ));
                            return (None, true);
                        }
                        Key::Char('d') => {
                            self.all_envs.remove(self.selected);
                            if self.selected > 0 {
                                self.selected -= 1;
                            }
                            self.current_kvs = into(self.all_envs[self.selected].envs.clone());
                            return (None, true);
                        }
                        _ => (),
                    },
                    _ => (),
                }
                match event.key {
                    Key::Down | Key::Enter => {
                        self.sync_envs();
                        if self.selected == self.all_envs.len() - 1 {
                            self.selected = 0;
                            self.current_kvs = into(self.all_envs[self.selected].envs.clone());
                            return (None, true);
                        }
                        if self.selected < self.all_envs.len() - 1 {
                            self.selected += 1;
                            self.current_kvs = into(self.all_envs[self.selected].envs.clone());
                            return (None, true);
                        }
                        (None, true)
                    }
                    Key::Up => {
                        self.sync_envs();
                        if self.selected == 0 {
                            self.selected = self.all_envs.len() - 1;
                            self.current_kvs = into(self.all_envs[self.selected].envs.clone());
                            return (None, true);
                        }
                        if self.selected > 0 {
                            self.selected -= 1;
                            self.current_kvs = into(self.all_envs[self.selected].envs.clone());
                            return (None, true);
                        }
                        (None, true)
                    }
                    _ => (None, true),
                }
            }
            EnvironmentSubSection::KVs => {
                if let Some(modifier) = &event.modifier {
                    match modifier {
                        Control => match event.key {
                            Key::Char('n') => {
                                self.create_new_kv();
                                return (None, true);
                            }
                            Key::Char('d') => {
                                self.remove_kv();
                                return (None, true);
                            }
                            Key::Char('l') | Key::Char('h') | Key::Right | Key::Left => {
                                self.current_kvs[self.selected_kv].change_active();
                                return (None, true);
                            }
                            _ => return (None, true),
                        },
                        _ => (),
                    }
                }
                match event.key {
                    Key::Down | Key::Enter => {
                        if self.current_kvs.len() == 0 {
                            return (None, true);
                        }
                        if self.selected_kv == self.current_kvs.len() - 1 {
                            self.selected_kv = 0;
                            return (None, true);
                        }
                        if self.selected_kv < self.current_kvs.len() - 1 {
                            self.selected_kv += 1;
                            return (None, true);
                        }
                        (None, true)
                    }
                    Key::Up => {
                        if self.selected_kv == 0 {
                            self.selected_kv = self.current_kvs.len() - 1;
                            return (None, true);
                        }
                        if self.selected_kv > 0 {
                            self.selected_kv -= 1;
                            return (None, true);
                        }
                        (None, true)
                    }
                    Key::Char(x) => {
                        if self.current_kvs.len() == 0 {
                            return (None, true);
                        }
                        self.current_kvs[self.selected_kv].add_to_active(x);
                        self.changed = true;
                        return (None, true);
                    }
                    Key::Backspace => {
                        if self.current_kvs.len() == 0 {
                            return (None, true);
                        }
                        self.current_kvs[self.selected_kv].remove_from_active();
                        self.changed = true;
                        return (None, true);
                    }
                    _ => (None, true),
                }
            }
        }
    }
    fn create_new_kv(&mut self) {
        self.current_kvs.push(KV::new());
        self.selected_kv = self.current_kvs.len() - 1;
    }
    fn remove_kv(&mut self) {
        self.current_kvs.remove(self.selected_kv);
        if self.selected_kv > 0 {
            self.selected_kv -= 1
        };
    }
    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let title = format!("Environment: {}", self.all_envs[self.selected].name);
        let chunks = Layout::horizontal(&[Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);
        let vertical_scroll = 0; // from app state
        let mut scrollbar_state =
            ScrollbarState::new(self.all_envs.len()).position(vertical_scroll);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_widget(Clear, rect);
        let mut list_state = ListState::default();
        list_state.select(Some(self.selected));

        f.render_stateful_widget(
            List::new(self.all_envs.iter().map(|env| env.name.clone()))
                .block(
                    Block::default()
                        .title("Environments")
                        .borders(Borders::NONE),
                )
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol(">>"),
            chunks[0],
            &mut list_state,
        );
        f.render_stateful_widget(
            scrollbar,
            chunks[0].inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
        let mut state = TableState::default();
        state.select(Some(self.selected_kv));
        let items = Table::new(
            self.current_kvs.iter().map(|item| {
                Row::new(vec![item.get_key().clone(), item.get_value().clone()])
                    .style(Style::default().fg(Color::LightBlue))
            }),
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(Row::new(vec!["Key", "Value"]))
        .block(default_block(
            Some(&title),
            matches!(self.environment_sub_selection, EnvironmentSubSection::KVs),
        ))
        .highlight_style(Style::new().fg(Color::Green))
        .highlight_symbol(">>");

        let mut scrollbar_state =
            ScrollbarState::new(self.current_kvs.len()).position(vertical_scroll);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        f.render_stateful_widget(items, chunks[1], &mut state);
        f.render_stateful_widget(
            scrollbar,
            chunks[1].inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
        if let Some(popup) = &self.popup {
            let r = centered_rect(60, 20, rect);
            f.render_widget(Clear, r);
            popup.draw(f, r);
        }
    }
}

fn into(data: HashMap<String, String>) -> Vec<KV> {
    let mut result = Vec::new();
    for (key, value) in data {
        result.push(KV::from(key, value));
    }
    result
}
