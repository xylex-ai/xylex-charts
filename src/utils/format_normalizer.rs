
pub fn convert_hex_color_to_normalized(hex_color: &str) -> [f32; 4] {
    // Check if the input is a valid hex color code
    let hex_color = if hex_color.starts_with('#') {
        &hex_color[1..]
    } else {
        hex_color
    };

    // Ensure the length is either 6 (RGB) or 8 (RGBA)
    if hex_color.len() != 6 && hex_color.len() != 8 {
        return [0.0, 0.0, 0.0, 0.0]; // Return default value for invalid input
    }

    // Parse each color component and return 0.0 if invalid
    let parse_component = |start: usize, end: usize| -> f32 {
        u8::from_str_radix(&hex_color[start..end], 16)
            .map(|value| value as f32 / 255.0)
            .unwrap_or(0.0)
    };

    let r = parse_component(0, 2);
    let g = parse_component(2, 4);
    let b = parse_component(4, 6);
    let a = if hex_color.len() == 8 {
        parse_component(6, 8)
    } else {
        1.0 // Default alpha value
    };

    [r, g, b, a]
}
