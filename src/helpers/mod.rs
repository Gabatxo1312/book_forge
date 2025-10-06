pub fn convert_params_string_to_id(id: Option<String>) -> Option<i32> {
    id.and_then(|s| if s.is_empty() { None } else { s.parse().ok() })
}
