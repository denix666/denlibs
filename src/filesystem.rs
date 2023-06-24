
/// Returns the full path of the application, from where it was executed.
/// 
/// # Example
/// ```rust
/// use denlibs::filesystem::executable_path;
/// 
/// fn main() {
///     println!("Application was executed from: {}", executable_path().unwrap());
/// }
/// ```
pub fn executable_path() -> Result<String, Box<dyn std::error::Error>> {
    if let Some(dir) = std::env::current_exe().unwrap().parent().unwrap().to_str() {
        return Ok(dir.to_string());
    } else {
        return Err("Unable to get the path.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executable_path_test() {
        assert!(executable_path().is_ok());
    }
}
