use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use crate::ActivePanel;

pub fn render_ui(
    f: &mut Frame,
    current_dir: &[(String, String)],
    parent_dir: &[(String, String)],
    selected_index_current: usize,
    selected_index_parent: usize,
    parent_list_state: &mut ListState,
    current_list_state: &mut ListState,
    preview_text: &Text<'static>,
    active_panel: &ActivePanel,
) {
    let size = f.area();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        // .constraints([Constraint::Percentage(30), Constraint::Min(70)])
        .constraints([
            Constraint::Percentage(15),
            Constraint:: Percentage(45),
            Constraint::Percentage(40),
        ])
        .split(size);

    let up_dir: Vec<ListItem> = parent_dir
        .iter()
        .enumerate()
        .map(|(i, (display_name, _file_name))| {
            let content =
                if i == selected_index_parent && matches!(active_panel, ActivePanel::Parent) {
                    format!(">  {}", display_name)
                } else {
                    display_name.clone()
                };
            ListItem::new(content)
        })
        .collect();

    let cur_dir: Vec<ListItem> = current_dir
        .iter()
        .enumerate()
        .map(|(i, (display_name, _file_name))| {
            let content =
                if i == selected_index_current && matches!(active_panel, ActivePanel::Current) {
                    format!(">  {}", display_name)
                } else {
                    display_name.clone()
                };
            ListItem::new(content)
        })
        .collect();

    //FIX: style selected item
    // item.style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))

    let updir = List::new(up_dir)
        .block(Block::default().title("/Root/"))
        .style(
            Style::default().fg(if matches!(active_panel, ActivePanel::Parent) {
                Color::Yellow
            } else {
                Color::White
            }),
        );
    let list = List::new(cur_dir)
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        .style(
            Style::default().fg(if matches!(active_panel, ActivePanel::Current) {
                Color::Yellow
            } else {
                Color::White
            }),
        );

    f.render_stateful_widget(updir, layout[0], parent_list_state);
    f.render_stateful_widget(list, layout[1], current_list_state);

    let preview = Paragraph::new(preview_text.clone())
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, layout[2]);
    f.render_widget(preview, layout[2]);
}
