use crate::{
    app::App,
    storage::db::get_events_in_range,
    ui::style::{selected_style, thick_rounded_borders, PASTEL_RED},
};
use chrono::{Datelike, Month, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_month_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let month = app.selected_date.month();

    let month_name = Month::try_from(month as u8)
        .unwrap_or(Month::January)
        .name();
    let title = format!("{} {}", month_name, year);

    let first_day_of_month =
        NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_else(|| app.selected_date);
    let last_day_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .and_then(|d| d.pred_opt())
            .unwrap_or(first_day_of_month)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .and_then(|d| d.pred_opt())
            .unwrap_or(first_day_of_month)
    };

    let start_timestamp = first_day_of_month
        .and_hms_opt(0, 0, 0)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();
    let end_timestamp = last_day_of_month
        .and_hms_opt(23, 59, 59)
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or_default();

    let events = get_events_in_range(&app.conn, start_timestamp, end_timestamp).unwrap_or_default();
    let mut event_days = std::collections::HashSet::new();
    for event in events {
        event_days.insert(event.start_datetime.day());
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    let header_block = Block::default().title(title).borders(Borders::NONE);
    f.render_widget(header_block, chunks[0]);

    let table = month_table(app, &event_days);
    f.render_widget(table, chunks[1]);
}

fn month_table<'a>(app: &App, event_days: &std::collections::HashSet<u32>) -> Table<'a> {
    let header_cells = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(PASTEL_RED)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let year = app.selected_date.year();
    let month = app.selected_date.month();

    let first_day =
        NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_else(|| app.selected_date);
    let weekday_of_first = first_day.weekday().num_days_from_monday();

    let mut rows = vec![];
    let mut days: Vec<Cell> = (0..weekday_of_first).map(|_| Cell::from("")).collect();

    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or(first_day)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(first_day)
    }
    .signed_duration_since(first_day)
    .num_days();

    for day in 1..=days_in_month {
        let day_str = if event_days.contains(&(day as u32)) {
            format!("{}\n•", day)
        } else {
            day.to_string()
        };
        let mut cell = Cell::from(day_str);
        if day as u32 == app.selected_date.day() {
            cell = cell.style(selected_style());
        }
        days.push(cell);
        if days.len() == 7 {
            let row = Row::new(days.drain(..)).height(4);
            rows.push(row);
        }
    }
    if !days.is_empty() {
        let remaining_len = days.len();
        for _ in 0..(7 - remaining_len) {
            days.push(Cell::from(""));
        }
        rows.push(Row::new(days.drain(..)).height(4));
    }

    let constraints = vec![Constraint::Percentage(14); 7];
    Table::new(rows, constraints)
        .header(header)
        .block(thick_rounded_borders())
        .column_spacing(1)
}
