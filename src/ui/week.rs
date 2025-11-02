use crate::{app::App, storage::db::get_events_in_range};
use chrono::{Datelike, Timelike, Weekday};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_week_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let week = app.selected_date.iso_week().week();
    let title = format!("Year {} - Week {}", year, week);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    let header_block = Block::default().title(title).borders(Borders::NONE);
    f.render_widget(header_block, chunks[0]);

    let table = week_table(app);
    f.render_widget(table, chunks[1]);
}

fn week_table<'a>(app: &App) -> Table<'a> {
    let header_cells = ["Time", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Blue))
        .height(1)
        .bottom_margin(1);

    let now = chrono::Local::now().naive_local();
    let year = now.year();
    let week = now.iso_week().week();
    let first_day_of_week =
        chrono::NaiveDate::from_isoywd_opt(year, week, Weekday::Mon).unwrap();
    let last_day_of_week =
        chrono::NaiveDate::from_isoywd_opt(year, week, Weekday::Sun).unwrap();

    let start_timestamp = first_day_of_week
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    let end_timestamp = last_day_of_week
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc()
        .timestamp();

    let events = get_events_in_range(&app.conn, start_timestamp, end_timestamp).unwrap_or_default();

    let mut rows = vec![];
    let start_hour = app
        .config
        .calendar
        .visible_hours_start
        .split(':')
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(6);
    let end_hour = app
        .config
        .calendar
        .visible_hours_end
        .split(':')
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(18);

    for hour in start_hour..end_hour {
        for minute in [0, 30] {
            let time_cell = Cell::from(format!("{:02}:{:02}", hour, minute));
            let mut cells = vec![time_cell];
            for day_offset in 0..7 {
                let current_day = first_day_of_week + chrono::Duration::days(day_offset);
                let mut event_text = String::new();
                for event in &events {
                    let event_start_time = event.start_datetime.time();
                    if event.start_datetime.date_naive() == current_day
                        && event_start_time.hour() == hour
                        && event_start_time.minute() == minute
                    {
                        event_text.push_str(&event.title);
                    }
                }
                cells.push(Cell::from(event_text));
            }
            rows.push(Row::new(cells).height(2));
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
