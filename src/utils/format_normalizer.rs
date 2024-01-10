fn convert_hex_color_to_normalized(
    hex_color: &str
) -> [f32; 4] {
    let hex_color = hex_color.trim_start_matches("#");

    let r: u8 = u8::from_str_radix(&hex_color[0..2], 16).unwrap();
    let g: u8 = u8::from_str_radix(&hex_color[2..4], 16).unwrap();
    let b: u8 = u8::from_str_radix(&hex_color[4..6], 16).unwrap();
    let a: u8 = u8::from_str_radix(&hex_color[6..8], 16).unwrap();

    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0
    ]
}