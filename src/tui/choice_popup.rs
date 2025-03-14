use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, StatefulWidgetRef, Widget},
};

use ranged_wrapping::RangedWrapping;

/// `ChoicePopup` is a centered popup box with multiple selectable choices
#[derive(Debug, Default, PartialEq)]
pub struct ChoicePopup {
    /// title of popup
    title: String,
    /// optional text to display above the choices
    description: Option<String>,
    /// list of choices and associated styles
    //TODO: maybe make this a hashmap or something for better find performance?
    choices: Vec<(String, Style)>,
    /// index of default choice in `choices` vector
    default_choice: usize,
    /// what percentage of the containing [`Rect`](`ratatui::layout::Rect)'s width the popup will
    /// take up
    percent_x: u16,
    /// what percentage of the containing [`Rect`](`ratatui::layout::Rect)'s height the popup will
    /// take up
    percent_y: u16,
    /// Style of outer block
    block_style: Style,
    /// Style to add to style of selected choice when selected
    selected_style: Style,
    /// Style of descriptive text
    description_style: Style,
}

impl ChoicePopup {
    // builder pattern

    /// set title of popup
    pub fn title(self, title: &str) -> Self {
        Self {
            title: title.to_owned(),
            ..self
        }
    }

    pub fn description(self, description: &str) -> Self {
        Self {
            description: Some(description.to_owned()),
            ..self
        }
    }

    /// set width percentage of containing [`Rect`](`ratatui::layout::Rect)
    pub fn percent_x(self, percent_x: u16) -> Self {
        Self { percent_x, ..self }
    }

    /// set height percentage of containing [`Rect`](`ratatui::layout::Rect)
    pub fn percent_y(self, percent_y: u16) -> Self {
        Self { percent_y, ..self }
    }

    /// set style of outer block
    pub fn block_style(self, block_style: Style) -> Self {
        Self { block_style, ..self }
    }

    /// set style of selected choice
    pub fn selected_style(self, selected_style: Style) -> Self {
        Self { selected_style, ..self }
    }

    /// set style of descriptive text
    pub fn description_style(self, description_style: Style) -> Self {
        Self {
            description_style,
            ..self
        }
    }

    /// directly set choices list
    pub fn choices(self, choices: Vec<(String, Style)>) -> Self {
        Self { choices, ..self }
    }

    /// append an individual choice to the list of choices
    pub fn append_choice(self, choice: &str, style: Style) -> Self {
        let mut choices = self.choices.clone();
        choices.push((choice.to_owned(), style));
        Self { choices, ..self }
    }

    pub fn set_default_choice(self, choice_idx: usize) -> Self {
        Self {
            default_choice: choice_idx,
            ..self
        }
    }
}

/// `State` is the state of the widget
#[derive(Debug, Default, PartialEq)]
pub struct State {
    /// which choice is selected
    selected_choice: RangedWrapping<usize>,
}

impl State {
    pub fn new(widget: ChoicePopup) -> Self {
        Self {
            selected_choice: RangedWrapping {
                value: 0,
                min: 0,
                max: widget.choices.len() - 1,
            },
        }
    }
    pub fn select_next(&mut self) {
        self.selected_choice += 1
    }
    pub fn select_previous(&mut self) {
        self.selected_choice -= 1
    }
    pub fn value(&self) -> usize {
        self.selected_choice.value
    }
}

impl StatefulWidgetRef for ChoicePopup {
    type State = State;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let save_popup_area = centered_rect(area, self.percent_x, self.percent_y);
        let clear = Clear;

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(self.block_style)
            .title(self.title.clone());

        //TODO: allow for multiple lines of options if there are more options than will fit on one line
        let [_, description_area, _, choices_area, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(save_popup_area);

        // use u16::MAX for max constraint size for now. TODO: see if there is a more sensible
        // default here
        let choice_constraints = self
            .choices
            .clone()
            .into_iter()
            .map(|choice| Constraint::Min(u16::try_from(choice.0.chars().count()).unwrap_or(u16::MAX)));

        let choice_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(choice_constraints)
            .horizontal_margin(1)
            .spacing(1)
            .flex(Flex::Center)
            .split(choices_area);

        let choice_paragraphs = self.choices.clone().into_iter().enumerate().map(|(idx, choice)| {
            let mut temp_style = choice.1;
            if state.selected_choice.value == idx {
                temp_style = temp_style.patch(self.selected_style)
            }
            Paragraph::new(choice.0)
                .block(Block::new().borders(Borders::NONE))
                .alignment(Alignment::Center)
                .style(temp_style)
        });

        clear.clone().render(save_popup_area, buf);
        if let Some(description) = &self.description {
            let description_paragraph = Paragraph::new(description.clone())
                .block(Block::new().borders(Borders::NONE))
                .alignment(Alignment::Center)
                .style(self.description_style);
            description_paragraph.render(description_area, buf);
        }
        popup_block.render(save_popup_area, buf);
        //clear.clone().render(choices_area, buf);
        let _ = choice_paragraphs
            .into_iter()
            .zip(choice_areas.iter())
            .for_each(|(pgh, area)| pgh.render(*area, buf));
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
