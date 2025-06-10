use std::collections::HashMap;
use std::fmt;
use std::io;
use std::num::Saturating;
use std::path::Path;

use gix::Repository;
use log::debug;
use num_traits::ToPrimitive;
use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, ScrollbarState, StatefulWidget, StatefulWidgetRef, Widget,
        WidgetRef, Wrap,
    },
    Frame,
};
use uuid::Uuid;

use crate::{
    datatypes::{
        equipment, ingredient,
        recipe::{self, Recipe, RecipeFieldOffset, RecipeFields},
        step,
        tag::Tag,
    },
    tui::{
        choice_popup::{self, ChoicePopup},
        keybinds::Keybinds as AppKeybinds,
        style::Style as AppStyle,
    },
};

/// main application struct
#[derive(Debug, Default, PartialEq)]
pub struct App {
    /// the recipes contained in the application
    pub recipes: HashMap<Uuid, Recipe>,
    /// either a new recipe, or a clone of the recipe that is currently being edited
    pub edit_recipe: Option<Recipe>,
    /// the current screen the application is on
    pub current_screen: CurrentScreen,
    /// editing flag, indicating which recipe you are editing. Not used for creating new recipes
    pub editing: Option<usize>,
    /// running flag
    pub running: bool,
    /// tag list
    pub tags: Vec<Tag>,
    /// git repository reference
    pub git_repo: Option<Repository>,
    /// keybinds for app
    pub keybinds: AppKeybinds,
    /// visual style for app
    pub style: AppStyle,
    /// storage for save prompt widget
    pub save_prompt: ChoicePopup,
}

/// `CurrentScreen` represents the screen the user is currently seeing
#[derive(Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum CurrentScreen {
    /// `RecipeBrowser` is the main screen that shows a list of recipes, and allows users to filter
    /// and search for recipes
    #[default]
    RecipeBrowser,
    /// `RecipeEditing` allows users to edit recipes
    RecipeEditor,
    /// `RecipeViewing` is the main way to view a recipe
    RecipeViewer,
    /// `RecipeCreator` is used for entry of new recipes
    RecipeCreator,
}

/// `EditingState` represents the current state of the editing/creation workflow
#[derive(Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum EditingState {
    #[default]
    /// Editing recipe
    Recipe,
    /// Editing step, first value is step index
    Step(Saturating<usize>),
    /// Editing ingredient, first value is step index, second value is ingredient index within step
    Ingredient(Saturating<usize>, Saturating<usize>),
    /// Editing equipment, first value is step index, second value is equipment index within step
    Equipment(Saturating<usize>, Saturating<usize>),
    ///Save Prompt, first value is index to insert into recipes, second value is if the recipe was
    ///found or not
    SavePrompt,
}

impl fmt::Display for EditingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditingState::Recipe => write!(f, "Recipe"),
            EditingState::Step(step_num) => write!(f, "Step: {step_num}"),
            EditingState::Ingredient(step_num, ingredient_num) => write!(f, "Ingredient {ingredient_num} of Step {step_num}"),
            EditingState::Equipment(step_num, equipment_num) => write!(f, "Equipment {equipment_num} of Step {step_num}"),
            EditingState::SavePrompt => {
                write!(f, "SavePrompt")
            }
        }
    }
}

impl App {
    /// `new` creates a new [`App`]
    #[must_use]
    pub fn new(keybinds: AppKeybinds, style: AppStyle) -> Self {
        Self {
            recipes: HashMap::new(),
            edit_recipe: None,
            current_screen: CurrentScreen::default(),
            running: false,
            editing: None,
            tags: Vec::new(),
            git_repo: None,
            keybinds,
            style: style.clone(),
            save_prompt: ChoicePopup::default()
                .title("Save Recipe?")
                .percent_x(75)
                .percent_y(10)
                .append_choice("Yes", style.yes_text)
                .append_choice("No", style.no_text)
                .append_choice("Cancel", style.cancel_text)
                .block_style(style.save_block)
                .description_style(style.normal_text)
                .selected_style(style.selected_text),
        }
    }

