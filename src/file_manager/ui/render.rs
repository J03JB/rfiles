use crate::file_manager::{FileManager, PaneState};
use devicons::FileIcon;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use std::path::Path;

pub fn render(file_manager: &FileManager, frame: &mut Frame) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(40),
            Constraint::Percentage(45),
        ])
        .split(size);

    // Render each pane
    for (i, pane) in file_manager.panes.iter().enumerate() {
        let _is_active = i == file_manager.state.active_pane.to_index();

        let border_style = Style::default().fg(Color::White);

        let block = Block::default()
            .borders(Borders::NONE)
            .border_style(border_style);

        let selected_index = *file_manager
            .state
            .selected_indices
            .get(&PaneState::from_index(i))
            .unwrap_or(&0);

        frame.render_widget(Clear, chunks[i]);

        if i == 2 && pane.preview_content.is_some() {
            let content = pane.preview_content.as_ref().unwrap();

            let paragraph = Paragraph::new(content.clone())
                .block(block)
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, chunks[2]);
        } else {
            let items: Vec<ListItem> = pane
                .contents
                .iter()
                .map(|entry| {
                    let is_directory = entry.is_dir || Path::new(&entry.path).is_dir();

                    let (icon, color) = if is_directory {
                        (FileIcon::from(&entry.path).to_string(), Color::Blue)
                    } else {
                        let icon = FileIcon::from(&entry.name);
                        let hex_color = icon.color;

                        let color = hex_to_tui_color(hex_color);

                        (icon.to_string(), color)
                    };
                    // let icon = entry.icons();
                    let icon_span = Span::styled(icon, Style::default().fg(color));

                    let line = Line::from(vec![
                        icon_span,
                        // Span::raw(icon.to_string()),
                        Span::raw(" "),
                        Span::raw(&entry.name),
                    ]);

                    ListItem::new(line)
                })
                .collect();

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().bg(Color::Blue))
                .highlight_symbol("> ");

            let mut state = ListState::default();
            state.select(Some(selected_index));

            frame.render_stateful_widget(list, chunks[i], &mut state);
        }
    }
}

pub fn hex_to_tui_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        6 => {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                Color::Rgb(r, g, b)
            } else {
                Color::White
            }
        }
        3 => {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..1], 16).map(|r| r * 17),
                u8::from_str_radix(&hex[1..2], 16).map(|g| g * 17),
                u8::from_str_radix(&hex[2..3], 16).map(|b| b * 17),
            ) {
                Color::Rgb(r, g, b)
            } else {
                Color::White
            }
        }
        _ => Color::White,
    }
}
