use dimensioned::ucum;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, Widget, WidgetRef},
};

use std::num::Wrapping;
use std::ops::{Add, AddAssign};

/// `Ingredient` is a unique item that represents the quantity of a particular ingredient
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ingredient {
    /// database ID
    pub id: u64,
    /// ingredient short name
    pub name: String,
    /// optional description
    pub description: Option<String>,
    /// Unit and quantity of ingredient
    pub unit_quantity: UnitType,
    //TODO: inventory reference
}

/// `UnitType` handles different unit types for an ingredient and allows flexibility rather than
/// needing to have 1 ingredient type per unit type
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitType {
    /// Represents a count or physical quantity of an `Ingredient`:
    /// Ex: 30 chocolate chips, 5 bananas, 10 carrots etc.
    Quantity(f64),
    /// Mass of an `Ingredient`
    Mass(ucum::Gram<f64>),
    /// Volume of an `Ingredent`
    Volume(ucum::Meter3<f64>),
}

/// [`IngredientState`]
#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct IngredientState {
    //TODO: selected field, etc
    pub selected_field: Wrapping<usize>,
}

// Display version of ingredient
impl WidgetRef for Ingredient {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        //TODO: implement
    }
}

// edit version of ingredient
#[allow(
    non_upper_case_globals,
    clippy::missing_docs_in_private_items,
    clippy::items_after_statements
)] //TODO: remove after derive implementation
impl StatefulWidgetRef for Ingredient {
    type State = IngredientState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Use split here, since we don't care about naming the fields specifically

        //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
        //function if too many fields
        //
        //Want
        //- fields to be 1 line high plus borders
        //- need 3 rows for space for blocks at bottom

        let mut ingredient_edit_constraints = Vec::new();

        // name, description, unit_quantity
        const num_fields: usize = 3;
        const num_special_fields: usize = 1;
        // subtract 1 for description
        // multiply by 3 for other field total height
        // add 7 for description
        // add 3 for bottom blocks
        // add 2 for border? //TODO: fix borders
        const required_field_height: usize = ((num_fields - num_special_fields) * 3) + 7 + 3 + 2;

        if usize::from(area.height) >= required_field_height {
            // recipe_area.height is greater than minimum required
            // need 2 for border and 1 for text.
            // name
            ingredient_edit_constraints.push(Constraint::Length(3));
            // description
            ingredient_edit_constraints.push(Constraint::Min(7));
            // unit_quantity
            //TODO: unit quantity stuff
        } else {
            //TODO: implement scrolling
            todo!()
        }

        // last constraint for step/equipment block
        ingredient_edit_constraints.push(Constraint::Length(3));
        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(ingredient_edit_constraints)
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

        //TODO: implement
        // name, description, unit_quantity
        //let unit_block = Block::default()
        //    .borders(Borders::ALL)
        //    .style(Style::default())
        //    .title("Quantity and units");
        //TODO: fix this input, and allow for proper unit/numeric entry and parsing
        //let unit_paragraph = Paragraph::new(Text::styled(
        //    self.time_needed.unwrap_or_default().to_string(),
        //    Style::default().fg(Color::Red),
        //))
        //.block(time_block);
        //TODO: update state here
        //time_paragraph.render(edit_layout[2], buf);

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

impl Add for UnitType {
    type Output = Self;

    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Quantity(l), Self::Quantity(r)) => Self::Quantity(l + r),
            (Self::Mass(l), Self::Mass(r)) => Self::Mass(l + r),
            (Self::Volume(l), Self::Volume(r)) => Self::Volume(l + r),
            _ => panic!(
                "Attempted to add different unit types together. This should not have happened"
            ),
        }
    }
}
impl AddAssign for UnitType {
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl Default for UnitType {
    fn default() -> Self {
        Self::Quantity(0.0_f64)
    }
}
