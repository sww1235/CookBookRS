use super::{
    equipment::Equipment,
    ingredient::Ingredient,
    step::{Step, StepType},
    tag::Tag,
};

use std::num::Wrapping;
use std::{collections::HashMap, fmt};

use dimensioned::ucum;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, Widget, WidgetRef},
};

/// `Recipe` represents one recipe from start to finish
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Recipe {
    /// database ID
    pub id: u64,
    /// short name of recipe
    pub name: String,
    /// optional description
    pub description: Option<String>,
    //TODO: maybe make comments a bit more formal, want to be able to record when recipe was last
    //made
    /// recipe comments
    pub comments: Option<String>,
    /// recipe source
    pub source: String,
    /// recipe author
    pub author: String,
    /// amount made
    pub amount_made: AmountMade,
    /// list of steps in recipe
    pub steps: Vec<Step>,
    /// list of tags on recipe
    pub tags: Vec<Tag>,
    //TODO: versions
    /// if the recipe has unsaved changes or not
    //TODO: figure out a save system
    pub saved: bool,
}

/// [`AmountMade`] represents the total finished quantity that the recipe makes, like 24 cookies,
/// 24 servings, 6 portions, etc.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AmountMade {
    /// amount made
    pub quantity: u64,
    /// units for amount made.
    ///
    /// Thse are not type checked at all and are treated as a base quantity internally.
    /// This is just a representation of the units to display.
    /// There may be a future addition that automatically calculates calories, or serving
    /// sizes based on calories.
    pub units: String,
}

impl fmt::Display for AmountMade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Makes: {} {}", self.quantity, self.units)
    }
}

impl Recipe {
    /// `new` creates a new [`Recipe`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: u64::MIN,
            name: String::default(),
            description: None,
            comments: None,
            source: String::default(),
            author: String::default(),
            amount_made: AmountMade::default(),
            steps: Vec::new(),
            tags: Vec::new(),
            //TODO: versions
            saved: false,
        }
    }

    /// `step_time_totals` provides the time required for each type of step as a `HashMap`
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    pub fn step_time_totals(&self) -> HashMap<StepType, Option<ucum::Second<f64>>> {
        let mut out_map: HashMap<StepType, Option<ucum::Second<f64>>> = HashMap::new();
        for step in &self.steps {
            out_map
                .entry(step.step_type)
                .and_modify(|e: &mut Option<ucum::Second<f64>>| {
                    add(e, step.time_needed);
                })
                .or_insert(step.time_needed);
        }
        out_map
    }
    /// `total_time` returns the total time required for a recipe
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
    pub fn total_time(&self) -> ucum::Second<f64> {
        let mut time = 0.0_f64 * ucum::S;
        for step in &self.steps {
            time += step.time_needed.unwrap_or(0.0_f64 * ucum::S);
        }
        time
    }
    /// `ingredient_list` returns the total amount of ingredients required to make the recipe
    #[must_use]
    pub fn ingredient_list(&self) -> HashMap<String, Ingredient> {
        let mut out: HashMap<String, Ingredient> = HashMap::new();
        #[allow(clippy::arithmetic_side_effects)] //TODO: fix this
        for step in &self.steps {
            for ingredient in &step.ingredients {
                if let Some(i) = out.get_mut(&ingredient.name) {
                    i.unit_quantity += ingredient.unit_quantity;
                } else {
                    //TODO: figure out if ingredients should be tracked using RC or not
                    out.insert(ingredient.name.clone(), ingredient.clone());
                }
            }
        }
        out
    }
    /// `equipment_list` returns the overall list of equipment needed to make the recipe
    #[must_use]
    pub fn equipment_list(&self) -> Vec<Equipment> {
        let mut out = Vec::new();
        for step in &self.steps {
            for equipment in &step.equipment {
                // all short circuits if the closure returns false, and then returns false. We
                // invert that false to true to see if a value is not contained in the vector
                if !out.iter().all(|e| e == equipment) {
                    out.push(equipment.clone());
                }
            }
        }
        out
    }
    /// `all_equipment_owned` returns the overall list of equipment needed to make the recipe
    #[must_use]
    pub fn all_equipment_owned(&self) -> bool {
        // iterate through all equipment in all steps, short circuiting if e.is_owned is false
        self.steps
            .iter()
            .all(|s| s.equipment.iter().all(|e| e.is_owned))
    }
}

/// [`RecipeState`]
#[derive(Default, Debug)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct RecipeState {
    //TODO: selected field, which step is selected, etc
    pub selected_field: Wrapping<usize>,
}

///// [`RecipeField`]
//#[derive(Default, Debug)]
//#[allow(clippy::module_name_repetitions)]
//#[allow(missing_docs)]
//pub enum RecipeField {
//    #[default]
//    Name,
//    Description,
//    Comments,
//    Source,
//    Author,
//    AmountMade,
//}

