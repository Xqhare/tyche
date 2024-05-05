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

    pub fn random_u32() -> Option<u32> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 4];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = u32::from_le_bytes(buffer);
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Call with the start and end of the range (both `usize`).
    /// The range is inclusive on both ends.
    /// 
    /// ## Returns
    /// Will return `None` if start is bigger than end, or if random() fails.
    /// Will return `Some(usize)` wrapping a number inside the given range.
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
            let rng = random_u32();
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

    /// Takes in the length of a collection, like a vector, and retruns a valid, random, index for
    /// it.
    ///
    /// ## Returns
    /// `None` if `collection_length` is smaller than 1, or `random_u32()` fails.
    /// `Some(usize)` containing the index otherwise.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_index;
    ///
    /// fn main() {
    ///    let collection = (0..100).collect::<Vec<usize>>();
    ///    let random_index = random_index(collection.len()).unwrap();
    ///    println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
    /// }
    /// ```
    pub fn random_index(collection_length: usize) -> Option<usize> {
        if collection_length >= 1 {
            random_with_ceiling(collection_length.saturating_sub(1))
        } else {
            None
        }
    }

    pub fn random_with_ceiling(ceiling: usize) -> Option<usize> {
        let min_usize = usize::MIN;
        random_from_range(min_usize, ceiling)
    }

    pub fn random_with_floor(floor: usize) -> Option<usize> {
        let max_usize = usize::MAX;
        random_from_range(floor, max_usize)
    }
}

