#[cfg(test)]
mod tests;

pub mod prelude {
    use std::{fs::File, io::Read};
    
    /// Generates a cryptographically secure pseudorandom u8. Takes no arguments.
    ///
    /// ## Returns:
    /// Returns `None` if the CSPRNG has no entropy available or no access to the CSPRNG was
    /// possible.
    /// Returns `Some(u8)` otherwise.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random;
    ///
    /// fn main() {
    ///   let random_number: u8 = random().unwrap();
    ///   println!("Generated random u8: {}", random_number);
    /// }
    /// ```
    pub fn random() -> Option<u8> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 1];
            if rng.unwrap().read_exact(&mut buffer).is_ok() {
                return Some(buffer[0].into());
            } else {
                return None;
            }
        } else {
            return None;
        }
    }


    /// Call with the start and end of the range (both `usize`).
    /// The range is inclusive on both ends.
    /// 
    /// ## Returns
    /// Will return `None` if start is bigger than end, or if random() fails.
    /// Will return `Some` wrapping a number inside the given range.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_from_range;
    ///
    /// fn main() {
    ///     let chosen_element = random_from_range(0, 100).unwrap();
    ///     println!("Chosen element {chosen_element}, in range 0-100");
    /// }
    /// ```
    pub fn random_from_range(start: usize, end: usize) -> Option<usize> {
        if start < end {
            let range_size = (end - start).saturating_add(1);
            let rng = random();
            if rng.is_some() {
                let random_index = rng.unwrap() as usize % range_size;
                Some(start + random_index)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn random_index(collection_length: usize) -> usize {
        todo!()
    }

    pub fn random_with_ceiling(ceiling: usize) -> usize {
        let min_usize = usize::MIN;
    }

    pub fn random_with_floor(floor: usize) -> usize {
        let max_usize = usize::MAX;
    }
}

