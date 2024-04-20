use crate::datatypes::{
    equipment::EquipmentState,
    ingredient::IngredientState,
    recipe::{Recipe, RecipeState},
    step::StepState,
    tag::Tag,
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, ScrollbarState, StatefulWidget, StatefulWidgetRef, Widget, WidgetRef},
};

/// main application struct
#[derive(Debug, Default, PartialEq)]
pub struct App {
    /// the recipes contained in the application
    pub recipes: Vec<Recipe>,
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
    /// Idle
    #[default]
    Idle,
    /// Editing recipe
    Recipe,
    /// Editing step, first index is step index
    Step(usize),
    /// Editing ingredient, first index is step index, second index is ingredient index within step
    Ingredient(usize, usize),
    /// Editing equipment, first index is step index, second index is equipment index within step
    Equipment(usize, usize),
    ///Save Prompt
    SavePrompt,
}

impl App {
    /// [`new`] creates a new [`App`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            recipes: Vec::new(),
            edit_recipe: None,
            current_screen: CurrentScreen::default(),
            running: false,
            editing: None,
            tags: Vec::new(),
        }
    }

    /// [`compile_tag_list`] scans through all tags on all recipes, compiles them into the main app
    /// tag list, then sorts and deduplicates the list
    pub fn compile_tag_list(&mut self) {
        for recipe in &self.recipes {
            //TODO: maybe switch to using try_reserve instead
            self.tags.reserve(recipe.tags.len());
            self.tags.extend(recipe.tags.clone());
        }
        // don't care about order of duplicate elements since we are removing them
        self.tags.sort_unstable();
        self.tags.dedup();
        self.tags.shrink_to_fit();
    }

    /// [`tick`] handles the tick event of the app
    pub fn tick(&self) {
        //TODO: investigate this further
        //https://github.com/ratatui-org/templates/blob/966cf2e2b5808de8c905eacd1f4209fe82f804fe/simple/src/app.rs#L31
    }

    /// [`exit`] exits App
    pub fn exit(&mut self) {
        self.running = false;
    }
}

/// [`AppState`] represents the main state of the application. It holds all states for subparts of
/// the app, and anything that might need to change during a call to
/// [`StatefulWidgetRef::render_ref()`]
#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions, missing_docs)]
pub struct AppState {
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
    pub recipe_state: RecipeState,
    /// step state
    pub step_state: StepState,
    /// ingredient state
    pub ingredient_state: IngredientState,
    /// equipment state
    pub equipment_state: EquipmentState,
}

