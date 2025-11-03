use crate::{
    app::{App, InteractionMode},
    storage::db::get_events_in_range,
    ui::style::{focused_style, selection_style, thick_rounded_borders, PASTEL_CYAN},
};
use chrono::{Datelike, Timelike};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph, BorderType},
    Frame,
};

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
    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

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

    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(inner_area);

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

    let num_hours = end_hour - start_hour;
    let constraints = (0..num_hours)
        .map(|_| Constraint::Ratio(1, num_hours))
        .collect::<Vec<_>>();

    let time_slots_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.clone())
        .split(outer_layout[0]);

    let event_slots_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(outer_layout[1]);

    for (i, hour) in (start_hour..end_hour).enumerate() {
        let time_slot = time_slots_layout[i];
        let event_slot = event_slots_layout[i];

        let time_text = format!("{:02}:00", hour);
        let time_paragraph = Paragraph::new(time_text);
        f.render_widget(time_paragraph, time_slot);

        let mut event_text = String::new();
        let mut cell_style = Style::default();

        for event in &events {
            let event_start_hour = event.start_datetime.hour();
            if event.start_datetime.date_naive() == app.selected_date && event_start_hour == hour {
                event_text = event.title.clone();
                cell_style = cell_style.bg(PASTEL_CYAN);
            }
        }

        let is_focused = app.selected_time.hour() == hour;
        let is_in_selection_range = if app.mode == InteractionMode::TimeSlot {
            if let Some(start_time) = app.selection_start {
                let (start, end) = if start_time <= app.selected_time {
                    (start_time.hour(), app.selected_time.hour())
                } else {
                    (app.selected_time.hour(), start_time.hour())
                };
                hour >= start && hour <= end
            } else {
                false
            }
        } else {
            false
        };

        let mut block = Block::default().borders(Borders::ALL).border_type(BorderType::Plain);
        if is_focused {
            block = block.border_style(focused_style());
        }
        if is_in_selection_range {
            cell_style = selection_style();
        }

        let event_paragraph = Paragraph::new(event_text).style(cell_style);
        f.render_widget(block.clone(), event_slot);
        f.render_widget(event_paragraph, block.inner(event_slot));
    }
}
