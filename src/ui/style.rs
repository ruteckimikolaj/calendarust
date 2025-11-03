use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};

// Pastel colors
pub const PASTEL_BLACK: Color = Color::Rgb(10, 10, 10);
pub const PASTEL_RED: Color = Color::Rgb(255, 102, 102);
pub const VIBRANT_YELLOW: Color = Color::Rgb(255, 204, 0);
pub const PASTEL_BLUE: Color = Color::Rgb(102, 102, 255);
pub const PASTEL_CYAN: Color = Color::Rgb(102, 255, 255);

// Style for the navigation cursor
pub fn focused_style() -> Style {
    Style::default().fg(PASTEL_CYAN)
}

// Style for a confirmed selection
pub fn selection_style() -> Style {
    Style::default()
        .fg(PASTEL_BLACK)
        .bg(VIBRANT_YELLOW)
        .add_modifier(Modifier::BOLD)
}

// Thick, rounded borders
pub fn thick_rounded_borders() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(PASTEL_BLUE))
}