impl StatefulWidgetRef for App {
    type State = AppState;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // This should create a layout of 3 vertical columns
        // with the outer 2 taking up 25% of the space, and
        // the middle one taking up the center 50%
        // use [`Layout.areas()'] rather than [`Layout.split()`] for better API
        let [recipe_list_area, main_area, tag_list_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(50), Constraint::Percentage(25)])
            .areas(area);

        // This should split the middle box into 3 areas, one on the bottom that will hold the menu and
        // be 3 unit tall, one on the top that will show the title of the current recipe and be 5
        // units tall, and the middle will take up the remaining space
        let [title_area, recipe_area, menu_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(5), Constraint::Percentage(100), Constraint::Min(3)])
            .areas(main_area);

        //TODO: fix this styling
        //Block is a box around the title
        let title_block = Block::default().borders(Borders::ALL).style(Style::default());

        let mut recipe_list_items = Vec::<ListItem>::new();

        if recipe_list_items.is_empty() {
            recipe_list_items.push(ListItem::new(Line::from(Span::styled("No Recipes", Style::default().fg(Color::Red)))));
        } else {
            for recipe in &self.recipes {
                recipe_list_items.push(ListItem::new(Line::from(Span::styled(recipe.name.clone(), Style::default().fg(Color::Green)))));
            }
        }

        let recipe_list = List::new(recipe_list_items).block(Block::default().borders(Borders::ALL).title("Recipe List"));
        state.recipe_list_len = recipe_list.len();

        StatefulWidget::render(recipe_list, recipe_list_area, buf, &mut state.recipe_list_state);

        let mut current_nav_text = Vec::new();

        match self.current_screen {
            CurrentScreen::RecipeBrowser => {
                let title = Paragraph::new(Text::styled("Cookbook", Style::default().fg(Color::Blue))).block(title_block);

                title.render(title_area, buf);

                //TODO: add this to the recipe creator/recipe editor section, but with a reference to
                //the tag list of the edited recipe
                let mut tag_list_items = Vec::<ListItem>::new();
                if self.tags.is_empty() {
                    tag_list_items.push(ListItem::new(Line::from(Span::styled("No Tags", Style::default().fg(Color::Red)))));
                } else {
                    for tag in &self.tags {
                        tag_list_items.push(ListItem::new(Line::from(Span::styled(tag, Style::default().fg(Color::White)))));
                    }
                }

                let tag_list = List::new(tag_list_items).block(Block::default().borders(Borders::ALL).title("Tag List"));
                state.tag_list_len = tag_list.len();
                StatefulWidget::render(tag_list, tag_list_area, buf, &mut state.tag_list_state);
                if !self.recipes.is_empty() {
                    WidgetRef::render_ref(&self.recipes[state.recipe_list_state.selected().unwrap_or_default()], recipe_area, buf);
                }
                //TODO: store this text, and the keyboard shortcuts somewhere centralized
                current_nav_text.push(Span::styled("Browsing", Style::default().fg(Color::Green)));
                current_nav_text.push(Span::styled(" | ", Style::default().fg(Color::White)));
                current_nav_text.push(Span::styled("q:quit, n:new, \u{2195}: scroll", Style::default().fg(Color::White)));
            }
            CurrentScreen::RecipeViewer => {
                // only show tags associated with recipe
                //TODO: implement
                current_nav_text.push(Span::styled("Viewing", Style::default().fg(Color::Blue)));
                current_nav_text.push(Span::styled(" | ", Style::default().fg(Color::White)));
                current_nav_text.push(Span::styled("ESC: return to browsing", Style::default().fg(Color::White)));
            }
            CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
                #[allow(clippy::expect_used)] //TODO: confirm this
                let recipe = &self.edit_recipe.as_ref().expect("No recipe currently being edited while in edit screen");

                if recipe.name.is_empty() && self.current_screen == CurrentScreen::RecipeCreator {
                    let title = Paragraph::new(Text::styled("New Recipe", Style::default().fg(Color::Green))).block(title_block);
                    title.render(title_area, buf);
                } else {
                    let title = Paragraph::new(Text::styled(recipe.name.clone(), Style::default().fg(Color::Blue))).block(title_block);
                    title.render(title_area, buf);
                }

                match state.editing_state {
                    EditingState::Recipe => StatefulWidgetRef::render_ref(*recipe, recipe_area, buf, &mut state.recipe_state),
                    EditingState::Step(step_num) => {
                        StatefulWidgetRef::render_ref(&recipe.steps[step_num], recipe_area, buf, &mut state.step_state);
                    }
                    EditingState::Ingredient(step_num, ingredient_num) => {
                        StatefulWidgetRef::render_ref(&recipe.steps[step_num].ingredients[ingredient_num], recipe_area, buf, &mut state.ingredient_state);
                    }
                    EditingState::Equipment(step_num, equipment_num) => {
                        StatefulWidgetRef::render_ref(&recipe.steps[step_num].equipment[equipment_num], recipe_area, buf, &mut state.equipment_state);
                    }
                    EditingState::Idle => {
                        if self.current_screen == CurrentScreen::RecipeCreator {
                            let instruction_block = Block::default().borders(Borders::ALL).style(Style::default());
                            let instructions = Paragraph::new(Text::styled("Press e to start editing new recipe", Style::default().fg(Color::Red))).block(instruction_block);
                            instructions.render(recipe_area, buf);
                        } else {
                            // if existing recipe, display same fields as editingstate::recipe, but don't
                            // allow edits
                            todo!()
                        }
                    }
                    EditingState::SavePrompt => {}
                }

                // TODO: only show tags associated with recipe

                if self.current_screen == CurrentScreen::RecipeCreator {
                    current_nav_text.push(Span::styled("Creating", Style::default().fg(Color::Magenta)));
                }
                if self.current_screen == CurrentScreen::RecipeEditor {
                    current_nav_text.push(Span::styled("Editing", Style::default().fg(Color::Yellow)));
                }
                current_nav_text.push(Span::styled(" | ", Style::default().fg(Color::White)));
                let mut keybinds = String::new();
                if state.editing_state == EditingState::Idle {
                    keybinds += "ESC: return to browsing ";
                } else {
                    keybinds += "ESC: exit text editing ";
                }
                keybinds += "TAB: switch focus between recipe parts ";
                // left/right arrows
                keybinds += "\u{2194}: cycle between fields ";
                // up/down arrows
                keybinds += "\u{2195}: cycle between steps/equipment entries";
                current_nav_text.push(Span::styled(keybinds, Style::default().fg(Color::White)));
            }
        }
        let footer = Paragraph::new(Line::from(current_nav_text)).block(Block::default().borders(Borders::ALL));
        footer.render(menu_area, buf);
    }
}
