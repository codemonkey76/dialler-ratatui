use crate::app::App;
use crate::dialog::dialog_field::DialogField;
use ratatui::prelude::{
    Alignment, Color, Constraint, Direction, Layout, Line, Margin, Rect, Span, Style, Stylize, Text,
};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub struct Renderer;

impl Renderer {
    pub fn render_add_contact_modal(app: &mut App, frame: &mut Frame) {
        let size = frame.size();

        let center_area = get_center_area((40, 14), size);

        let centered_box = Block::default()
            .title("Add Contact")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan).bg(Color::Black));

        frame.render_widget(centered_box, center_area);

        let center = center_area.inner(&Margin::new(2, 1));

        let field_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(3),
            ])
            .split(center);

        let max_label = app.state.modal.get_max_label();

        draw_fields_in_rects(
            frame,
            &app.state.modal.fields,
            &field_areas[1..],
            max_label,
            app.state.modal.focused_index,
        );
    }

    pub fn render_delete_confirmation_modal(app: &mut App, frame: &mut Frame) {
        let size = frame.size();

        let center_area = get_center_area((10, 5), size);

        frame.render_widget(
            Block::default().title("Delete?").borders(Borders::ALL),
            center_area,
        );
    }

    pub fn render_main_window(app: &mut App, frame: &mut Frame) {
        let items: Vec<_> = app
            .state
            .contacts
            .iter()
            .enumerate()
            .map(|(index, contact)| {
                let mut style = Style::default();

                if app.state.selected_contact_index == index {
                    style = style.fg(Color::Black).bg(Color::Cyan);
                } else {
                    style = style.fg(Color::Cyan).bg(Color::Black);
                }
                ListItem::new(format!("{contact}")).style(style)
            })
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
                        .title(" Contact Area"),
                )
                .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
            contact_area,
        );

        let style = Style::default().fg(Color::Gray).bg(Color::Black).bold();
        let cyan = Style::default().fg(Color::Gray).bg(Color::Gray).bold();
        let reversed = Style::default().fg(Color::Black).bg(Color::Gray).bold();
        let red = Style::default().fg(Color::Red).bg(Color::Gray).bold();

        let line = Line::from(vec![
            Span::styled("Ctrl + ", style.bold()),
            Span::styled("", reversed),
            Span::styled("", cyan),
            Span::styled("<", reversed),
            Span::styled("a", red),
            Span::styled("> ADD ", reversed),
            Span::styled("", style),
            Span::styled("", reversed),
            Span::styled(" <", reversed),
            Span::styled("e", red),
            Span::styled("> EDIT ", reversed),
            Span::styled("", style),
            Span::styled("", reversed),
            Span::styled(" <", reversed),
            Span::styled("d", red),
            Span::styled("> DELETE ", reversed),
            Span::styled("", style),
        ]);

        frame.render_widget(Paragraph::new(line), status_area);
        frame.set_cursor(
            filter_area.x + app.state.filter.get_cursor_position() as u16 + 9,
            filter_area.y,
        );
    }
}

fn get_center_area(dimensions: (u16, u16), size: Rect) -> Rect {
    let margin_x = (size.width.saturating_sub(dimensions.0) / 2).max(1);
    let margin_y = (size.height.saturating_sub(dimensions.1) / 2).max(1);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(margin_x),
            Constraint::Min(dimensions.0),
            Constraint::Length(margin_x),
        ])
        .split(size);

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(margin_y),
            Constraint::Min(dimensions.1),
            Constraint::Length(margin_y),
        ])
        .split(horiz[1]);

    areas[1]
}

fn get_middle_split(direction: Direction, dimensions: (u16, u16, u16), size: Rect) -> Rect {
    let areas = Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Length(dimensions.0),
            Constraint::Min(dimensions.1),
            Constraint::Length(dimensions.2),
        ])
        .split(size);

    areas[1]
}

fn draw_field_in_rect(frame: &mut Frame, field: &DialogField, label_area: Rect, input_area: Rect) {
    let value = field.get_value();
    frame.render_widget(
        Paragraph::new(format!("{}: ", field.label))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Right),
        label_area,
    );
    let mut style = Style::default();

    frame.render_widget(
        Paragraph::new(format!("{}", value))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black)),
        input_area,
    );
}

fn draw_fields_in_rects(
    frame: &mut Frame,
    fields: &[DialogField],
    areas: &[Rect],
    max_label: u16,
    focused_index: usize,
) {
    for (i, field) in fields.iter().enumerate() {
        if i < areas.len() {
            let rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(max_label + 2), Constraint::Min(1)])
                .split(areas[i]);
            draw_field_in_rect(frame, field, rects[0], rects[1]);
            if i == focused_index {
                frame.set_cursor(rects[1].x + field.get_cursor_pos(), rects[1].y);
            }
        } else {
            panic!("Can't draw field, no area to draw it in");
        }
    }
}
