use super::app::{App, CurrentScreen, EditingState};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// `handle_key_event` handles all `KeyEvent`s
pub fn handle_key_events(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::RecipeBrowser => match key_event.code {
            KeyCode::Char('q') => app.exit(),
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
            _ => {}
        },
        CurrentScreen::RecipeViewer => match key_event.code {
            KeyCode::Esc => app.current_screen = CurrentScreen::RecipeBrowser,
            _ => {}
        },
        CurrentScreen::RecipeCreator => match key_event.code {
            KeyCode::Esc => {
                //TODO: prompt for save
                app.current_screen = CurrentScreen::RecipeBrowser;
            }
            _ => {}
        },
    }
}