    /// `save_recipes_to_file` outputs all recipes contained in app to individual files in the
    /// specified directory
    pub fn save_recipes_to_directory(&self, dir: &Path) -> anyhow::Result<()> {
        if dir.is_dir() {
            if !self.recipes.is_empty() {
                for recipe in self.recipes.values() {
                    let mut path = dir.join(recipe.name.replace(' ', "_"));
                    _ = path.set_extension("toml");
                    Recipe::write_recipe(recipe.clone(), path.as_path())?
                }
                Ok(())
            } else {
                // no recipes loaded
                //TODO: log this
                Ok(())
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format! {"Provided filepath not a directory {}", dir.display()},
            ))?
        }
    }

    /// `tick` handles the tick event of the app
    pub fn tick(&self) {
        //TODO: investigate this further
        //https://github.com/ratatui-org/templates/blob/966cf2e2b5808de8c905eacd1f4209fe82f804fe/simple/src/app.rs#L31
    }

    /// `exit` exits App
    pub fn exit(&mut self) {
        self.running = false;
    }
    // use draw instead of implementing RenderRef for App so we can have a frame reference within
    // this code. See https://ratatui.rs/examples/apps/user_input/ for where this idea spawned
    //TODO: track and show cursor when editing fields
    /// `draw` contains the main rendering logic of the app
    pub fn draw(&self, frame: &mut Frame, state: &mut self::State) {
        let area = frame.area();
        //actually render everything at once, at the bottom of this function
        let mut recipe_list_items = Vec::<ListItem>::new();

        if self.recipes.is_empty() {
            recipe_list_items.push(ListItem::new(Line::from(Span::styled("No Recipes", self.style.missing_text))));
        } else {
            for recipe in self.recipes.values() {
                recipe_list_items.push(ListItem::new(Line::from(Span::styled(
                    recipe.name.clone(),
                    self.style.recipe_list_entries,
                ))));
            }
        }

        let recipe_list = List::new(recipe_list_items).block(Block::default().borders(Borders::ALL).title("Recipe List"));
        state.recipe_list_len = recipe_list.len();

        let mut tag_list = List::default();

        //Block is a box around the title
        let title_block = Block::default().borders(Borders::ALL).style(self.style.title_block);
        let mut title_paragraph = Paragraph::default();

        // this doesn't need to be mutable, as I am not initializing here, and only giving it a
        // value once down below
        let keybind_area_height: u16;
        let mut current_keybind_text: Vec<Line> = Vec::new();
        let status_block = Block::default().borders(Borders::ALL).style(self.style.status_block);
        let mut status_paragraph = Paragraph::default();

        match self.current_screen {
            CurrentScreen::RecipeBrowser => {
                title_paragraph = Paragraph::new(Text::styled("Cookbook", self.style.browse_title_text)).block(title_block);
                status_paragraph = Paragraph::new(Text::styled("Browsing", self.style.browsing_status)).block(status_block);

                //TODO: add this to the recipe creator/recipe editor section, but with a reference to
                //the tag list of the edited recipe
                let mut tag_list_items = Vec::<ListItem>::new();
                if self.tags.is_empty() {
                    tag_list_items.push(ListItem::new(Line::from(Span::styled("No Tags", self.style.missing_text))));
                } else {
                    for tag in &self.tags {
                        tag_list_items.push(ListItem::new(Line::from(Span::styled(tag, self.style.tag_list_entries))));
                    }
                }

                tag_list = List::new(tag_list_items).block(Block::default().borders(Borders::ALL).title("Tag List"));
                state.tag_list_len = tag_list.len();
                //TODO: see if this can be moved to the keybinds module
                let browser_kb_text = vec![
                    Span::styled(format!("{}", self.keybinds.browsing.quit), self.style.keyboard_shortcut_text),
                    Span::raw(" | "),
                    Span::styled(format!("{}", self.keybinds.browsing.new), self.style.keyboard_shortcut_text),
                    Span::raw(" | "),
                    Span::styled(
                        format!("{}", self.keybinds.browsing.recipe_scroll),
                        self.style.keyboard_shortcut_text,
                    ),
                ];

                //TODO: use fmt/display of recipe here to display a preview as folks are scrolling

                // keybind area height should never be larger than half of the total height of the
                // screen
                // TODO: enforce this limit somewhere else
                keybind_area_height = u16::try_from(browser_kb_text.len()).unwrap_or(area.height / 2);
                current_keybind_text.push(Line::from_iter(browser_kb_text));
            }
            CurrentScreen::RecipeViewer => {
                //TODO: set title paragraph to name of viewing recipe.
                //title_paragraph = Paragraph::new(Text::styled(recipe.name.clone(), self.style.view_title_text)).block(title_block);
                //TODO: only show tags associated with recipe
                status_paragraph = Paragraph::new(Text::styled("Viewing", self.style.viewing_status)).block(status_block);
                //TODO: update this once keybinds for viewer are finished
                let viewer_kb_text = vec![
                    Span::styled(format!("{}", self.keybinds.viewing.exit), self.style.keyboard_shortcut_text),
                    //Span::styled(format!("{}", self.keybinds.browsing.quit), self.style.keyboard_shortcut_text),
                    //Span::styled(format!("{}", self.keybinds.browsing.quit), self.style.keyboard_shortcut_text),
                ];
                // keybind area height should never be larger than half of the total height of the
                // screen
                // TODO: enforce this limit somewhere else
                keybind_area_height = u16::try_from(viewer_kb_text.len()).unwrap_or(area.height / 2);
                current_keybind_text.push(Line::from_iter(viewer_kb_text));
            }
            CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
                match &self.edit_recipe {
                    Some(recipe) => {
                        //This is purely getting the name of the edit recipe if it exists. Here it should
                        //always exist, as it should either be a new recipe, or an existing recipe by the
                        //time you get to CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor

                        // TODO: add a check here to see if recipe.is_empty() or something
                        if recipe.name.is_empty() && self.current_screen == CurrentScreen::RecipeCreator {
                            title_paragraph =
                                Paragraph::new(Text::styled("New Recipe", self.style.create_title_text)).block(title_block);
                        } else {
                            title_paragraph =
                                Paragraph::new(Text::styled(recipe.name.clone(), self.style.edit_title_text)).block(title_block);
                        }
                    }
                    None => {
                        //TODO: change back to recipe browsing here and throw error.
                        //self.edit_recipe should never be None by the time you are here
                    }
                }

                // TODO: only show tags associated with recipe

                match self.current_screen {
                    //TODO: provide better indication for editing state
                    CurrentScreen::RecipeCreator => {
                        status_paragraph =
                            Paragraph::new(Text::styled("Creating", self.style.creating_status)).block(status_block);
                    }
                    CurrentScreen::RecipeEditor => {
                        status_paragraph = Paragraph::new(Text::styled("Editing", self.style.editing_status)).block(status_block);
                    }
                    _ => {}
                }
                let mut editor_kb_text = Vec::new();
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.exit),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.edit),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.field_scroll),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.item_scroll),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.item_switch),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                editor_kb_text.push(Span::styled(
                    format!("{}", self.keybinds.editing.item_switch),
                    self.style.keyboard_shortcut_text,
                ));
                editor_kb_text.push(Span::raw(" | "));
                match state.editing_state {
                    EditingState::Recipe => {
                        editor_kb_text.push(Span::styled(
                            format!("{}", self.keybinds.editing.new_step),
                            self.style.keyboard_shortcut_text,
                        ));
                    }
                    EditingState::Step(_) => {
                        editor_kb_text.push(Span::styled(
                            format!("{}", self.keybinds.editing.new_ingredient),
                            self.style.keyboard_shortcut_text,
                        ));
                        editor_kb_text.push(Span::raw(" | "));
                        editor_kb_text.push(Span::styled(
                            format!("{}", self.keybinds.editing.new_equipment),
                            self.style.keyboard_shortcut_text,
                        ));
                    }

                    EditingState::SavePrompt => {
                        editor_kb_text.clear();
                        editor_kb_text.push(Span::styled("Save Recipe?", self.style.keyboard_shortcut_text));
                    }
                    _ => {}
                }
                // keybind area height should never be larger than half of the total height of the
                // screen
                // TODO: enforce this limit somewhere else
                keybind_area_height = u16::try_from(editor_kb_text.len()).unwrap_or(area.height / 2);
                current_keybind_text.push(Line::from_iter(editor_kb_text));
            }
        }

        //define layout areas at very bottom so we can manipulate their sizes in the code above.
        //
        //and then render everything below.
        //
        //Yes this makes the code a bit more difficult to follow, but is more flexible.

        let clear = Clear;

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
            .areas(area);

        // This should split the middle box into 4 areas, that are used as follows from top to
        // bottom:
        // - Title of current recipe - 3 units tall
        // - Display area for recipe and its contents - remaining space
        // - Display the keybinds - 9 units tall
        // - Display a status line - 3 units tall
        // TODO: automatically resize the menu_area based on number of lines
        let [title_area, recipe_area, keybinds_area, status_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(3),
                Constraint::Percentage(100),
                Constraint::Min(keybind_area_height),
                Constraint::Min(3),
            ])
            .areas(main_area);

        // render everything after defining areas (based on state)
        title_paragraph.render(title_area, frame.buffer_mut());

        StatefulWidget::render(
            recipe_list,
            recipe_list_area,
            frame.buffer_mut(),
            &mut state.recipe_list_state,
        );

        match self.current_screen {
            CurrentScreen::RecipeBrowser => {
                StatefulWidget::render(tag_list, tag_list_area, frame.buffer_mut(), &mut state.tag_list_state);
                //TODO: use fmt/display of recipe here to display a preview as folks are scrolling
                //
                //TODO: provide a keybind to select recipe and change to recipeViewer mode
                if !self.recipes.is_empty() {
                    //TODO: fix this state lookup, after switching to hashmap of recipes
                    //    WidgetRef::render_ref(
                    //        &self.recipes[state.recipe_list_state.selected().unwrap_or_default()],
                    //        recipe_area,
                    //        frame.buffer_mut(),
                    //    );
                } else {
                    clear.render(recipe_area, frame.buffer_mut());
                }
            }
            CurrentScreen::RecipeViewer => {
                //TODO use actual render widget methods here
                StatefulWidget::render(tag_list, tag_list_area, frame.buffer_mut(), &mut state.tag_list_state);
                if !self.recipes.is_empty() {
                    //TODO: fix this state lookup, after switching to hashmap of recipes
                    //WidgetRef::render_ref(
                    //    &self.recipes[state.recipe_list_state.selected().unwrap_or_default()],
                    //    recipe_area,
                    //    frame.buffer_mut(),
                    //);
                } else {
                    clear.render(recipe_area, frame.buffer_mut());
                }
            }
            CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => match &self.edit_recipe {
                Some(recipe) => match state.editing_state {
                    EditingState::Recipe => {
                        // field is currently being edited and cursor should be visible
                        match state.recipe_state.editing_selected_field {
                            Some(RecipeFields::Name) => {
                                #[expect(clippy::unwrap_used)]
                                frame.set_cursor_position(Position::new(
                                    //draw cursor at current position in field
                                    //
                                    //add +1 to skip border
                                    recipe_area.x + state.recipe_state.editing_field_cursor_position.unwrap() + 1,
                                    // RecipeFieldOffset is a automatically derived enum
                                    // via proc_macro. It contains the y offset of the
                                    // field, need +1 to skip border
                                    recipe_area.y + RecipeFieldOffset::Name.to_u16().unwrap() + 1,
                                ));
                            }
                            Some(RecipeFields::Description) => {
                                #[expect(clippy::unwrap_used)]
                                frame.set_cursor_position(Position::new(
                                    //draw cursor at current position in field
                                    //
                                    //add +1 to skip border
                                    recipe_area.x + state.recipe_state.editing_field_cursor_position.unwrap() + 1,
                                    // RecipeFieldOffset is a automatically derived enum
                                    // via proc_macro. It contains the y offset of the
                                    // field, need +1 to skip border
                                    recipe_area.y + RecipeFieldOffset::Description.to_u16().unwrap() + 1,
                                ));
                            }

                            Some(RecipeFields::Comments) => {
                                #[expect(clippy::unwrap_used)]
                                frame.set_cursor_position(Position::new(
                                    //draw cursor at current position in field
                                    //
                                    //add +1 to skip border
                                    recipe_area.x + state.recipe_state.editing_field_cursor_position.unwrap() + 1,
                                    // RecipeFieldOffset is a automatically derived enum
                                    // via proc_macro. It contains the y offset of the
                                    // field, need +1 to skip border
                                    recipe_area.y + RecipeFieldOffset::Comments.to_u16().unwrap() + 1,
                                ));
                            }

                            Some(RecipeFields::Source) => {
                                #[expect(clippy::unwrap_used)]
                                frame.set_cursor_position(Position::new(
                                    //draw cursor at current position in field
                                    //
                                    //add +1 to skip border
                                    recipe_area.x + state.recipe_state.editing_field_cursor_position.unwrap() + 1,
                                    // RecipeFieldOffset is a automatically derived enum
                                    // via proc_macro. It contains the y offset of the
                                    // field, need +1 to skip border
                                    recipe_area.y + RecipeFieldOffset::Source.to_u16().unwrap() + 1,
                                ));
                            }

                            Some(RecipeFields::Author) => {
                                #[expect(clippy::unwrap_used)]
                                frame.set_cursor_position(Position::new(
                                    //draw cursor at current position in field
                                    //
                                    //add +1 to skip border
                                    recipe_area.x + state.recipe_state.editing_field_cursor_position.unwrap() + 1,
                                    // RecipeFieldOffset is a automatically derived enum
                                    // via proc_macro. It contains the y offset of the
                                    // field, need +1 to skip border
                                    recipe_area.y + RecipeFieldOffset::Author.to_u16().unwrap() + 1,
                                ));
                            }

                            Some(RecipeFields::AmountMade) => {
                                todo!("AmountMade editing not implemented yet")
                            }
                            _ => {}
                        }
                        StatefulWidgetRef::render_ref(recipe, recipe_area, frame.buffer_mut(), &mut state.recipe_state)
                    }
                    EditingState::Step(step_num) => {
                        StatefulWidgetRef::render_ref(
                            &recipe.steps[step_num.0],
                            recipe_area,
                            frame.buffer_mut(),
                            &mut state.step_state,
                        );
                    }
                    EditingState::Ingredient(step_num, ingredient_num) => {
                        StatefulWidgetRef::render_ref(
                            &recipe.steps[step_num.0].ingredients[ingredient_num.0],
                            recipe_area,
                            frame.buffer_mut(),
                            &mut state.ingredient_state,
                        );
                    }
                    EditingState::Equipment(step_num, equipment_num) => {
                        StatefulWidgetRef::render_ref(
                            &recipe.steps[step_num.0].equipment[equipment_num.0],
                            recipe_area,
                            frame.buffer_mut(),
                            &mut state.equipment_state,
                        );
                    }
                    EditingState::SavePrompt => {
                        state.save_prompt_state.set_description(&recipe.name);
                        debug! {"selected_choice: {}", state.save_prompt_state.value()}
                        self.save_prompt
                            .render_ref(recipe_area, frame.buffer_mut(), &mut state.save_prompt_state);
                    }
                },
                None => {
                    //TODO: change back to recipe browsing here and throw error.
                    //self.edit_recipe should never be None by the time you are here
                }
            },
        }

        let keybinds_paragraph = Paragraph::new(Text::from_iter(current_keybind_text))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        keybinds_paragraph.render(keybinds_area, frame.buffer_mut());

        status_paragraph.render(status_area, frame.buffer_mut());
    }
}

