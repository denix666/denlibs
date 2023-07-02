/// Calculates area of circle. Returns f32 rounded to two numbers after point.
/// 
/// # Example
/// ```rust
/// use denlibs::math::circle_area;
/// 
/// // To calculate circle area: radius^2 * Pi
/// assert_eq!(circle_area(5.1), 81.67);
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
/// // To calculate area of rectangle: length * width
/// assert_eq!(rectangle_area(2.0, 3.0), 6.0);
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
/// // To calculate perimeter of circle: 2 * 3.14 * radius
/// assert_eq!(circle_perimeter(1.2), 7.54);
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
/// // To calculate perimeter of rectangle: length * 2 + width * 2
/// assert_eq!(rectangle_perimeter(2.0, 3.0), 10.0);
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
/// // To calculate volume of a box: length * height * width
/// assert_eq!(box_volume(3.0, 5.0, 2.0), 30.0);
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
/// // To calculate volume of a rectangular pyramid: (base_length * base_width * pyramid_height) / 3
/// assert_eq!(rectangular_pyramid_volume(3.0, 4.0, 5.0), 20.0);
/// ```
pub fn rectangular_pyramid_volume(base_length: f32, base_width: f32, pyramid_height: f32) -> f32 {
    let result = format!("{:.2}", (base_length * base_width * pyramid_height) / 3.0);
    return result.parse().unwrap()
}

/// Luhn algorithm for credit card number validation or other purposes. Performs a Luhn algorithm check on given string, returns true/false.
/// 
/// # Example
/// 
/// ```rust
/// use denlibs::math::validate_luhn_sum;
/// 
/// assert!(validate_luhn_sum("4793480318851588"));
/// ```
pub fn validate_luhn_sum(number: &str) -> bool {
    let number_chars: Vec<char> = number.chars().collect();
    let mut first_char = true;
    let mut luhn_sum: u32 = 0;

    for i in number_chars {
        if let Some(mut char) = i.to_digit(10) {
            if first_char {
                char *= 2;
                if char >= 10  {
                    let mut tmp_sum: u32 = 0;
                    for j in char.to_string().chars() {
                        tmp_sum += j.to_digit(10).unwrap();
                    }
                    luhn_sum += tmp_sum;
                } else {
                    luhn_sum += char;
                }
            } else {
                luhn_sum += char;
            }
        } else {
            return false
        }
        first_char = !first_char;
    }

    return luhn_sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_test() {
        assert_eq!(circle_area(5.1), 81.67);
    }

    #[test]
    fn rectangle_area_test() {
        assert_eq!(rectangle_area(2.0, 3.0), 6.0);
    }

    #[test]
    fn circle_perimeter_test() {
        assert_eq!(circle_perimeter(1.2), 7.54);
    }

    #[test]
    fn rectangle_perimeter_test() {
        assert_eq!(rectangle_perimeter(2.0, 3.0), 10.0);
    }

    #[test]
    fn box_volume_test() {
        assert_eq!(box_volume(3.0, 5.0, 2.0), 30.0);
    }

    #[test]
    fn rectangular_pyramid_volume_test() {
        assert_eq!(rectangular_pyramid_volume(3.0, 4.0, 5.0), 20.0);
    }

    #[test]
    fn validate_luhn_sum_test() {
        assert!(validate_luhn_sum("4793480318851588"));
    }
}