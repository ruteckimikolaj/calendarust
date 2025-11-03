use crate::{
    app::{App, AppState, EventFormState, InteractionMode},
    models::event::Event,
    storage::db::{create_event, delete_event, get_events_in_range, update_event},
};
use chrono::{Datelike, Duration, TimeZone, Utc};
use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::TextArea;

pub fn handle_input<'a>(key: KeyEvent, app: &mut App<'a>) {
    if key.code == KeyCode::Tab {
        app.state = match app.state {
            AppState::Year => AppState::Month,
            AppState::Month => AppState::Week,
            AppState::Week => AppState::Day,
            AppState::Day => AppState::Year,
        };
        return;
    }
    match app.mode {
        InteractionMode::Navigation => handle_navigation_input(key, app),
        InteractionMode::Selection => handle_selection_input(key, app),
        InteractionMode::TimeSlot => handle_timeslot_input(key, app),
        InteractionMode::EventForm => handle_event_form_input(key, app),
    }
}

fn handle_navigation_input(key: KeyEvent, app: &mut App) {
    match app.state {
        AppState::Year => match key.code {
            KeyCode::Enter => app.mode = InteractionMode::Selection,
            KeyCode::Left => {
                app.selected_date = app
                    .selected_date
                    .with_month(app.selected_date.month() - 1)
                    .unwrap_or_else(|| {
                        app.selected_date
                            .with_year(app.selected_date.year() - 1)
                            .unwrap()
                            .with_month(12)
                            .unwrap()
                    })
            }
            KeyCode::Right => {
                app.selected_date = app
                    .selected_date
                    .with_month(app.selected_date.month() + 1)
                    .unwrap_or_else(|| {
                        app.selected_date
                            .with_year(app.selected_date.year() + 1)
                            .unwrap()
                            .with_month(1)
                            .unwrap()
                    })
            }
            KeyCode::Up => {
                app.selected_date = app
                    .selected_date
                    .with_year(app.selected_date.year() - 1)
                    .unwrap_or(app.selected_date)
            }
            KeyCode::Down => {
                app.selected_date = app
                    .selected_date
                    .with_year(app.selected_date.year() + 1)
                    .unwrap_or(app.selected_date)
            }
            _ => {}
        },
        AppState::Month => match key.code {
            KeyCode::Enter => app.mode = InteractionMode::Selection,
            KeyCode::Left => app.selected_date -= Duration::days(1),
            KeyCode::Right => app.selected_date += Duration::days(1),
            KeyCode::Up => app.selected_date -= Duration::weeks(1),
            KeyCode::Down => app.selected_date += Duration::weeks(1),
            _ => {}
        },
        AppState::Week | AppState::Day => match key.code {
            KeyCode::Enter => app.mode = InteractionMode::Selection,
            KeyCode::Left => app.selected_date -= Duration::days(1),
            KeyCode::Right => app.selected_date += Duration::days(1),
            KeyCode::Up => app.selected_time = app.selected_time.overflowing_sub_signed(Duration::minutes(30)).0,
            KeyCode::Down => app.selected_time = app.selected_time.overflowing_add_signed(Duration::minutes(30)).0,
            _ => {}
        },
    }
}

