use crate::{
    app::App,
    ui::style::{selection_style, thick_rounded_borders},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_event_form(f: &mut Frame, app: &mut App, area: Rect) {
    let popup_area = centered_rect(60, 50, area);
    let block = thick_rounded_borders().title(" Create Event ");
    f.render_widget(Clear, popup_area);
    f.render_widget(block, popup_area);

    if let Some(form_state) = &mut app.event_form_state {
        let form_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(popup_area);

        let datetime_str = format!(
            "Date: {} | Time: {} - {}",
            form_state.start_datetime.date(),
            form_state.start_datetime.time(),
            form_state.end_datetime.time()
        );
        let datetime_paragraph = Paragraph::new(datetime_str);
        f.render_widget(datetime_paragraph, form_chunks[0]);

        let title_block = Block::default().borders(Borders::ALL).title(" Title ");
        let description_block = Block::default().borders(Borders::ALL).title(" Description ");
        let location_block = Block::default().borders(Borders::ALL).title(" Location ");

        form_state.title.set_block(if form_state.focused_field == 0 {
            title_block.style(selection_style())
        } else {
            title_block
        });
        form_state.description.set_block(if form_state.focused_field == 1 {
            description_block.style(selection_style())
        } else {
            description_block
        });
        form_state.location.set_block(if form_state.focused_field == 2 {
            location_block.style(selection_style())
        } else {
            location_block
        });

        f.render_widget(&form_state.title, form_chunks[1]);
        f.render_widget(&form_state.description, form_chunks[2]);
        f.render_widget(&form_state.location, form_chunks[3]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
