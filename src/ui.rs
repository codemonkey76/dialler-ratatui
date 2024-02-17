use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::widgets::{Borders, List, ListItem};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let items: Vec<_> = app
        .state
        .contacts
        .iter()
        .map(|contact| ListItem::new(format!("{contact}")))
        .collect();

    let [filter_area, contact_area, status_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.size());

    frame.render_widget(
        Paragraph::new(format!(" Filter: {}", app.state.filter))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        filter_area,
    );
    frame.render_widget(
        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Contact Area"),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        contact_area,
    );
    frame.render_widget(
        Paragraph::new(" Status Line".to_string())
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        status_area,
    );
    frame.set_cursor(
        filter_area.x + app.state.filter.get_cursor_position() as u16 + 9,
        filter_area.y,
    );
}
