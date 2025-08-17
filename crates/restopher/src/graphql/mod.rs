mod models;
mod schema;
mod ui;

use std::f64::consts::E;

use crate::keys::keys::{Event, Key};
use crate::trace_dbg;
use anyhow::Result;
use models::Focus;
use ratatui::prelude::*;
use ratatui::widgets::*;
pub use ui::GqlUI;

use crate::request::Request;

struct State {
    focus: Focus,

    object_idx: usize,
    object_field_idx: usize,

    query_idx: usize,
    query_field_idx: usize,

    mutation_idx: usize,
    mutation_field_idx: usize,
}
impl State {
    pub fn new() -> Self {
        Self {
            focus: Focus::Objects,
            object_idx: 0,
            object_field_idx: 0,
            query_idx: 0,
            query_field_idx: 0,
            mutation_idx: 0,
            mutation_field_idx: 0,
        }
    }
}

pub struct GraphQL {
    schema: schema::Schema,
    ui: GqlUI,
    state: State,
}

impl GraphQL {
    pub fn new() -> Result<Self> {
        let s = schema::Schema::new()?;
        Ok(Self {
            schema: s,
            ui: GqlUI::new(),
            state: State::new(),
        })
    }
    pub fn draw(&mut self, f: &mut Frame, _: &Request, rect: Rect) {
        self.ui.draw(f, &self.state, &self.schema, rect);
    }
    pub fn update(&mut self, _: &mut Request, event: &Event) {
        match self.state.focus {
            Focus::Objects | Focus::Queries | Focus::Mutations => {
                self.handle_main_navigation(event)
            }
            Focus::ObjectsFields | Focus::QueriesFields | Focus::MutationsFields => {
                self.handle_fields_navigation(event);
            }
            _ => {}
        }
    }

    pub fn handle_main_navigation(&mut self, event: &Event) {
        match event.key {
            Key::Up => match self.state.focus {
                Focus::Objects => {
                    self.state.object_idx = self.state.object_idx.saturating_sub(1);
                    self.state.object_field_idx = 0; // Reset field index when moving up
                }
                Focus::Queries => {
                    self.state.query_idx = self.state.query_idx.saturating_sub(1);
                    self.state.query_field_idx = 0; // Reset field index when moving up
                }
                Focus::Mutations => {
                    self.state.mutation_idx = self.state.mutation_idx.saturating_sub(1);
                    self.state.mutation_field_idx = 0; // Reset field index when moving up
                }
                _ => {}
            },
            Key::Down => match self.state.focus {
                Focus::Objects => {
                    incr(
                        &mut self.state.object_idx,
                        self.schema.schema.definitions.len(),
                    );
                    self.state.object_field_idx = 0; // Reset field index when moving down
                }
                Focus::Queries => {
                    incr(&mut self.state.query_idx, self.schema.get_queries().len());
                    self.state.query_field_idx = 0; // Reset field index when moving down
                }
                Focus::Mutations => {
                    incr(
                        &mut self.state.mutation_idx,
                        self.schema.get_mutations().len(),
                    );
                    self.state.mutation_field_idx = 0; // Reset field index when moving down
                }
                _ => {}
            },
            Key::Tab => match self.state.focus {
                Focus::Objects => self.state.focus = Focus::Queries,
                Focus::Queries => self.state.focus = Focus::Mutations,
                Focus::Mutations => self.state.focus = Focus::Objects,
                _ => {}
            },
            Key::Right => {
                self.move_to_fields();
            }
            _ => {}
        }
    }

    /// Handles navigation within fields screens
    pub fn handle_fields_navigation(&mut self, event: &Event) {
        match event.key {
            Key::Left => match self.state.focus {
                Focus::ObjectsFields => self.state.focus = Focus::Objects,
                Focus::QueriesFields => self.state.focus = Focus::Queries,
                Focus::MutationsFields => self.state.focus = Focus::Mutations,
                _ => {}
            },
            Key::Up => match self.state.focus {
                Focus::ObjectsFields => {
                    // Get the current object to access its fields
                    let all_objects = self.schema.schema.definitions.iter();
                    let objects = all_objects
                        .filter_map(|def| match def {
                            graphql_parser::schema::Definition::TypeDefinition(x) => match x {
                                graphql_parser::schema::TypeDefinition::Object(obj) => Some(obj),
                                _ => None,
                            },
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    if let Some(object) = objects.get(self.state.object_field_idx) {
                        self.state.object_field_idx = self.state.object_field_idx.saturating_sub(1);
                    }
                }
                Focus::QueriesFields => {
                    // Handle queries fields navigation
                    self.state.query_field_idx = self.state.query_field_idx.saturating_sub(1);
                }
                Focus::MutationsFields => {
                    // Handle mutations fields navigation
                    self.state.mutation_field_idx = self.state.mutation_field_idx.saturating_sub(1);
                }
                _ => {}
            },
            Key::Down => match self.state.focus {
                Focus::ObjectsFields => {
                    // Get the current object to access its fields
                    let all_objects = self.schema.schema.definitions.iter();
                    let objects = all_objects
                        .filter_map(|def| match def {
                            graphql_parser::schema::Definition::TypeDefinition(x) => match x {
                                graphql_parser::schema::TypeDefinition::Object(obj) => Some(obj),
                                _ => None,
                            },
                            _ => None,
                        })
                        .collect::<Vec<_>>();

                    if let Some(object) = objects.get(self.state.object_field_idx) {
                        incr(&mut self.state.object_field_idx, object.fields.len());
                    }
                }
                Focus::QueriesFields => {
                    // Handle queries fields navigation
                    let queries = self.schema.get_queries();
                    if let Some(query) = queries.get(self.state.query_idx) {
                        incr(
                            &mut self.state.query_field_idx,
                            self.schema.get_fields(query).len(),
                        );
                    }
                }
                Focus::MutationsFields => {
                    // Handle mutations fields navigation
                    let mutations = self.schema.get_mutations();
                    if let Some(mutation) = mutations.get(self.state.mutation_idx) {
                        incr(&mut self.state.mutation_field_idx, mutation.len());
                    }
                }
                _ => {}
            },
            Key::Enter => match self.state.focus {
                Focus::QueriesFields => {}
                Focus::MutationsFields => {}
                _ => {}
            },
            _ => {}
        }
    }

    fn move_to_fields(&mut self) {
        match self.state.focus {
            Focus::Objects => self.state.focus = Focus::ObjectsFields,
            Focus::Queries => self.state.focus = Focus::QueriesFields,
            Focus::Mutations => self.state.focus = Focus::MutationsFields,
            _ => (),
        }
    }

    /// Increments a counter with wrapping behavior
    pub fn incr(counter: &mut usize, max: usize) {
        *counter = (*counter + 1) % max.max(1);
    }
}

pub fn incr(counter: &mut usize, max: usize) {
    *counter = (*counter + 1) % max.max(1);
}