/// [`State`] represents the main state of the application. It holds all states for subparts of
/// the app, and anything that might need to change during a call to
/// [`StatefulWidgetRef::render_ref()`]
#[derive(Debug)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct State {
    /// state for recipe list
    pub recipe_list_state: ListState,
    /// tag list state
    pub tag_list_state: ListState,
    /// tag list length
    pub tag_list_len: usize,
    /// recipe list scrollbar state
    pub recipe_scroll_state: ScrollbarState,
    /// length of recipe list
    pub recipe_list_len: usize,
    /// scrollbar state for viewer/editor
    pub middle_scrollbar_state: ScrollbarState,
    /// editing state
    pub editing_state: EditingState,
    /// recipe state
    pub recipe_state: recipe::State,
    /// step state
    pub step_state: step::State,
    /// ingredient state
    pub ingredient_state: ingredient::State,
    /// equipment state
    pub equipment_state: equipment::State,
    /// save_response
    pub save_prompt_state: choice_popup::State,
}

impl State {
    pub fn new(save_prompt: &ChoicePopup) -> Self {
        Self {
            recipe_list_state: ListState::default(),
            tag_list_state: ListState::default(),
            tag_list_len: usize::default(),
            recipe_scroll_state: ScrollbarState::default(),
            recipe_list_len: usize::default(),
            middle_scrollbar_state: ScrollbarState::default(),
            editing_state: EditingState::default(),
            recipe_state: recipe::State::default(),
            step_state: step::State::default(),
            ingredient_state: ingredient::State::default(),
            equipment_state: equipment::State::default(),
            save_prompt_state: choice_popup::State::new(save_prompt),
        }
    }
}
