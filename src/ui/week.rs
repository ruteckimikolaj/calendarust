use crate::{
    app::{App, InteractionMode},
    storage::db::get_events_in_range,
    ui::style::{focused_style, selection_style, thick_rounded_borders, PASTEL_CYAN, PASTEL_RED},
};
use chrono::{Datelike, Weekday};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_week_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let week = app.selected_date.iso_week().week();
    let title = format!(" Year {} - Week {} ", year, week);

    let main_block = thick_rounded_borders().title(title);
    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

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

    let header_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6), // Gutter for time
            Constraint::Min(0),      // Space for days
        ])
        .split(inner_area);

    let day_headers_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 7); 7])
        .split(header_layout[1]);

    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (i, day) in days.iter().enumerate() {
        let p = Paragraph::new(*day).style(Style::default().fg(PASTEL_RED).bold());
        f.render_widget(p, day_headers_layout[i]);
    }

    let grid_area = Rect {
        y: inner_area.y + 1,
        height: inner_area.height - 1,
        ..inner_area
    };

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(6),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
        ])
        .split(grid_area);

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

    let time_slots_count = ((end_hour - start_hour) * 2) as usize;
    let time_slots_constraints = vec![Constraint::Length(1); time_slots_count];

    for day_index in 0..=7 {
        let column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(time_slots_constraints.clone())
            .split(outer_layout[day_index]);

        for i in 0..time_slots_count {
            let slot_area = column_layout[i];
            let hour = start_hour + (i as u32 / 2);
            let minute = (i as u32 % 2) * 30;
            let current_time =
                chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, 0).unwrap();

            if day_index == 0 { // Time column
                let time_text = format!("{:02}:{:02}", hour, minute);
                let time_paragraph = Paragraph::new(time_text);
                f.render_widget(time_paragraph, slot_area);
            } else { // Day columns
                let current_day = first_day_of_week + chrono::Duration::days(day_index as i64 - 1);

                let mut event_text = String::new();
                let mut cell_style = Style::default();

                for event in &events {
                    let event_start_time = event.start_datetime.time();
                    let event_end_time = event.end_datetime.time();
                    if event.start_datetime.date_naive() == current_day && current_time >= event_start_time && current_time < event_end_time {
                        event_text = event.title.clone();
                        cell_style = cell_style.bg(PASTEL_CYAN);
                    }
                }

                let is_focused = app.selected_date == current_day && app.selected_time == current_time;

                let is_in_selection_range = if app.mode == InteractionMode::TimeSlot {
                    if let Some(start_time) = app.selection_start {
                        let (start, end) = if start_time <= app.selected_time {
                            (start_time, app.selected_time)
                        } else {
                            (app.selected_time, start_time)
                        };
                         app.selected_date == current_day && current_time >= start && current_time <= end
                    } else { false }
                } else { false };

                let mut block = Block::default().borders(Borders::ALL);
                if is_focused {
                    block = block.border_style(focused_style());
                }
                if is_in_selection_range {
                    cell_style = selection_style();
                }

                let event_paragraph = Paragraph::new(event_text).style(cell_style);
                f.render_widget(block.clone(), slot_area);
                f.render_widget(event_paragraph, block.inner(slot_area));
            }
        }
    }
}
