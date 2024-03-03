use crate::tui::app::{App, CurrentScreen};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// `layout` contains the layout code for the initial UI
pub fn layout(frame: &mut Frame, app: &App) {
    // This should create a layout of 3 vertical columns
    // with the outer 2 taking up 25% of the space, and
    // the middle one taking up the center 50%
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(frame.size());

    // This should split the middle box into 3 areas, one on the bottom that will hold the menu and
    // be 1 unit tall, one on the top that will show the title of the current recipe and be 10
    // units tall, and the middle will take up the remaining space
    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(5),
            Constraint::Percentage(100),
            Constraint::Min(3),
        ])
        .split(outer_layout[1]);

    //TODO: fix this styling
    //Block is a box around the title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    //TODO: change title
    let title = Paragraph::new(Text::styled("Cookbook", Style::default().fg(Color::Blue)))
        .block(title_block);

    //render_widget essentially pushes each widget into a frameusing the layout handler defined
    //earlier
    frame.render_widget(title, inner_layout[0]);

    let mut recipe_list_items = Vec::<ListItem>::new();

    for recipe in &app.recipes {
        recipe_list_items.push(ListItem::new(Line::from(Span::styled(
            recipe.name.clone(),
            Style::default().fg(Color::Green),
        ))));
    }
    if recipe_list_items.is_empty() {
        recipe_list_items.push(ListItem::new(Line::from(Span::styled(
            "No Recipes",
            Style::default().fg(Color::Red),
        ))));
    }

    let recipe_list = List::new(recipe_list_items).block(Block::default().borders(Borders::ALL));

    frame.render_widget(recipe_list, outer_layout[0]);

    let current_nav_text = vec![
        // what you are doing, first part of text
        match app.current_screen {
            CurrentScreen::RecipeBrowser => {
                Span::styled("Browsing", Style::default().fg(Color::Green))
            }
            CurrentScreen::RecipeEditing => {
                Span::styled("Editing", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::RecipeViewing => {
                Span::styled("Viewing", Style::default().fg(Color::Blue))
            }
        },
        // divider bar
        Span::styled(" | ", Style::default().fg(Color::White)),
        // keyboard shortcuts
        Span::styled("q:quit", Style::default().fg(Color::White)),
    ];
    let footer =
        Paragraph::new(Line::from(current_nav_text)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, inner_layout[2]);
}
