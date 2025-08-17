use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{
        Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget,
    },
};

/// A reusable scrollable list widget for ratatui that:
/// - Uses `ListState` for selection and built-in scrolling behavior
/// - Renders a vertical scrollbar that tracks the selection
/// - Supports custom item formatting via a formatter function
pub struct ScrollableList<'a, T> {
    /// Items to render inside the list
    pub items: Vec<T>,
    block: Block<'a>,
    formatter: Option<Box<dyn Fn(&T) -> ListItem>>,
}

impl<'a, T: ToString + Clone> ScrollableList<'a, T> {
    /// Create a new `ScrollableList` with the given items.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            formatter: None,
            block: Block::default(),
        }
    }

    /// Provide a custom formatter for items, enabling custom per-item rendering.
    ///
    /// Example:
    /// - Change symbols
    /// - Color some items conditionally
    /// - Add prefixes/suffixes
    pub fn with_formatter<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> ListItem + 'static,
    {
        self.formatter = Some(Box::new(f));
        self
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }
}

impl<'a, T: ToString + Clone> StatefulWidget for ScrollableList<'a, T> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = "title".to_string();

        // Use the whole area for the list and draw the scrollbar on the right side
        let list_area = area;

        // Track selection via ListState and mirror it to the scrollbar's position
        let selected = state.selected().unwrap_or(0);
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.items.len())
            .position(selected);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        // Build items, using a formatter if provided
        let items = self.items.iter().map(|item| {
            if let Some(ref f) = self.formatter {
                (f)(item)
            } else {
                ListItem::new(item.to_string())
            }
        });

        let list = List::new(items)
            .block(Block::new().title(title).borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol(">>");

        StatefulWidget::render(list, list_area, buf, state);
        scrollbar.render(list_area, buf, &mut scrollbar_state);
    }
}
