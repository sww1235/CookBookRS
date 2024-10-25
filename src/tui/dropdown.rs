use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidgetRef, Widget},
};

use ranged_wrapping::RangedWrapping;
/// representation of a drop down menu
#[derive(Default, Debug)]
pub struct Dropdown<'a> {
    /// entries in the dropdown
    entries: Vec<String>,
    block: Option<Block<'a>>,
    style: Style,
}

impl<'a> Dropdown<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            block: None,
            style: Style::default(),
        }
    }
    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
        self.entries.sort_unstable();
    }
    pub fn add_entries(&mut self, entries: Vec<String>) {
        self.entries.extend(entries);
        self.entries.sort_unstable();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn block(&mut self, block: Block<'a>) {
        self.block = Some(block);
    }
    pub fn style<S: Into<Style>>(&mut self, style: S) {
        self.style = style.into();
    }
}
#[derive(Default, Debug)]
pub struct DropdownState {
    pub selected_entry: RangedWrapping<usize, usize>,
    pub expanded: bool,
    pub scrolling: bool,
    pub visible_entries: Vec<String>,
    pub num_entries: RangedWrapping<usize, usize>,
}

//TODO: finish implementing dropdown widget, scrolling
impl StatefulWidgetRef for Dropdown<'_> {
    type State = DropdownState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.expanded {
            // area is the area of the collapsed box
            let mut entry_constraints = Vec::new();
            state.visible_entries = self.entries[..10].to_vec();
            for _ in &self.entries {
                //TODO: this may change
                entry_constraints.push(Constraint::Length(3));
            }
            if self.len() > 10 {
                state.scrolling = true
            }
            // don't want expanded height to be too big
            // TODO: List will scroll
            let expanded_rect_height: u16 = if self.len() >= 10 {
                3 * 10
            } else {
                match u16::try_from(3 * self.len()) {
                    Ok(val) => val,
                    Err(_) => u16::MAX,
                }
            };
            let expanded_rect = Rect::new(area.x, area.y, area.width, expanded_rect_height);
            // creating a vertical layout of boxes that will each contain one entry
            let entry_rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints(entry_constraints)
                .split(expanded_rect);
            for (i, area) in entry_rects.iter().enumerate() {
                // alternating colors
                let block_style = if i % 2 == 0 {
                    Style::new().on_black().white()
                } else {
                    Style::new().on_gray().white()
                };
                let paragraph = Paragraph::new(state.visible_entries[i].clone())
                    .block(Block::default().borders(Borders::LEFT | Borders::RIGHT).style(block_style));
                paragraph.render(*area, buf);
            }
        } else {
            // collapsed
            let collapsed_view =
                Paragraph::new(self.entries[state.selected_entry.0].clone()).block(self.block.clone().unwrap_or_default());
            collapsed_view.render(area, buf);
        }
    }
}
