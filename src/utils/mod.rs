// Utilities ============
// TODO: Implement failure handling
// TODO: Should return a tuple, since we epect the type to always be (&str, &str)
pub fn get_key_value_from_key_value_string(key_value_string: &str) -> Vec<&str> {
    // TODO: Use array here instead to improve performance
    let key_and_value: Vec<&str> = key_value_string.split("=").collect();
    return key_and_value;
}

pub fn remove_trailing_comma(string: &str) -> &str {
    string.strip_suffix(',').unwrap()
}
// END Utilities ============
