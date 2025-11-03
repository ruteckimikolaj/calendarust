use crate::{models::config::load_config, models::event::Event};
use chrono::{Datelike, NaiveDate, NaiveTime};
use rusqlite::Connection;
use tui_textarea::TextArea;

#[derive(PartialEq, Eq)]
pub enum AppState {
    Year,
    Month,
    Week,
    Day,
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
    pub conn: Connection,
    pub event_form_state: Option<EventFormState<'a>>,
    pub selected_event_id: Option<i64>,
    pub selected_date: NaiveDate,
    pub selected_time: NaiveTime,
    pub selection_start: Option<NaiveTime>,
    pub events: Vec<Event>,
    pub visible_start_hour: u32,
    pub visible_end_hour: u32,
}

impl<'a> App<'a> {
    pub fn new(conn: Connection) -> App<'a> {
        let config = load_config().unwrap();
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
            conn,
            event_form_state: None,
            selected_event_id: None,
            selected_date: chrono::Local::now().naive_local().date(),
            selected_time: chrono::Local::now().naive_local().time(),
            selection_start: None,
            events: vec![],
            visible_start_hour: 6,
            visible_end_hour: 18,
        }
    }

    pub fn scroll_up(&mut self) {
        if self.visible_start_hour > 0 {
            self.visible_start_hour -= 1;
            self.visible_end_hour -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.visible_end_hour < 24 {
            self.visible_start_hour += 1;
            self.visible_end_hour += 1;
        }
    }

    pub fn load_events(&mut self) {
        let (start_timestamp, end_timestamp) = match self.state {
            AppState::Day => {
                let start = self.selected_date.and_hms_opt(0, 0, 0).unwrap();
                let end = self.selected_date.and_hms_opt(23, 59, 59).unwrap();
                (
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp(),
                )
            }
            AppState::Week => {
                let week = self.selected_date.iso_week().week();
                let year = self.selected_date.year();
                let start = chrono::NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = chrono::NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Sun)
                    .unwrap()
                    .and_hms_opt(23, 59, 59)
                    .unwrap();
                (
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp(),
                )
            }
            AppState::Month => {
                let month = self.selected_date.month();
                let year = self.selected_date.year();
                let start = chrono::NaiveDate::from_ymd_opt(year, month, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = if month == 12 {
                    chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
                } else {
                    chrono::NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
                }
                .pred_opt()
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap();
                (
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp(),
                )
            }
            AppState::Year => {
                let year = self.selected_date.year();
                let start = chrono::NaiveDate::from_ymd_opt(year, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap();
                let end = chrono::NaiveDate::from_ymd_opt(year, 12, 31)
                    .unwrap()
                    .and_hms_opt(23, 59, 59)
                    .unwrap();
                (
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp(),
                )
            }
        };

        if start_timestamp > 0 {
            self.events =
                crate::storage::db::get_events_in_range(&self.conn, start_timestamp, end_timestamp)
                    .unwrap_or_default();
        } else {
            self.events = vec![];
        }
    }
}
