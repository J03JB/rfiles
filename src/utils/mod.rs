use ratatui::style::Color;

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
