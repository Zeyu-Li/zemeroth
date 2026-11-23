use std::fmt::Debug;

use rand::{distributions::uniform::SampleUniform, seq::SliceRandom, Rng};

/// Adapts macroquad's global `quad_rand` generator to `rand::RngCore`.
///
/// `quad_rand::compat::QuadRand` isn't used because quad-rand's `rand` feature
/// turns on rand's default features, whose `getrandom` dependency doesn't build
/// for `wasm32-unknown-unknown`.
struct QuadRand;

impl rand::RngCore for QuadRand {
    fn next_u32(&mut self) -> u32 {
        quad_rand::rand()
    }

    fn next_u64(&mut self) -> u64 {
        (u64::from(self.next_u32()) << 32) | u64::from(self.next_u32())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let bytes = self.next_u32().to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

pub fn zrng() -> impl rand::Rng {
    QuadRand
}

pub fn roll_dice<T: SampleUniform + PartialOrd>(low: T, high: T) -> T {
    zrng().gen_range(low..high)
}

pub fn shuffle_vec<T>(mut vec: Vec<T>) -> Vec<T> {
    vec.shuffle(&mut zrng());
    vec
}

/// Remove an element from a vector.
pub fn try_remove_item<T: Debug + PartialEq>(vec: &mut Vec<T>, e: &T) -> bool {
    vec.iter()
        .position(|current| current == e)
        .map(|e| vec.remove(e))
        .is_some()
}

pub fn clamp_min<T: PartialOrd>(value: T, min: T) -> T {
    if value < min {
        min
    } else {
        value
    }
}

pub fn clamp_max<T: PartialOrd>(value: T, max: T) -> T {
    if value > max {
        max
    } else {
        value
    }
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clamp_min() {
        assert_eq!(super::clamp_min(1, 0), 1);
        assert_eq!(super::clamp_min(0, 0), 0);
        assert_eq!(super::clamp_min(-1, 0), 0);
    }

    #[test]
    fn test_clamp_max() {
        assert_eq!(super::clamp_max(1, 2), 1);
        assert_eq!(super::clamp_max(2, 2), 2);
        assert_eq!(super::clamp_max(3, 2), 2);
    }

    #[test]
    fn test_clamp() {
        let min = 0;
        let max = 2;
        assert_eq!(super::clamp(1, min, max), 1);
        assert_eq!(super::clamp(0, min, max), 0);
        assert_eq!(super::clamp(-1, min, max), 0);
        assert_eq!(super::clamp(1, min, max), 1);
        assert_eq!(super::clamp(2, min, max), 2);
        assert_eq!(super::clamp(3, min, max), 2);
    }

    #[test]
    fn test_try_remove_item() {
        let mut a = vec![1, 2, 3];
        assert!(super::try_remove_item(&mut a, &1));
        assert_eq!(&a, &[2, 3]);
        assert!(!super::try_remove_item(&mut a, &666));
    }
}
