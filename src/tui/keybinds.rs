use std::fmt;

use crossterm::event::{KeyCode, KeyModifiers};

/// `AppKeybinds` contains all keybinds used by the TUI app.
#[derive(Debug, Default, PartialEq)]
pub struct Keybinds {
    pub browsing: BrowsingKeybinds,
    pub editing: EditingKeybinds,
    pub viewing: ViewingKeybinds,
    pub field_editing: FieldEditingKeybinds,
    pub core: CoreKeybinds,
}

/// `BrowsingKeybinds` contains the keybinds used when in [`CurrentScreen::RecipeBrowser`]
#[derive(Debug, PartialEq)]
pub struct BrowsingKeybinds {
    pub quit: KeybindDefinition,
    pub new: KeybindDefinition,
    pub recipe_scroll: KeybindGroup,
}

impl Default for BrowsingKeybinds {
    fn default() -> Self {
        Self {
            quit: KeybindDefinition {
                key: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "quit".to_owned(),
                display_text: "q".to_owned(),
            },
            new: KeybindDefinition {
                key: KeyCode::Char('n'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "new".to_owned(),
                display_text: "n".to_owned(),
            },
            recipe_scroll: KeybindGroup {
                instructional_text: "scroll to select recipe".to_owned(),
                display_text: "\u{2195}".to_owned(),
                keybinds: vec![
                    KeybindDefinition {
                        key: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "scroll down in recipe list".to_owned(),
                        display_text: "\u{2193}".to_owned(),
                    },
                    KeybindDefinition {
                        key: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "scroll up in recipe list".to_owned(),
                        display_text: "\u{2191}".to_owned(),
                    },
                ],
            },
        }
    }
}

/// `EditingKeybinds` contains the keybinds used when in [`CurrentScreen::RecipeCreator`] or
/// [`CurrentScreen::RecipeEditor`]
#[derive(Debug, PartialEq)]
pub struct EditingKeybinds {
    /// start editing field
    pub edit: KeybindGroup,
    /// exit out of editing a recipe
    pub exit: KeybindDefinition,
    /// switch between fields in a recipe/step/equipment/ingredient
    pub field_scroll: KeybindGroup,
    /// scroll through steps/equipment/ingredients in a recipe
    pub item_scroll: KeybindGroup,
    /// switch between editing recipe/step/ingredient/equipment
    pub item_switch: KeybindGroup,
    /// insert a new step into a recipe
    pub new_step: KeybindDefinition,
    /// insert a new ingredient into a step
    pub new_ingredient: KeybindDefinition,
    /// insert a new equipment into a step
    pub new_equipment: KeybindDefinition,
}

impl Default for EditingKeybinds {
    fn default() -> Self {
        Self {
            edit: KeybindGroup {
                instructional_text: "Edit selected field".to_string(),
                display_text: "e || i".to_string(),
                keybinds: vec![
                    KeybindDefinition {
                        key: KeyCode::Char('e'),
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "Edit selected field".to_string(),
                        display_text: "e".to_string(),
                    },
                    KeybindDefinition {
                        key: KeyCode::Char('i'),
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "Edit selected fielde".to_string(),
                        display_text: "i".to_string(),
                    },
                ],
            },
            exit: KeybindDefinition {
                key: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Finish editing recipe".to_string(),
                display_text: "ESC".to_string(),
            },
            field_scroll: KeybindGroup {
                instructional_text: "cycle between fields".to_string(),
                display_text: "\u{2195}".to_string(),
                keybinds: vec![
                    KeybindDefinition {
                        key: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "scroll to next field".to_owned(),
                        display_text: "\u{2193}".to_owned(),
                    },
                    KeybindDefinition {
                        key: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "scroll to previous field".to_owned(),
                        display_text: "\u{2191}".to_owned(),
                    },
                ],
            },
            item_scroll: KeybindGroup {
                instructional_text: "cycle between items".to_string(),
                display_text: "\u{21E7}+\u{2195}".to_string(),
                keybinds: vec![
                    KeybindDefinition {
                        key: KeyCode::Down,
                        modifiers: KeyModifiers::SHIFT,
                        instructional_text: "scroll to next item".to_owned(),
                        display_text: "\u{21E7} + \u{2193}".to_owned(),
                    },
                    KeybindDefinition {
                        key: KeyCode::Up,
                        modifiers: KeyModifiers::SHIFT,
                        instructional_text: "scroll to previous item".to_owned(),
                        display_text: "\u{21E7} + \u{2191}".to_owned(),
                    },
                ],
            },
            item_switch: KeybindGroup {
                instructional_text: "switch between item types".to_string(),
                display_text: "(\u{21E7}) + \u{2B7E}".to_string(),
                keybinds: vec![
                    KeybindDefinition {
                        key: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                        instructional_text: "switch to next item type".to_owned(),
                        display_text: "\u{2B7E}".to_owned(),
                    },
                    KeybindDefinition {
                        key: KeyCode::Up,
                        modifiers: KeyModifiers::SHIFT,
                        instructional_text: "switch to next item type".to_owned(),
                        display_text: "\u{21E7}+\u{2B7E}".to_owned(),
                    },
                ],
            },
            new_step: KeybindDefinition {
                key: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new Step".to_string(),
                display_text: "s".to_string(),
            },
            new_ingredient: KeybindDefinition {
                key: KeyCode::Char('g'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new inGredient".to_string(),
                display_text: "g".to_string(),
            },
            new_equipment: KeybindDefinition {
                key: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new eQuipment".to_string(),
                display_text: "q".to_string(),
            },
        }
    }
}

//TODO: finish keybinds for viewer
/// `ViewingKeybinds` contains the keybinds used when in [`CurrentScreen::RecipeViewer`]
#[derive(Debug, PartialEq)]
pub struct ViewingKeybinds {
    pub exit: KeybindDefinition,
    // scroll through entire recipe, go to previous/next step
}

impl Default for ViewingKeybinds {
    fn default() -> Self {
        Self {
            exit: KeybindDefinition {
                key: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Return to Browsing".to_string(),
                display_text: "ESC".to_string(),
            },
        }
    }
}

/// `FieldEditingKeybinds` contains the keybinds used when editing a field
#[derive(Debug, PartialEq)]
pub struct FieldEditingKeybinds {
    /// key to exit editing a field
    pub exit: KeybindDefinition,
}

impl Default for FieldEditingKeybinds {
    fn default() -> Self {
        Self {
            exit: KeybindDefinition {
                key: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Finish editing recipe".to_string(),
                display_text: "ESC".to_string(),
            },
        }
    }
}

/// `CoreKeybinds` contains keybinds that are available at all points during usage of the app.
#[derive(Debug, PartialEq)]
pub struct CoreKeybinds {
    /// force exits app without saving
    pub exit: KeybindDefinition,
}

impl Default for CoreKeybinds {
    fn default() -> Self {
        Self {
            exit: KeybindDefinition {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                instructional_text: "force quit app without saving".to_string(),
                display_text: "^c".to_string(),
            },
        }
    }
}

//TODO: maybe change the text fields here to spans that can have formatting embedded
/// `KeybindDefinition` defines a keybind for the TUI application.
#[derive(Debug, PartialEq)]
pub struct KeybindDefinition {
    /// the [`crossterm::event::KeyCode`] of the key
    pub key: KeyCode,
    /// any [`crossterm::event::KeyModifiers`] needed to be associated with the key
    pub modifiers: KeyModifiers,
    /// user instructions for what the key will do
    pub instructional_text: String,
    /// symbols representing this key for display purposes
    pub display_text: String,
}

// TODO: remove this when switching to using ratatui spans
impl fmt::Display for KeybindDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // only print display_text
            write!(f, "{}", self.display_text)
        } else {
            // normal display output
            write!(f, "{}: {}", self.display_text, self.instructional_text)
        }
    }
}

/// `KeybindGroup` defines a group of [`KeyDefinition`]s that can be merged together in the
/// on-screen documentation with a single display_text and instruction.
#[derive(Debug, PartialEq)]
pub struct KeybindGroup {
    /// user instructions for the key group
    pub instructional_text: String,
    /// symbols representing this key group for display purposes
    pub display_text: String,
    /// keys in group
    pub keybinds: Vec<KeybindDefinition>,
}

impl fmt::Display for KeybindGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // only print display_text
            write!(f, "{}", self.display_text)
        } else {
            // normal display output
            write!(f, "{}: {}", self.display_text, self.instructional_text)
        }
    }
}
