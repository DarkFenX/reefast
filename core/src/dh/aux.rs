use std::{error, result};

/// Alias for a `Result` which accepts any error type
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// Convenience type to pass data and accumulated warnings to the caller.
#[derive(Debug)]
pub struct Container<T> {
    /// Vector with actual data.
    pub data: Vec<T>,
    /// Vector with strings which represent warnings encountered during data generation.
    pub warns: Vec<String>,
}
impl<T> Container<T> {
    /// Make a new empty container.
    pub fn new() -> Container<T> {
        Container {
            data: Vec::new(),
            warns: Vec::new(),
        }
    }
    /// Make a new container out of passed data.
    pub fn new_with_data(data: Vec<T>, warns: Vec<String>) -> Container<T> {
        Container { data, warns }
    }
}
