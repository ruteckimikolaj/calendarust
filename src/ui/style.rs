use ratatui::style::{Color, Style};

// Pastel colors
pub const PASTEL_BLACK: Color = Color::Rgb(10, 10, 10);
pub const PASTEL_RED: Color = Color::Rgb(255, 102, 102);
pub const PASTEL_CYAN: Color = Color::Rgb(102, 255, 255);

// Style for the navigation cursor
pub fn focused_style() -> Style {
    Style::default().bg(Color::Black).fg(Color::White)
}

// Style for a confirmed selection
pub fn selection_style() -> Style {
    Style::default().bg(Color::Rgb(0, 0, 100))
}

// Style for a cell with an event
pub fn event_style() -> Style {
    Style::default().bg(PASTEL_CYAN).fg(PASTEL_BLACK)
}

// Style for a normal cell
pub fn normal_style() -> Style {
    Style::default().fg(Color::Gray).bg(Color::DarkGray)
}
