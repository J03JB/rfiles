use crate::file_manager::{FileManager, PaneState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

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
        let is_active = i == file_manager.state.active_pane.to_index();

        let border_style = Style::default().fg(Color::White);

        let title = pane.path.to_string_lossy().to_string();
        let block = Block::default()
            // .title(title)
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
                    // let is_directory =
                    //     entry.is_dir || Path::new(&entry.path).is_dir();
                    //
                    // let icon = if is_directory {
                    //     icon_for_file(&entry.path, &Some(Theme::Dark)).to_string()
                    // } else {
                    //     icon_for_file(&entry.name, &Some(Theme::Dark)).to_string()
                    // };
                    // let icon = entry.icons();
                    //
                    // Create a styled icon
                    // let icon_span = if entry.is_dir {
                    //     Span::styled(icon, Style::default())
                    // } else {
                    //     // You can customize colors based on file type if desired
                    //     Span::styled(icon, Style::default())
                    // };
                    let icon = entry.get_icons();
                    let icon_span = Span::styled(entry.get_icons(), Style::default());
                    // Create a line with the icon and file name
                    let line = Line::from(vec![
                        Span::raw(icon.to_string()),
                        Span::raw(" "),
                        Span::raw(&entry.name).white(),
                    ]);

                    ListItem::new(line)
                })
                .collect();

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::Blue))
                .highlight_symbol("> ");

            let mut state = ListState::default();
            state.select(Some(selected_index));

            frame.render_stateful_widget(list, chunks[i], &mut state);
        }
    }
}
