use super::{equipment::Equipment, ingredient::Ingredient};

use std::fmt;

use dimensioned::ucum;
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

/// `Step` represents a discrete step within a recipe
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Step {
    /// database ID
    pub id: u64,
    /// time needed to perform this step in the recipe
    /// Optional for informational steps, or steps that
    /// don't traditionally have durations associated
    pub time_needed: Option<ucum::Second<f64>>,
    /// cook temperature. Optional for steps that don't involve temperature or cooking
    pub temperature: Option<ucum::Kelvin<f64>>,
    /// instructions for step
    pub instructions: String,
    /// ingredients used in this step
    pub ingredients: Vec<Ingredient>,
    /// equipment used in this step
    pub equipment: Vec<Equipment>,
    /// Step type
    pub step_type: StepType,
}

#[derive(Default, Debug)]
pub struct StepState {
    //TODO: selected field, which ingredient/equipment is selected, etc
}

// Display version of step
impl WidgetRef for Step {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        //TODO: implement
    }
}

// edit version of step
impl StatefulWidgetRef for Step {
    type State = StepState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Use split here, since we don't care about naming the fields specifically

        //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
        //function if too many fields
        //
        //Want
        //- want fields to be 1 line high plus borders
        //- need 3 rows for space for blocks at bottom

        let mut step_edit_constraints = Vec::new();

        //time_needed, temperature, instructions, step_type
        let num_fields = 4;
        let num_special_fields = 0;
        // multiply by 3 for other field total height
        // add 3 for bottom blocks
        // add 2 for border? //TODO: fix borders
        let required_field_height = ((num_fields - num_special_fields) * 3) + 3 + 2;

        if usize::from(area.height) >= required_field_height {
            // recipe_area.height is greater than minimum required
            // need 2 for border and 1 for text.

            // time_needed
            step_edit_constraints.push(Constraint::Length(3));
            // temperature
            step_edit_constraints.push(Constraint::Length(3));
            // instructions
            step_edit_constraints.push(Constraint::Min(3));
            // step_type
            step_edit_constraints.push(Constraint::Length(3));
        } else {
            //TODO: implement scrolling
            todo!()
        }
        // last constraint for step/equipment block
        step_edit_constraints.push(Constraint::Length(3));
        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(step_edit_constraints)
            .split(area);

        let time_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Time Needed");
        //TODO: fix this input, and allow for proper unit/numeric entry and parsing
        let time_paragraph = Paragraph::new(Text::styled(
            self.time_needed.unwrap_or_default().to_string(),
            Style::default().fg(Color::Red),
        ))
        .block(time_block);
        //TODO: update state here
        time_paragraph.render(edit_layout[0], buf);

        let temp_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!("{} temp", self.step_type));
        //TODO: fix this input, and allow for proper unit/numeric entry and parsing
        let temp_paragraph = Paragraph::new(Text::styled(
            self.temperature.unwrap_or_default().to_string(),
            Style::default().fg(Color::Red),
        ))
        .block(temp_block);
        //TODO: update state here
        temp_paragraph.render(edit_layout[1], buf);

        //time_needed, temperature, instructions, step_type
        let instruction_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Instructions");
        let instruction_paragraph = Paragraph::new(Text::styled(
            self.instructions.clone(),
            Style::default().fg(Color::Red),
        ))
        .block(instruction_block);
        //TODO: update state here
        instruction_paragraph.render(edit_layout[2], buf);

        let step_type_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Step Type");
        //TODO: fix this input, and allow for proper enum parsing/entry
        let step_type_paragraph = Paragraph::new(Text::styled(
            self.step_type.to_string(),
            Style::default().fg(Color::Red),
        ))
        .block(step_type_block);
        //TODO: update state here
        step_type_paragraph.render(edit_layout[3], buf);

        // step_edit_layout should always have something in it.
        // This is a valid place to panic
        #[allow(clippy::expect_used)]
        let [left_info_area, right_info_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(*edit_layout.last().expect("No edit areas defined"));

        let equipment_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Equipment Count for Step"); //TODO: add step number?

        let equipment_count = Paragraph::new(Text::styled(
            self.equipment.len().to_string(),
            Style::default().fg(Color::Green),
        ))
        .block(equipment_block);
        equipment_count.render(left_info_area, buf);

        let ingredient_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Ingredient Count for Step"); //TODO: add step number?

        let ingredient_count = Paragraph::new(Text::styled(
            self.ingredients.len().to_string(),
            Style::default().fg(Color::Green),
        ))
        .block(ingredient_block);

        ingredient_count.render(right_info_area, buf);
    }
}

/// `StepType` represents what type of step each step is in a recipe. It is used to bucket times
/// for recipe total duration
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum StepType {
    /// Prep steps
    Prep,
    /// cook steps
    Cook,
    /// waiting steps
    Wait,
    /// Other steps
    #[default]
    Other,
}

impl fmt::Display for StepType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Prep => write!(f, "Prep"),
            Self::Cook => write!(f, "Cook"),
            Self::Wait => write!(f, "Wait"),
            Self::Other => write!(f, "Other"),
        }
    }
}