// display version of recipe
impl WidgetRef for Recipe {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        //TODO: implement
    }
}
// edit version of recipe
#[allow(
    non_upper_case_globals,
    clippy::missing_docs_in_private_items,
    clippy::items_after_statements
)] //TODO: remove after derive implementation
impl StatefulWidgetRef for Recipe {
    type State = RecipeState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Use split here, since we don't care about naming the fields specifically

        //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
        //function if too many fields
        //
        //Want
        //- comment field to be at least 5 lines high plus borders
        //- description field to be at least 5 lines high plus borders
        //- want other fields to be 1 line high plus borders
        //- need 3 rows for space for blocks at bottom

        let mut recipe_edit_constraints = Vec::new();

        // name, description, comments, source, author, amount_made
        const num_fields: usize = 6;
        const num_special_fields: usize = 2;
        // subtract 2 for comment/description fields
        // multiply by 3 for other field total height
        // add 7 for comment field
        // add 7 for description field
        // add 3 for bottom blocks
        // add 2 for border? //TODO: fix borders
        const required_field_height: usize =
            ((num_fields - num_special_fields) * 3) + 7 + 7 + 3 + 2;
        if usize::from(area.height) >= required_field_height {
            // recipe_area.height is greater than minimum required

            // need 2 for border and 1 for text.
            // name
            recipe_edit_constraints.push(Constraint::Length(3));
            // description
            recipe_edit_constraints.push(Constraint::Min(7));
            //for now, just a bigger area.
            //TODO: special case this for additional comment functionality
            // comments
            recipe_edit_constraints.push(Constraint::Min(7));
            // source
            recipe_edit_constraints.push(Constraint::Length(3));
            // author
            recipe_edit_constraints.push(Constraint::Length(3));
            // amount_made
            recipe_edit_constraints.push(Constraint::Length(3));
        } else {
            //TODO: implement scrolling
            todo!()
        }
        // last constraint for step/equipment block
        recipe_edit_constraints.push(Constraint::Length(3));
        let edit_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(recipe_edit_constraints)
            .split(area);

        let name_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Name");
        let mut name_style = Style::default();

        //TODO: repeat this for other fields
        if state.selected_field.0 == 0 {
            // this is edit style. //TODO: standardize this
            name_style = name_style.fg(Color::Red);
        }
        let name_paragraph =
            Paragraph::new(Text::styled(self.name.clone(), name_style)).block(name_block);
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

        let comment_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("comments");
        let comment_paragraph = Paragraph::new(Text::styled(
            //TODO: remove this clone in the future
            self.comments.clone().unwrap_or_default(),
            Style::default().fg(Color::Red),
        ))
        .block(comment_block);
        //TODO: update state here
        comment_paragraph.render(edit_layout[2], buf);

        let source_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("source");
        let source_paragraph = Paragraph::new(Text::styled(
            self.source.clone(),
            Style::default().fg(Color::Red),
        ))
        .block(source_block);
        //TODO: update state here
        source_paragraph.render(edit_layout[3], buf);

        let author_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("source");
        let author_paragraph = Paragraph::new(Text::styled(
            self.author.clone(),
            Style::default().fg(Color::Red),
        ))
        .block(author_block);
        //TODO: update state here
        author_paragraph.render(edit_layout[4], buf);

        let amount_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("amount");
        let amount_paragraph = Paragraph::new(Text::styled(
            self.amount_made.to_string(),
            Style::default().fg(Color::Red),
        ))
        .block(amount_block);
        //TODO: update state here
        amount_paragraph.render(edit_layout[5], buf);

        // recipe_edit_layout should always have something in it.
        // This is a valid place to panic
        #[allow(clippy::expect_used)]
        let [left_info_area, right_info_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(*edit_layout.last().expect("No edit areas defined"));

        let step_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Number of Steps");

        let step_count = Paragraph::new(Text::styled(
            self.steps.len().to_string(),
            Style::default().fg(Color::Green),
        ))
        .block(step_block);
        step_count.render(left_info_area, buf);
        // render an empty block with borders on the right
        Widget::render(Block::default().borders(Borders::ALL), right_info_area, buf);
    }
}

//https://www.reddit.com/r/learnrust/comments/1b1xwci/best_way_to_add_an_optiont_to_an_optiont/
/// helper function for `step_time_totals` to allow adding an option and an option togther
fn add(
    lhs: &mut Option<ucum::Second<f64>>,
    rhs: Option<ucum::Second<f64>>,
) -> Option<ucum::Second<f64>> {
    #[allow(clippy::arithmetic_side_effects)] //TODO: change this to saturating
    match (lhs, rhs) {
        (Some(l), Some(r)) => Some(*l + r),
        (Some(l), None) => Some(*l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}
