pub mod day;
pub mod event_form;
pub mod month;
pub mod style;
pub mod week;
pub mod year;

use crate::app::{App, AppState, InteractionMode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    let main_area = chunks[0];
    let footer_area = chunks[1];

    match app.state {
        AppState::Year => year::draw_year_view(f, app, main_area),
        AppState::Month => month::draw_month_view(f, app, main_area),
        AppState::Week => week::draw_week_view(f, app, main_area),
        AppState::Day => day::draw_day_view(f, app, main_area),
    }

    if app.mode == InteractionMode::EventForm {
        event_form::draw_event_form(f, app, f.area());
    }

    let footer_text = get_footer_text(app);
    let footer = Paragraph::new(footer_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray))
            .title(" Controls "),
    );
    f.render_widget(footer, footer_area);
}

fn get_footer_text(app: &App) -> &'static str {
    match app.mode {
        InteractionMode::Navigation => match app.state {
            AppState::Year | AppState::Month => "Controls: [↑/↓/←/→] Navigate | [Enter] Select | [q] Quit",
            AppState::Day | AppState::Week => "Controls: [↑/↓/←/→] Navigate | [n] New Event | [e] Edit Event | [q] Quit",
        },
        InteractionMode::Selection => "Controls: [↑/↓/←/→] Navigate | [Enter] Confirm | [Esc] Cancel",
        InteractionMode::TimeSlot => "Controls: [↑/↓] Adjust End Time | [Enter] Confirm | [Esc] Cancel",
        InteractionMode::EventForm => "Controls: [Tab] Navigate | [Enter] Submit | [Esc] Cancel",
    }
}
