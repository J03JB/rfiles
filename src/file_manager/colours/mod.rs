use ratatui::style::Color;
// Helper function to convert hex color to ratatui Color
pub fn hex_to_tui_color(hex: &str) -> Color {
    // Remove '#' if present
    let hex = hex.trim_start_matches('#');
    
    // Parse the hex color
    match hex.len() {
        6 => {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                Color::Rgb(r, g, b)
            } else {
                Color::White // Fallback
            }
        },
        3 => {
            // Handle shorthand hex like #RGB
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..1], 16).map(|r| r * 17),
                u8::from_str_radix(&hex[1..2], 16).map(|g| g * 17),
                u8::from_str_radix(&hex[2..3], 16).map(|b| b * 17),
            ) {
                Color::Rgb(r, g, b)
            } else {
                Color::White // Fallback
            }
        },
        _ => Color::White, 
    }
}
