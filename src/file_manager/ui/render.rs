use crate::file_manager::Pane;
use crate::file_manager::{FileManager, PaneState};
use devicons::{FileIcon, Theme, icon_for_file};
use std::path::Path;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, List, ListItem, ListState, Wrap},
};

// pub fn debug_pane_contents(panes: &[Pane; 3]) {
//     use std::fs::OpenOptions;
//     use std::io::Write;
//
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("paths_debug.log")
//         .unwrap();
//
//     for (i, pane) in panes.iter().enumerate() {
//         writeln!(file, "Pane {}: Path = {:?}", i, pane.path).unwrap();
//
//         for (j, entry) in pane.contents.iter().enumerate() {
//             writeln!(file, "  Entry {}: name={}, path={:?}, is_dir={}", 
//                 j, entry.name, entry.path, entry.is_dir).unwrap();
//
//             // Also check the actual path
//             let path_exists = Path::new(&entry.path).exists();
//             let path_is_dir = Path::new(&entry.path).is_dir();
//             writeln!(file, "    Path exists: {}, Is directory: {}", path_exists, path_is_dir).unwrap();
//         }
//         writeln!(file, "").unwrap();
//     }
// }
pub fn render(file_manager: &FileManager, frame: &mut Frame) {
    // Add this at the beginning of your render function or elsewhere for debugging
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
        // Determine if this pane is active
        let is_active = i == file_manager.state.active_pane.to_index();

        // Create border style based on active state
        let border_style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };
        let title = pane.path.to_string_lossy().to_string();
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        // Get selected index for this pane
        let selected_index = *file_manager
            .state
            .selected_indices
            .get(&PaneState::from_index(i))
            .unwrap_or(&0);

        // First, explicitly clear the pane
        frame.render_widget(Clear, chunks[i]);

        if i == 2 && pane.preview_content.is_some() {
            // If this is the preview pane and we have preview content
            let content = pane.preview_content.as_ref().unwrap();

            // Split content into lines
            // let lines: Vec<Line> = content
            //     .lines()
            //     .map(|line| Line::from(line.to_string()))
            //     .collect();

            // Create a paragraph widget with the content
            let paragraph = Paragraph::new(content.clone())
                .block(block)
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, chunks[2]);
        } else {
            // Create list items from directory contents
            let items: Vec<ListItem> = pane
                .contents
                .iter()
              .map(|entry| {
                    let is_directory = entry.is_dir || 
                                      entry.name == ".." || 
                                      Path::new(&entry.path).is_dir();
                    // let icon = FileIcon::from(&entry.name).to_string();
                    // let icon = icon_for_file(&entry.name, &Some(Theme::Dark)).to_string();

                    let icon: String = if is_directory {
                        // Folder icon
                        "\u{f07b}".to_string()
                        // "".to_string()
                        // FileIcon::from(&entry.name).to_string()  // Default folder icon from Nerd Fonts
                    } else {
                        // Get appropriate file icon based on file extension
                        FileIcon::from(&entry.name).to_string()
                        // icon_for_file(&entry.name, None).unwrap_or(FileIcon::Default).as_str()
                    };
                    
                    // Create a styled icon
                    let icon_span = if entry.is_dir {
                        Span::styled(icon, Style::default())
                    } else {
                        // You can customize colors based on file type if desired
                        Span::styled(icon, Style::default())
                    };
                    
                    // Create a line with the icon and file name
                    let line = Line::from(vec![
                        icon_span,
                        Span::raw(" "),  // Add a space between icon and name
                        Span::raw(&entry.name)
                    ]);
                    
                    ListItem::new(line)
                })
                .collect();

            // Create list widget
            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().fg(Color::Blue))
                .highlight_symbol("> ");

            // Render list with selected item
            let mut state = ListState::default();
            state.select(Some(selected_index));

            frame.render_stateful_widget(list, chunks[i], &mut state);
        }
    }
}

// pub fn render(file_manager: &FileManager, frame: &mut Frame) {
//     let size = frame.area();
//
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage(15),
//             Constraint::Percentage(40),
//             Constraint::Percentage(45),
//         ])
//         .split(size);
//
//     // Render each pane
//     for (i, pane) in file_manager.panes.iter().enumerate() {
//         // Determine if this pane is active
//         let is_active = i == file_manager.state.active_pane.to_index();
//
//         // Create border style based on active state
//         let border_style = if is_active {
//             Style::default().fg(Color::Yellow)
//         } else {
//             Style::default().fg(Color::White)
//         };
//         let title = pane.path.to_string_lossy().to_string();
//         let block = Block::default()
//             .title(title)
//             .borders(Borders::ALL)
//             .border_style(border_style);
//
//         // Get selected index for this pane
//         let selected_index = *file_manager
//             .state
//             .selected_indices
//             .get(&PaneState::from_index(i))
//             .unwrap_or(&0);
//
//         if let Some(content) = &pane.preview_content {
//             // Split content into lines
//             let lines: Vec<Line> = content
//                 .lines()
//                 .map(|line| Line::from(line.to_string()))
//                 .collect();
//
//             // Create a paragraph widget with the content
//             let paragraph = Paragraph::new(lines)
//                 .block(block)
//                 .wrap(Wrap { trim: false });
//
//             frame.render_widget(paragraph, chunks[2]);
//         } else {
//             // Create list items from directory contents
//             let items: Vec<ListItem> = pane
//                 .contents
//                 .iter()
//                 .map(|entry| {
//                     let prefix = if entry.is_dir { "📁 " } else { "📄 " };
//                     ListItem::new(format!("{}{}", prefix, entry.name))
//                 })
//                 .collect();
//
//             // Create list widget
//             let list = List::new(items)
//                 .block(block)
//                 .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
//                 .highlight_symbol("> ");
//
//             // Render list with selected item
//             let mut state = ListState::default();
//             state.select(Some(selected_index));
//
//             frame.render_stateful_widget(list, chunks[i], &mut state);
//             // frame.render_widget(Clear, chunks[i]);
//         }
//     }
// }
