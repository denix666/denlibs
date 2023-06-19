/// Calculates area of circle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::circle_area;
/// 
/// fn main() {
///     println!("{}", circle_area(5.1));
/// }
/// ```
pub fn circle_area(radius: f32) -> f32 {
    let result = format!("{:.2}", radius * radius * 3.14);
    return result.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_test() {
        assert!(circle_area(5.1) == 81.67);
    }
}