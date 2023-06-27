use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Returns an option of random choosed direction.
/// 
/// # Example
/// ```rust
/// use denlibs::random::random_direction;
/// 
/// fn main() {
///     match random_direction().unwrap() {
///         denlibs::random::Direction::Up => {println!("Choosed direction is Up")},
///         denlibs::random::Direction::Down => {println!("Choosed direction is Down")},
///         denlibs::random::Direction::Left => {println!("Choosed direction is Left")},
///         denlibs::random::Direction::Right => {println!("Choosed direction is Right")},
///     };
/// }
/// ```
///
pub fn random_direction() -> Option<Direction> {
    quad_rand::srand(RandomState::new().build_hasher().finish() as _);

    match quad_rand::gen_range(0, 4) {
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