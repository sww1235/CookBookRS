use crate::datatypes::{
    equipment::EquipmentState,
    filetypes,
    ingredient::IngredientState,
    recipe::{Recipe, RecipeState},
    step::StepState,
    tag::Tag,
};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, ScrollbarState, StatefulWidget, StatefulWidgetRef, Widget,
        WidgetRef, Wrap,
    },
};

use std::fs;
use std::io;
use std::num::Saturating;
use std::path;

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
    /// Editing step, first value is step index
    Step(Saturating<usize>),
    /// Editing ingredient, first value is step index, second value is ingredient index within step
    Ingredient(Saturating<usize>, Saturating<usize>),
    /// Editing equipment, first value is step index, second value is equipment index within step
    Equipment(Saturating<usize>, Saturating<usize>),
    ///Save Prompt, first value is index to insert into recipes, second value is if the recipe was
    ///found or not
    SavePrompt(usize, bool),
}

impl App {
    /// `new` creates a new [`App`]
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
    /// `load_recipes_from_directory` recursively parses the provided directory path to parse all
    /// `*.toml` files found and load them into the cookbook.
    ///
    /// # Errors
    ///
    /// Will error if:
    /// - reading any of the individual recipes fails
    /// - the specified path is not a directory
    /// - [`OsStr`](std::ffi::OsStr) failed to parse to UTF-8
    pub fn load_recipes_from_directory(&mut self, dir: path::PathBuf) -> Result<(), io::Error> {
        if dir.as_path().is_dir() {
            Self::load_recipes_from_directory_inner(dir, &mut self.recipes)?;
            self.recipes.sort_unstable_by_key(|r| r.id);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format! {"Provided filepath not a directory {}", dir.display()},
            ))
            //TODO: return is not directory error
        }
    }

    fn load_recipes_from_directory_inner(inner_dir: path::PathBuf, recipes: &mut Vec<Recipe>) -> Result<(), io::Error> {
        let ext = match inner_dir.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => ext,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "os_str failed to parse to valid utf-8",
                    ))
                }
            },
            None => "",
        };
        if inner_dir.is_file() && ext == "toml" {
            let recipe = match Self::parse_recipe(inner_dir.clone()) {
                Ok(r) => r,
                Err(error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format! {"Parsing TOML file {} failed: {}", &inner_dir.display(), error},
                    ))
                }
            };
            recipes.push(recipe);
            Ok(())
        } else if inner_dir.is_dir() {
            for entry in fs::read_dir(&inner_dir)? {
                let entry = entry?; // read_dir returns result
                let path = entry.path();
                Self::load_recipes_from_directory_inner(path, recipes)?;
            }
            Ok(())
        } else {
            // not a directory or file (maybe a symlink or something?
            Ok(())
        }
    }

    pub fn parse_recipe(recipe_file: path::PathBuf) -> Result<Recipe, io::Error> {
        let contents = fs::read_to_string(recipe_file)?;
        let output: Result<filetypes::Recipe, toml::de::Error> = toml::from_str(contents.as_str());
        output
            .map(Into::into)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, format! {"Inner TOML parsing error: {}", err}))
    }

    /// `compile_tag_list` scans through all tags on all recipes, compiles them into the main app
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

    /// `tick` handles the tick event of the app
    pub fn tick(&self) {
        //TODO: investigate this further
        //https://github.com/ratatui-org/templates/blob/966cf2e2b5808de8c905eacd1f4209fe82f804fe/simple/src/app.rs#L31
    }

    /// `exit` exits App
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
    /// save_response
    pub save_response: SaveResponse,
}
/// [`SaveResponse`] is the return value from the save recipe prompt
#[derive(Debug, Default)]
pub enum SaveResponse {
    #[default]
    Yes,
    No,
    Cancel,
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
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .areas(area);

