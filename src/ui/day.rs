use crate::{
    app::App,
    storage::db::get_events_in_range,
    ui::style::{selected_style, thick_rounded_borders, PASTEL_CYAN, PASTEL_RED},
};
use chrono::{Datelike, Timelike};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_day_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let month = app.selected_date.month();
    let day = app.selected_date.day();
    let title = format!(
        "{} {}, {}",
        chrono::Month::try_from(month as u8)
            .unwrap_or(chrono::Month::January)
            .name(),
        day,
        year
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    let header_block = Block::default().title(title).borders(Borders::NONE);
    f.render_widget(header_block, chunks[0]);

    let table = day_table(app);
    f.render_widget(table, chunks[1]);
}

fn day_table<'a>(app: &App) -> Table<'a> {
    let header_cells = ["Time", "Event"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(PASTEL_RED)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

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
            let time_cell = Cell::from(format!("{:02}:{:02}", hour, minute));
            let mut event_text = String::new();
            let mut row_style = Style::default();

            for event in &events {
                let event_start_time = event.start_datetime.time();
                let event_end_time = event.end_datetime.time();
                if let Some(current_time) = chrono::NaiveTime::from_hms_opt(hour, minute, 0) {
                    if current_time >= event_start_time && current_time < event_end_time {
                        event_text = event.title.clone();
                        row_style = row_style.bg(PASTEL_CYAN);
                    }
                }
            }
            let event_cell = Cell::from(event_text);
            let mut row = Row::new(vec![time_cell, event_cell]).height(2);
            if hour == app.selected_time.hour() && minute == app.selected_time.minute() {
                row = row.style(selected_style());
            } else {
                row = row.style(row_style);
            }
            rows.push(row);
        }
    }

    let constraints = vec![Constraint::Length(6), Constraint::Percentage(90)];
    Table::new(rows, constraints)
        .header(header)
        .block(thick_rounded_borders())
        .column_spacing(1)
}
