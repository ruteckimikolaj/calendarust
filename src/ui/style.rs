use ratatui::style::{Color, Modifier, Style};

// Pastel colors
pub const PASTEL_BLACK: Color = Color::Rgb(10, 10, 10);
pub const PASTEL_RED: Color = Color::Rgb(255, 102, 102);
pub const VIBRANT_YELLOW: Color = Color::Rgb(255, 204, 0);
pub const PASTEL_CYAN: Color = Color::Rgb(102, 255, 255);

// Style for the navigation cursor
pub fn focused_style() -> Style {
    Style::default().bg(PASTEL_CYAN).fg(PASTEL_BLACK)
}

// Style for a confirmed selection
pub fn selection_style() -> Style {
    Style::default()
        .fg(PASTEL_BLACK)
        .bg(VIBRANT_YELLOW)
        .add_modifier(Modifier::BOLD)
}

// Style for a normal cell
pub fn normal_style() -> Style {
    Style::default().fg(Color::Gray)
}
