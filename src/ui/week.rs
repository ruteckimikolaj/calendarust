use crate::{
    app::{App, InteractionMode},
    ui::style::{focused_style, normal_style, selection_style, PASTEL_CYAN, PASTEL_RED},
};
use chrono::{Datelike, Local, Timelike, Weekday};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Cell, Row, Table, BorderType},
    Frame,
};

pub fn draw_week_view(f: &mut Frame, app: &mut App, area:Rect) {
    let year = app.selected_date.year();
    let week = app.selected_date.iso_week().week();
    let title = format!(" Year {} - Week {} ", year, week);

    let first_day_of_week =
        chrono::NaiveDate::from_isoywd_opt(year, week, Weekday::Mon).unwrap_or(app.selected_date);

    let header_cells = ["", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(PASTEL_RED).bold()));
    let header = Row::new(header_cells).height(1);

    let start_hour = app.visible_start_hour;
    let end_hour = app.visible_end_hour;

    let mut rows = vec![];
    for hour in start_hour..end_hour {
        for minute in [0, 30].iter() {
            let mut row_cells =
                vec![Cell::from(format!("{:02}:{:02}", hour, minute)).style(normal_style())];
            for day_index in 0..7 {
                let current_day = first_day_of_week + chrono::Duration::days(day_index);

                let mut event_text = String::new();
                let mut cell_style = normal_style();

                for event in &app.events {
                    let local_start_time = event.start_datetime.with_timezone(&Local);
                    if local_start_time.date_naive() == current_day
                        && local_start_time.hour() == hour
                        && local_start_time.minute() == *minute
                    {
                        event_text = event.title.clone();
                        cell_style = cell_style.bg(PASTEL_CYAN);
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

            let is_focused = app.selected_date == current_day
                && app.selected_time.hour() == hour
                && app.selected_time.minute() == *minute;
            if is_focused {
                cell_style = cell_style.patch(focused_style());
            }
            if is_in_selection_range {
                cell_style = cell_style.patch(selection_style());
            }

            row_cells.push(Cell::from(event_text).style(cell_style));
        }
        rows.push(Row::new(row_cells).height(1));
        }
    }

    let constraints = [
        Constraint::Length(6),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
        Constraint::Ratio(1, 7),
    ];
    let table = Table::new(rows, constraints)
        .header(header)
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
