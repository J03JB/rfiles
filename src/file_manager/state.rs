use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaneState {
    Parent,
    Current,
    Preview,
}

impl PaneState {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Parent,
            1 => Self::Current,
            2 => Self::Preview,
            _ => panic!("Invalid pane index"),
        }
    }
    
    pub fn to_index(&self) -> usize {
        match self {
            Self::Parent => 0,
            Self::Current => 1,
            Self::Preview=> 2,
        }
    }
}

pub struct FileManagerState {
    pub active_pane: PaneState,
    pub previous_pane: PaneState,
    // pub previous_pane: Option<PaneState>,
    pub selected_indices: HashMap<PaneState, usize>,
}

impl FileManagerState {
    pub fn new() -> Self {
        let mut selected_indices = HashMap::new();
        selected_indices.insert(PaneState::Parent, 0);
        selected_indices.insert(PaneState::Current, 0);
        selected_indices.insert(PaneState::Preview, 0);
        
        Self {
            active_pane: PaneState::Current,
            previous_pane: PaneState::Parent,
            selected_indices,
        }
    }
}

// State transition methods - separated from FileManager for clarity
pub trait StateManager {
    fn change_focus(&mut self, target_pane: PaneState) -> Result<(), &'static str>;
    fn shift_directories_forward(&mut self) -> Result<(), &'static str>;
    fn shift_directories_backward(&mut self) -> Result<(), &'static str>;
    fn move_selection(&mut self, delta: isize) -> Result<(), &'static str>;
    fn update_preview_pane(&mut self);
}
