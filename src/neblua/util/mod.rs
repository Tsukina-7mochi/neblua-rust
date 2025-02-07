pub fn fmt_u8_vec(value: &Vec<u8>) -> String {
    let display_value = match String::from_utf8(value.clone()) {
        Ok(s) => s,
        Err(_) => value
            .iter()
            .map(|&c| format!("\\x{:02x}", c))
            .collect::<Vec<String>>()
            .join(""),
    };
    display_value
}
