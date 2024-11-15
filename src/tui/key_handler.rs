use std::num::{Saturating, Wrapping};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
    tui::app::{App, AppState, CurrentScreen, EditingState, SaveResponse},
};

//TODO: switch to using defined keybinds
/// `handle_key_event` handles all `KeyEvent`s
///
/// default keybinds are defined in [`default_options`] and modified by the config file.
pub fn handle_key_events(app: &mut App, app_state: &mut AppState, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Release {
        // Skip events that are not KeyEventKind::Press
        return;
    }
    match app.current_screen {
        //TODO: show/hide tag browser
        CurrentScreen::RecipeBrowser => {
            debug! {"entering CurrentScreen::RecipeBrowser branch of keyhandler"}
            match key_event.code {
                KeyCode::Char('q') => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    app.exit()
                }
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    trace! {"key {} pressed with modifiers {}", key_event.code, key_event.modifiers}
                    app.exit();
                }
                KeyCode::Char('n') => {
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
                }
                //https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
                KeyCode::Down => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    // selected is the integer index of the selected item in the list
                    // TODO: change to ranged_wrapping
                    if let Some(selected) = app_state.recipe_list_state.selected() {
                        app_state.recipe_list_state.select(Some(
                            ((Wrapping(selected) + Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                        ));
                    }
                }
                KeyCode::Up => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if let Some(selected) = app_state.recipe_list_state.selected() {
                        // not at top of list, so move up
                        app_state.recipe_list_state.select(Some(
                            ((Wrapping(selected) - Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                        ));
                    }
                }
                _ => {}
            }
        }
        CurrentScreen::RecipeViewer => {
            debug! {"entering CurrentScreen::RecipeViewer branch of keyhandler"}
            match key_event.code {
                KeyCode::Esc => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    debug! {"changing CurrentScreen to RecipeBrowser"}
                    app.current_screen = CurrentScreen::RecipeBrowser
                }
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    trace! {"key {} pressed with modifiers {}", key_event.code, key_event.modifiers}
                    app.exit();
                }
                _ => {}
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
            match key_event.code {
                // want to be able to stop editing but still be in creation/editing view
                KeyCode::Esc => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    match app_state.editing_state {
                        EditingState::Idle => {
                            debug! {"changing CurrentScreen to RecipeBrowser"}
                            app.current_screen = CurrentScreen::RecipeBrowser;
                        }
                        EditingState::Recipe => {
                            if app.edit_recipe.is_some() {
                                if app_state.recipe_state.editing_selected_field.is_some() {
                                    debug! {"unselecting field"}
                                    app_state.recipe_state.editing_selected_field = None;
                                } else {
                                    // don't want to prompt to save an empty recipe
                                    if app.edit_recipe == Some(Recipe::new()) {
                                        debug! {"unsaved recipe is blank, not prompting to save"}
                                        app_state.editing_state = EditingState::Idle;
                                        debug! {"changing CurrentScreen to RecipeBrowser"}
                                        app.current_screen = CurrentScreen::RecipeBrowser;
                                    } else {
                                        debug! {"saving recipe"}
                                        app.recipes.sort_unstable_by_key(|r| r.id);
                                        match app
                                            .recipes
                                            .binary_search_by_key(&app.edit_recipe.as_ref().unwrap().id, |r| r.id)
                                        {
                                            Ok(index) => {
                                                // editing_recipe id matches the id of a recipe in recipes
                                                // Prompt to save
                                                debug! {"prompt to save existing edited recipe"}
                                                app_state.editing_state = EditingState::SavePrompt(index, true);
                                            }
                                            Err(index) => {
                                                // editing_recipe id not found in recipes
                                                // Prompt to save
                                                debug! {"prompt to save new edited recipe"}
                                                app_state.editing_state = EditingState::SavePrompt(index, false);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        EditingState::Step(_) => match app_state.step_state.editing_selected_field {
                            Some(StepFields::StepType) if app_state.step_state.dropdown_state.expanded => {
                                debug! {"Step: field closing dropdown"}
                                app_state.step_state.dropdown_state.expanded = false;
                            }
                            Some(StepFields::StepType) if !app_state.step_state.dropdown_state.expanded => {
                                debug! {"Step: not editing selected field"}
                                app_state.step_state.editing_selected_field = None;
                            }
                            None => {
                                debug! {"changing EditingState to Recipe from Step"}
                                app_state.editing_state = EditingState::Recipe;
                            }

                            _ if app_state.step_state.editing_selected_field.is_some() => {
                                debug! {"Step: not editing selected field"}
                                app_state.step_state.editing_selected_field = None;
                            }
                            _ => {}
                        },
                        EditingState::Ingredient(_, _) => {
                            if app_state.ingredient_state.editing_selected_field.is_some() {
                                debug! {"Ingredient: not editing selected field"}
                                app_state.ingredient_state.editing_selected_field = None;
                            } else {
                                debug! {"changing EditingState to Recipe from Ingredient"}
                                app_state.editing_state = EditingState::Recipe;
                            }
                        }
                        EditingState::Equipment(_, _) => {
                            if app_state.equipment_state.editing_selected_field.is_some() {
                                debug! {"Equipment: not editing selected field"}
                                app_state.equipment_state.editing_selected_field = None;
                            } else {
                                debug! {"changing EditingState to Recipe from Equipment"}
                                app_state.editing_state = EditingState::Recipe;
                            }
                        }
                        EditingState::SavePrompt(_, _) => {}
                    }
                }
                // only scroll fields if a field is not selected
                KeyCode::Up => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    match app_state.editing_state {
                        // editing main recipe part
                        EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
                            debug! {"Recipe: select previous field"}
                            app_state.recipe_state.selected_field -= RangedWrapping {
                                value: 1,
                                min: 0,
                                max: Recipe::NUM_FIELDS,
                            };
                        }
                        EditingState::Step(step_num) => {
                            match app_state.step_state.editing_selected_field {
                                // select field in current step
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Step: select previous field"}
                                    app_state.step_state.selected_field -= RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                }
                                // change selected step
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Step: select previous step"}
                                    app_state.step_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                    let selected_step = (step_num - Saturating(1)) % Saturating(Step::NUM_FIELDS);
                                    app_state.editing_state = EditingState::Step(selected_step);
                                }
                                // scroll in dropdown
                                Some(StepFields::StepType) if app_state.step_state.dropdown_state.expanded => {
                                    debug! {"Step: scroll up in dropdown"}
                                    app_state.step_state.dropdown_state.selected_entry -= RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                }
                                _ => {}
                            }
                        }
                        EditingState::Ingredient(step_num, ingredient_num) => {
                            match app_state.ingredient_state.editing_selected_field {
                                // select field in current ingredient
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Ingredient: select previous field"}
                                    app_state.ingredient_state.selected_field -= RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                }
                                // change selected ingredient
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Ingredient: select previous ingredient"}
                                    app_state.ingredient_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                    let selected_ingredient =
                                        (ingredient_num - Saturating(1)) % Saturating(Ingredient::NUM_FIELDS);
                                    app_state.editing_state = EditingState::Ingredient(step_num, selected_ingredient);
                                }
                                // do nothing for now
                                // TODO: dropdown stuff here
                                Some(_) => {}
                                _ => {}
                            }
                        }
                        EditingState::Equipment(step_num, equipment_num) => {
                            match app_state.equipment_state.editing_selected_field {
                                // select field in current equipment
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Equipment: select previous field"}
                                    app_state.equipment_state.selected_field -= RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                }
                                // change selected equipment
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Equipment: select previous equipment"}
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                    let selected_equipment = (equipment_num - Saturating(1)) & Saturating(Equipment::NUM_FIELDS);
                                    app_state.editing_state = EditingState::Equipment(step_num, selected_equipment);
                                }
                                // do nothing for now
                                // TODO: dropdown stuff here
                                Some(_) => {}
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Down => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    match app_state.editing_state {
                        EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
                            debug! {"Recipe: select next field"}
                            app_state.recipe_state.selected_field += RangedWrapping {
                                value: 1,
                                min: 0,
                                max: Recipe::NUM_FIELDS,
                            };
                        }
                        EditingState::Step(step_num) => {
                            match app_state.step_state.editing_selected_field {
                                // select field in current step
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Step : select next field"}
                                    debug!(
                                        "selected_field_value: {}, min: {}, max: {}",
                                        app_state.step_state.selected_field.value,
                                        app_state.step_state.selected_field.min,
                                        app_state.step_state.selected_field.max
                                    );
                                    let test = RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                    debug!("selected_field_value: {}, min: {}, max: {}", test.value, test.min, test.max);
                                    app_state.step_state.selected_field += RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                }
                                // change selected step
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Step : select next step"}
                                    app_state.step_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                    let mut selected_step = (step_num + Saturating(1)) % Saturating(Step::NUM_FIELDS);
                                    if selected_step > Saturating(Step::NUM_FIELDS) {
                                        selected_step = Saturating(Step::NUM_FIELDS);
                                    }
                                    app_state.editing_state = EditingState::Step(selected_step);
                                }
                                // scroll in dropdown
                                Some(StepFields::StepType) if app_state.step_state.dropdown_state.expanded => {
                                    debug! {"Step : scrooll down in dropdown"}
                                    app_state.step_state.dropdown_state.selected_entry += RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Step::NUM_FIELDS,
                                    };
                                }
                                _ => {}
                            }
                        }
                        EditingState::Ingredient(step_num, ingredient_num) => {
                            match app_state.ingredient_state.editing_selected_field {
                                // select field in current ingredient
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Ingredient: select next field"}
                                    app_state.ingredient_state.selected_field += RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                }
                                // change selected ingredient
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Ingredient: select next ingredient"}
                                    app_state.ingredient_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Ingredient::NUM_FIELDS,
                                    };
                                    let mut selected_ingredient =
                                        (ingredient_num + Saturating(1)) % Saturating(Ingredient::NUM_FIELDS);
                                    if selected_ingredient > Saturating(Ingredient::NUM_FIELDS) {
                                        selected_ingredient = Saturating(Ingredient::NUM_FIELDS);
                                    }
                                    app_state.editing_state = EditingState::Ingredient(step_num, selected_ingredient);
                                }
                                // do nothing for now
                                // TODO: dropdown stuff here
                                Some(_) => {}
                                _ => {}
                            }
                        }
                        // select field in current equipment
                        EditingState::Equipment(step_num, equipment_num) => {
                            match app_state.equipment_state.editing_selected_field {
                                None if key_event.modifiers == KeyModifiers::NONE => {
                                    debug! {"Equipment: select next field"}
                                    app_state.equipment_state.selected_field += RangedWrapping {
                                        value: 1,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                }
                                // change selected equipment
                                None if key_event.modifiers == KeyModifiers::SHIFT => {
                                    debug! {"Equipment: select next equipment"}
                                    app_state.equipment_state.selected_field = RangedWrapping {
                                        value: 0,
                                        min: 0,
                                        max: Equipment::NUM_FIELDS,
                                    };
                                    let mut selected_equipment =
                                        (equipment_num + Saturating(1)) % Saturating(Equipment::NUM_FIELDS);
                                    if selected_equipment > Saturating(Equipment::NUM_FIELDS) {
                                        selected_equipment = Saturating(Equipment::NUM_FIELDS);
                                    }
                                    app_state.editing_state = EditingState::Equipment(step_num, selected_equipment);
                                }
                                // do nothing for now
                                // TODO: dropdown stuff here
                                Some(_) => {}
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                //TODO: want to be able to scroll through text during text entry, need to make sure
                //backspace and insert character are handled correctly
                #[expect(clippy::single_match)]
                KeyCode::Left => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    match app_state.editing_state {
                        EditingState::SavePrompt(_, _) => match app_state.save_response {
                            SaveResponse::Yes => app_state.save_response = SaveResponse::Cancel,
                            SaveResponse::No => app_state.save_response = SaveResponse::Yes,
                            SaveResponse::Cancel => app_state.save_response = SaveResponse::No,
                        },
                        _ => {}
                    }
                }
                #[expect(clippy::single_match)]
                KeyCode::Right => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    match app_state.editing_state {
                        EditingState::SavePrompt(_, _) => match app_state.save_response {
                            SaveResponse::Yes => app_state.save_response = SaveResponse::No,
                            SaveResponse::No => app_state.save_response = SaveResponse::Cancel,
                            SaveResponse::Cancel => app_state.save_response = SaveResponse::Yes,
                        },
                        _ => {}
                    }
                }
                KeyCode::Tab => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    //toggle between editing recipe, steps, or ingredients
                    match app_state.editing_state {
                        EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
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
                        EditingState::Step(step) if app_state.step_state.editing_selected_field.is_none() => {
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
                        EditingState::Ingredient(step, _) if app_state.ingredient_state.editing_selected_field.is_none() => {
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
                        EditingState::Equipment(_, _) if app_state.equipment_state.editing_selected_field.is_none() => {
                            debug! {"Equipment: wrapping back around to Recipe"}
                            app_state.editing_state = EditingState::Recipe;
                            app_state.recipe_state.selected_field = RangedWrapping {
                                value: 0,
                                min: 0,
                                max: Recipe::NUM_FIELDS,
                            };
                        }
                        EditingState::SavePrompt(_, _) => match app_state.save_response {
                            SaveResponse::Yes => app_state.save_response = SaveResponse::Cancel,
                            SaveResponse::No => app_state.save_response = SaveResponse::Yes,
                            SaveResponse::Cancel => app_state.save_response = SaveResponse::No,
                        },
                        _ => {}
                    }
                }
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    trace! {"key {} pressed with modifier {}", key_event.code, key_event.modifiers}
                    //TODO: prompt for save
                    app.exit();
                }
                //KeyCode
                KeyCode::Char(chr) => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if app.edit_recipe.is_some() {
                        match app_state.editing_state {
                            EditingState::Idle if chr == 'e' || chr == 'i' => {
                                debug! {"Idle: changing to EditingState::Recipe when e or i pressed"}
                                app_state.editing_state = EditingState::Recipe;
                                app_state.recipe_state.selected_field = RangedWrapping {
                                    value: 0,
                                    min: 0,
                                    max: Recipe::NUM_FIELDS,
                                };
                            }
                            EditingState::Recipe => {
                                #[expect(clippy::unwrap_used)] // already checking for is_some above
                                match app_state.recipe_state.editing_selected_field {
                                    Some(RecipeFields::Name) => app.edit_recipe.as_mut().unwrap().name.push(chr),
                                    //TODO: fix comment and description text entry
                                    Some(RecipeFields::Description) => app
                                        .edit_recipe
                                        .as_mut()
                                        .unwrap()
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .push(chr),
                                    Some(RecipeFields::Comments) => app
                                        .edit_recipe
                                        .as_mut()
                                        .unwrap()
                                        .comments
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .push(chr),
                                    Some(RecipeFields::Source) => app.edit_recipe.as_mut().unwrap().source.push(chr),
                                    Some(RecipeFields::Author) => app.edit_recipe.as_mut().unwrap().author.push(chr),
                                    Some(RecipeFields::AmountMade) => {
                                        todo!()
                                    }
                                    // edit selected field
                                    None if chr == 'e' || chr == 'i' => {
                                        debug! {"Recipe: editing selected field {} when i or e pressed", app_state.recipe_state.selected_field}
                                        // the use of unwrap should be fine, since the FromPrimitive
                                        // is being derived automatically on an enum of
                                        // known size
                                        app_state.recipe_state.editing_selected_field =
                                            match FromPrimitive::from_usize(app_state.recipe_state.selected_field.value).unwrap()
                                            {
                                                RecipeFields::Name => Some(RecipeFields::Name),
                                                RecipeFields::Description => Some(RecipeFields::Description),
                                                RecipeFields::Comments => Some(RecipeFields::Comments),
                                                RecipeFields::Source => Some(RecipeFields::Source),
                                                RecipeFields::Author => Some(RecipeFields::Author),
                                                RecipeFields::AmountMade => Some(RecipeFields::AmountMade),
                                            }
                                    }
                                    // insert empty step into recipe
                                    None if chr == 's' => {
                                        debug! {"Recipe: insert new step into recipe when s is pressed"}
                                        app.edit_recipe.as_mut().unwrap().steps.push(Step::default());
                                        // do not change to display newly inserted step as multiple
                                        // steps may be inserted at once.
                                    }
                                    _ => {}
                                };
                            }
                            EditingState::Step(step) => {
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

                                    // editing selected field
                                    None if chr == 'e' || chr == 'i' => {
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
                                    }
                                    // Insert new equipment into step
                                    //q for eQuipment
                                    None if chr == 'q' => {
                                        debug! {"Step: insert new eQuipment into step when q is pressed"}
                                        app.edit_recipe.as_mut().unwrap().steps[step.0]
                                            .equipment
                                            .push(Equipment::default());
                                        // do not change to display newly inserted equipment as
                                        // multiple pieces of equipment may be inserted at once
                                    }
                                    // insert new ingredient into step
                                    //g for inGredient
                                    None if chr == 'g' => {
                                        debug! {"Step: insert new inGredient into step when g is pressed"}
                                        app.edit_recipe.as_mut().unwrap().steps[step.0]
                                            .ingredients
                                            .push(Ingredient::default());
                                        // do not change to display newly inserted ingredient as
                                        // multiple ingredients may be inserted at once
                                    }
                                    _ => {}
                                }
                            }
                            EditingState::Ingredient(step, ingredient) => {
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
                                    None if chr == 'e' || chr == 'i' => {
                                        // the use of unwrap should be fine, since the FromPrimitive
                                        // is being derived automatically on an enum of
                                        // known size
                                        debug! {"Ingredient: editing selected field when i or e is pressed"}
                                        app_state.ingredient_state.editing_selected_field =
                                            match FromPrimitive::from_usize(app_state.ingredient_state.selected_field.value)
                                                .unwrap()
                                            {
                                                IngredientFields::Name => Some(IngredientFields::Name),
                                                IngredientFields::Description => Some(IngredientFields::Description),
                                            }
                                    }
                                    _ => {}
                                }
                            }
                            EditingState::Equipment(step, equip) => {
                                // the use of unwrap should be fine, since the FromPrimitive
                                // is being derived automatically on an enum of
                                // known size
                                match app_state.equipment_state.editing_selected_field {
                                    Some(EquipmentFields::Name) => app.edit_recipe.as_mut().unwrap().steps[step.0].equipment
                                        [equip.0]
                                        .name
                                        .push(chr),
                                    Some(EquipmentFields::Description) => app.edit_recipe.as_mut().unwrap().steps[step.0]
                                        .equipment[equip.0]
                                        .description
                                        .as_mut()
                                        .unwrap_or(&mut String::new())
                                        .push(chr),
                                    Some(EquipmentFields::IsOwned) => {} //TODO:
                                    None if chr == 'e' || chr == 'i' => {
                                        // the use of unwrap should be fine, since the FromPrimitive
                                        // is being derived automatically on an enum of
                                        // known size
                                        debug! {"Equipment: editing selected field when i or e is pressed"}
                                        app_state.equipment_state.editing_selected_field =
                                            match FromPrimitive::from_usize(app_state.equipment_state.selected_field.value)
                                                .unwrap()
                                            {
                                                EquipmentFields::Name => Some(EquipmentFields::Name),
                                                EquipmentFields::Description => Some(EquipmentFields::Description),
                                                EquipmentFields::IsOwned => Some(EquipmentFields::IsOwned),
                                            }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Backspace => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if app.edit_recipe.is_some() {
                        match app_state.editing_state {
                            EditingState::Recipe => {
                                #[allow(clippy::unwrap_used)] // already checking for is_some above
                                match app_state.recipe_state.editing_selected_field {
                                    Some(RecipeFields::Name) => _ = app.edit_recipe.as_mut().unwrap().name.pop(),
                                    //TODO: fix comment and description text entry
                                    Some(RecipeFields::Description) => {
                                        _ = app
                                            .edit_recipe
                                            .as_mut()
                                            .unwrap()
                                            .description
                                            .as_mut()
                                            .unwrap_or(&mut String::new())
                                            .pop()
                                    }
                                    Some(RecipeFields::Comments) => {
                                        _ = app
                                            .edit_recipe
                                            .as_mut()
                                            .unwrap()
                                            .comments
                                            .as_mut()
                                            .unwrap_or(&mut String::new())
                                            .pop()
                                    }
                                    Some(RecipeFields::Source) => _ = app.edit_recipe.as_mut().unwrap().source.pop(),
                                    Some(RecipeFields::Author) => _ = app.edit_recipe.as_mut().unwrap().author.pop(),
                                    Some(RecipeFields::AmountMade) => {
                                        todo!()
                                    }
                                    _ => {}
                                };
                            }
                            EditingState::Step(step) => {
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
                            }
                            EditingState::Ingredient(step, ingredient) => {
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
                            }
                            EditingState::Equipment(step, equip) => {
                                // the use of unwrap should be fine, since the FromPrimitive
                                // is being derived automatically on an enum of
                                // known size
                                match app_state.equipment_state.editing_selected_field {
                                    Some(EquipmentFields::Name) => {
                                        _ = app.edit_recipe.as_mut().unwrap().steps[step.0].equipment[equip.0].name.pop()
                                    }
                                    Some(EquipmentFields::Description) => {
                                        _ = app.edit_recipe.as_mut().unwrap().steps[step.0].equipment[equip.0]
                                            .description
                                            .as_mut()
                                            .unwrap_or(&mut String::new())
                                            .pop()
                                    }
                                    Some(EquipmentFields::IsOwned) => {} //TODO:
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                } //TODO
                KeyCode::Delete => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    //TODO
                }
                KeyCode::Enter => {
                    trace! {"key {} pressed with modifiers: {}", key_event.code, key_event.modifiers}
                    if app.edit_recipe.is_some() {
                        match app_state.editing_state {
                            EditingState::Step(_) =>
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
                            EditingState::SavePrompt(_, _) => {
                                match app_state.save_response {
                                    SaveResponse::Yes => {
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
                                    SaveResponse::No => {
                                        debug! {"SavePrompt: Save = No"}
                                        app.edit_recipe = None;
                                    }
                                    SaveResponse::Cancel => {
                                        debug! {"SavePrompt: Save = Cancel"}
                                        app_state.editing_state = EditingState::Recipe
                                    }
                                }
                                app_state.editing_state = EditingState::Idle;
                                app_state.save_response = SaveResponse::Yes;
                                app.current_screen = CurrentScreen::RecipeBrowser;
                            }
                            _ => {}
                        }
                    }
                } //TODO: complete prompt on saveprompt
                _ => {}
            }
        }
    }
}
