use std::num::{Saturating, Wrapping};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use log::{debug, trace};
use num_traits::FromPrimitive;
use ranged_wrapping::RangedWrapping;

use crate::{
    datatypes::{
        equipment::{Equipment, EquipmentFields},
        ingredient::{Ingredient, IngredientFields},
        recipe::{Recipe, RecipeFields},
        step::{Step, StepFields},
    },
    tui::app::{self, App, CurrentScreen, EditingState},
};

/// `handle_key_event` handles all `KeyEvent`s
///
/// default keybinds are defined in [`default_options`] and modified by the config file.
pub fn handle_key_events(app: &mut App, app_state: &mut app::State, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Release {
        // Skip events that are not KeyEventKind::Press
        return;
    }
    if app
        .keybinds
        .core
        .exit
        .keybinds
        .values()
        .any(|x| x.key == key_event.code && x.modifiers == key_event.modifiers)
    {
        trace! {"key {} pressed with modifiers {}", key_event.code, key_event.modifiers}
        app.exit();
    }
    match app.current_screen {
        //TODO: show/hide tag browser
        CurrentScreen::RecipeBrowser => {
            debug! {"entering CurrentScreen::RecipeBrowser branch of keyhandler"}
            // not using match here, even though it is the much better option, because match can
            // only match on constant values, and not variables for 'some' reason...
            if key_event.code == app.keybinds.browsing.quit.key && key_event.modifiers == app.keybinds.browsing.quit.modifiers {
                trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                app.exit()
            } else if key_event.code == app.keybinds.browsing.new.key
                && key_event.modifiers == app.keybinds.browsing.quit.modifiers
            {
                trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                // create new recipe and start editing it
                debug! {"creating new recipe"}
                app.edit_recipe = Some(Recipe::new());
                //TODO: fix this with proper error handling
                //
                //TODO: confirm changing directly to Recipe editing state works
                debug! {"changing EditingState to Recipe"}
                app_state.editing_state = EditingState::Recipe;
                debug! {"changing CurrentScreen to RecipeCreator"}
                app.current_screen = CurrentScreen::RecipeCreator;
            } else if key_event.code == app.keybinds.browsing.recipe_scroll.keybinds["recipe_scroll_down"].key
                && key_event.modifiers == app.keybinds.browsing.recipe_scroll.keybinds["recipe_scroll_down"].modifiers
            {
                trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                // selected is the integer index of the selected item in the list
                // TODO: change to ranged_wrapping
                if let Some(selected) = app_state.recipe_list_state.selected() {
                    app_state.recipe_list_state.select(Some(
                        ((Wrapping(selected) + Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                    ));
                }
            } else if key_event.code == app.keybinds.browsing.recipe_scroll.keybinds["recipe_scroll_up"].key
                && key_event.modifiers == app.keybinds.browsing.recipe_scroll.keybinds["recipe_scroll_up"].modifiers
            {
                trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                if let Some(selected) = app_state.recipe_list_state.selected() {
                    // not at top of list, so move up
                    app_state.recipe_list_state.select(Some(
                        ((Wrapping(selected) - Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                    ));
                }
            }
        }
        CurrentScreen::RecipeViewer => {
            debug! {"entering CurrentScreen::RecipeViewer branch of keyhandler"}
            // not using match here, even though it is the much better option, because match can
            // only match on constant values, and not variables for 'some' reason...
            if key_event.code == app.keybinds.viewing.exit.key && key_event.modifiers == app.keybinds.viewing.exit.modifiers {
                trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                debug! {"changing CurrentScreen to RecipeBrowser"}
                app.current_screen = CurrentScreen::RecipeBrowser
            }
        }
        // TODO: finish implementing keybinds, want similar to VIM, but maybe hybrid of VIM and
        // nano
        // - Esc exits insert mode and goes to idle mode.
        //      If already in idle mode, exits editing if recipe hasn't been saved yet?
        // - ^C exits app
        // - Tab toggles between editing recipe fields, recipe steps, equipment or ingredients
        // - up/down arrow keys cycle between fields
        // - left/right arrow keys cycle between the individual steps/equipment
        // - e starts editing the recipe
        // -
        // want a separate editing screen for the steps in the recipe
        CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
            debug! {"entering CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor branch of keyhandler"}
            match app_state.editing_state {
                EditingState::Recipe => {
                    debug! {"entering EditingState::Recipe branch of keyhandler"}
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if key_event.code == app.keybinds.editing.exit.key
                        && key_event.modifiers == app.keybinds.editing.exit.modifiers
                    {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if app.edit_recipe.is_some() {
                            if app_state.recipe_state.editing_selected_field.is_some() {
                                debug! {"unselecting field"}
                                app_state.recipe_state.editing_selected_field = None;
                                app_state.recipe_state.editing_field_cursor_position = None;
                            } else {
                                // don't want to prompt to save an empty recipe
                                if app.edit_recipe == Some(Recipe::new()) {
                                    debug! {"changing CurrentScreen to RecipeBrowser"}
                                    app.current_screen = CurrentScreen::RecipeBrowser;
                                } else {
                                    debug! {"saving recipe"}
                                    debug! {"changing to EditingState::SavePrompt"}
                                    app_state.editing_state = EditingState::SavePrompt;
                                }
                            }
                        }
                        //TODO: modify cursor position here
                        //TODO: need to add new keybinds for left/right scroll with arrows
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].modifiers
                    {
                        // only scroll fields if a field is not selected
                        //TODO: modify cursor position here
                        //TODO: need to add new keybinds for left/right scroll with arrows
                        trace!("key {} pressed with modifiers: {}", key_event.code, key_event.modifiers);
                        // editing main recipe part
                        if app_state.recipe_state.editing_selected_field.is_none() {
                            debug! {"Recipe: select previous field"}
                            app_state.recipe_state.selected_field -= 1;
                        }
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].modifiers
                    {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if app_state.recipe_state.editing_selected_field.is_none() {
                            debug! {"Recipe: select next field"}
                            app_state.recipe_state.selected_field += 1;
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].modifiers
                    {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        //toggle between editing recipe, steps, or ingredients
                        if app_state.recipe_state.editing_selected_field.is_none() {
                            if let Some(recipe) = &app.edit_recipe {
                                // are there steps in recipe?
                                if !recipe.steps.is_empty() {
                                    debug! {"Recipe: switch to editing steps in recipe"}
                                    debug! {"how many step fields? {}", Step::NUM_FIELDS}
                                    debug! {"start out editing first step"}
                                    app_state.editing_state = EditingState::Step(Saturating(0));
                                    debug! {"Recipe: selecting first field in step {}", 0};
                                    app_state.step_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                    debug!(
                                        "selected_field_value: {}, min: {}, max: {}",
                                        app_state.step_state.selected_field.value,
                                        app_state.step_state.selected_field.min,
                                        app_state.step_state.selected_field.max
                                    );
                                } else {
                                    //TODO: display an error if there are no steps defined, flash
                                    //screen and display message in message bar
                                    debug! {"Recipe: can't switch to display step, no steps in recipe"}
                                }
                            }
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].modifiers
                    {
                        //TODO: fix this section to reverse the directions
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        //toggle between editing recipe, steps, or ingredients
                        //TODO: this needs to get split up
                        if app_state.recipe_state.editing_selected_field.is_none() {
                            if let Some(recipe) = &app.edit_recipe {
                                // are there steps in recipe?
                                if !recipe.steps.is_empty() {
                                    debug! {"Recipe: switch to editing steps in recipe"}
                                    debug! {"how many step fields? {}", Step::NUM_FIELDS}
                                    debug! {"start out editing first step"}
                                    app_state.editing_state = EditingState::Step(Saturating(0));
                                    debug! {"Recipe: selecting first field in step {}", 0};
                                    app_state.step_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                    debug!(
                                        "selected_field_value: {}, min: {}, max: {}",
                                        app_state.step_state.selected_field.value,
                                        app_state.step_state.selected_field.min,
                                        app_state.step_state.selected_field.max
                                    );
                                } else {
                                    //TODO: display an error if there are no steps defined, flash
                                    //screen and display message in message bar
                                    debug! {"Recipe: can't switch to display step, no steps in recipe"}
                                }
                            }
                        }
                    } else if (app
                        .keybinds
                        .editing
                        .edit
                        .keybinds
                        .values()
                        .any(|x| x.key == key_event.code && x.modifiers == key_event.modifiers))
                        && app_state.recipe_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        debug! {"Recipe: editing selected field {} when i or e pressed", app_state.recipe_state.selected_field}
                        // the use of unwrap should be fine, since the FromPrimitive
                        // is being derived automatically on an enum of
                        // known size
                        app_state.recipe_state.editing_selected_field =
                            match FromPrimitive::from_usize(app_state.recipe_state.selected_field.value).unwrap() {
                                RecipeFields::Name => Some(RecipeFields::Name),
                                RecipeFields::Description => Some(RecipeFields::Description),
                                RecipeFields::Comments => Some(RecipeFields::Comments),
                                RecipeFields::Source => Some(RecipeFields::Source),
                                RecipeFields::Author => Some(RecipeFields::Author),
                                RecipeFields::AmountMade => Some(RecipeFields::AmountMade),
                            }
                    } else if app.keybinds.editing.new_step.key == key_event.code
                        && app.keybinds.editing.new_step.modifiers == key_event.modifiers
                        && app_state.recipe_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        debug! {"Recipe: insert new step into recipe when s is pressed"}
                        app.edit_recipe.as_mut().unwrap().steps.push(Step::default());
                        // do not change to display newly inserted step as multiple
                        // steps may be inserted at once.
                    } else if key_event.code == app.keybinds.editing.move_cursor.keybinds["move_cursor_left"].key
                        && key_event.modifiers == app.keybinds.editing.move_cursor.keybinds["move_cursor_left"].modifiers
                        && app_state.recipe_state.editing_selected_field.is_some()
                    {
                        if let Some(ref mut temp) = app_state.recipe_state.editing_field_cursor_position {
                            *temp -= 1;
                        }
                    } else if key_event.code == app.keybinds.editing.move_cursor.keybinds["move_cursor_right"].key
                        && key_event.modifiers == app.keybinds.editing.move_cursor.keybinds["move_cursor_right"].modifiers
                        && app_state.recipe_state.editing_selected_field.is_some()
                    {
                        if let Some(ref mut temp) = app_state.recipe_state.editing_field_cursor_position {
                            *temp += 1;
                        }
                    } else if key_event.code == app.keybinds.editing.back_delete.key
                        && key_event.modifiers == app.keybinds.editing.back_delete.modifiers
                        && app_state.recipe_state.editing_selected_field.is_some()
                    {
                        #[expect(clippy::unwrap_used)] // already checking for is_some above
                        match app_state.recipe_state.editing_selected_field {
                            Some(RecipeFields::Name) => _ = app.edit_recipe.as_mut().unwrap().name.pop(),
                            Some(RecipeFields::Description) => {
                                _ = app
                                    .edit_recipe
                                    .as_mut()
                                    .unwrap()
                                    .description
                                    .get_or_insert(String::new())
                                    .pop()
                            }
                            Some(RecipeFields::Comments) => {
                                _ = app.edit_recipe.as_mut().unwrap().comments.get_or_insert(String::new()).pop()
                            }
                            Some(RecipeFields::Source) => _ = app.edit_recipe.as_mut().unwrap().source.pop(),
                            Some(RecipeFields::Author) => _ = app.edit_recipe.as_mut().unwrap().author.pop(),
                            Some(RecipeFields::AmountMade) => {
                                todo!()
                            }
                            _ => {}
                        };
                    } else if key_event.code == app.keybinds.editing.front_delete.key
                        && key_event.modifiers == app.keybinds.editing.front_delete.modifiers
                        && app_state.recipe_state.editing_selected_field.is_some()
                    {
                        todo!()
                    }
                    // handling text entry into fields and deletion here with else
                    else {
                        //TODO: monitor cursor position
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if let KeyCode::Char(chr) = key_event.code {
                            if app.edit_recipe.is_some() {
                                debug! {"editing selected field in recipe: {:?}", app_state.recipe_state.editing_selected_field}
                                #[expect(clippy::unwrap_used)] // already checking for is_some above
                                match app_state.recipe_state.editing_selected_field {
                                    //TODO: need to increment/decrement position of cursor here as
                                    //well
                                    Some(RecipeFields::Name) => app.edit_recipe.as_mut().unwrap().name.push(chr),
                                    Some(RecipeFields::Description) => app
                                        .edit_recipe
                                        .as_mut()
                                        .unwrap()
                                        .description
                                        .get_or_insert(String::new())
                                        .push(chr),
                                    Some(RecipeFields::Comments) => app
                                        .edit_recipe
                                        .as_mut()
                                        .unwrap()
                                        .comments
                                        .get_or_insert(String::new())
                                        .push(chr),
                                    Some(RecipeFields::Source) => app.edit_recipe.as_mut().unwrap().source.push(chr),
                                    Some(RecipeFields::Author) => app.edit_recipe.as_mut().unwrap().author.push(chr),
                                    Some(RecipeFields::AmountMade) => {
                                        todo!("AmountMade editing not implemented yet")
                                    }
                                    _ => {}
                                };
                            }
                        //delete key, etc here
                        } else if key_event.code == app.keybinds.editing.confirm.key
                            && key_event.modifiers == app.keybinds.editing.confirm.modifiers
                        {
                            todo!()
                        }
                    }
                }
                // scroll here
                EditingState::Step(step) => {
                    debug! {"entering EditingState::Step branch of keyhandler"}
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if key_event.code == app.keybinds.editing.exit.key
                        && key_event.modifiers == app.keybinds.editing.exit.modifiers
                    {
                        match app_state.step_state.editing_selected_field {
                            Some(StepFields::StepType) if app_state.step_state.dropdown_state.expanded => {
                                debug! {"Step: field closing dropdown"}
                                app_state.step_state.dropdown_state.expanded = false;
                            }
                            Some(StepFields::StepType) if !app_state.step_state.dropdown_state.expanded => {
                                debug! {"Step: not editing selected field"}
                                app_state.step_state.editing_selected_field = None;
                            }
                            None => {
                                //TODO: rethink this. Should enforce the use of arrows to navigate
                                //between step/recipe/ingredient/equipment
                                debug! {"changing EditingState to Recipe from Step"}
                                app_state.editing_state = EditingState::Recipe;
                            }

                            _ if app_state.step_state.editing_selected_field.is_some() => {
                                debug! {"Step: not editing selected field"}
                                app_state.step_state.editing_selected_field = None;
                            }
                            _ => {}
                        }
                        //TODO: modify cursor position here
                        //TODO: need to add new keybinds for left/right scroll with arrows
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].modifiers
                    {
                        // only scroll fields if a field is not selected
                        if app_state.step_state.editing_selected_field.is_none() {
                            debug! {"Step: select previous field"}
                            app_state.step_state.selected_field -= 1
                        } else if app_state.step_state.editing_selected_field.is_some()
                            && app_state.step_state.dropdown_state.expanded
                        {
                            debug! {"Step: scroll up in dropdown"}
                            app_state.step_state.dropdown_state.selected_entry -= 1
                        }
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].modifiers
                    {
                        if app_state.step_state.editing_selected_field.is_none() {
                            debug! {"Step : select next field"}
                            app_state.step_state.selected_field += 1
                        } else if app_state.step_state.editing_selected_field.is_some()
                            && app_state.step_state.dropdown_state.expanded
                        {
                            debug! {"Step : scrooll down in dropdown"}
                            app_state.step_state.dropdown_state.selected_entry += 1
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].modifiers
                    {
                        if app_state.step_state.editing_selected_field.is_none() {
                            debug! {"Step: select previous step"}
                            app_state.step_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Step::NUM_FIELDS,
                            };
                            let selected_step = (step - Saturating(1)) % Saturating(Step::NUM_FIELDS);
                            app_state.editing_state = EditingState::Step(selected_step);
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].modifiers
                    {
                        if app_state.step_state.editing_selected_field.is_none() {
                            debug! {"Step : select next step"}
                            app_state.step_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Step::NUM_FIELDS,
                            };
                            let mut selected_step = (step + Saturating(1)) % Saturating(Step::NUM_FIELDS);
                            if selected_step > Saturating(Step::NUM_FIELDS) {
                                selected_step = Saturating(Step::NUM_FIELDS);
                            }
                            app_state.editing_state = EditingState::Step(selected_step);
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].modifiers
                    {
                        if app_state.step_state.editing_selected_field.is_none() {
                            //TODO: check if step is even an index of the vector
                            if let Some(recipe) = &app.edit_recipe {
                                // are there ingredients in step?
                                if !recipe.steps.is_empty() && !recipe.steps[step.0].ingredients.is_empty() {
                                    debug! {"Step: switch to editing ingredients in step"}
                                    app_state.editing_state = EditingState::Ingredient(step, Saturating(0));
                                    app_state.ingredient_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                } else if !recipe.steps.is_empty()
                                    && !recipe.steps[step.0].equipment.is_empty()
                                    && recipe.steps[step.0].ingredients.is_empty()
                                {
                                    debug! {"Step: switch to editing equipment in step. Skip editing ingredients as they don't exist."}
                                    app_state.editing_state = EditingState::Equipment(step, Saturating(0));
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                } else {
                                    //already in step, but ingredient and equipemnt are None
                                    // no ingredients or equipment defined in step, wrap back around to Recipe
                                    debug! {"Step: wrapping back around to Recipe if no ingredients or equipment are defined"}
                                    app_state.editing_state = EditingState::Recipe;
                                    app_state.recipe_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Recipe::NUM_FIELDS,
                                    };
                                }
                            }
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].modifiers
                    {
                        if app_state.step_state.editing_selected_field.is_none() {
                            //TODO: fix this section to reverse the directions
                            //TODO: check if step is even an index of the vector
                            if let Some(recipe) = &app.edit_recipe {
                                // are there ingredients in step?
                                if !recipe.steps.is_empty() && !recipe.steps[step.0].ingredients.is_empty() {
                                    debug! {"Step: switch to editing ingredients in step"}
                                    app_state.editing_state = EditingState::Ingredient(step, Saturating(0));
                                    app_state.ingredient_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                } else if !recipe.steps.is_empty()
                                    && !recipe.steps[step.0].equipment.is_empty()
                                    && recipe.steps[step.0].ingredients.is_empty()
                                {
                                    debug! {"Step: switch to editing equipment in step. Skip editing ingredients as they don't exist."}
                                    app_state.editing_state = EditingState::Equipment(step, Saturating(0));
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                } else {
                                    //already in step, but ingredient and equipemnt are None
                                    // no ingredients or equipment defined in step, wrap back around to Recipe
                                    debug! {"Step: wrapping back around to Recipe if no ingredients or equipment are defined"}
                                    app_state.editing_state = EditingState::Recipe;
                                    app_state.recipe_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Recipe::NUM_FIELDS,
                                    };
                                }
                            }
                        }
                    } else if app
                        .keybinds
                        .editing
                        .edit
                        .keybinds
                        .values()
                        .any(|x| x.key == key_event.code && x.modifiers == key_event.modifiers)
                        && app_state.step_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        // the use of unwrap should be fine, since the FromPrimitive
                        // is being derived automatically on an enum of
                        // known size
                        debug! {"Step: editing selected field when i or e is pressed"}
                        app_state.step_state.editing_selected_field =
                            match FromPrimitive::from_usize(app_state.step_state.selected_field.value).unwrap() {
                                StepFields::TimeNeeded => Some(StepFields::TimeNeeded),
                                StepFields::Temperature => Some(StepFields::Temperature),
                                StepFields::Instructions => Some(StepFields::Instructions),
                                StepFields::StepType => Some(StepFields::StepType),
                            }
                    } else if app.keybinds.editing.new_ingredient.key == key_event.code
                        && app.keybinds.editing.new_ingredient.modifiers == key_event.modifiers
                        && app_state.step_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        debug! {"Step: insert new inGredient into step when g is pressed"}
                        app.edit_recipe.as_mut().unwrap().steps[step.0]
                            .ingredients
                            .push(Ingredient::default());
                        // do not change to display newly inserted ingredient as
                        // multiple ingredients may be inserted at once
                    } else if app.keybinds.editing.new_equipment.key == key_event.code
                        && app.keybinds.editing.new_equipment.modifiers == key_event.modifiers
                        && app_state.step_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        debug! {"Step: insert new eQuipment into step when q is pressed"}
                        app.edit_recipe.as_mut().unwrap().steps[step.0]
                            .equipment
                            .push(Equipment::default());
                        // do not change to display newly inserted equipment as
                        // multiple pieces of equipment may be inserted at once
                    }
                    // handling text entry into fields and deletion here with else
                    else {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if let KeyCode::Char(chr) = key_event.code {
                            if app.edit_recipe.is_some() {
                                #[allow(clippy::unwrap_used)] // already checking for is_some above
                                match app_state.step_state.editing_selected_field {
                                    //TODO: need to create temp strings then parse numbers from them.
                                    //Also step type
                                    Some(StepFields::TimeNeeded) => {} //TODO: app.edit_recipe.as_mut().steps[step].time_needed,
                                    Some(StepFields::Temperature) => {} //TODO:
                                    //app.edit_recipe.as_mut().steps,
                                    Some(StepFields::Instructions) => {
                                        app.edit_recipe.as_mut().unwrap().steps[step.0].instructions.push(chr)
                                    }
                                    Some(StepFields::StepType) => {
                                        // StepType doesn't have any interactions with other key codes
                                        // currently
                                    }
                                    _ => {}
                                }
                            }
                        }
                        // delete key, etc here
                        else if key_event.code == app.keybinds.editing.back_delete.key
                            && key_event.modifiers == app.keybinds.editing.back_delete.modifiers
                        {
                            #[allow(clippy::unwrap_used)] // already checking for is_some above
                            match app_state.step_state.editing_selected_field {
                                //TODO: need to create temp strings then parse numbers from them.
                                //Also step type
                                Some(StepFields::TimeNeeded) => {} //TODO: app.edit_recipe.as_mut().steps[step].time_needed,
                                Some(StepFields::Temperature) => {} //TODO:
                                //app.edit_recipe.as_mut().steps,
                                Some(StepFields::Instructions) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step.0].instructions.pop()
                                }
                                Some(StepFields::StepType) => {} //TODO,
                                _ => {}
                            }
                        } else if key_event.code == app.keybinds.editing.front_delete.key
                            && key_event.modifiers == app.keybinds.editing.front_delete.modifiers
                        {
                            //TODO
                        } else if key_event.code == app.keybinds.editing.confirm.key
                            && key_event.modifiers == app.keybinds.editing.confirm.modifiers
                        {
                            #[expect(clippy::single_match)]
                            match app_state.step_state.editing_selected_field {
                                Some(StepFields::StepType) => {
                                    debug! {"Step: expand dropdown"}
                                    app_state.step_state.dropdown_state.expanded = true
                                }
                                _ => {}
                            }
                        }
                    }
                }
                EditingState::Ingredient(step, ingredient) => {
                    debug! {"entering EditingState::Ingredient branch of keyhandler"}
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if key_event.code == app.keybinds.editing.exit.key
                        && key_event.modifiers == app.keybinds.editing.exit.modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_some() {
                            debug! {"Ingredient: not editing selected field"}
                            app_state.ingredient_state.editing_selected_field = None;
                        } else {
                            //TODO: rethink this. Should enforce the use of arrows to navigate
                            //between step/recipe/ingredient/equipment
                            debug! {"changing EditingState to Recipe from Ingredient"}
                            app_state.editing_state = EditingState::Recipe;
                        }
                        //TODO: modify cursor position here
                        //TODO: need to add new keybinds for left/right scroll with arrows
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            debug! {"Ingredient: select previous field"}
                            app_state.ingredient_state.selected_field -= 1
                        }
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            debug! {"Ingredient: select next field"}
                            app_state.ingredient_state.selected_field += 1
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            debug! {"Ingredient: select previous ingredient"}
                            app_state.ingredient_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Ingredient::NUM_FIELDS,
                            };
                            let selected_ingredient = (ingredient - Saturating(1)) % Saturating(Ingredient::NUM_FIELDS);
                            app_state.editing_state = EditingState::Ingredient(step, selected_ingredient);
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            debug! {"Ingredient: select next ingredient"}
                            app_state.ingredient_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Ingredient::NUM_FIELDS,
                            };
                            let mut selected_ingredient = (ingredient + Saturating(1)) % Saturating(Ingredient::NUM_FIELDS);
                            if selected_ingredient > Saturating(Ingredient::NUM_FIELDS) {
                                selected_ingredient = Saturating(Ingredient::NUM_FIELDS);
                            }
                            app_state.editing_state = EditingState::Ingredient(step, selected_ingredient);
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            //TODO: check if step is even an index of the vector
                            if let Some(recipe) = &app.edit_recipe {
                                // are there equipment in step
                                if !recipe.steps.is_empty() && !recipe.steps[step.0].equipment.is_empty() {
                                    debug! {"Ingredient: switch to editing equipment in step"}
                                    app_state.editing_state = EditingState::Equipment(step, Saturating(0));
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                } else {
                                    //already in ingredient, but equipment is None
                                    //no equipment defined in step, wrap back around to Recipe
                                    debug! {"Ingredient: wrapping back around to Recipe if no equipment are defined"}
                                    app_state.editing_state = EditingState::Recipe;
                                    app_state.recipe_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Recipe::NUM_FIELDS,
                                    };
                                }
                            }
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].modifiers
                    {
                        if app_state.ingredient_state.editing_selected_field.is_none() {
                            //TODO: fix this section to reverse the direction
                            //TODO: check if step is even an index of the vector
                            if let Some(recipe) = &app.edit_recipe {
                                // are there equipment in step
                                if !recipe.steps.is_empty() && !recipe.steps[step.0].equipment.is_empty() {
                                    debug! {"Ingredient: switch to editing equipment in step"}
                                    app_state.editing_state = EditingState::Equipment(step, Saturating(0));
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                } else {
                                    //already in ingredient, but equipment is None
                                    //no equipment defined in step, wrap back around to Recipe
                                    debug! {"Ingredient: wrapping back around to Recipe if no equipment are defined"}
                                    app_state.editing_state = EditingState::Recipe;
                                    app_state.recipe_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Recipe::NUM_FIELDS,
                                    };
                                }
                            }
                        }
                    } else if app
                        .keybinds
                        .editing
                        .edit
                        .keybinds
                        .values()
                        .any(|x| x.key == key_event.code && x.modifiers == key_event.modifiers)
                        && app_state.ingredient_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        // the use of unwrap should be fine, since the FromPrimitive
                        // is being derived automatically on an enum of
                        // known size
                        debug! {"Ingredient: editing selected field when i or e is pressed"}
                        app_state.ingredient_state.editing_selected_field =
                            match FromPrimitive::from_usize(app_state.ingredient_state.selected_field.value).unwrap() {
                                IngredientFields::Name => Some(IngredientFields::Name),
                                IngredientFields::Description => Some(IngredientFields::Description),
                            }
                    }
                    // handling text entry into fields and deletion here with else
                    else {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if let KeyCode::Char(chr) = key_event.code {
                            if app.edit_recipe.is_some() {
                                // the use of unwrap should be fine, since the FromPrimitive
                                // is being derived automatically on an enum of
                                // known size
                                match app_state.ingredient_state.editing_selected_field {
                                    Some(IngredientFields::Name) => app.edit_recipe.as_mut().unwrap().steps[step.0].ingredients
                                        [ingredient.0]
                                        .name
                                        .push(chr),
                                    Some(IngredientFields::Description) => app.edit_recipe.as_mut().unwrap().steps[step.0]
                                        .ingredients[ingredient.0]
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .push(chr),
                                    _ => {}
                                }
                            }
                        }
                        // delete key, etc here
                        else if key_event.code == app.keybinds.editing.back_delete.key
                            && key_event.modifiers == app.keybinds.editing.back_delete.modifiers
                        {
                            // the use of unwrap should be fine, since the FromPrimitive
                            // is being derived automatically on an enum of
                            // known size
                            match app_state.ingredient_state.editing_selected_field {
                                Some(IngredientFields::Name) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step.0].ingredients[ingredient.0]
                                        .name
                                        .pop()
                                }
                                Some(IngredientFields::Description) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step.0].ingredients[ingredient.0]
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .pop()
                                }
                                _ => {}
                            }
                        } else if key_event.code == app.keybinds.editing.front_delete.key
                            && key_event.modifiers == app.keybinds.editing.front_delete.modifiers
                        {
                            todo!()
                        } else if key_event.code == app.keybinds.editing.confirm.key
                            && key_event.modifiers == app.keybinds.editing.confirm.modifiers
                        {
                            todo!()
                        }
                    }
                }
                EditingState::Equipment(step, equipment) => {
                    debug! {"entering EditingState::Equipment branch of keyhandler"}
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if key_event.code == app.keybinds.editing.exit.key
                        && key_event.modifiers == app.keybinds.editing.exit.modifiers
                    {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}

                        if app_state.equipment_state.editing_selected_field.is_some() {
                            debug! {"Equipment: not editing selected field"}
                            app_state.equipment_state.editing_selected_field = None;
                        } else {
                            //TODO: rethink this. Should enforce the use of arrows to navigate
                            //between step/recipe/ingredient/equipment
                            debug! {"changing EditingState to Recipe from Equipment"}
                            app_state.editing_state = EditingState::Recipe;
                        }
                        //TODO: modify cursor position here
                        //TODO: need to add new keybinds for left/right scroll with arrows
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_up"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            debug! {"Equipment: select previous field"}
                            app_state.equipment_state.selected_field -= 1
                        }
                    } else if key_event.code == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.field_scroll.keybinds["field_scroll_down"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            debug! {"Equipment: select next field"}
                            app_state.equipment_state.selected_field += 1
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_up"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            debug! {"Equipment: select previous equipment"}
                            app_state.equipment_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Equipment::NUM_FIELDS,
                            };
                            let selected_equipment = (equipment - Saturating(1)) & Saturating(Equipment::NUM_FIELDS);
                            app_state.editing_state = EditingState::Equipment(step, selected_equipment);
                        }
                    } else if key_event.code == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].key
                        && key_event.modifiers == app.keybinds.editing.item_scroll.keybinds["item_scroll_down"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            debug! {"Equipment: select next equipment"}
                            app_state.equipment_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Equipment::NUM_FIELDS,
                            };
                            let mut selected_equipment = (equipment + Saturating(1)) % Saturating(Equipment::NUM_FIELDS);
                            if selected_equipment > Saturating(Equipment::NUM_FIELDS) {
                                selected_equipment = Saturating(Equipment::NUM_FIELDS);
                            }
                            app_state.editing_state = EditingState::Equipment(step, selected_equipment);
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_forward"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            debug! {"Equipment: wrapping back around to Recipe"}
                            app_state.editing_state = EditingState::Recipe;
                            app_state.recipe_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Recipe::NUM_FIELDS,
                            };
                        }
                    } else if key_event.code == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].key
                        && key_event.modifiers == app.keybinds.editing.item_switch.keybinds["item_switch_reverse"].modifiers
                    {
                        if app_state.equipment_state.editing_selected_field.is_none() {
                            //TODO: fix this section to reverse the direction
                            debug! {"Equipment: wrapping back around to Recipe"}
                            app_state.editing_state = EditingState::Recipe;
                            app_state.recipe_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Recipe::NUM_FIELDS,
                            };
                        }
                    } else if app
                        .keybinds
                        .editing
                        .edit
                        .keybinds
                        .values()
                        .any(|x| x.key == key_event.code && x.modifiers == key_event.modifiers)
                        && app_state.equipment_state.editing_selected_field.is_none()
                    // need the last part of the logic chain here, rather than nested so it
                    // short circuits and goes to the `else` at the bottom
                    {
                        // the use of unwrap should be fine, since the FromPrimitive
                        // is being derived automatically on an enum of
                        // known size
                        debug! {"Equipment: editing selected field when i or e is pressed"}
                        app_state.equipment_state.editing_selected_field =
                            match FromPrimitive::from_usize(app_state.equipment_state.selected_field.value).unwrap() {
                                EquipmentFields::Name => Some(EquipmentFields::Name),
                                EquipmentFields::Description => Some(EquipmentFields::Description),
                                EquipmentFields::IsOwned => Some(EquipmentFields::IsOwned),
                            }
                    }
                    // handling text entry into fields and deletion here with else
                    else {
                        trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                        if let KeyCode::Char(chr) = key_event.code {
                            if app.edit_recipe.is_some() {
                                // the use of unwrap should be fine, since the FromPrimitive
                                // is being derived automatically on an enum of
                                // known size
                                match app_state.equipment_state.editing_selected_field {
                                    Some(EquipmentFields::Name) => app.edit_recipe.as_mut().unwrap().steps[step.0].equipment
                                        [equipment.0]
                                        .name
                                        .push(chr),
                                    Some(EquipmentFields::Description) => app.edit_recipe.as_mut().unwrap().steps[step.0]
                                        .equipment[equipment.0]
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .push(chr),
                                    Some(EquipmentFields::IsOwned) => {} //TODO:
                                    _ => {}
                                }
                            }
                        }
                        // delete key, etc here
                        else if key_event.code == app.keybinds.editing.back_delete.key
                            && key_event.modifiers == app.keybinds.editing.back_delete.modifiers
                        {
                            // the use of unwrap should be fine, since the FromPrimitive
                            // is being derived automatically on an enum of
                            // known size
                            match app_state.equipment_state.editing_selected_field {
                                Some(EquipmentFields::Name) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step.0].equipment[equipment.0]
                                        .name
                                        .pop()
                                }
                                Some(EquipmentFields::Description) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step.0].equipment[equipment.0]
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .pop()
                                }
                                Some(EquipmentFields::IsOwned) => {} //TODO:
                                _ => {}
                            }
                        } else if key_event.code == app.keybinds.editing.front_delete.key
                            && key_event.modifiers == app.keybinds.editing.front_delete.modifiers
                        {
                            todo!()
                        } else if key_event.code == app.keybinds.editing.confirm.key
                            && key_event.modifiers == app.keybinds.editing.confirm.modifiers
                        {
                            todo!()
                        }
                    }
                }
                EditingState::SavePrompt => {
                    debug! {"entering EditingState::SavePrompt branch of keyhandler"}
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}

                    if key_event.code == app.keybinds.editing.prompt_scroll.keybinds["prompt_scroll_left"].key
                        && key_event.modifiers == app.keybinds.editing.prompt_scroll.keybinds["prompt_scroll_left"].modifiers
                    {
                        app_state.save_prompt_state.select_previous();
                    } else if key_event.code == app.keybinds.editing.prompt_scroll.keybinds["prompt_scroll_right"].key
                        && key_event.modifiers == app.keybinds.editing.prompt_scroll.keybinds["prompt_scroll_right"].modifiers
                    {
                        app_state.save_prompt_state.select_next();
                    } else if key_event.code == app.keybinds.editing.confirm.key
                        && key_event.modifiers == app.keybinds.editing.confirm.modifiers
                    {
                        match app_state.save_prompt_state.value() {
                            // These indexes are in the order they are inserted during the
                            // creation of save_prompt in app.rs
                            // Yes
                            0 => {
                                debug! {"SavePrompt: Save = Yes"}
                                app.recipes.sort_unstable_by_key(|k| k.id);
                                if app.edit_recipe.is_some() {
                                    match app
                                        .recipes
                                        .binary_search_by_key(&app.edit_recipe.as_ref().unwrap().id, |k| k.id)
                                    {
                                        Ok(index) => {
                                            app.recipes[index] = app.edit_recipe.clone().unwrap();
                                            app.edit_recipe = None;
                                        }
                                        Err(index) => {
                                            app.recipes.insert(index, app.edit_recipe.clone().unwrap());
                                            app.edit_recipe = None;
                                        }
                                    }
                                }
                            }
                            // No
                            1 => {
                                debug! {"SavePrompt: Save = No"}
                                app.edit_recipe = None;
                            }
                            // Cancel
                            2 => {
                                debug! {"SavePrompt: Save = Cancel"}
                                app_state.editing_state = EditingState::Recipe
                            }
                            //Not used
                            _ => {}
                        }
                        app.current_screen = CurrentScreen::RecipeBrowser;
                    }
                }
            }
        }
    }
}
