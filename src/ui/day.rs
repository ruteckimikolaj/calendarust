use crate::{
    app::{App, InteractionMode},
    ui::style::{event_style, focused_style, normal_style, selection_style},
};
use chrono::{Datelike, Local, Timelike};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table, BorderType},
    Frame,
};

pub fn draw_day_view(f: &mut Frame, app: &mut App, area: Rect) {
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

    let start_hour = app.visible_start_hour;
    let end_hour = app.visible_end_hour;

    let mut rows = vec![];
    for hour in start_hour..end_hour {
        for minute in [0, 30].iter() {
            let time_text = format!("{:02}:{:02}", hour, minute);
            let time_cell = Cell::from(time_text).style(normal_style());

            let mut event_text = String::new();
            let mut cell_style = normal_style();

            for event in &app.events {
                let local_start_time = event.start_datetime.with_timezone(&Local);
                let event_start_hour = local_start_time.hour();
                let event_start_minute = local_start_time.minute();
                if local_start_time.date_naive() == app.selected_date
                    && event_start_hour == hour
                    && event_start_minute == *minute
                {
                    event_text = event.title.clone();
                    cell_style = event_style();
                }
            }

            let is_in_selection_range = if app.mode == InteractionMode::TimeSlot {
                if let Some(start_time) = app.selection_start {
                    let start = start_time.hour() * 60 + start_time.minute();
                    let end = app.selected_time.hour() * 60 + app.selected_time.minute();
                    let current = hour * 60 + minute;
                    if start <= end {
                        current >= start && current <= end
                    } else {
                        current >= end && current <= start
                    }
                } else {
                    false
                }
            } else {
                false
            };

        let is_focused =
            app.selected_time.hour() == hour && app.selected_time.minute() == *minute;
        if is_focused {
            cell_style = cell_style.patch(focused_style());
        }
        if is_in_selection_range {
            cell_style = cell_style.patch(selection_style());
        }
        if is_focused {
            cell_style = cell_style.patch(focused_style());
        }
        let event_cell = Cell::from(event_text).style(cell_style);

        rows.push(Row::new(vec![time_cell, event_cell]).height(1));
        }
    }

    let constraints = [Constraint::Length(6), Constraint::Min(0)];
    let table = Table::new(rows, constraints)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Gray))
                .title(title),
        )
        .column_spacing(0);

    f.render_widget(table, area);
}