fn handle_selection_input(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => {
            app.mode = InteractionMode::Navigation;
        }
        KeyCode::Enter => match app.state {
            AppState::Year => {
                app.state = AppState::Month;
                app.mode = InteractionMode::Navigation;
            }
            AppState::Month => {
                app.state = AppState::Day;
                app.mode = InteractionMode::Navigation;
            }
            AppState::Week | AppState::Day => {
                app.mode = InteractionMode::TimeSlot;
                app.selection_start = Some(app.selected_time);
            }
        },
        KeyCode::Left => match app.state {
            AppState::Year => {
                app.selected_date = app
                    .selected_date
                    .with_month(app.selected_date.month() - 1)
                    .unwrap_or_else(|| {
                        app.selected_date
                            .with_year(app.selected_date.year() - 1)
                            .unwrap()
                            .with_month(12)
                            .unwrap()
                    })
            }
            AppState::Month => app.selected_date -= Duration::days(1),
            _ => {}
        },
        KeyCode::Right => match app.state {
            AppState::Year => {
                app.selected_date = app
                    .selected_date
                    .with_month(app.selected_date.month() + 1)
                    .unwrap_or_else(|| {
                        app.selected_date
                            .with_year(app.selected_date.year() + 1)
                            .unwrap()
                            .with_month(1)
                            .unwrap()
                    })
            }
            AppState::Month => app.selected_date += Duration::days(1),
            _ => {}
        },
        KeyCode::Up => match app.state {
            AppState::Year => {
                app.selected_date = app
                    .selected_date
                    .with_year(app.selected_date.year() - 3)
                    .unwrap_or(app.selected_date)
            }
            AppState::Month => app.selected_date -= Duration::weeks(1),
            AppState::Week | AppState::Day => {
                app.selected_time = app
                    .selected_time
                    .overflowing_sub_signed(Duration::minutes(30))
                    .0
            }
        },
        KeyCode::Down => match app.state {
            AppState::Year => {
                app.selected_date = app
                    .selected_date
                    .with_year(app.selected_date.year() + 3)
                    .unwrap_or(app.selected_date)
            }
            AppState::Month => app.selected_date += Duration::weeks(1),
            AppState::Week | AppState::Day => {
                app.selected_time = app
                    .selected_time
                    .overflowing_add_signed(Duration::minutes(30))
                    .0
            }
        },
        KeyCode::Char('e') | KeyCode::Char('d') => {
            let start_of_slot = app.selected_date.and_time(app.selected_time);
            let end_of_slot = start_of_slot + Duration::minutes(30);
            let start_timestamp = start_of_slot.and_utc().timestamp();
            let end_timestamp = end_of_slot.and_utc().timestamp();

            if let Ok(events) = get_events_in_range(&app.conn, start_timestamp, end_timestamp) {
                if let Some(event) = events.first() {
                    if key.code == KeyCode::Char('e') {
                        app.mode = InteractionMode::EventForm;
                        app.event_form_state = Some(EventFormState {
                            title: TextArea::from(event.title.lines().map(|s| s.to_string())),
                            description: TextArea::from(
                                event.description.as_deref().unwrap_or("").lines().map(|s| s.to_string()),
                            ),
                            location: TextArea::from(
                                event.location.as_deref().unwrap_or("").lines().map(|s| s.to_string()),
                            ),
                            start_datetime: event.start_datetime.naive_utc(),
                            end_datetime: event.end_datetime.naive_utc(),
                            focused_field: 0,
                        });
                        app.selected_event_id = event.id;
                    } else if key.code == KeyCode::Char('d') {
                        if let Some(id) = event.id {
                            let _ = delete_event(&app.conn, id);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_timeslot_input(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => {
            app.mode = InteractionMode::Selection;
            app.selection_start = None;
        }
        KeyCode::Up => {
            app.selected_time = app.selected_time.overflowing_sub_signed(Duration::minutes(30)).0;
        }
        KeyCode::Down => {
            app.selected_time = app.selected_time.overflowing_add_signed(Duration::minutes(30)).0;
        }
        KeyCode::Enter => {
            if let Some(start_time) = app.selection_start {
                app.mode = InteractionMode::EventForm;
                let (start, end) = if start_time < app.selected_time {
                    (start_time, app.selected_time)
                } else {
                    (app.selected_time, start_time)
                };
                app.event_form_state = Some(EventFormState {
                    title: TextArea::default(),
                    description: TextArea::default(),
                    location: TextArea::default(),
                    start_datetime: app.selected_date.and_time(start),
                    end_datetime: app.selected_date.and_time(end) + Duration::minutes(30),
                    focused_field: 0,
                });
                app.selection_start = None;
            }
        }
        _ => {}
    }
}

fn handle_event_form_input<'a>(key: KeyEvent, app: &mut App<'a>) {
    if let Some(form_state) = &mut app.event_form_state {
        match key.code {
            KeyCode::Esc => {
                app.mode = InteractionMode::Navigation;
                app.event_form_state = None;
            }
            KeyCode::Tab => {
                form_state.focused_field = (form_state.focused_field + 1) % 3;
            }
            KeyCode::Enter => {
                let event = Event {
                    id: app.selected_event_id,
                    title: form_state.title.lines().join("\n"),
                    description: Some(form_state.description.lines().join("\n")),
                    start_datetime: Utc.from_utc_datetime(&form_state.start_datetime),
                    end_datetime: Utc.from_utc_datetime(&form_state.end_datetime),
                    location: Some(form_state.location.lines().join("\n")),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                if event.id.is_some() {
                    let _ = update_event(&app.conn, &event);
                } else {
                    let _ = create_event(&app.conn, &event);
                }
                app.mode = InteractionMode::Navigation;
                app.event_form_state = None;
            }
            _ => {
                let key_event: tui_textarea::Input = key.into();
                match form_state.focused_field {
                    0 => form_state.title.input(key_event),
                    1 => form_state.description.input(key_event),
                    2 => form_state.location.input(key_event),
                    _ => false,
                };
            }
        }
    }
}
