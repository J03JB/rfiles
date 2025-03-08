use ratatui::{
    layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::Text, widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap}, Frame
};
use crate::tui::Tui;

pub fn render_ui(
    f: &mut Frame,
    file_list: &[(String,String)],
    selected_index: usize,
    list_state: &mut ListState,
    // preview_text: &Text<'static>,
    preview_text: &String,
) {
    let size = f.area();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Min(70)])
        .split(size);

    let items: Vec<ListItem> = file_list
        .iter()
        .enumerate()
        .map(|(i, (display_name, _file_name))| {
            let content = if i == selected_index {
                format!(">  {}", display_name)
            } else {
                display_name.clone()
            };
            ListItem::new(content)
        })
        // .map(|(i, file)| {
        //     let mut item = ListItem::new(file.clone());
        //     if i == selected_index {
        //         item = item.style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
        //     }
        //     item
        // })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .style(Style::default().fg(Color::White));

    f.render_stateful_widget(list, layout[0], list_state);


    let preview = Paragraph::new(preview_text.clone())
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, layout[1]);
    f.render_widget(preview, layout[1]);
}

