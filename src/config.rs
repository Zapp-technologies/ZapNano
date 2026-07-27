use crossterm::style::Color;

pub struct Config {
    pub bg_color: Color,
    pub fg_color: Color,
    pub line_number_color: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bg_color: Color::Reset,
            fg_color: Color::Reset,
            line_number_color: Color::DarkGrey,
            status_bar_bg: Color::White,
            status_bar_fg: Color::Black,
        }
    }
}
