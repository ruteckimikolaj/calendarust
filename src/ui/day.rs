use crate::{
    app::App,
    storage::db::get_events_in_range,
    ui::style::{selected_style, thick_rounded_borders, PASTEL_CYAN},
};
use chrono::Datelike;
use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::InteractionMode;

pub fn draw_day_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let month = app.selected_date.month();
    let day = app.selected_date.day();
    let title = format!(
        " {} {}, {} ",
        chrono::Month::try_from(month as u8)
            .unwrap_or(chrono::Month::January)
            .name(),
        day,
        year
    );

    let main_block = thick_rounded_borders().title(title);
    f.render_widget(main_block, area);

    let table = day_table(app);
    f.render_widget(table, area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 }));
}

fn day_table<'a>(app: &App) -> Table<'a> {
    let start_timestamp = app
        .selected_date
        .and_hms_opt(0, 0, 0)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();
    let end_timestamp = app
        .selected_date
        .and_hms_opt(23, 59, 59)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();

    let events = get_events_in_range(&app.conn, start_timestamp, end_timestamp).unwrap_or_default();
    let mut rows = vec![];

    for hour in 0..24 {
        for minute in [0, 30] {
            let current_time = chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
            let time_cell = Cell::from(format!("{:02}:{:02}", hour, minute));
            let mut event_text = String::new();
            let mut row_style = Style::default();

            for event in &events {
                let event_start_time = event.start_datetime.time();
                let event_end_time = event.end_datetime.time();
                if current_time >= event_start_time && current_time < event_end_time {
                    event_text = event.title.clone();
                    row_style = row_style.bg(PASTEL_CYAN);
                }
            }

            let is_selected = app.selected_time == current_time;
            let is_in_selection_range = if let Some(start_time) = app.selection_start {
                let (start, end) = if start_time < app.selected_time {
                    (start_time, app.selected_time)
                } else {
                    (app.selected_time, start_time)
                };
                current_time >= start && current_time <= end
            } else {
                false
            };

            let event_cell_style = if app.mode == InteractionMode::TimeSlot && is_in_selection_range {
                selected_style()
            } else if app.mode != InteractionMode::TimeSlot && is_selected {
                selected_style()
            } else {
                row_style
            };

            let event_cell = Cell::from(event_text).style(event_cell_style);
            let row = Row::new(vec![time_cell, event_cell]).height(1);
            rows.push(row);
        }
    }

    let constraints = vec![Constraint::Length(6), Constraint::Min(0)];
    Table::new(rows, constraints)
        .block(Block::default().borders(Borders::ALL))
        .column_spacing(1)
}
