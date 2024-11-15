use ratatui::style::{Style as TUIStyle, Stylize};

/// `Style` contains all the TUI styles
#[derive(Debug, PartialEq)]
pub struct Style {
    pub normal_text: TUIStyle,
    pub missing_text: TUIStyle,
    pub title_block: TUIStyle,
    pub browse_title_text: TUIStyle,
    pub view_title_text: TUIStyle,
    pub create_title_text: TUIStyle,
    pub edit_title_text: TUIStyle,
    pub recipe_list_entries: TUIStyle,
    pub tag_list_entries: TUIStyle,
    pub keyboard_shortcut_text: TUIStyle,
    pub status_block: TUIStyle,
    pub browsing_status: TUIStyle,
    pub viewing_status: TUIStyle,
    pub creating_status: TUIStyle,
    pub editing_status: TUIStyle,
    pub save_block: TUIStyle,
    pub yes_text: TUIStyle,
    pub no_text: TUIStyle,
    pub cancel_text: TUIStyle,
    pub selected_text: TUIStyle,
}

//TODO: fix these default styles, also document them better
impl Default for Style {
    fn default() -> Self {
        let base_text_style = TUIStyle::new();
        let base_block_style = TUIStyle::new();
        let base_list_style = TUIStyle::new();
        Self {
            // use shortcut color/style functions from style::Stylize instead of fg/bg functions
            normal_text: base_text_style,
            missing_text: base_text_style.red(),
            title_block: base_block_style,
            browse_title_text: base_text_style.blue(),
            view_title_text: base_text_style,
            create_title_text: base_text_style.green(),
            edit_title_text: base_text_style.blue(),
            recipe_list_entries: base_list_style.green(),
            tag_list_entries: base_list_style.white(),
            keyboard_shortcut_text: base_text_style.white(),
            status_block: base_block_style,
            browsing_status: base_text_style.green(),
            viewing_status: base_text_style.blue(),
            creating_status: base_text_style.magenta(),
            editing_status: base_text_style.yellow(),
            save_block: base_block_style,
            yes_text: base_text_style.on_green().white(),
            no_text: base_text_style.on_red().white(),
            cancel_text: base_text_style.on_blue().white(),
            selected_text: base_text_style.black(),
        }
    }
}
