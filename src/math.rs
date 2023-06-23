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

/// Calculates area of rectangle or square. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::rectangle_area;
/// 
/// fn main() {
///     println!("{}", rectangle_area(2.0, 3.0));
/// }
/// ```
pub fn rectangle_area(length: f32, breadth: f32) -> f32 {
    let result = format!("{:.2}", length * breadth);
    return result.parse().unwrap()
}

/// Calculates perimeter of circle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::circle_perimeter;
/// 
/// fn main() {
///     println!("{}", circle_perimeter(1.2));
/// }
/// ```
pub fn circle_perimeter(radius: f32) -> f32 {
    let result = format!("{:.2}", 2.0 * 3.14 * radius);
    return result.parse().unwrap()
}

/// Calculates perimeter of rectangle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::rectangle_perimeter;
/// 
/// fn main() {
///     println!("{}", rectangle_perimeter(2.0, 3.0));
/// }
/// ```
pub fn rectangle_perimeter(length: f32, breadth: f32) -> f32 {
    let result = format!("{:.2}", length * 2.0 + breadth * 2.0);
    return result.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_test() {
        assert!(circle_area(5.1) == 81.67);
    }

    #[test]
    fn rectangle_area_test() {
        assert!(rectangle_area(2.0, 3.0) == 6.0);
    }

    #[test]
    fn circle_perimeter_test() {
        assert!(circle_perimeter(1.2) == 7.54);
    }

    #[test]
    fn rectangle_perimeter_test() {
        assert!(rectangle_perimeter(2.0, 3.0) == 10.0);
    }
}