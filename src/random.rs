
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Returns option of random choosed direction.
/// 
/// # Example
/// ```rust
/// use denlibs::random::random_direction;
/// 
/// fn main() {
///     let _dir = match random_direction().unwrap() {
///         Direction::Up => {println!("direction is Up")},
///         Direction::Down => {println!("direction is Down")},
///         Direction::Left => {println!("direction is Left")},
///         Direction::Right => {println!("direction is Right")},
///     };
/// }
/// ```
///
pub fn random_direction() -> Option<Direction> {
    quad_rand::srand(chrono::Utc::now().timestamp() as _);
    match quad_rand::gen_range(0, 3) {
        0 => Some(Direction::Down),
        1 => Some(Direction::Left),
        2 => Some(Direction::Up),
        3 => Some(Direction::Right),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_direction_test() {
        assert!(random_direction().is_some())
    }
}