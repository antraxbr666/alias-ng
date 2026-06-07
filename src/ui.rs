use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState,
    },
    Frame,
};

// Catppuccin Mocha palette
pub const MAUVE: Color = Color::Rgb(203, 166, 247);
pub const BLUE: Color = Color::Rgb(137, 180, 250);
pub const GREEN: Color = Color::Rgb(166, 227, 161);
pub const YELLOW: Color = Color::Rgb(249, 226, 175);
pub const OVERLAY: Color = Color::Rgb(108, 112, 134);
pub const SURFACE: Color = Color::Rgb(49, 50, 68);
pub const TEXT: Color = Color::Rgb(205, 214, 244);
pub const PEACH: Color = Color::Rgb(250, 179, 135);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create a centered, smaller window (80x31 max)
    let popup_area = centered_rect(80, 31, area);

    // Clear the background around the popup
    frame.render_widget(Clear, popup_area);

    // Main layout: search + table
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(popup_area);

    draw_search(frame, app, chunks[0]);
    draw_table(frame, app, chunks[1]);
}

fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let title = if matches!(app.mode, crate::app::AppMode::Search) {
        " ang > SEARCH (type to filter) "
    } else {
        " ang > press '/' to search "
    };

    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(MAUVE).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(OVERLAY))
        .border_set(border::ROUNDED);

    let input = Paragraph::new(app.search_query.clone())
        .style(
            Style::default()
                .fg(TEXT)
                .bg(SURFACE)
                .add_modifier(Modifier::BOLD),
        )
        .block(block);

    frame.render_widget(input, area);

    // Cursor in search box
    if matches!(app.mode, crate::app::AppMode::Search) {
        let x = area.x + app.search_query.len() as u16 + 2;
        let y = area.y + 1;
        frame.set_cursor_position((x, y));
    }
}

fn draw_table(frame: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["GROUP", "ALIAS", "COMMAND", "DESCRIPTION"]
        .iter()
        .map(|h| {
            Cell::new(*h).style(
                Style::default()
                    .fg(MAUVE)
                    .add_modifier(Modifier::BOLD),
            )
        });

    let header = Row::new(header_cells)
        .height(1)
        .style(Style::default().bg(SURFACE));

    let rows: Vec<Row> = app
        .filtered
        .iter()
        .enumerate()
        .map(|(display_idx, &alias_idx)| {
            let alias = &app.aliases[alias_idx];
            let is_selected = display_idx == app.selected;

            let style = if is_selected {
                Style::default()
                    .bg(Color::Rgb(69, 71, 90))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let group_style = Style::default().fg(MAUVE);
            let name_style = Style::default().fg(BLUE);
            let cmd_style = Style::default().fg(GREEN);
            let desc_style = Style::default().fg(YELLOW);

            if is_selected {
                let selected_fg = PEACH;
                Row::new(vec![
                    Cell::new(alias.group.clone())
                        .style(group_style.fg(selected_fg)),
                    Cell::new(alias.name.clone())
                        .style(name_style.fg(selected_fg)),
                    Cell::new(truncate(&alias.command, 25))
                        .style(cmd_style.fg(selected_fg)),
                    Cell::new(alias.description.clone())
                        .style(desc_style.fg(selected_fg)),
                ])
                .style(style)
                .height(1)
            } else {
                Row::new(vec![
                    Cell::new(alias.group.clone()).style(group_style),
                    Cell::new(alias.name.clone()).style(name_style),
                    Cell::new(truncate(&alias.command, 25)).style(cmd_style),
                    Cell::new(alias.description.clone()).style(desc_style),
                ])
                .style(style)
                .height(1)
            }
        })
        .collect();

    let count = app.filtered.len();
    let total = app.aliases.len();
    let title = format!(" ang v0.1.0 — {}/{} aliases ", count, total);

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(12),
            Constraint::Percentage(15),
            Constraint::Percentage(28),
            Constraint::Percentage(45),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(title)
            .title_style(Style::default().fg(MAUVE).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(OVERLAY))
            .border_set(border::ROUNDED),
    )
    .column_spacing(1)
    .row_highlight_style(
        Style::default()
            .bg(Color::Rgb(69, 71, 90))
            .add_modifier(Modifier::BOLD),
    );

    let mut state = TableState::default();
    state.select(Some(app.selected));

    frame.render_stateful_widget(table, area, &mut state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

pub fn draw_notification(frame: &mut Frame, alias_name: &str) {
    let area = frame.area();
    let popup_area = centered_rect(50, 15, area);

    frame.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ✓  Alias '", Style::default().fg(TEXT)),
            Span::styled(alias_name, Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("' copied to clipboard  ", Style::default().fg(TEXT)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to continue  ",
            Style::default().fg(OVERLAY),
        )),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(SURFACE))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MAUVE))
                .border_set(border::ROUNDED),
        );

    frame.render_widget(paragraph, popup_area);
}
