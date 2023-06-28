/// Removes double quota of the string passed
/// 
/// # Example
/// ```rust
/// use denlibs::string_manipulations::remove_double_quotas;
/// 
/// println!("String with removed double quotas: {}", remove_double_quotas("\"test\"".to_string()));
/// ```
pub fn remove_double_quotas(mut string: String) -> String {
    string.retain(|c| c != '"');
    return string
}

/// Function for searching plain-text data sets for lines that match a patterns in a provided file.
/// 
/// # Example
/// ```rust
/// use denlibs::string_manipulations::grep;
/// 
/// let path_buf = std::path::PathBuf::from("/tmp/my_file");
/// let path = path_buf.as_os_str();
/// 
/// if let Ok(result) = grep(path, "my text") {
///     println!("Found line: {}", result);
/// }
/// 
/// ```
pub fn grep(path_to_file: &std::ffi::OsStr, search_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    match std::fs::File::open(path_to_file) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            for line in std::io::BufRead::lines(reader) {
                let res: String = line.unwrap();
                if res.contains(search_text) {
                    return Ok(res.to_string())
                }
            }
            Err("Searching text not found".into())
        },
        Err(_) => {
            return Err("Unable to open the file.".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_double_quotas_test() {
        let result = remove_double_quotas("\"test\"".to_string());
        assert!(result == "test");
    }
}
