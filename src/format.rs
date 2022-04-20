use chrono::Duration;

pub fn readable_number(num: u32) -> String {
    let num = num.to_string();
    let mut readable_num = String::new();
    for (i, char) in num.chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 {
            readable_num.insert(0, ',');
        }
        readable_num.insert(0, char);
    }
    readable_num
}

// TODO: add days to output if needed.
pub fn duration_to_hms(d: Duration) -> String {
    let mut result = String::new();
    if d.num_days() > 0 {
        result += &((((d.num_seconds() / 60) / 60) / 24).to_string() + "d ");
    }
    if d.num_hours() > 0 {
        result += &((((d.num_seconds() / 60) / 60) % 24).to_string() + "h ");
    }
    if d.num_minutes() > 0 {
        result += &(((d.num_seconds() / 60) % 60).to_string() + "m ");
    }
    if d.num_seconds() > 0 {
        result += &((d.num_seconds() % 60).to_string() + "s")
    }
    result
}

pub fn hex_to_rgb(mut hex: &str) -> (u8, u8, u8) {
    hex = hex.trim_start_matches('#');
    let hex = i64::from_str_radix(hex, 16).unwrap();
    let r: u8 = ((hex as u32 >> 16u8) & 0xFF) as u8;
    let g: u8 = ((hex as u32 >> 8u8) & 0xFF) as u8;
    let b: u8 = (hex as u8) & 0xFF;
    (r, g, b)
}
