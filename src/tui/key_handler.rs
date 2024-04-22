use crate::{
    datatypes::{
        equipment::{Equipment, EquipmentFields},
        ingredient::{Ingredient, IngredientFields},
        recipe::{Recipe, RecipeFields},
        step::{Step, StepFields},
    },
    tui::app::{App, AppState, CurrentScreen, EditingState, SaveResponse},
};

use std::num::Wrapping;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use num_traits::FromPrimitive;

/// `handle_key_event` handles all `KeyEvent`s
pub fn handle_key_events(app: &mut App, app_state: &mut AppState, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Release {
        // Skip events that are not KeyEventKind::Press
        return;
    }
    match app.current_screen {
        //TODO: show/hide tag browser
        CurrentScreen::RecipeBrowser => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.exit();
                }
            }
            KeyCode::Char('n') => {
                // create new recipe and start editing it
                app.edit_recipe = Some(Recipe::new());
                //TODO: fix this with proper error handling
                app_state.editing_state = EditingState::Idle;
                app.current_screen = CurrentScreen::RecipeCreator;
            }
            //https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
            KeyCode::Down => {
                // selected is the integer index of the selected item in the list
                if let Some(selected) = app_state.recipe_list_state.selected() {
                    app_state.recipe_list_state.select(Some(
                        ((Wrapping(selected) + Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                    ));
                }
            }
            KeyCode::Up => {
                if let Some(selected) = app_state.recipe_list_state.selected() {
                    // not at top of list, so move up
                    app_state.recipe_list_state.select(Some(
                        ((Wrapping(selected) - Wrapping(1_usize)).0) % (app_state.recipe_list_len),
                    ));
                }
            }
            _ => {}
        },
        CurrentScreen::RecipeViewer => match key_event.code {
            KeyCode::Esc => app.current_screen = CurrentScreen::RecipeBrowser,
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    //TODO: prompt for save
                    app.exit();
                }
            }
            _ => {}
        },
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
        CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => match key_event.code {
            // want to be able to stop editing but still be in creation/editing view
            KeyCode::Esc => {
                match app_state.editing_state {
                    EditingState::Idle => {
                        //TODO: prompt for save
                        app.current_screen = CurrentScreen::RecipeBrowser;
                    }
                    EditingState::Recipe => {
                        if app.edit_recipe.is_some() {
                            if app_state.recipe_state.editing_selected_field.is_some() {
                                app_state.recipe_state.editing_selected_field = None;
                            } else {
                                // recipe is empty
                                if app.edit_recipe == Some(Recipe::new()) {
                                    app_state.editing_state = EditingState::Idle;
                                    app.current_screen = CurrentScreen::RecipeBrowser;
                                } else {
                                    app.recipes.sort_unstable_by_key(|r| r.id);
                                    match app
                                        .recipes
                                        .binary_search_by_key(&app.edit_recipe.as_ref().unwrap().id, |r| r.id)
                                    {
                                        Ok(index) => {
                                            // editing_recipe id matches the id of a recipe in recipes
                                            // Prompt to save
                                            app_state.editing_state = EditingState::SavePrompt(index, true);
                                        }
                                        Err(index) => {
                                            // editing_recipe id not found in recipes
                                            // Prompt to save
                                            app_state.editing_state = EditingState::SavePrompt(index, false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    EditingState::Step(_) => {
                        if app_state.step_state.editing_selected_field.is_some() {
                            app_state.step_state.editing_selected_field = None;
                        } else {
                            app_state.editing_state = EditingState::Recipe;
                        }
                    }
                    EditingState::Ingredient(_, _) => {
                        if app_state.ingredient_state.editing_selected_field.is_some() {
                            app_state.ingredient_state.editing_selected_field = None;
                        } else {
                            app_state.editing_state = EditingState::Recipe;
                        }
                    }
                    EditingState::Equipment(_, _) => {
                        if app_state.equipment_state.editing_selected_field.is_some() {
                            app_state.equipment_state.editing_selected_field = None;
                        } else {
                            app_state.editing_state = EditingState::Recipe;
                        }
                    }
                    EditingState::SavePrompt(_, _) => {}
                }
            }
            //TODO: use shift to select different steps/ingredients/equipment
            KeyCode::Up => match app_state.editing_state {
                EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
                    app_state.recipe_state.selected_field -= Wrapping(1);
                    app_state.recipe_state.selected_field %= app_state.recipe_state.num_fields;
                }
                EditingState::Step(_) if app_state.step_state.editing_selected_field.is_none() => {
                    app_state.step_state.selected_field -= Wrapping(1);
                    app_state.step_state.selected_field %= app_state.step_state.num_fields;
                }
                EditingState::Ingredient(_, _) if app_state.ingredient_state.editing_selected_field.is_none() => {
                    app_state.ingredient_state.selected_field -= Wrapping(1);
                    app_state.ingredient_state.selected_field %= app_state.ingredient_state.num_fields;
                }
                EditingState::Equipment(_, _) if app_state.equipment_state.editing_selected_field.is_none() => {
                    app_state.equipment_state.selected_field -= Wrapping(1);
                    app_state.equipment_state.selected_field %= app_state.equipment_state.num_fields;
                }
                _ => {}
            },
            KeyCode::Down => match app_state.editing_state {
                EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
                    app_state.recipe_state.selected_field += Wrapping(1);
                    app_state.recipe_state.selected_field %= app_state.recipe_state.num_fields;
                }
                EditingState::Step(_) if app_state.step_state.editing_selected_field.is_none() => {
                    app_state.step_state.selected_field += Wrapping(1);
                    app_state.step_state.selected_field %= app_state.step_state.num_fields;
                }
                EditingState::Ingredient(_, _) if app_state.ingredient_state.editing_selected_field.is_none() => {
                    app_state.ingredient_state.selected_field += Wrapping(1);
                    app_state.ingredient_state.selected_field %= app_state.ingredient_state.num_fields;
                }
                EditingState::Equipment(_, _) if app_state.equipment_state.editing_selected_field.is_none() => {
                    app_state.equipment_state.selected_field += Wrapping(1);
                    app_state.equipment_state.selected_field %= app_state.equipment_state.num_fields;
                }
                _ => {}
            },
            //TODO: want to be able to scroll through text during text entry, need to make sure
            //backspace and insert character are handled correctly
            #[allow(clippy::single_match)]
            KeyCode::Left => match app_state.editing_state {
                EditingState::SavePrompt(_, _) => match app_state.save_response {
                    SaveResponse::Yes => app_state.save_response = SaveResponse::Cancel,
                    SaveResponse::No => app_state.save_response = SaveResponse::Yes,
                    SaveResponse::Cancel => app_state.save_response = SaveResponse::No,
                },
                _ => {}
            },
            #[allow(clippy::single_match)]
            KeyCode::Right => match app_state.editing_state {
                EditingState::SavePrompt(_, _) => match app_state.save_response {
                    SaveResponse::Yes => app_state.save_response = SaveResponse::No,
                    SaveResponse::No => app_state.save_response = SaveResponse::Cancel,
                    SaveResponse::Cancel => app_state.save_response = SaveResponse::Yes,
                },
                _ => {}
            },
            KeyCode::Tab => {
                //toggle between editing recipe, steps, or ingredients
                match app_state.editing_state {
                    EditingState::Recipe if app_state.recipe_state.editing_selected_field.is_none() => {
                        if let Some(recipe) = &app.edit_recipe {
                            if !recipe.steps.is_empty() {
                                app_state.editing_state = EditingState::Step(0);
                                app_state.step_state.selected_field = Wrapping(0);
                            }
                            //TODO: display an error if there are no steps defined
                        }
                    }
                    EditingState::Step(step) if app_state.step_state.editing_selected_field.is_none() => {
                        //TODO: check if step is even an index of the vector
                        if let Some(recipe) = &app.edit_recipe {
                            if !recipe.steps.is_empty() && !recipe.steps[step].ingredients.is_empty() {
                                app_state.editing_state = EditingState::Ingredient(step, 0);
                                app_state.ingredient_state.selected_field = Wrapping(0);
                            } else {
                                //already in step, but ingredient is None
                                app_state.editing_state = EditingState::Recipe;
                                app_state.recipe_state.selected_field = Wrapping(0);
                            }
                        }
                    }
                    EditingState::Ingredient(step, _) if app_state.ingredient_state.editing_selected_field.is_none() => {
                        //TODO: check if step is even an index of the vector
                        if let Some(recipe) = &app.edit_recipe {
                            if !recipe.steps.is_empty() && !recipe.steps[step].equipment.is_empty() {
                                app_state.editing_state = EditingState::Equipment(step, 0);
                                app_state.equipment_state.selected_field = Wrapping(0);
                            } else {
                                //already in ingredient, but equipment is None
                                app_state.editing_state = EditingState::Recipe;
                                app_state.recipe_state.selected_field = Wrapping(0);
                            }
                        }
                    }
                    EditingState::Equipment(_, _) if app_state.equipment_state.editing_selected_field.is_none() => {
                        app_state.editing_state = EditingState::Recipe;
                        app_state.recipe_state.selected_field = Wrapping(0);
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
                //TODO: prompt for save
                app.exit();
            }
            //KeyCode
            KeyCode::Char(chr) => {
                if app.edit_recipe.is_some() {
                    match app_state.editing_state {
                        EditingState::Idle if chr == 'e' || chr == 'i' => {
                            app_state.editing_state = EditingState::Recipe;
                            app_state.recipe_state.selected_field = Wrapping(0);
                        }
                        EditingState::Recipe => {
                            #[allow(clippy::unwrap_used)] // already checking for is_some above
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
                                None if chr == 'e' || chr == 'i' => {
                                    // the use of unwrap should be fine, since the FromPrimitive
                                    // is being derived automatically on an enum of
                                    // known size
                                    app_state.recipe_state.editing_selected_field =
                                        match FromPrimitive::from_usize(app_state.recipe_state.selected_field.0).unwrap() {
                                            RecipeFields::Name => Some(RecipeFields::Name),
                                            RecipeFields::Description => Some(RecipeFields::Description),
                                            RecipeFields::Comments => Some(RecipeFields::Comments),
                                            RecipeFields::Source => Some(RecipeFields::Source),
                                            RecipeFields::Author => Some(RecipeFields::Author),
                                            RecipeFields::AmountMade => Some(RecipeFields::AmountMade),
                                        }
                                }
                                None if chr == 's' => {
                                    app.edit_recipe.as_mut().unwrap().steps.push(Step::default());
                                    //TODO: should the editing state change automatically here?
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
                                    app.edit_recipe.as_mut().unwrap().steps[step].instructions.push(chr)
                                }
                                Some(StepFields::StepType) => {} //TODO,
                                None if chr == 'e' || chr == 'i' => {
                                    // the use of unwrap should be fine, since the FromPrimitive
                                    // is being derived automatically on an enum of
                                    // known size
                                    app_state.step_state.editing_selected_field =
                                        match FromPrimitive::from_usize(app_state.step_state.selected_field.0).unwrap() {
                                            StepFields::TimeNeeded => Some(StepFields::TimeNeeded),
                                            StepFields::Temperature => Some(StepFields::Temperature),
                                            StepFields::Instructions => Some(StepFields::Instructions),
                                            StepFields::StepType => Some(StepFields::StepType),
                                        }
                                }
                                //q for eQuipment
                                None if chr == 'q' => {
                                    app.edit_recipe.as_mut().unwrap().steps[step]
                                        .equipment
                                        .push(Equipment::default());
                                    //TODO: should the editing state change automatically here?
                                }
                                //g for inGredient
                                None if chr == 'g' => {
                                    app.edit_recipe.as_mut().unwrap().steps[step]
                                        .ingredients
                                        .push(Ingredient::default());
                                    //TODO: should the editing state change automatically here?
                                }
                                _ => {}
                            }
                        }
                        EditingState::Ingredient(step, ingredient) => {
                            // the use of unwrap should be fine, since the FromPrimitive
                            // is being derived automatically on an enum of
                            // known size
                            match app_state.ingredient_state.editing_selected_field {
                                Some(IngredientFields::Name) => app.edit_recipe.as_mut().unwrap().steps[step].ingredients
                                    [ingredient]
                                    .name
                                    .push(chr),
                                Some(IngredientFields::Description) => app.edit_recipe.as_mut().unwrap().steps[step].ingredients
                                    [ingredient]
                                    .description
                                    .as_mut()
                                    .unwrap_or(&mut String::new())
                                    .push(chr),
                                None if chr == 'e' || chr == 'i' => {
                                    // the use of unwrap should be fine, since the FromPrimitive
                                    // is being derived automatically on an enum of
                                    // known size
                                    app_state.ingredient_state.editing_selected_field =
                                        match FromPrimitive::from_usize(app_state.ingredient_state.selected_field.0).unwrap() {
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
                                Some(EquipmentFields::Name) => {
                                    app.edit_recipe.as_mut().unwrap().steps[step].equipment[equip].name.push(chr)
                                }
                                Some(EquipmentFields::Description) => app.edit_recipe.as_mut().unwrap().steps[step].equipment
                                    [equip]
                                    .description
                                    .as_mut()
                                    .unwrap_or(&mut String::new())
                                    .push(chr),
                                Some(EquipmentFields::IsOwned) => {} //TODO:
                                None if chr == 'e' || chr == 'i' => {
                                    // the use of unwrap should be fine, since the FromPrimitive
                                    // is being derived automatically on an enum of
                                    // known size
                                    app_state.equipment_state.editing_selected_field =
                                        match FromPrimitive::from_usize(app_state.equipment_state.selected_field.0).unwrap() {
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
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step].instructions.pop()
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
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step].ingredients[ingredient]
                                        .name
                                        .pop()
                                }
                                Some(IngredientFields::Description) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step].ingredients[ingredient]
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
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step].equipment[equip].name.pop()
                                }
                                Some(EquipmentFields::Description) => {
                                    _ = app.edit_recipe.as_mut().unwrap().steps[step].equipment[equip]
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
            KeyCode::Delete => {} //TODO
            KeyCode::Enter => {
                if app.edit_recipe.is_some() {
                    #[allow(clippy::single_match)]
                    match app_state.editing_state {
                        EditingState::SavePrompt(_, _) => {
                            match app_state.save_response {
                                SaveResponse::Yes => {
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
                                    app.edit_recipe = None;
                                }
                                SaveResponse::Cancel => app_state.editing_state = EditingState::Recipe,
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
        },
    }
}
