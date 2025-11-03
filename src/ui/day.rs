use crate::{
    app::{App, InteractionMode},
    ui::style::{focused_style, selection_style, PASTEL_CYAN},
};
use chrono::{Datelike, Timelike};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
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

    let mut rows = vec![];
    for hour in start_hour..end_hour {
        let time_text = format!("{:02}:00", hour);
        let time_cell = Cell::from(time_text);

        let mut event_text = String::new();
        let mut cell_style = Style::default();

        for event in &app.events {
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

        let mut event_cell = Cell::from(event_text);
        if is_in_selection_range {
            cell_style = selection_style();
        } else if is_focused {
            cell_style = focused_style();
        }
        event_cell = event_cell.style(cell_style);

        rows.push(Row::new(vec![time_cell, event_cell]).height(1));
    }

    let constraints = [Constraint::Length(6), Constraint::Min(0)];
    let table = Table::new(rows, constraints)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(title),
        )
        .column_spacing(0);

    f.render_widget(table, area);
}