        // This should split the middle box into 3 areas, one on the bottom that will hold the menu and
        // be 3 unit tall, one on the top that will show the title of the current recipe and be 5
        // units tall, and the middle will take up the remaining space
        // TODO: automatically resize the menu_area based on number of lines
        let [title_area, recipe_area, menu_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(3), Constraint::Percentage(100), Constraint::Min(9)])
            .areas(main_area);

        //TODO: fix this styling
        //Block is a box around the title
        let title_block = Block::default().borders(Borders::ALL).style(Style::default());

        let mut recipe_list_items = Vec::<ListItem>::new();

        if self.recipes.is_empty() {
            recipe_list_items.push(ListItem::new(Line::from(Span::styled(
                "No Recipes",
                Style::default().fg(Color::Red),
            ))));
        } else {
            for recipe in &self.recipes {
                recipe_list_items.push(ListItem::new(Line::from(Span::styled(
                    recipe.name.clone(),
                    Style::default().fg(Color::Green),
                ))));
            }
        }

        let recipe_list = List::new(recipe_list_items).block(Block::default().borders(Borders::ALL).title("Recipe List"));
        state.recipe_list_len = recipe_list.len();

        StatefulWidget::render(recipe_list, recipe_list_area, buf, &mut state.recipe_list_state);

        let mut current_nav_text: Vec<Line> = Vec::new();

        match self.current_screen {
            CurrentScreen::RecipeBrowser => {
                let title = Paragraph::new(Text::styled("Cookbook", Style::default().fg(Color::Blue))).block(title_block);

                let clear = Clear;

                title.render(title_area, buf);

                //TODO: add this to the recipe creator/recipe editor section, but with a reference to
                //the tag list of the edited recipe
                let mut tag_list_items = Vec::<ListItem>::new();
                if self.tags.is_empty() {
                    tag_list_items.push(ListItem::new(Line::from(Span::styled(
                        "No Tags",
                        Style::default().fg(Color::Red),
                    ))));
                } else {
                    for tag in &self.tags {
                        tag_list_items.push(ListItem::new(Line::from(Span::styled(
                            tag,
                            Style::default().fg(Color::White),
                        ))));
                    }
                }

                let tag_list = List::new(tag_list_items).block(Block::default().borders(Borders::ALL).title("Tag List"));
                state.tag_list_len = tag_list.len();
                StatefulWidget::render(tag_list, tag_list_area, buf, &mut state.tag_list_state);
                //TODO: change this rendering to print list of steps with ingredients/equipment at
                //the top
                if !self.recipes.is_empty() {
                    WidgetRef::render_ref(
                        &self.recipes[state.recipe_list_state.selected().unwrap_or_default()],
                        recipe_area,
                        buf,
                    );
                } else {
                    clear.render(recipe_area, buf);
                }
                //TODO: store this text, and the keyboard shortcuts somewhere centralized
                let browser_nav_text = vec![
                    Span::styled("Browsing", Style::default().fg(Color::Green)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled(
                        "q:quit, n:new, \u{2195}: scroll to select recipe",
                        Style::default().fg(Color::White),
                    ),
                ];
                current_nav_text.push(Line::from_iter(browser_nav_text));
            }
            CurrentScreen::RecipeViewer => {
                // only show tags associated with recipe
                //TODO: implement
                let viewer_nav_text = vec![
                    Span::styled("Viewing", Style::default().fg(Color::Blue)),
                    Span::styled(" | ", Style::default().fg(Color::White)),
                    Span::styled("ESC: return to browsing", Style::default().fg(Color::White)),
                ];
                current_nav_text.push(Line::from_iter(viewer_nav_text));
            }
            CurrentScreen::RecipeCreator | CurrentScreen::RecipeEditor => {
                #[allow(clippy::expect_used)] //TODO: confirm this
                let recipe = &self
                    .edit_recipe
                    .as_ref()
                    .expect("No recipe currently being edited while in edit screen");

                if recipe.name.is_empty() && self.current_screen == CurrentScreen::RecipeCreator {
                    let title = Paragraph::new(Text::styled("New Recipe", Style::default().fg(Color::Green))).block(title_block);
                    title.render(title_area, buf);
                } else {
                    let title =
                        Paragraph::new(Text::styled(recipe.name.clone(), Style::default().fg(Color::Blue))).block(title_block);
                    title.render(title_area, buf);
                }

                match state.editing_state {
                    EditingState::Recipe => StatefulWidgetRef::render_ref(*recipe, recipe_area, buf, &mut state.recipe_state),
                    EditingState::Step(step_num) => {
                        StatefulWidgetRef::render_ref(&recipe.steps[step_num.0], recipe_area, buf, &mut state.step_state);
                    }
                    EditingState::Ingredient(step_num, ingredient_num) => {
                        StatefulWidgetRef::render_ref(
                            &recipe.steps[step_num.0].ingredients[ingredient_num.0],
                            recipe_area,
                            buf,
                            &mut state.ingredient_state,
                        );
                    }
                    EditingState::Equipment(step_num, equipment_num) => {
                        StatefulWidgetRef::render_ref(
                            &recipe.steps[step_num.0].equipment[equipment_num.0],
                            recipe_area,
                            buf,
                            &mut state.equipment_state,
                        );
                    }
                    EditingState::Idle => {
                        if self.current_screen == CurrentScreen::RecipeCreator {
                            let instruction_block = Block::default().borders(Borders::ALL).style(Style::default());
                            let instructions = Paragraph::new(Text::styled(
                                "Press e or i to start editing new recipe",
                                Style::default().fg(Color::Red),
                            ))
                            .block(instruction_block);
                            instructions.render(recipe_area, buf);
                        } else {
                            // if existing recipe, display same fields as editingstate::recipe, but don't
                            // allow edits
                            todo!()
                        }
                    }
                    EditingState::SavePrompt(_, _) => {
                        let save_prompt_area = centered_rect(recipe_area, 75, 10);
                        //TODO: display recipe name
                        let clear = Clear;
                        let popup_block = Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .title("Save Recipe?");
                        let [_, inner_layout_area, _] = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Fill(1)])
                            .areas(save_prompt_area);
                        let [yes_area, no_area, cancel_area] = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
                            .areas(inner_layout_area);
                        let mut yes_style = Style::new().on_green().white();
                        let mut no_style = Style::new().on_red().white();
                        let mut cancel_style = Style::new().on_blue().white();

                        // if selected
                        match state.save_response {
                            SaveResponse::Yes => {
                                yes_style = yes_style.black();
                            }
                            SaveResponse::No => {
                                no_style = no_style.black();
                            }
                            SaveResponse::Cancel => {
                                cancel_style = cancel_style.black();
                            }
                        }
                        let yes_paragraph = Paragraph::new("Yes")
                            .block(Block::new().borders(Borders::NONE))
                            .alignment(Alignment::Center)
                            .style(yes_style);
                        let no_paragraph = Paragraph::new("No")
                            .block(Block::new().borders(Borders::NONE))
                            .alignment(Alignment::Center)
                            .style(no_style);
                        let cancel_paragraph = Paragraph::new("Cancel")
                            .block(Block::new().borders(Borders::NONE))
                            .alignment(Alignment::Center)
                            .style(cancel_style);

                        clear.clone().render(save_prompt_area, buf);
                        //TODO: does this need to happen?
                        popup_block.render(save_prompt_area, buf);
                        clear.clone().render(inner_layout_area, buf);
                        yes_paragraph.render(yes_area, buf);
                        no_paragraph.render(no_area, buf);
                        cancel_paragraph.render(cancel_area, buf);
                    }
                }

                // TODO: only show tags associated with recipe

                let mut editor_nav_lines: Vec<Line> = Vec::new();
                let mut line_contents: Vec<Span> = Vec::new();
                match self.current_screen {
                    //TODO: provide better indication for editing state
                    CurrentScreen::RecipeCreator => {
                        line_contents.push(Span::styled("Creating", Style::default().magenta()));
                    }
                    CurrentScreen::RecipeEditor => {
                        line_contents.push(Span::styled("Editing", Style::default().yellow()));
                    }
                    _ => {}
                }
                line_contents.push(Span::styled(" | ", Style::default().white()));
                match state.editing_state {
                    EditingState::Idle => {
                        line_contents.push(Span::styled("ESC: return to browsing", Style::default().white()));
                        editor_nav_lines.push(Line::from_iter(line_contents));
                    }
                    EditingState::Recipe => {
                        line_contents.push(Span::styled("ESC: exit text editing", Style::default().white()));
                        editor_nav_lines.push(Line::from_iter(line_contents.clone()));
                        editor_nav_lines.push(Line::from(Span::styled(
                            "e || i: edit selected field",
                            Style::default().white(),
                        )));
                        // left/right arrows
                        editor_nav_lines.push(Line::from(Span::styled(
                            "\u{2195}: cycle between fields",
                            Style::default().white(),
                        )));
                        // tab
                        editor_nav_lines.push(Line::from(Span::styled(
                            "TAB: cycle between Recipes/Steps/Equipment/Ingredients",
                            Style::default().white(),
                        )));
                        // up/down arrows
                        //editor_nav_lines.push(Line::from(Span::styled(
                        //    "\u{2194}: cycle between steps/equipment entries",
                        //    Style::default().white(),
                        //)));
                        editor_nav_lines.push(Line::from(Span::styled("s: insert new Step", Style::default().white())));
                        current_nav_text.extend(editor_nav_lines);
                    }
                    EditingState::Step(_) => {
                        line_contents.push(Span::styled("ESC: exit text editing", Style::default().white()));
                        editor_nav_lines.push(Line::from_iter(line_contents.clone()));
                        editor_nav_lines.push(Line::from(Span::styled(
                            "e || i: edit selected field",
                            Style::default().white(),
                        )));
                        // left/right arrows
                        editor_nav_lines.push(Line::from(Span::styled(
                            "\u{2195}: cycle between fields",
                            Style::default().white(),
                        )));
                        // tab
                        editor_nav_lines.push(Line::from(Span::styled(
                            "TAB: cycle between Recipes/Steps/Equipment/Ingredients",
                            Style::default().white(),
                        )));
                        // up/down arrows
                        editor_nav_lines.push(Line::from(Span::styled(
                            "\u{2194}: Select recipe step",
                            Style::default().white(),
                        )));
                        editor_nav_lines.push(Line::from(Span::styled("g: insert new inGredient", Style::default().white())));
                        editor_nav_lines.push(Line::from(Span::styled("q: insert new eQuipment", Style::default().white())));
                        current_nav_text.extend(editor_nav_lines);
                    }
                    EditingState::Equipment(_, _) | EditingState::Ingredient(_, _) => {
                        line_contents.push(Span::styled("ESC: exit text editing", Style::default().white()));
                        editor_nav_lines.push(Line::from_iter(line_contents.clone()));
                        editor_nav_lines.push(Line::from(Span::styled(
                            "e || i: edit selected field",
                            Style::default().white(),
                        )));
                        // left/right arrows
                        editor_nav_lines.push(Line::from(Span::styled(
                            "\u{2195}: cycle between fields",
                            Style::default().white(),
                        )));
                        // tab
                        editor_nav_lines.push(Line::from(Span::styled(
                            "TAB: cycle between Recipes/Steps/Equipment/Ingredients",
                            Style::default().white(),
                        )));
                        // up/down arrows
                        match state.editing_state {
                            EditingState::Equipment(_, _) => {
                                editor_nav_lines.push(Line::from(Span::styled(
                                    "\u{2194}: cycle between Equipment",
                                    Style::default().white(),
                                )));
                            }
                            EditingState::Ingredient(_, _) => {
                                editor_nav_lines.push(Line::from(Span::styled(
                                    "\u{2194}: cycle between Ingredients",
                                    Style::default().white(),
                                )));
                            }
                            _ => {}
                        }
                        editor_nav_lines.push(Line::from(Span::styled("g: insert new inGredient", Style::default().white())));
                        editor_nav_lines.push(Line::from(Span::styled("q: insert new eQuipment", Style::default().white())));
                        current_nav_text.extend(editor_nav_lines);
                    }

                    EditingState::SavePrompt(_, _) => {
                        current_nav_text.clear();
                        current_nav_text.push(Line::from(Span::styled("Save Recipe?", Style::default().white())));
                    }
                }
            }
        }
        let footer = Paragraph::new(Text::from_iter(current_nav_text))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        footer.render(menu_area, buf);
    }
}
/// `centered_rect` generates a centered [`Rect`](ratatui::layout::Rect) for your application
///
/// Commonly used for generating popup dialog boxes, etc
///
/// Copied from [Ratatui's How To page](https://ratatui.rs/how-to/layout/center-a-rect/)
/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
