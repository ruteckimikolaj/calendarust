use crate::models::config::Config;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::Connection;
use tui_textarea::TextArea;

pub enum AppState {
    Year,
    Month,
    Week,
    Day,
}

pub enum InteractionMode {
    Navigation,
    Selection,
    TimeSlot,
    EventForm,
}

pub struct EventFormState<'a> {
    pub title: TextArea<'a>,
    pub description: TextArea<'a>,
    pub location: TextArea<'a>,
    pub start_datetime: chrono::NaiveDateTime,
    pub end_datetime: chrono::NaiveDateTime,
    pub focused_field: usize,
}

pub struct App<'a> {
    pub state: AppState,
    pub mode: InteractionMode,
    pub config: Config,
    pub conn: Connection,
    pub event_form_state: Option<EventFormState<'a>>,
    pub selected_event_id: Option<i64>,
    pub selected_date: NaiveDate,
    pub selected_time: NaiveTime,
    pub selection_start: Option<NaiveTime>,
}

impl<'a> App<'a> {
    pub fn new(config: Config, conn: Connection) -> App<'a> {
        let default_view = match config.ui.default_view.as_str() {
            "year" => AppState::Year,
            "month" => AppState::Month,
            "week" => AppState::Week,
            "day" => AppState::Day,
            _ => AppState::Month,
        };

        App {
            state: default_view,
            mode: InteractionMode::Navigation,
            config,
            conn,
            event_form_state: None,
            selected_event_id: None,
            selected_date: chrono::Local::now().naive_local().date(),
            selected_time: chrono::Local::now().naive_local().time(),
            selection_start: None,
        }
    }
}
