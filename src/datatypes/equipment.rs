use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, StatefulWidget, StatefulWidgetRef, Widget,
        WidgetRef,
    },
};
/// `Equipment` represents any implement you might use to prepare a recipe,
/// from a stove, to a microwave, to a stand mixer, to a potato peeler
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Equipment {
    /// database unique ID
    pub id: u64,
    /// short name of item
    pub name: String,
    /// longer description of item
    pub description: Option<String>,
    /// If item is owned. Allows filtering out recipes that require equipment you don't own so you
    /// don't get half way through a recipe and realize it needs some specialized piece of
    /// equipment like a melon baller or pineapple corer
    pub is_owned: bool,
}

#[derive(Debug, Default)]
pub struct EquipmentState {
    //TODO: selected field, etc
}

// Display version of ingredient
impl WidgetRef for Equipment {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        //TODO: implement
    }
}

// edit version of ingredient
impl StatefulWidgetRef for Equipment {
    type State = EquipmentState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Use split here, since we don't care about naming the fields specifically

        //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
        //function if too many fields
        //
        //Want
        //- fields to be 1 line high plus borders
        //- need 3 rows for space for blocks at bottom

        let mut layout_constraints = Vec::new();

        // name, description, unit_quantity
        let num_fields = 3;
        let num_special_fields = 1;
        // subtract 1 for description
        // multiply by 3 for other field total height
        // add 7 for description
        // add 3 for bottom blocks
        // add 2 for border? //TODO: fix borders
        let required_field_height = ((num_fields - num_special_fields) * 3) + 7 + 3 + 2;

        if usize::from(area.height) >= required_field_height {
            // recipe_area.height is greater than minimum required
            // need 2 for border and 1 for text.
            // name
            layout_constraints.push(Constraint::Length(3));
            // description
            layout_constraints.push(Constraint::Min(7));
            // is_owned
            layout_constraints.push(Constraint::Length(3));
        } else {
            //TODO: implement scrolling
            todo!()
        }

        // last constraint for step/equipment block
        layout_constraints.push(Constraint::Length(3));
        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(layout_constraints)
            .split(area);

        let name_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Name");
        let name_paragraph = Paragraph::new(Text::styled(
            self.name.clone(),
            Style::default().fg(Color::Red),
        ))
        .block(name_block);
        //TODO: update state here
        name_paragraph.render(edit_layout[0], buf);

        let description_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("description");
        let description_paragraph = Paragraph::new(Text::styled(
            self.description.clone().unwrap_or_default(),
            Style::default().fg(Color::Red),
        ))
        .block(description_block);
        //TODO: update state here
        description_paragraph.render(edit_layout[1], buf);

        let owned_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Is Equipment Owned");
        //TODO: fix this input, and allow for proper true/false parsing
        let owned_paragraph = Paragraph::new(Text::styled(
            self.is_owned.to_string(),
            Style::default().fg(Color::Red),
        ))
        .block(owned_block);
        //TODO: update state here
        owned_paragraph.render(edit_layout[2], buf);

        // ingredient_edit_layout should always have something in it.
        // This is a valid place to panic
        #[allow(clippy::expect_used)]
        let [left_info_area, right_info_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(*edit_layout.last().expect("No edit areas defined"));

        let step_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Step Number");

        //FIXME: figure out how to display step_id or something else
        let step_id = Paragraph::new(Text::styled("fixme", Style::default().fg(Color::Green)))
            .block(step_block);
        step_id.render(left_info_area, buf);
        // render an empty block with borders on the right
        Widget::render(Block::default().borders(Borders::ALL), right_info_area, buf);
    }
}
