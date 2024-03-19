use super::app::{App, CurrentScreen, EditingState};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// `handle_key_event` handles all `KeyEvent`s
pub fn handle_key_events(app: &mut App, key_event: KeyEvent) {
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
            KeyCode::Char('n') => app.current_screen = CurrentScreen::RecipeCreator,
            //https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
            KeyCode::Down => {
                // selected is the integer index of the selected item in the list
                if let Some(selected) = app.recipe_list_state.selected() {
                    // checking to see if at bottom of list, so we can wrap
                    if selected >= app.recipe_list_len - 1 {
                        app.recipe_list_state.select(Some(0));
                    } else {
                        app.recipe_list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Up => {
                if let Some(selected) = app.recipe_list_state.selected() {
                    // not at top of list, so move up
                    if selected > 0 {
                        app.recipe_list_state.select(Some(selected - 1));
                    } else {
                        // go to bottom of list
                        app.recipe_list_state.select(Some(app.recipe_list_len - 1));
                    }
                }
            }
            _ => {}
        },
        CurrentScreen::RecipeEditor => match key_event.code {
            KeyCode::Esc => {
                //TODO: prompt for save
                app.current_screen = CurrentScreen::RecipeBrowser;
            }
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    //TODO: prompt for save
                    app.exit();
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
        // - left/right arrow keys cycle between fields
        // - up/down arrow keys cycle between the individual steps/equipment
        // -
        // want a separate editing screen for the steps in the recipe
        CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => match key_event.code {
            // want to be able to stop editing but still be in creation/editing view
            KeyCode::Esc => {
                if app.editing_state == EditingState::Idle {
                    //TODO: prompt for save
                    app.current_screen = CurrentScreen::RecipeBrowser;
                } else {
                    app.editing_state = EditingState::Idle;
                }
            }
            KeyCode::Tab => {
                //toggle between editing recipe, steps, or ingredients
                match app.editing_state {
                    EditingState::Recipe => {
                        app.editing_state = EditingState::Step(0);
                    }
                    EditingState::Step(step) => {
                        app.editing_state = EditingState::Ingredient(step, 0);
                    }
                    EditingState::Ingredient(step, _) => {
                        app.editing_state = EditingState::Equipment(step, 0);
                    }
                    EditingState::Equipment(_, _) => {
                        app.editing_state = EditingState::Recipe;
                    }
                    _ => {}
                }
            }
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    //TODO: prompt for save
                    app.exit();
                }
            }
            //KeyCode
            _ => {}
        },
    }
}
