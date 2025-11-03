pub mod day;
pub mod event_form;
pub mod month;
pub mod week;
pub mod year;

use crate::app::{App, AppState, InteractionMode};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &mut App) {
    let size = f.area();
    match app.state {
        AppState::Year => year::draw_year_view(f, app, size),
        AppState::Month => month::draw_month_view(f, app, size),
        AppState::Week => week::draw_week_view(f, app, size),
        AppState::Day => day::draw_day_view(f, app, size),
    }

    if let InteractionMode::EventForm = app.mode {
        event_form::draw_event_form(f, app, size);
    }
}
