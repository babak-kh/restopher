use crate::components::KV;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Rows},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Row, Table, TableState},
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
}

#[derive(Debug)]
pub enum EnvironmentSubSection {
    Name,
    KVs,
}
#[derive(Debug)]
pub struct TempEnv {
    sections: EnvironmentSubSection,
    temp_env: Environment,
    changed: bool,
    selected: usize,
    selected_kv: usize,
    kvs: Vec<KV>,
}

impl Into<TempEnv> for Environment {
    fn into(self) -> TempEnv {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_env: self.clone(),
            changed: false,
            selected: 0,
            selected_kv: 0,
            kvs: into(self.envs),
        }
    }
}

impl TempEnv {
    pub fn new() -> Self {
        TempEnv {
            sections: EnvironmentSubSection::Name,
            temp_env: Environment::default(),
            changed: false,
            selected: 0,
            selected_kv: 0,
            kvs: Vec::new(),
        }
    }
    pub fn get_modified(&self) -> Option<Environment> {
        if self.changed {
            Some(Environment {
                name: self.temp_env.name.clone(),
                envs: self
                    .kvs
                    .iter()
                    .map(|kv| (kv.get_key(), kv.get_value()))
                    .collect(),
            })
        } else {
            None
        }
    }
    pub fn update(&mut self, event: &Event) -> Option<Environment> {
        if let Some(modifier) = &event.modifier {
            match modifier {
                Control => match event.key {
                    Key::Char('n') => {
                        self.create_new_kv();
                        return None;
                    }
                    Key::Char('d') => {
                        self.remove_kv();
                        return None;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        match event.key {
            Key::Tab => {
                self.kvs[self.selected_kv].change_active();
                return None;
            }
            Key::Down | Key::Enter => {
                if self.kvs.len() == 0 {
                    return None;
                }
                if self.selected_kv < self.kvs.len() - 1 {
                    self.selected_kv += 1;
                    return None;
                }
                None
            }
            Key::Up => {
                if self.selected_kv > 0 {
                    self.selected_kv -= 1;
                    return None;
                }
                None
            }
            Key::Char(x) => {
                self.kvs[self.selected_kv].add_to_active(x);
                self.changed = true;
                return None;
            }
            _ => None,
        }
    }
    fn create_new_kv(&mut self) {
        self.kvs.push(KV::new());
    }
    fn remove_kv(&mut self) {
        self.kvs.remove(self.selected_kv);
        if self.selected_kv > 0 {
            self.selected_kv -= 1
        };
    }
    pub fn change_environment_subsection(&mut self) {
        match self.sections {
            EnvironmentSubSection::Name => self.sections = EnvironmentSubSection::KVs,
            EnvironmentSubSection::KVs => self.sections = EnvironmentSubSection::Name,
        }
    }
    //pub fn clear_temp_envs(&mut self) -> Result<(), Error> {
    //    if let Some(idx) = &mut self.current_env_idx {
    //        let mut found: bool = false;
    //        let name = &self.all_envs[*idx].name;
    //        if let Some(temp) = &self.temp_envs {
    //            for item in temp.temp_envs.iter().enumerate() {
    //                if item.1.name == *name {
    //                    self.current_env_idx = Some(item.0);
    //                    found = true;
    //                    break;
    //                }
    //            }
    //        }
    //        if !found {
    //            self.current_env_idx = None;
    //        }
    //    }
    //    if let Some(te) = &self.temp_envs {
    //        if te.changed {
    //            self.all_envs = te.temp_envs.clone();
    //        }
    //    }
    //    self.temp_envs = None;

    //    let mut env_file = fs::File::create(format!("{}/{}", DATA_DIRECTORY, ENV_PATH))
    //        .map_err(|e| Error::FileOperationsErr(e))?;

    //    if self.all_envs.len() > 0 {
    //        env_file
    //            .write_all(
    //                serde_json::to_string(&self.all_envs.clone())
    //                    .map_err(|e| Error::JsonErr(e))?
    //                    .as_bytes(),
    //            )
    //            .map_err(|e| Error::FileOperationsErr(e))?;
    //    }
    //    Ok(())
    //}
    //pub fn new_environment(&mut self, name: String) {
    //    if let Some(te) = &mut self.temp_envs {
    //        te.temp_envs.push(environments::Environment::new(name));
    //        te.with_name_insertion = false;
    //        te.changed = true;
    //    }
    //}
    //pub fn change_active_env_panel(&mut self) {
    //    if let Some(temp) = &mut self.temp_envs {
    //        temp.change_environment_subsection()
    //    }
    //}
    //pub fn has_new_env_name(&self) -> bool {
    //    match &self.temp_envs {
    //        Some(t) => t.with_name_insertion,
    //        None => false,
    //    }
    //}
    //pub fn has_new_env_kv(&self) -> bool {
    //    match &self.temp_envs {
    //        Some(t) => t.with_kv_insertion,
    //        None => false,
    //    }
    //}
    //pub fn change_active_env_kv(&mut self) {
    //    match &mut self.temp_envs {
    //        Some(t) => t.kv_insertion.is_key_active = !t.kv_insertion.is_key_active,
    //        None => (),
    //    }
    //}
    //pub fn add_to_env_kv(&mut self, name: String, key: String, value: String) {
    //    if name == "".to_string() || key == "".to_string() || value == "".to_string() {
    //        return;
    //    }
    //    for item in self.temp_envs {
    //        if item.name == name {
    //            item.envs.insert(key.clone(), value.clone());
    //            item.envs_to_show.push([key, value]);
    //            break;
    //        }
    //    }
    //    t.kv_insertion.key = "".to_string();
    //    t.kv_insertion.value = "".to_string();
    //    t.with_kv_insertion = false;
    //    t.kv_insertion.is_key_active = true;
    //    t.changed = true;
    //}
    pub fn draw(&mut self, f: &mut Frame, rect: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(rect);
        let title = format!("Environment: {}", self.temp_env.name);
        let block = Block::default().title(title).borders(Borders::ALL);
        f.render_widget(block, rect);

        let mut state = TableState::default();
        state.select(Some(self.selected_kv));
        let items = Table::new(
            self.kvs.iter().map(|item| {
                Row::new(vec![item.get_key().clone(), item.get_value().clone()])
                    .style(Style::default().fg(Color::Green))
            }),
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .header(Row::new(vec!["Key", "Value"]))
        .block(Block::default().title("Environments").borders(Borders::ALL))
        .highlight_symbol(">>");
        f.render_widget(Clear, chunks[1]);
        f.render_stateful_widget(items, chunks[1], &mut state);
    }
}

fn into(data: HashMap<String, String>) -> Vec<KV> {
    let mut result = Vec::new();
    for (key, value) in data {
        result.push(KV::from(key, value));
    }
    result
}
