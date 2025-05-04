use std::collections::HashMap;
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
                keybinds: HashMap::from([
                    (
                        "recipe_scroll_down".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "scroll down in recipe list".to_owned(),
                            display_text: "\u{2193}".to_owned(),
                        },
                    ),
                    (
                        "recipe_scroll_up".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "scroll up in recipe list".to_owned(),
                            display_text: "\u{2191}".to_owned(),
                        },
                    ),
                ]),
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
    /// scroll between options in popup prompts
    pub prompt_scroll: KeybindGroup,
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
    /// delete character behind the cursor
    pub back_delete: KeybindDefinition,
    /// delete character in front of cursor
    pub front_delete: KeybindDefinition,
    /// move cursor while editing fields
    pub move_cursor: KeybindGroup,
    /// confirm choices and insert new lines
    pub confirm: KeybindDefinition,
}

impl Default for EditingKeybinds {
    fn default() -> Self {
        Self {
            edit: KeybindGroup {
                instructional_text: "Edit selected field".to_owned(),
                display_text: "e || i".to_owned(),
                keybinds: HashMap::from([
                    (
                        "edit".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Char('e'),
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Edit selected field".to_owned(),
                            display_text: "e".to_owned(),
                        },
                    ),
                    (
                        "edit_alt".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Char('i'),
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Edit selected fielde".to_owned(),
                            display_text: "i".to_owned(),
                        },
                    ),
                ]),
            },
            exit: KeybindDefinition {
                key: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Finish editing recipe".to_owned(),
                display_text: "ESC".to_owned(),
            },
            prompt_scroll: KeybindGroup {
                instructional_text: "Scroll prompt options".to_owned(),
                display_text: "\u{2190} || \u{2192}".to_owned(),
                keybinds: HashMap::from([
                    (
                        "prompt_scroll_left".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Scroll Prompt Option Left".to_owned(),
                            display_text: "\u{2190}".to_owned(),
                        },
                    ),
                    (
                        "prompt_scroll_right".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Right,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Scroll Prompt Option Right".to_owned(),
                            display_text: "\u{2192}".to_owned(),
                        },
                    ),
                ]),
            },
            field_scroll: KeybindGroup {
                instructional_text: "cycle between fields".to_owned(),
                display_text: "\u{2195}".to_owned(),
                keybinds: HashMap::from([
                    (
                        "field_scroll_down".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "scroll to next field".to_owned(),
                            display_text: "\u{2193}".to_owned(),
                        },
                    ),
                    (
                        "field_scroll_up".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "scroll to previous field".to_owned(),
                            display_text: "\u{2191}".to_owned(),
                        },
                    ),
                ]),
            },
            item_scroll: KeybindGroup {
                instructional_text: "cycle between items".to_owned(),
                display_text: "\u{21E7}+\u{2195}".to_owned(),
                keybinds: HashMap::from([
                    (
                        "item_scroll_down".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Down,
                            modifiers: KeyModifiers::SHIFT,
                            instructional_text: "scroll to next item".to_owned(),
                            display_text: "\u{21E7} + \u{2193}".to_owned(),
                        },
                    ),
                    (
                        "item_scroll_up".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Up,
                            modifiers: KeyModifiers::SHIFT,
                            instructional_text: "scroll to previous item".to_owned(),
                            display_text: "\u{21E7} + \u{2191}".to_owned(),
                        },
                    ),
                ]),
            },
            item_switch: KeybindGroup {
                instructional_text: "switch between item types".to_owned(),
                display_text: "(\u{21E7}) + \u{2B7E}".to_owned(),
                keybinds: HashMap::from([
                    (
                        "item_switch_forward".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Tab,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "switch to next item type".to_owned(),
                            display_text: "\u{2B7E}".to_owned(),
                        },
                    ),
                    (
                        "item_switch_reverse".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Tab,
                            modifiers: KeyModifiers::SHIFT,
                            instructional_text: "switch to previous item type".to_owned(),
                            display_text: "\u{21E7}+\u{2B7E}".to_owned(),
                        },
                    ),
                ]),
            },
            new_step: KeybindDefinition {
                key: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new Step".to_owned(),
                display_text: "s".to_owned(),
            },
            new_ingredient: KeybindDefinition {
                key: KeyCode::Char('g'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new inGredient".to_owned(),
                display_text: "g".to_owned(),
            },
            new_equipment: KeybindDefinition {
                key: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                instructional_text: "Insert new eQuipment".to_owned(),
                display_text: "q".to_owned(),
            },
            back_delete: KeybindDefinition {
                key: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Delete text behind cursor".to_owned(),
                display_text: "\u{232B}".to_owned(),
            },
            front_delete: KeybindDefinition {
                key: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Delete text in front of cursor".to_owned(),
                display_text: "\u{2326}".to_owned(),
            },
            move_cursor: KeybindGroup {
                instructional_text: "Move cursor while editing".to_owned(),
                display_text: "\u{2194}".to_owned(),
                keybinds: HashMap::from([
                    (
                        "move_cursor_left".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Move cursor left while editing".to_owned(),
                            display_text: "\u{2190}".to_owned(),
                        },
                    ),
                    (
                        "move_cursor_right".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Right,
                            modifiers: KeyModifiers::NONE,
                            instructional_text: "Move cursor right while editing".to_owned(),
                            display_text: "\u{2192}".to_owned(),
                        },
                    ),
                ]),
            },
            confirm: KeybindDefinition {
                key: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                instructional_text: "Confirm selection or insert newline".to_owned(),
                display_text: "\u{21B5}".to_owned(),
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
                instructional_text: "Return to Browsing".to_owned(),
                display_text: "ESC".to_owned(),
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
                instructional_text: "Finish editing recipe".to_owned(),
                display_text: "ESC".to_owned(),
            },
        }
    }
}

/// `CoreKeybinds` contains keybinds that are available at all points during usage of the app.
#[derive(Debug, PartialEq)]
pub struct CoreKeybinds {
    /// force exits app without saving
    pub exit: KeybindGroup,
}

impl Default for CoreKeybinds {
    fn default() -> Self {
        Self {
            exit: KeybindGroup {
                display_text: "^c".to_owned(),
                instructional_text: "force quit app without saving".to_owned(),
                keybinds: HashMap::from([
                    (
                        "^c".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            instructional_text: "force quit app without saving".to_owned(),
                            display_text: "^c".to_owned(),
                        },
                    ),
                    (
                        "^C".to_owned(),
                        KeybindDefinition {
                            key: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL & KeyModifiers::SHIFT,
                            instructional_text: "force quit app without saving".to_owned(),
                            display_text: "^c".to_owned(),
                        },
                    ),
                ]),
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
    pub keybinds: HashMap<String, KeybindDefinition>,
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
