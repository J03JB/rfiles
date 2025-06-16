use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub enum PopupMode {
    NewFile,
    NewFolder,
    Rename,
    Delete,
    Copy,
    Move,
}

pub struct InputPopup {
    pub active: bool,
    pub input: String,
    pub cursor_position: usize,
    pub mode: PopupMode,
    pub title: String,
    pub prompt: String,
}

impl InputPopup {
    pub fn new() -> Self {
        Self {
            active: false,
            input: String::new(),
            cursor_position: 0,
            mode: PopupMode::NewFile,
            title: String::new(),
            prompt: String::new(),
        }
    }

    pub fn open(&mut self, mode: PopupMode) {
        self.active = true;
        self.input.clear();
        self.cursor_position = 0;
        self.mode = mode;

        match self.mode {
            PopupMode::NewFile => {
                self.prompt = "New file:".to_string();
            }
            PopupMode::NewFolder => {
                self.prompt = "New folder:".to_string();
            }
            PopupMode::Rename => {
                self.prompt = "Rename:".to_string();
            }
            PopupMode::Delete => {
                self.prompt = "Delete (y/n):".to_string();
            }
            PopupMode::Copy => {
                self.prompt = "Copy to:".to_string();
            }
            PopupMode::Move => {
                self.prompt = "Move to:".to_string();
            }
        }
    }

    pub fn close(&mut self) {
        self.active = false;
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Option<String> {
        if !self.active {
            return None;
        }

        match key.code {
            KeyCode::Enter => {
                let result = self.input.clone();
                self.close();
                Some(result)
            }
            KeyCode::Esc => {
                self.close();
                None
            }
            KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.input.remove(self.cursor_position);
                }
                None
            }
            KeyCode::Delete => {
                if !self.input.is_empty() && self.cursor_position < self.input.len() {
                    self.input.remove(self.cursor_position);
                }
                None
            }
            KeyCode::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    self.cursor_position += 1;
                }
                None
            }
            KeyCode::Home => {
                self.cursor_position = 0;
                None
            }
            KeyCode::End => {
                self.cursor_position = self.input.len();
                None
            }
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += 1;
                None
            }
            _ => None,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.active {
            return;
        }

        let popup_area = centered_rect(50, 7, area);

        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));

        // let inner_area = Layout::default()
        //     .direction(Direction::Vertical)
        //     .margin(1)
        //     .constraints([Constraint::Length(1)])
        //     .split(popup_area);

        let mut display_text = self.prompt.clone();
        display_text.push(' ');

        let input_text = self.input.clone();
        display_text.push_str(&input_text);

        if self.cursor_position == input_text.len() {
            display_text.push('█'); // Cursor at end
        } else {
            let cursor_pos = display_text.len() - input_text.len() + self.cursor_position;
            display_text.insert(cursor_pos, '█');
        }

        let input = Paragraph::new(display_text)
            .block(block)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left);

        frame.render_widget(input, popup_area);
    }
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
