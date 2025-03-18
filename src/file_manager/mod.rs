pub mod config;
pub mod entry;
pub mod fs;
pub mod pane;
pub mod state;
pub mod tui;
pub mod ui;

// Re-export main structures
pub use entry::FileEntry;
pub use pane::Pane;
pub use state::{FileManagerState, PaneState};

// Main FileManager struct that ties everything together
pub struct FileManager {
    pub panes: [Pane; 3],
    pub state: FileManagerState,
    // pub config: config::Config,
}

impl FileManager {
    //  Quick creation without error checking
    pub fn new() -> Self {
        let state = FileManagerState::new();

        let panes = [Pane::new(), Pane::new(), Pane::new()];

        // let config = config::Config::default();

        Self {
            panes,
            state,
            // config,
        }
    }
    // Initialization with error handling
    fn create() -> Result<Self, &'static str> {
        let mut file_manager = Self::new();

        let _ = file_manager.init_paths();

        Ok(file_manager)
    }

    // Core methods delegating to appropriate modules
    // pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     ui::run(self)
    // }
    pub fn init_paths(&mut self) -> Result<(), &'static str> {
        let current_dir = std::env::current_dir().map_err(|_| "Failed to get current directory")?;

        // Set path for current pane (index 1)
        self.panes[1].path = current_dir.clone();

        // Set path for parent pane (index 0)
        let parent_dir = current_dir
            .parent()
            .ok_or("Failed to get parent directory")?
            .to_path_buf();
        self.panes[0].path = parent_dir;

        // Reload contents for both panes
        self.panes[0].reload_contents()?;
        self.panes[1].reload_contents()?;

        // Find the current directory in the parent pane's contents and select it
        if let Some(index) = self.panes[0]
            .contents
            .iter()
            .position(|entry| entry.path == current_dir)
        {
            self.state.selected_indices.insert(PaneState::Parent, index);
        }

        self.update_preview_pane();

        Ok(())
    }

    pub fn update_preview_pane(&mut self) {
    // Get the current selected index
    let current_index = *self
        .state
        .selected_indices
        .get(&PaneState::Current)
        .unwrap_or(&0);

    // Clear previous preview content
    self.panes[2].preview_content = None;
    self.panes[2].contents.clear();

    // Get the selected item in the current pane, if any
    if let Some(selected) = self.panes[1].contents.get(current_index) {
        // Set preview pane path
        self.panes[2].path = selected.path.clone();

        // Load content or preview based on whether it's a file or directory
        if selected.is_dir {
            // For directories, just load their contents
            let _ = self.panes[2].reload_contents();
        } else {
            // For files, create the runtime and load the preview
            match tokio::runtime::Runtime::new() {
                Ok(rt) => {
                    // Use block_on to wait for the async operation to complete
                    let _ = rt.block_on(self.panes[2].reload_contents_or_preview());
               }
                Err(e) => {
                    // Log the error if runtime creation fails
                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("debug.log")
                    {
                        use std::io::Write;
                        let _ = writeln!(file, "Failed to create runtime: {}", e);
                    }
                    }
                }
            }
        }
    }

    // pub fn update_preview_pane(&mut self) {
    //     let current_index = *self
    //         .state
    //         .selected_indices
    //         .get(&PaneState::Current)
    //         .unwrap_or(&0);
    //
    //     // Get the selected item in the current pane, if any
    //     if let Some(selected) = self.panes[1].contents.get(current_index) {
    //         // Set preview pane path
    //         self.panes[2].path = selected.path.clone();
    //
    //         // Load content or preview
    //         self.panes[2].contents.clear();
    //
    //         let _ = tokio::runtime::Runtime::new().unwrap().block_on(self.panes[2].reload_contents_or_preview());
    //         // let _ = self.panes[2].reload_contents_or_preview().await;
    //     } else {
    //         // If current pane is empty, clear preview pane
    //         self.panes[2].contents.clear();
    //     }
    // }
    pub fn shift_directories_forward(&mut self) -> Result<(), &'static str> {
        // Check if there's a valid directory selected in the current pane
        let current_index = *self
            .state
            .selected_indices
            .get(&PaneState::Current)
            .unwrap_or(&0);

        if let Some(selected) = self.panes[1].contents.get(current_index) {
            if selected.path.is_dir() {
                let current_dir_path = self.panes[1].path.clone();
                // Shift the views: current → left, preview → current
                self.panes[0].path = self.panes[1].path.clone();
                self.panes[1].path = selected.path.clone();

                // Reload contents for both panes
                self.panes[0].reload_contents()?;
                self.panes[1].reload_contents()?;

                // Set the selection in parent pane to the directory we just came from
                if let Some(index) = self.panes[0]
                    .contents
                    .iter()
                    .position(|entry| entry.path == current_dir_path)
                {
                    self.state.selected_indices.insert(PaneState::Parent, index);
                } else {
                    // If not found, select the first item
                    self.state.selected_indices.insert(PaneState::Parent, 0);
                }

                // Reset selection for the new current pane
                self.state.selected_indices.insert(PaneState::Current, 0);

                // Update preview for the new selection
                self.update_preview_pane();

                return Ok(());
            }
        }

        Err("No directory selected to navigate into")
    }

    // Navigate to parent directory (move back)
    pub fn shift_directories_backward(&mut self) -> Result<(), &'static str> {
        // Get the parent of the current directory (pane 1)
        let parent_dir = self.panes[1]
            .path
            .parent()
            .ok_or("Current directory has no parent")?
            .to_path_buf();

        // Get the grandparent (if exists)
        let grandparent_dir = parent_dir.parent().map(|p| p.to_path_buf());

        // Set pane 0 to grandparent (if exists)
        // Set pane 0 to grandparent (if exists)
        if let Some(grand) = grandparent_dir {
            self.panes[0].path = grand;
            self.panes[0].reload_contents()?;

            // Find the parent directory in the grandparent's listing (for parent pane selection)
            if let Some(index) = self.panes[0]
                .contents
                .iter()
                .position(|entry| entry.path == parent_dir)
            {
                self.state.selected_indices.insert(PaneState::Parent, index);
            } else {
                // If not found, select the first item
                self.state.selected_indices.insert(PaneState::Parent, 0);
            }
        } else {
            self.panes[0].contents.clear();
        }
        // Set the current directory to the parent
        let current_path = self.panes[1].path.clone();
        self.panes[1].path = parent_dir;
        self.panes[1].reload_contents()?;

        // Try to find the previous directory in the listing to select it
        if let Some(index) = self.panes[1]
            .contents
            .iter()
            .position(|entry| entry.path == current_path)
        {
            self.state
                .selected_indices
                .insert(PaneState::Current, index);
        } else {
            // If not found, select the first item
            self.state.selected_indices.insert(PaneState::Current, 0);
        }

        // Update preview pane
        self.update_preview_pane();

        Ok(())
    }

    // Change focus between panes
    pub fn change_focus(&mut self, new_focus: PaneState) -> Result<(), &'static str> {
        // Store the old focus for potentially needed updates
        let old_focus = self.state.active_pane;

        // Set the new active pane
        self.state.active_pane = new_focus;

        // Ensure there's a selection in the newly focused pane
        if !self.state.selected_indices.contains_key(&new_focus) {
            self.state.selected_indices.insert(new_focus, 0);
        }

        // If changing to preview pane, we might need to update its contents
        if new_focus == PaneState::Preview && old_focus != PaneState::Preview {
            self.update_preview_pane();
        }

        Ok(())
    }
}
