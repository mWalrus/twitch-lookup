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
