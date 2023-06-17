
/// Removes double quota of the string passed
/// 
/// # Example
/// ```rust
/// use denlibs::string_manipulations::remove_double_quotas;
/// 
/// fn main() {
///     println!("String with removed quotas: {}", remove_double_quotas("\"test\"".to_string()));
/// }
/// ```
pub fn remove_double_quotas(mut string: String) -> String {
    string.retain(|c| c != '"');
    return string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_quotas_test1() {
        let result = remove_double_quotas("\"test\"".to_string());
        assert!(result == "test");
    }
}
