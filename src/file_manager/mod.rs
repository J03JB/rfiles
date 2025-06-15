pub mod config;
pub mod entry;
pub mod fs;
pub mod pane;
pub mod state;
pub mod tui;
pub mod ui;

pub use pane::Pane;
pub use state::{FileManagerState, PaneState};
pub use ui::popup_input;

use self::popup_input::InputPopup;

use anyhow::{Context, Result};

pub struct FileManager {
    pub panes: [Pane; 3],
    pub state: FileManagerState,
    pub input_popup: InputPopup,
    // pub config: config::Config,
}

impl FileManager {
    pub fn new() -> Self {
        let state = FileManagerState::new();
        let panes = [Pane::new(), Pane::new(), Pane::new()];
        let input_popup = InputPopup::new();

        // let config = config::Config::default();

        Self {
            panes,
            state,
            input_popup,
            // config,
        }
    }

    fn create() -> Result<Self> {
        let mut file_manager = Self::new();

        let _ = file_manager.init_paths();

        Ok(file_manager)
    }

    pub fn init_paths(&mut self) -> Result<()> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;

        // Set path for current pane (index 1)
        self.panes[1].path = current_dir.clone();

        // Set path for parent pane (index 0)
        let parent_dir = current_dir
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get parent directory"))?
            // .ok_or("Failed to get parent directory")?
            .to_path_buf();
        self.panes[0].path = parent_dir;

        // Reload contents for both panes
        self.panes[0].reload_contents().context("Failed to reload parent pane contents")?;
        self.panes[1].reload_contents().context("Failed to reload current pane contents")?;

        // Find the current directory in the parent pane's contents and select it
        if let Some(index) =
            self.panes[0].contents.iter().position(|entry| entry.path == current_dir)
        {
            self.state.selected_indices.insert(PaneState::Parent, index);
        }

        self.update_preview_pane();

        Ok(())
    }

    pub fn update_preview_pane(&mut self) {
        let current_index = *self.state.selected_indices.get(&PaneState::Current).unwrap_or(&0);

        self.panes[2].preview_content = None;
        self.panes[2].contents.clear();

        if let Some(selected) = self.panes[1].contents.get(current_index) {
            self.panes[2].path = selected.path.clone();

            if selected.is_dir {
                let _ = self.panes[2].reload_contents();
            } else {
                match tokio::runtime::Runtime::new() {
                    Ok(rt) => {
                        let _ = rt.block_on(self.panes[2].reload_contents_or_preview());
                    }
                    Err(e) => {
                        if let Ok(mut file) =
                            std::fs::OpenOptions::new().create(true).append(true).open("debug.log")
                        {
                            use std::io::Write;
                            let _ = writeln!(file, "Failed to create runtime: {}", e);
                        }
                    }
                }
            }
        }
    }

    pub fn shift_directories_forward(&mut self) -> Result<()> {
        let current_index = *self.state.selected_indices.get(&PaneState::Current).unwrap_or(&0);

        if let Some(selected_entry) = self.panes[1].contents.get(current_index) {
            if selected_entry.path.is_dir() {
                let previous_current_pane_path = self.panes[1].path.clone();

                let new_current_pane_path = selected_entry.path.clone();

                self.panes[0].path = previous_current_pane_path.clone();
                self.panes[1].path = new_current_pane_path.clone();

                self.panes[0].reload_contents().context("Failed to reload parent pane contents")?;
                self.panes[1]
                    .reload_contents()
                    .context("Failed to reload current pane contents")?;

                if let Some(index) = self.panes[0]
                    .contents
                    .iter()
                    .position(|entry| entry.path == new_current_pane_path)
                {
                    self.state.selected_indices.insert(PaneState::Parent, index);
                } else {
                    self.state.selected_indices.insert(PaneState::Parent, 0);
                }

                self.state.selected_indices.insert(PaneState::Current, 0);

                self.update_preview_pane();

                return Ok(());
            }
        }

        Ok(())
    }

    pub fn shift_directories_backward(&mut self) -> Result<()> {
        let parent_dir = self.panes[1]
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Current directory has no parent"))?
            .to_path_buf();

        let grandparent_dir = parent_dir.parent().map(|p| p.to_path_buf());

        if let Some(grand) = grandparent_dir {
            self.panes[0].path = grand;
            self.panes[0].reload_contents()?;

            if let Some(index) =
                self.panes[0].contents.iter().position(|entry| entry.path == parent_dir)
            {
                self.state.selected_indices.insert(PaneState::Parent, index);
            } else {
                self.state.selected_indices.insert(PaneState::Parent, 0);
            }
        } else {
            self.panes[0].contents.clear();
        }
        let current_path = self.panes[1].path.clone();
        self.panes[1].path = parent_dir;
        self.panes[1].reload_contents()?;

        if let Some(index) =
            self.panes[1].contents.iter().position(|entry| entry.path == current_path)
        {
            self.state.selected_indices.insert(PaneState::Current, index);
        } else {
            self.state.selected_indices.insert(PaneState::Current, 0);
        }

        self.update_preview_pane();

        Ok(())
    }

    pub fn change_focus(&mut self, new_focus: PaneState) -> Result<()> {
        // Store the old focus for potentially needed updates
        let old_focus = self.state.active_pane;

        // Set the new active pane
        self.state.active_pane = new_focus;

        // Ensure there's a selection in the newly focused pane
        self.state.selected_indices.entry(new_focus).or_insert(0);

        // If changing to preview pane, we might need to update its contents
        if new_focus == PaneState::Preview && old_focus != PaneState::Preview {
            self.update_preview_pane();
        }

        Ok(())
    }
}
