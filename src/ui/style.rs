use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};

// Pastel colors
pub const PASTEL_BLACK: Color = Color::Rgb(10, 10, 10);
pub const PASTEL_RED: Color = Color::Rgb(255, 102, 102);
pub const PASTEL_GREEN: Color = Color::Rgb(102, 255, 102);
pub const PASTEL_YELLOW: Color = Color::Rgb(255, 255, 102);
pub const VIBRANT_YELLOW: Color = Color::Rgb(255, 204, 0);
pub const PASTEL_BLUE: Color = Color::Rgb(102, 102, 255);
pub const PASTEL_MAGENTA: Color = Color::Rgb(255, 102, 255);
pub const PASTEL_CYAN: Color = Color::Rgb(102, 255, 255);
pub const PASTEL_WHITE: Color = Color::Rgb(245, 245, 245);

// Default style
pub fn default_style() -> Style {
    Style::default().fg(PASTEL_WHITE).bg(PASTEL_BLACK)
}

// Selected style
pub fn selected_style() -> Style {
    Style::default().fg(PASTEL_BLACK).bg(VIBRANT_YELLOW)
}

// Thick, rounded borders
pub fn thick_rounded_borders() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(PASTEL_BLUE))
}
