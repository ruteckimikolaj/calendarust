use crate::{
    app::App,
    storage::db::get_events_in_range,
    ui::style::{selected_style, thick_rounded_borders, PASTEL_CYAN, PASTEL_RED},
};
use chrono::{Datelike, Weekday};
use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::InteractionMode;

pub fn draw_week_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let week = app.selected_date.iso_week().week();
    let title = format!(" Year {} - Week {} ", year, week);

    let main_block = thick_rounded_borders().title(title);
    f.render_widget(main_block, area);

    let table = week_table(app);
    f.render_widget(table, area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 1 }));
}

fn week_table<'a>(app: &App) -> Table<'a> {
    let header_cells = ["Time", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(PASTEL_RED)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let year = app.selected_date.year();
    let week = app.selected_date.iso_week().week();
    let first_day_of_week = chrono::NaiveDate::from_isoywd_opt(year, week, Weekday::Mon)
        .unwrap_or(app.selected_date);
    let last_day_of_week = chrono::NaiveDate::from_isoywd_opt(year, week, Weekday::Sun)
        .unwrap_or(app.selected_date);

    let start_timestamp = first_day_of_week
        .and_hms_opt(0, 0, 0)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();
    let end_timestamp = last_day_of_week
        .and_hms_opt(23, 59, 59)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();

    let events = get_events_in_range(&app.conn, start_timestamp, end_timestamp).unwrap_or_default();
    let mut rows = vec![];
    let start_hour = app
        .config
        .calendar
        .visible_hours_start
        .split(':')
        .next()
        .and_then(|h| h.parse::<u32>().ok())
        .unwrap_or(0);
    let end_hour = app
        .config
        .calendar
        .visible_hours_end
        .split(':')
        .next()
        .and_then(|h| h.parse::<u32>().ok())
        .unwrap_or(24);

    for hour in start_hour..end_hour {
        for minute in [0, 30] {
            let current_time = chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
            let time_cell = Cell::from(format!("{:02}:{:02}", hour, minute));
            let mut cells = vec![time_cell];
            for day_offset in 0..7 {
                let current_day = first_day_of_week + chrono::Duration::days(day_offset);
                let mut event_text = String::new();
                let mut cell_style = Style::default();

                for event in &events {
                    let event_start_time = event.start_datetime.time();
                    let event_end_time = event.end_datetime.time();
                    if event.start_datetime.date_naive() == current_day
                        && current_time >= event_start_time
                        && current_time < event_end_time
                    {
                        event_text.push_str(&event.title);
                        cell_style = cell_style.bg(PASTEL_CYAN);
                    }
                }

                let is_selected = app.selected_date == current_day && app.selected_time == current_time;
                let is_in_selection_range = if let Some(start_time) = app.selection_start {
                    let (start, end) = if start_time < app.selected_time {
                        (start_time, app.selected_time)
                    } else {
                        (app.selected_time, start_time)
                    };
                    app.selected_date == current_day && current_time >= start && current_time <= end
                } else {
                    false
                };

                let final_style = if app.mode == InteractionMode::TimeSlot && is_in_selection_range {
                    selected_style()
                } else if app.mode != InteractionMode::TimeSlot && is_selected {
                    selected_style()
                } else {
                    cell_style
                };

                let event_text_with_markers = if is_selected || is_in_selection_range {
                    format!("> {} <", event_text)
                } else {
                    event_text
                };
                cells.push(Cell::from(event_text_with_markers).style(final_style));
            }
            rows.push(Row::new(cells).height(1));
        }
    }

    let constraints = vec![
        Constraint::Length(6),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
        Constraint::Percentage(13),
    ];
    Table::new(rows, constraints)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .column_spacing(1)
}
