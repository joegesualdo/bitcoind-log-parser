// Utilities ============
pub fn get_key_value_from_key_value_string(key_value_string: &str) -> Vec<&str> {
    let key_and_value: Vec<&str> = key_value_string.split("=").collect();
    return key_and_value;
}

pub fn remove_trailing_comma(string: &str) -> &str {
    string.strip_suffix(',').unwrap()
}
// END Utilities ============
