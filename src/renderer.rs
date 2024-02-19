use crate::app::App;
use crate::dialog::dialog_field::DialogField;
use ratatui::prelude::{
    Alignment, Color, Constraint, Direction, Layout, Line, Margin, Rect, Span, Style, Stylize,
};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap, ScrollbarOrientation, Scrollbar, ScrollbarState, ListState};
use ratatui::Frame;
use ratatui::symbols::scrollbar;
use tracing::info;

pub struct Renderer;

impl Renderer {

    pub fn render_edit_contact_modal(app: &mut App, frame: &mut Frame) {
        Self::render_contact_modal("Edit Contact", app, frame);
    }

    pub fn render_add_contact_modal(app: &mut App, frame: &mut Frame) {
        Self::render_contact_modal("Add Contact", app, frame);
    }

    pub fn render_contact_modal(title: impl Into<String>, app: &mut App, frame: &mut Frame) {
        let size = frame.size();

        let center_area = get_center_area((40, 14), size);

        let centered_box = Block::default()
            .title(title.into())
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

    pub fn render_delete_confirmation_modal(_: &mut App, frame: &mut Frame) {
        let size = frame.size();

        let center_area = get_center_area((25, 7), size);

        frame.render_widget(
            Block::default().title("Delete?").borders(Borders::ALL).style(
                Style::default().fg(Color::Red).bg(Color::Black),
            ),
            center_area,
        );

        let style = Style::default().fg(Color::Gray).bg(Color::Black);
        let red = style.fg(Color::Red);
        let white = style.fg(Color::White);

        let line = Line::from(vec![
            Span::styled("Are you sure you want to delete this contact? (", style),
            Span::styled("y", red),
            Span::styled("/", style),
            Span::styled("n", white),
            Span::styled(")", style),

        ]);

        let question_area = center_area.inner(&Margin::new(2, 2));
        frame.render_widget(
            Paragraph::new(line)
                .wrap(Wrap::default()),
            question_area
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

        let [status_area, quit_message] = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Min(0),
            Constraint::Length(10),
        ]).areas(status_area);


        frame.render_widget(
            Paragraph::new(format!(" Filter: {}", app.state.filter))
                .style(Style::default().fg(Color::Magenta).bg(Color::Black)),
            filter_area,
        );

        let block_style = Style::default().fg(Color::Cyan).bg(Color::Black);

        let [contact_area, scrollbar] = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ]).areas(contact_area);

        let mut list_state = ListState::default();
        list_state.select(Some(app.state.selected_contact_index));
        frame.render_stateful_widget(
            List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(" Contacts"),
                )
                .style(block_style),
            contact_area,
            &mut list_state,
        );
        let mut state = ScrollbarState::new(app.state.contacts.len());
        state = state.position(app.state.selected_contact_index);

        frame.render_stateful_widget(Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight).symbols(scrollbar::VERTICAL), scrollbar, &mut state);

        let mut spans = vec![Span::styled("Ctrl + ", Style::default().fg(Color::Gray).bg(Color::Black).bold())];

        let mut include_text = true;
        if status_area.width < 58 {
            include_text = false;
        }

        spans.extend(construct_span("Add", 'a', include_text));
        spans.extend(construct_span("Edit", 'e', include_text));
        spans.extend(construct_span("Delete", 'd', include_text));
        spans.extend(construct_span("Call", 'c', include_text));

        let line = Line::from(spans);

        info!("status_width: {}", status_area.width);


        frame.render_widget(Paragraph::new(line), status_area);
        frame.render_widget(Paragraph::new("ESC = Quit"), quit_message);
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

fn construct_span(text: &str, short_code: char, include_text: bool) -> Vec<Span>{
    let mut spans = vec![];
    let style = Style::default().fg(Color::Gray).bg(Color::Black).bold();

    let gray = style.bg(Color::Gray);
    let reversed = gray.fg(Color::Black);
    let red = gray.fg(Color::Red);

    spans.push(Span::styled("", reversed));
    spans.push(Span::styled("", gray));
    spans.push(Span::styled("<", reversed));
    spans.push(Span::styled(short_code.to_string(), red));
    spans.push(Span::styled("> ", reversed));
    if include_text {
        spans.push(Span::styled(format!("{text} "), reversed));
    }
    spans.push(Span::styled("", style));

    spans
}

fn draw_field_in_rect(frame: &mut Frame, field: &DialogField, label_area: Rect, input_area: Rect) {
    let value = field.get_value();
    frame.render_widget(
        Paragraph::new(format!("{}: ", field.label))
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Right),
        label_area,
    );

    frame.render_widget(
        Paragraph::new(value.to_string())
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
