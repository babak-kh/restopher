use super::models::Focus;
use super::{schema::Schema, State};
use crate::graphql::GraphQL;
use crate::request::Request;
use ratatui::prelude::*;
use ratatui::widgets::*;
use scrollable_widget::ScrollableList;

pub struct GqlUI {}

impl GqlUI {
    pub fn new() -> Self {
        Self {}
    }
    pub(super) fn draw(&mut self, f: &mut Frame, state: &State, schema: &Schema, rect: Rect) {
        f.render_widget(Clear, rect);
        let chunks = Layout::horizontal([Constraint::Min(0)].as_ref()).split(rect);

        self.render_main_content(f, state, schema, chunks[0]);
    }

    fn render_main_content(&mut self, f: &mut Frame, state: &State, schema: &Schema, rect: Rect) {
        // Create tabs for main navigation
        match state.focus {
            Focus::Objects | Focus::ObjectsFields => {
                // Inline rendering of objects and fields
                let objects = schema.get_object_types();

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                    .split(rect);

                // Objects list
                let items: Vec<ListItem> = objects
                    .iter()
                    .map(|name| ListItem::new(name.clone()))
                    .collect();

                let objects_list = ScrollableList::new(items)
                    .block(
                        Block::default()
                            .title(" Objects ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(
                                if matches!(state.focus, Focus::Objects) {
                                    Color::Yellow
                                } else {
                                    Color::Gray
                                },
                            )),
                    )
                    .highlight_style(Style::default().fg(Color::Yellow));

                let mut list_state = ListState::default();
                if !objects.is_empty() {
                    list_state.select(Some(state.object_idx.min(objects.len().saturating_sub(1))));
                }
                f.render_stateful_widget(objects_list, chunks[0], &mut list_state);

                // Fields table for selected object
                if !objects.is_empty() {
                    let selected_idx = state.object_idx.min(objects.len().saturating_sub(1));
                    let selected_object = &objects[selected_idx];

                    let fields: Vec<(String, String)> = schema
                        .schema
                        .definitions
                        .iter()
                        .filter_map(|def| {
                            if let graphql_parser::schema::Definition::TypeDefinition(
                                graphql_parser::schema::TypeDefinition::Object(obj),
                            ) = def
                            {
                                if &obj.name == selected_object {
                                    Some(
                                        obj.fields
                                            .iter()
                                            .map(|f| (f.name.clone(), f.field_type.to_string()))
                                            .collect::<Vec<_>>(),
                                    )
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect();

                    let rows: Vec<Row> = fields
                        .iter()
                        .map(|(name, ty)| {
                            Row::new(vec![Cell::from(name.clone()), Cell::from(ty.clone())])
                        })
                        .collect();

                    let table = Table::new(
                        rows,
                        &[Constraint::Percentage(40), Constraint::Percentage(60)],
                    )
                    .block(
                        Block::default()
                            .title(" Fields ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(
                                if matches!(state.focus, Focus::ObjectsFields) {
                                    Color::Yellow
                                } else {
                                    Color::Gray
                                },
                            )),
                    )
                    .highlight_style(Style::default().fg(Color::Yellow));

                    let mut tstate = TableState::default();
                    if !fields.is_empty() {
                        tstate.select(Some(
                            state.object_field_idx.min(fields.len().saturating_sub(1)),
                        ));
                    }
                    f.render_stateful_widget(table, chunks[1], &mut tstate);
                }
            }
            Focus::Queries | Focus::QueriesFields => {
                // Show list of queries with current selection
                let items: Vec<ListItem> = schema
                    .get_queries()
                    .into_iter()
                    .map(|q| ListItem::new(q))
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .title(" Queries ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_style(Style::default().fg(Color::Yellow));

                let mut list_state = ListState::default();
                let queries_len = schema.get_queries().len();
                if queries_len > 0 {
                    list_state.select(Some(state.query_idx.min(queries_len.saturating_sub(1))));
                }

                f.render_stateful_widget(list, rect, &mut list_state);
            }
            Focus::Mutations | Focus::MutationsFields => {
                // Show list of mutations with current selection
                let items: Vec<ListItem> = schema
                    .get_mutations()
                    .into_iter()
                    .map(|m| ListItem::new(m))
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .title(" Mutations ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_style(Style::default().fg(Color::Yellow));

                let mut list_state = ListState::default();
                let mutations_len = schema.get_mutations().len();
                if mutations_len > 0 {
                    list_state.select(Some(
                        state.mutation_idx.min(mutations_len.saturating_sub(1)),
                    ));
                }

                f.render_stateful_widget(list, rect, &mut list_state);
            }
            _ => {
                // Default fallback
                let block = Block::default()
                    .title(" GraphQL Explorer ")
                    .borders(Borders::ALL);
                f.render_widget(block, rect);
            }
        }
    }

    //fn render_footer(f: &mut Frame, app: &App, rect: Rect) {
    //    let status_text = if let Some(error) = &app.error_message {
    //        error.clone()
    //    } else {
    //        match app.focus {
    //        Focus::Objects => "Tab: Switch panels | ↑↓: Navigate | →: View fields | Ctrl+Q: Quit",
    //        Focus::ObjectsFields => "←: Back to objects | ↑↓: Navigate | Ctrl+Q: Quit",
    //        Focus::Queries => {
    //            "Tab: Switch panels | ↑↓: Navigate | →: View fields | Enter: Select | Ctrl+Q: Quit"
    //        }
    //        Focus::QueriesFields => "←: Back to queries | ↑↓: Navigate | Ctrl+Q: Quit",
    //        Focus::Mutations => {
    //            "Tab: Switch panels | ↑↓: Navigate | →: View fields | Enter: Select | Ctrl+Q: Quit"
    //        }
    //        Focus::MutationsFields => "←: Back to mutations | ↑↓: Navigate | Ctrl+Q: Quit",
    //        Focus::ArgumentInput => {
    //            "Esc: Cancel | Enter: Next | ↑↓: Navigate fields | Ctrl+Q: Quit"
    //        }
    //        Focus::FieldSelection => {
    //            "Esc: Back | Space: Toggle field | ↑↓←→: Navigate | Enter: Generate | Ctrl+Q: Quit"
    //        }
    //        Focus::ResultPreview => "Esc: Back to fields | Ctrl+Q: Quit",
    //        _ => "Ctrl+Q: Quit",
    //    }
    //    .to_string()
    //    };

    //    let status_style = if app.error_message.is_some() {
    //        Style::default().fg(Color::Red)
    //    } else {
    //        Style::default().fg(Color::White)
    //    };

    //    let footer = Paragraph::new(status_text).style(status_style).block(
    //        Block::default()
    //            .borders(Borders::ALL)
    //            .border_style(Style::default().fg(Color::Blue))
    //            .title(" Status "),
    //    );

    //    f.render_widget(footer, rect);
    //}

    ///// Render a help overlay
    //fn render_help_overlay(f: &mut Frame, rect: Rect) {
    //    // Create a slightly smaller centered rect for the help dialog
    //    let help_area = Layout::default()
    //        .direction(Direction::Vertical)
    //        .constraints([
    //            Constraint::Percentage(20),
    //            Constraint::Percentage(60),
    //            Constraint::Percentage(20),
    //        ])
    //        .split(rect);

    //    let help_horizontal = Layout::default()
    //        .direction(Direction::Horizontal)
    //        .constraints([
    //            Constraint::Percentage(15),
    //            Constraint::Percentage(70),
    //            Constraint::Percentage(15),
    //        ])
    //        .split(help_area[1])[1];

    //    let help_content = "
    //GraphQL Explorer Help

    //Global Shortcuts:
    //- Ctrl+Q: Quit application
    //- Ctrl+H: Toggle this help screen

    //Navigation:
    //- Tab: Switch between Objects, Queries, and Mutations
    //- ↑↓: Navigate within lists
    //- ←→: Move between lists and details
    //- Enter: Select an item or proceed
    //- Esc: Go back or cancel

    //Field Selection:
    //- Space: Toggle field selection
    //- Enter: Confirm selection
    //";

    //    let help_paragraph = Paragraph::new(help_content).block(
    //        Block::default()
    //            .title(" Help ")
    //            .borders(Borders::ALL)
    //            .border_style(Style::default().fg(Color::Cyan)),
    //    );

    //    f.render_widget(help_paragraph, help_horizontal);
    //}

    ///// Render a confirmation dialog
    //fn render_confirmation_dialog(f: &mut Frame, app: &App, rect: Rect) {
    //    // Create a small centered rect for the confirmation dialog
    //    let dialog_area = Layout::default()
    //        .direction(Direction::Vertical)
    //        .constraints([
    //            Constraint::Percentage(30),
    //            Constraint::Percentage(40),
    //            Constraint::Percentage(30),
    //        ])
    //        .split(rect);

    //    let dialog_horizontal = Layout::default()
    //        .direction(Direction::Horizontal)
    //        .constraints([
    //            Constraint::Percentage(25),
    //            Constraint::Percentage(50),
    //            Constraint::Percentage(25),
    //        ])
    //        .split(dialog_area[1])[1];

    //    let message = app
    //        .ui_state
    //        .confirmation_message
    //        .clone()
    //        .unwrap_or_default();
    //    let dialog_content = format!("{}\n\nPress Enter to confirm or Esc to cancel", message);

    //    let dialog = Paragraph::new(dialog_content).block(
    //        Block::default()
    //            .title(" Confirmation ")
    //            .borders(Borders::ALL)
    //            .border_style(Style::default().fg(Color::Yellow)),
    //    );

    //    f.render_widget(dialog, dialog_horizontal);
    //}
}
