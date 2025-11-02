use crate::app::App;
use chrono::{Datelike, Month, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw_year_view(f: &mut Frame, app: &App, area: Rect) {
    let year = app.selected_date.year();
    let title = format!("Year {}", year);

    let block = Block::default().title(title).borders(Borders::ALL);
    f.render_widget(block, area);

    let inner_area = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, 4); 4])
        .split(inner_area);

    for (i, chunk) in chunks.iter().enumerate() {
        let month_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 3); 3])
            .split(*chunk);

        for (j, month_chunk) in month_chunks.iter().enumerate() {
            let month_index = (i * 3 + j + 1) as u32;
            let month_table = mini_month_table(year, month_index);
            f.render_widget(month_table, *month_chunk);
        }
    }
}

fn mini_month_table<'a>(year: i32, month: u32) -> Table<'a> {
    let month_name = Month::try_from(month as u8)
        .unwrap_or(Month::January)
        .name();
    let header_cells = ["M", "T", "W", "T", "F", "S", "S"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).height(1);

    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_default();
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
        days.push(Cell::from(day.to_string()));
        if days.len() == 7 {
            let row = Row::new(days.drain(..));
            rows.push(row);
        }
    }
    if !days.is_empty() {
        let remaining_len = days.len();
        for _ in 0..(7 - remaining_len) {
            days.push(Cell::from(""));
        }
        rows.push(Row::new(days.drain(..)));
    }

    Table::new(rows, vec![Constraint::Length(2); 7])
        .header(header)
        .block(
            Block::default()
                .title(month_name)
                .borders(Borders::ALL),
        )
}
