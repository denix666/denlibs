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
