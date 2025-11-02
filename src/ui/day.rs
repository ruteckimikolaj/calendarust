use crate::{app::App, storage::db::get_events_in_range};
use chrono::{Datelike, Timelike};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_day_view(f: &mut Frame, app: &App, area:Rect) {
    let year = app.selected_date.year();
    let month = app.selected_date.month();
    let day = app.selected_date.day();
    let title = format!(
        "{} {}, {}",
        chrono::Month::try_from(month as u8).unwrap().name(),
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
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);

    let now = chrono::Local::now().naive_local();
    let start_timestamp = now.date().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let end_timestamp = now
        .date()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc()
        .timestamp();

    let events = get_events_in_range(&app.conn, start_timestamp, end_timestamp).unwrap_or_default();

    let mut rows = vec![];

    for hour in 0..24 {
        for minute in [0, 30] {
            let time_cell = Cell::from(format!("{:02}:{:02}", hour, minute));
            let mut event_text = String::new();
            for event in &events {
                let event_start_time = event.start_datetime.time();
                if event_start_time.hour() == hour && event_start_time.minute() == minute {
                    event_text.push_str(&event.title);
                }
            }
            let event_cell = Cell::from(event_text);
            rows.push(Row::new(vec![time_cell, event_cell]).height(2));
        }
    }

    let constraints = vec![Constraint::Length(6), Constraint::Percentage(90)];
    Table::new(rows, constraints)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .column_spacing(1)
}
