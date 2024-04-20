use crate::{
    datatypes::recipe::Recipe,
    tui::app::{App, AppState, CurrentScreen, EditingState},
};

use std::num::Wrapping;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

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
                    app_state.recipe_list_state.select(Some((Wrapping(selected) + Wrapping(1_usize)).0));
                }
            }
            KeyCode::Up => {
                if let Some(selected) = app_state.recipe_list_state.selected() {
                    // not at top of list, so move up
                    app_state.recipe_list_state.select(Some((Wrapping(selected) - Wrapping(1_usize)).0));
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
                if app_state.editing_state == EditingState::Idle {
                    //TODO: prompt for save
                    app.current_screen = CurrentScreen::RecipeBrowser;
                } else {
                    app_state.editing_state = EditingState::Idle;
                }
            }
            KeyCode::Left => match app_state.editing_state {
                EditingState::Recipe => {
                    app_state.recipe_state.selected_field -= Wrapping(1);
                }
                EditingState::Step(_) => {
                    app_state.step_state.selected_field -= Wrapping(1);
                }
                EditingState::Ingredient(_, _) => {
                    app_state.ingredient_state.selected_field -= Wrapping(1);
                }
                EditingState::Equipment(_, _) => {
                    app_state.equipment_state.selected_field -= Wrapping(1);
                }
                _ => {}
            },
            KeyCode::Right => match app_state.editing_state {
                EditingState::Recipe => {
                    app_state.recipe_state.selected_field += Wrapping(1);
                }
                EditingState::Step(_) => {
                    app_state.step_state.selected_field += Wrapping(1);
                }
                EditingState::Ingredient(_, _) => {
                    app_state.ingredient_state.selected_field += Wrapping(1);
                }
                EditingState::Equipment(_, _) => {
                    app_state.equipment_state.selected_field += Wrapping(1);
                }
                _ => {}
            },
            KeyCode::Tab => {
                //toggle between editing recipe, steps, or ingredients
                match app_state.editing_state {
                    EditingState::Recipe => {
                        app_state.editing_state = EditingState::Step(0);
                    }
                    EditingState::Step(step) => {
                        app_state.editing_state = EditingState::Ingredient(step, 0);
                    }
                    EditingState::Ingredient(step, _) => {
                        app_state.editing_state = EditingState::Equipment(step, 0);
                    }
                    EditingState::Equipment(_, _) => {
                        app_state.editing_state = EditingState::Recipe;
                    }
                    _ => {}
                }
            }
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                //TODO: prompt for save
                app.exit();
            }
            //TODO: maybe change this to r for recpe?
            KeyCode::Char('e') if app_state.editing_state == EditingState::Idle => {
                app_state.editing_state = EditingState::Recipe;
            }
            //KeyCode
            _ => {}
        },
    }
}
