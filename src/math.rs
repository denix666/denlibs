/// Calculates area of circle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::circle_area;
/// 
/// fn main() {
///     println!("Area of circle is: {}", circle_area(5.1));
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
///     println!("Area of rectangle is: {}", rectangle_area(2.0, 3.0));
/// }
/// ```
pub fn rectangle_area(length: f32, width: f32) -> f32 {
    let result = format!("{:.2}", length * width);
    return result.parse().unwrap()
}

/// Calculates perimeter of circle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::circle_perimeter;
/// 
/// fn main() {
///     println!("Perimeter of circle is: {}", circle_perimeter(1.2));
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
///     println!("Perimeter of rectangle is: {}", rectangle_perimeter(2.0, 3.0));
/// }
/// ```
pub fn rectangle_perimeter(length: f32, width: f32) -> f32 {
    let result = format!("{:.2}", length * 2.0 + width * 2.0);
    return result.parse().unwrap()
}

/// Calculates the volume of a box. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::box_volume;
/// 
/// fn main() {
///     println!("Volume of the box is: {}", box_volume(3.0, 5.0, 2.0));
/// }
/// ```
pub fn box_volume(length: f32, width: f32, height: f32) -> f32 {
    let result = format!("{:.2}", length * height * width);
    return result.parse().unwrap()
}

/// Calculates the volume of a rectangular pyramid. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::rectangular_pyramid_volume;
/// 
/// fn main() {
///     println!("Volume of the rectangular pyramid is: {}", rectangular_pyramid_volume(3.0, 4.0, 5.0));
/// }
/// ```
pub fn rectangular_pyramid_volume(base_length: f32, base_width: f32, pyramid_height: f32) -> f32 {
    let result = format!("{:.2}", (base_length * base_width * pyramid_height) / 3.0);
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

    #[test]
    fn box_volume_test() {
        assert!(box_volume(3.0, 5.0, 2.0) == 30.0);
    }

    #[test]
    fn rectangular_pyramid_volume_test() {
        assert!(rectangular_pyramid_volume(3.0, 4.0, 5.0) == 20.0);
    }
}