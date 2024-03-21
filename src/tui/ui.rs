use crate::{
    datatypes::{equipment::Equipment, ingredient::Ingredient, recipe::Recipe, step::Step},
    tui::app::{App, CurrentScreen, EditingState},
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use struct_field_names_as_array::FieldNamesAsSlice;

/// `layout` contains the layout code for the initial UI
///
/// # Panics
/// - Can panic if constraint vectors are null. This should never happen
#[allow(clippy::too_many_lines)] //TODO: remove this after refactoring
pub fn layout(frame: &mut Frame, app: &mut App) {
    let screen_area = frame.size();
    // This should create a layout of 3 vertical columns
    // with the outer 2 taking up 25% of the space, and
    // the middle one taking up the center 50%
    // use [`Layout.areas()'] rather than [`Layout.split()`] for better API
    let [recipe_list_area, main_area, tag_list_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .areas(screen_area);

    // This should split the middle box into 3 areas, one on the bottom that will hold the menu and
    // be 3 unit tall, one on the top that will show the title of the current recipe and be 5
    // units tall, and the middle will take up the remaining space
    let [title_area, recipe_area, menu_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(5),
            Constraint::Percentage(100),
            Constraint::Min(3),
        ])
        .areas(main_area);

    //TODO: fix this styling
    //Block is a box around the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    match app.current_screen {
        CurrentScreen::RecipeBrowser => {
            //TODO: change title
            let title = Paragraph::new(Text::styled("Cookbook", Style::default().fg(Color::Blue)))
                .block(title_block);

            //render_widget essentially pushes each widget into a frameusing the layout handler defined
            //earlier
            frame.render_widget(title, title_area);

            let mut recipe_list_items = Vec::<ListItem>::new();

            if recipe_list_items.is_empty() {
                recipe_list_items.push(ListItem::new(Line::from(Span::styled(
                    "No Recipes",
                    Style::default().fg(Color::Red),
                ))));
            } else {
                for recipe in &app.recipes {
                    recipe_list_items.push(ListItem::new(Line::from(Span::styled(
                        recipe.name.clone(),
                        Style::default().fg(Color::Green),
                    ))));
                }
            }

            let recipe_list =
                List::new(recipe_list_items).block(Block::default().borders(Borders::ALL));
            app.recipe_list_len = recipe_list.len();

            frame.render_stateful_widget(recipe_list, recipe_list_area, &mut app.recipe_list_state);

            let mut tag_list_items = Vec::<ListItem>::new();
            if app.tags.is_empty() {
                tag_list_items.push(ListItem::new(Line::from(Span::styled(
                    "No Tags",
                    Style::default().fg(Color::Red),
                ))));
            } else {
                for tag in &app.tags {
                    tag_list_items.push(ListItem::new(Line::from(Span::styled(
                        tag,
                        Style::default().fg(Color::White),
                    ))));
                }
            }

            let tag_list = List::new(tag_list_items).block(Block::default().borders(Borders::ALL));
            app.tag_list_len = tag_list.len();
            frame.render_stateful_widget(tag_list, tag_list_area, &mut app.tag_list_state);
        }
        CurrentScreen::RecipeViewer => {
            // only show tags associated with recipe
            todo!()
        }
        CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
            #[allow(clippy::expect_used)] //TODO: confirm this
            let recipe = &app
                .edit_recipe
                .as_mut()
                .expect("No recipe currently being edited while in edit screen");
            if recipe.name.is_empty() && app.current_screen == CurrentScreen::RecipeCreator {
                let title = Paragraph::new(Text::styled(
                    "New Recipe",
                    Style::default().fg(Color::Green),
                ))
                .block(title_block);
                frame.render_widget(title, title_area);
            } else {
                let title = Paragraph::new(Text::styled(
                    recipe.name.clone(),
                    Style::default().fg(Color::Blue),
                ))
                .block(title_block);
                frame.render_widget(title, title_area);
            }
            match app.editing_state {
                EditingState::Recipe => {
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
                    // subtract 2 for comment/description fields
                    // multiply by 3 for other field total height
                    // add 7 for comment field
                    // add 7 for description field
                    // add 3 for bottom blocks
                    // add 2 for border? //TODO: fix borders
                    //
                    // This should always be a constant number, since FIELD_NAMES_AS_SLICE is
                    // constant
                    #[allow(clippy::arithmetic_side_effects)]
                    if usize::from(recipe_area.height)
                        >= (((Recipe::FIELD_NAMES_AS_SLICE.len() - 2) * 3) + 7 + 7 + 3 + 2)
                    {
                        // recipe_area.height is greater than minimum required
                        for field_name in Recipe::FIELD_NAMES_AS_SLICE {
                            // want to special case some fields
                            match *field_name {
                                //for now, just a bigger area.
                                //TODO: special case this for additional comment functionality
                                "comments" => recipe_edit_constraints.push(Constraint::Min(7)),
                                "description" => {
                                    recipe_edit_constraints.push(Constraint::Min(7));
                                }

                                // need 2 for border and 1 for text.
                                _ => recipe_edit_constraints.push(Constraint::Length(3)),
                            }
                        }
                    } else {
                        //TODO: implement scrolling
                        todo!()
                    }
                    // last constraint for step/equipment block
                    recipe_edit_constraints.push(Constraint::Length(3));
                    let recipe_edit_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(recipe_edit_constraints)
                        .split(recipe_area);

                    // recipe_edit_layout should always have something in it.
                    // This is a valid place to panic
                    #[allow(clippy::expect_used)]
                    let [left_info_block, right_info_block] = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .areas(
                            *recipe_edit_layout
                                .last()
                                .expect("No recipe_edit areas defined"),
                        );

                    let step_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default());

                    let step_count = Paragraph::new(Text::styled(
                        recipe.steps.len().to_string(),
                        Style::default().fg(Color::Green),
                    ))
                    .block(step_block);
                    frame.render_widget(step_count, left_info_block);
                    // render an empty block with borders on the right
                    frame.render_widget(Block::default().borders(Borders::ALL), right_info_block);
                }

                EditingState::Step(step_num) => {
                    // Use split here, since we don't care about naming the fields specifically

                    //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
                    //function if too many fields
                    //
                    //Want
                    //- want fields to be 1 line high plus borders
                    //- need 3 rows for space for blocks at bottom

                    let mut step_edit_constraints = Vec::new();
                    // multiply by 3 for other field total height
                    // add 3 for bottom blocks
                    // add 2 for border? //TODO: fix borders
                    //
                    // This should always be a constant number, since FIELD_NAMES_AS_SLICE is
                    // constant
                    #[allow(clippy::arithmetic_side_effects)]
                    if usize::from(recipe_area.height)
                        >= ((Step::FIELD_NAMES_AS_SLICE.len() * 3) + 3 + 2)
                    {
                        // recipe_area.height is greater than minimum required
                        for _ in Step::FIELD_NAMES_AS_SLICE {
                            // need 2 for border and 1 for text.
                            step_edit_constraints.push(Constraint::Length(3));
                        }
                    } else {
                        //TODO: implement scrolling
                        todo!()
                    }
                    // last constraint for step/equipment block
                    step_edit_constraints.push(Constraint::Length(3));
                    let step_edit_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(step_edit_constraints)
                        .split(recipe_area);

                    // step_edit_layout should always have something in it.
                    // This is a valid place to panic
                    #[allow(clippy::expect_used)]
                    let [left_info_block, right_info_block] = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .areas(
                            *step_edit_layout
                                .last()
                                .expect("No recipe_edit areas defined"),
                        );

                    let equipment_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default());

                    let equipment_count = Paragraph::new(Text::styled(
                        recipe.steps[step_num].equipment.len().to_string(),
                        Style::default().fg(Color::Green),
                    ))
                    .block(equipment_block);
                    frame.render_widget(equipment_count, left_info_block);

                    let ingredient_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default());

                    let ingredient_count = Paragraph::new(Text::styled(
                        recipe.steps[step_num].ingredients.len().to_string(),
                        Style::default().fg(Color::Green),
                    ))
                    .block(ingredient_block);

                    frame.render_widget(ingredient_count, right_info_block);
                }
                EditingState::Ingredient(step_num, _) => {
                    // Use split here, since we don't care about naming the fields specifically

                    //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
                    //function if too many fields
                    //
                    //Want
                    //- fields to be 1 line high plus borders
                    //- need 3 rows for space for blocks at bottom

                    let mut ingredient_edit_constraints = Vec::new();
                    // subtract 1 for description
                    // multiply by 3 for other field total height
                    // add 7 for description
                    // add 3 for bottom blocks
                    // add 2 for border? //TODO: fix borders
                    //
                    // This should always be a constant number, since FIELD_NAMES_AS_SLICE is
                    // constant
                    #[allow(clippy::arithmetic_side_effects)]
                    if usize::from(recipe_area.height)
                        >= (((Ingredient::FIELD_NAMES_AS_SLICE.len() - 1) * 3) + 7 + 3 + 2)
                    {
                        // recipe_area.height is greater than minimum required
                        for field_name in Ingredient::FIELD_NAMES_AS_SLICE {
                            match *field_name {
                                "description" => {
                                    ingredient_edit_constraints.push(Constraint::Min(7));
                                }
                                "unit_quantity" => todo!(),

                                // need 2 for border and 1 for text.
                                _ => ingredient_edit_constraints.push(Constraint::Length(3)),
                            }
                        }
                    } else {
                        //TODO: implement scrolling
                        todo!()
                    }
                    // last constraint for step/equipment block
                    ingredient_edit_constraints.push(Constraint::Length(3));
                    let ingredient_edit_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(ingredient_edit_constraints)
                        .split(recipe_area);

                    // ingredient_edit_layout should always have something in it.
                    // This is a valid place to panic
                    #[allow(clippy::expect_used)]
                    let [left_info_block, right_info_block] = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .areas(
                            *ingredient_edit_layout
                                .last()
                                .expect("No ingredient_edit areas defined"),
                        );

                    let step_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default());

                    let step_id = Paragraph::new(Text::styled(
                        step_num.to_string(),
                        Style::default().fg(Color::Green),
                    ))
                    .block(step_block);
                    frame.render_widget(step_id, left_info_block);
                    // render an empty block with borders on the right
                    frame.render_widget(Block::default().borders(Borders::ALL), right_info_block);
                }
                EditingState::Equipment(step_num, _) => {
                    // Use split here, since we don't care about naming the fields specifically

                    //TODO: fix this ratio calc to not squeeze fields on display. Implement scroll
                    //function if too many fields
                    //
                    //Want
                    //- fields to be 1 line high plus borders
                    //- need 3 rows for space for blocks at bottom

                    let mut equipment_edit_constraints = Vec::new();
                    // subtract 1 for description
                    // multiply by 3 for other field total height
                    // add 7 for description
                    // add 3 for bottom blocks
                    // add 2 for border? //TODO: fix borders
                    //
                    // This should always be a constant number, since FIELD_NAMES_AS_SLICE is
                    // constant
                    #[allow(clippy::arithmetic_side_effects)]
                    if usize::from(recipe_area.height)
                        >= (((Equipment::FIELD_NAMES_AS_SLICE.len() - 1) * 3) + 7 + 3 + 2)
                    {
                        // recipe_area.height is greater than minimum required
                        for field_name in Equipment::FIELD_NAMES_AS_SLICE {
                            match *field_name {
                                "description" => {
                                    equipment_edit_constraints.push(Constraint::Min(7));
                                }

                                // need 2 for border and 1 for text.
                                _ => equipment_edit_constraints.push(Constraint::Length(3)),
                            }
                        }
                    } else {
                        //TODO: implement scrolling
                        todo!()
                    }
                    // last constraint for step/equipment block
                    equipment_edit_constraints.push(Constraint::Length(3));
                    let equipment_edit_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(equipment_edit_constraints)
                        .split(recipe_area);

                    // equipment_edit_layout should always have something in it.
                    // This is a valid place to panic
                    #[allow(clippy::expect_used)]
                    let [left_info_block, right_info_block] = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .areas(
                            *equipment_edit_layout
                                .last()
                                .expect("No equipment_edit areas defined"),
                        );

                    let step_block = Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default());

                    let step_id = Paragraph::new(Text::styled(
                        step_num.to_string(),
                        Style::default().fg(Color::Green),
                    ))
                    .block(step_block);
                    frame.render_widget(step_id, left_info_block);
                    // render an empty block with borders on the right
                    frame.render_widget(Block::default().borders(Borders::ALL), right_info_block);
                }

                EditingState::Idle => {
                    todo!()
                }
                EditingState::SavePrompt => {}
            }
        }
    }

    let current_nav_text = vec![
        // what you are doing, first part of text
        match app.current_screen {
            CurrentScreen::RecipeBrowser => {
                Span::styled("Browsing", Style::default().fg(Color::Green))
            }
            CurrentScreen::RecipeEditor => {
                Span::styled("Editing", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::RecipeViewer => {
                Span::styled("Viewing", Style::default().fg(Color::Blue))
            }
            CurrentScreen::RecipeCreator => {
                Span::styled("Creating", Style::default().fg(Color::Magenta))
            }
        },
        // divider bar
        Span::styled(" | ", Style::default().fg(Color::White)),
        // keyboard shortcuts
        match app.current_screen {
            CurrentScreen::RecipeBrowser => Span::styled(
                "q:quit, n:new, \u{2195}: scroll",
                Style::default().fg(Color::White),
            ),
            CurrentScreen::RecipeViewer => {
                Span::styled("ESC: return to browsing", Style::default().fg(Color::White))
            }
            CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
                let mut keybinds = String::new();
                if app.editing_state == EditingState::Idle {
                    keybinds += "ESC: return to browsing ";
                } else {
                    keybinds += "ESC: exit text editing ";
                }
                keybinds += "TAB: switch focus between recipe parts ";
                // left/right arrows
                keybinds += "\u{2194}: cycle between fields ";
                // up/down arrows
                keybinds += "\u{2195}: cycle between steps/equipment entries";

                Span::styled(keybinds, Style::default().fg(Color::White))
            }
        },
    ];
    let footer =
        Paragraph::new(Line::from(current_nav_text)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, menu_area);
}
