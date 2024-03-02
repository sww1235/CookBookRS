use crate::tui::app::App;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

/// `ui_init` contains the layout code for the initial UI
pub fn ui_init(frame: &mut Frame, app: &App) {
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
            Constraint::Max(10),
            Constraint::Percentage(100),
            Constraint::Max(1),
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

    // render_widget essentially pushes each widget into a frameusing the layout handler defined
    // earlier
    frame.render_widget(title, inner_layout[0]);

    //let recipe_list = List::default()
    //
    //frame.render_widget(recipe_list, outer_layout[0]);
}
