use askama::Values;

pub fn display_some<T>(value: &Option<T>, _args: &dyn Values) -> askama::Result<String>
where
    T: std::fmt::Display,
{
    Ok(match value {
        Some(value) => value.to_string(),
        None => String::new(),
    })
}
