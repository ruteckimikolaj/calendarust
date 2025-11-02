use crate::{
    app::{App, AppState, EventFormState, InteractionMode},
    models::event::Event,
    storage::db::{create_event, delete_event},
};
use chrono::{Duration, TimeZone, Utc};
use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::TextArea;

pub fn handle_input<'a>(key: KeyEvent, app: &mut App<'a>) {
    match app.mode {
        InteractionMode::Navigation => handle_navigation_input(key, app),
        InteractionMode::EventForm => handle_event_form_input(key, app),
        _ => {}
    }
}

fn handle_navigation_input(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => {
            // In a real app, you'd want to set a flag to exit the loop
        }
        KeyCode::Tab => {
            app.state = match app.state {
                AppState::Year => AppState::Month,
                AppState::Month => AppState::Week,
                AppState::Week => AppState::Day,
                AppState::Day => AppState::Year,
            }
        }
        KeyCode::Char('n') => {
            app.mode = InteractionMode::EventForm;
            app.event_form_state = Some(EventFormState {
                title: TextArea::default(),
                description: TextArea::default(),
                location: TextArea::default(),
                start_datetime: Utc::now().naive_utc(),
                end_datetime: Utc::now().naive_utc(),
                focused_field: 0,
            });
        }
        KeyCode::Char('d') => {
            if let Some(event_id) = app.selected_event_id {
                delete_event(&app.conn, event_id).unwrap();
                app.selected_event_id = None;
            }
        }
        KeyCode::Char('e') => {
            if let Some(_event_id) = app.selected_event_id {
                // In a real app, you'd fetch the event and populate the form
                app.mode = InteractionMode::EventForm;
                app.event_form_state = Some(EventFormState {
                    title: TextArea::from(vec!["Existing Event"]),
                    description: TextArea::default(),
                    location: TextArea::default(),
                    start_datetime: Utc::now().naive_utc(),
                    end_datetime: Utc::now().naive_utc(),
                    focused_field: 0,
                });
            }
        }
        KeyCode::Left => {
            app.selected_date = app.selected_date - Duration::days(1);
        }
        KeyCode::Right => {
            app.selected_date = app.selected_date + Duration::days(1);
        }
        KeyCode::Up => {
            app.selected_date = app.selected_date - Duration::weeks(1);
        }
        KeyCode::Down => {
            app.selected_date = app.selected_date + Duration::weeks(1);
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
                let new_event = Event {
                    id: 0,
                    title: form_state.title.lines().join("\n"),
                    description: Some(form_state.description.lines().join("\n")),
                    start_datetime: Utc.from_utc_datetime(&form_state.start_datetime),
                    end_datetime: Utc.from_utc_datetime(&form_state.end_datetime),
                    location: Some(form_state.location.lines().join("\n")),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                create_event(&app.conn, &new_event).unwrap();
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
