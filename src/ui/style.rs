use ratatui::style::{Color, Modifier, Style};

// Pastel colors
pub const PASTEL_RED: Color = Color::Rgb(255, 102, 102);
pub const PASTEL_CYAN: Color = Color::Rgb(102, 255, 255);

// Style for the navigation cursor
pub fn focused_style() -> Style {
    Style::default().bg(Color::Black).fg(Color::White)
}

// Style for a confirmed selection
pub fn selection_style() -> Style {
    Style::default().add_modifier(Modifier::REVERSED)
}

// Style for a normal cell
pub fn normal_style() -> Style {
    Style::default().fg(Color::Gray).bg(Color::DarkGray)
}
