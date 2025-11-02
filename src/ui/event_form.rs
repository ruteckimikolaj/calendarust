use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear},
    Frame,
};

pub fn draw_event_form(f: &mut Frame, app: &mut App, area: Rect) {
    let popup_area = centered_rect(60, 50, area);
    let block = Block::default().title("Create Event").borders(Borders::ALL);
    f.render_widget(Clear, popup_area); // this clears the area behind the popup
    f.render_widget(block.clone(), popup_area);

    if let Some(form_state) = &mut app.event_form_state {
        let form_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(popup_area);

        form_state.title.set_block(Block::default().borders(Borders::ALL).title("Title"));
        form_state.description.set_block(Block::default().borders(Borders::ALL).title("Description"));
        form_state.location.set_block(Block::default().borders(Borders::ALL).title("Location"));

        f.render_widget(form_state.title.widget(), form_chunks[0]);
        f.render_widget(form_state.description.widget(), form_chunks[1]);
        f.render_widget(form_state.location.widget(), form_chunks[2]);
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
