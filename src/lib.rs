//! # Tyche: A Rust Randomness Library
//!
//! The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate.
//! Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.
//!
//! This project was tested and developed on linux, but should / could work on MacOS. No windows
//! support at the moment.
//!
//! ## Function examples:
//!
//! For a detailed explanation of a function, please refer to it's dedicated documentation page.
//!
//! ```
//! use tyche::prelude::*;
//!
//! fn main() {
//!     let random_number8: u8 = random_u8().unwrap();
//!     println!("Generated random u8: {}", random_number8);
//!
//!     let random_number16: u16 = random_u16().unwrap();
//!     println!("Generated random u16: {}", random_number16);
//!    
//!     let random_number32: u32 = random_u32().unwrap();
//!     println!("Generated random u32: {}", random_number32);
//!     
//!     let random_number64: u64 = random_u64().unwrap();
//!     println!("Generated random u64: {}", random_number64);
//!     
//!     let random_number_f32: f32 = random_f32().unwrap();
//!     println!("Generated random f32: {}", random_number_f32);
//! 
//!     let chosen_element = random_from_range(0, 100).unwrap();
//!     println!("Chosen element {chosen_element}, in range 0-100");
//!
//!     let collection = (0..100).collect::<Vec<usize>>();
//!     let random_index = random_index(collection.len()).unwrap();
//!     println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
//! 
//!     let random_ceiling = random_with_ceiling(100);
//!     println!("The random number between 0 and 100 is: {}", random_ceiling.unwrap());
//!     assert!(random_ceiling.is_some());
//!
//!     let random_floor = random_with_floor(0);
//!     let max_usize = usize::MAX;
//!     println!("The random number between 0 and {} is: {}", max_usize, random_floor.unwrap());
//!     assert!(random_floor.is_some());
//! }
//! ```

#[cfg(test)]
mod tests;

pub mod prelude {
    use std::{fs::File, io::Read};

    /// Generates a cryptographically secure pseudorandom f32 by combining 4 random u8 numbers and
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(f32)` with the random f32 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_f32;
    ///
    /// fn main() {
    ///   let random_number: f32 = random_f32().unwrap();
    ///   println!("Generated random f32: {}", random_number);
    /// }
    /// ```
    pub fn random_f32() -> Option<f32> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 4];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = f32::from_le_bytes(buffer);
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Generates a cryptographically secure pseudorandom u8. Leveraging the inbuilt Linux or MacOSX CSPRING.
    ///
    /// ## Returns:
    /// Returns `None` if the CSPRNG has no entropy available or no access to the CSPRNG was possible. This is highly unlikely, but possible.
    /// Returns `Some(u8)` otherwise.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_u8;
    ///
    /// fn main() {
    ///   let random_number: u8 = random_u8().unwrap();
    ///   println!("Generated random u8: {}", random_number);
    /// }
    /// ```
    pub fn random_u8() -> Option<u8> {
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

    /// Generates a cryptographically secure pseudorandom u16 by combining 2 random u8 numbers and
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(u16)` with the random u16 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_u16;
    ///
    /// fn main() {
    ///   let random_number: u16 = random_u16().unwrap();
    ///   println!("Generated random u16: {}", random_number);
    /// }
    /// ```
    pub fn random_u16() -> Option<u16> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 2];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = u16::from_le_bytes(buffer);
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Generates a cryptographically secure pseudorandom u32 by combining 4 random u8 numbers and
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(u32)` with the random u32 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_u32;
    ///
    /// fn main() {
    ///   let random_number: u32 = random_u32().unwrap();
    ///   println!("Generated random u32: {}", random_number);
    /// }
    /// ```
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

    /// Generates a cryptographically secure pseudorandom u64 by combining 8 random u8 numbers and
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// Please note that this function needs a 64bit system for obvious reasons.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(u64)` with the random u64 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_u64;
    ///
    /// fn main() {
    ///   let random_number: u64 = random_u64().unwrap();
    ///   println!("Generated random u64: {}", random_number);
    /// }
    /// ```
    pub fn random_u64() -> Option<u64> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 8];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = u64::from_le_bytes(buffer);
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
    /// Will return `None` if start is bigger than end, or if random() fails. This is highly unlikely, but possible.
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
    /// `None` if `collection_length` is smaller than 1, or `random_u32()` fails. This is highly unlikely, but possible.
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

    /// Computes a random number between 0 and the `ceiling` argument.
    ///
    /// ## Returns
    /// `None` if `random_u32()` fails to generate. This is highly unlikely, but possible.
    /// `Some(usize)` containing the number otherwise.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_with_ceiling;
    ///
    /// fn main() {
    ///    for n in 100000..200000  {
    ///         let answ = random_with_ceiling(n);
    ///         println!("The random number between 0 and {} is: {}", n, answ.unwrap());
    ///         assert!(answ.is_some());
    ///    }
    /// }
    /// ```
    pub fn random_with_ceiling(ceiling: usize) -> Option<usize> {
        let min_usize = usize::MIN;
        random_from_range(min_usize, ceiling)
    }

    /// Computes a random number between `usize::MAX` and the `floor` argument.
    ///
    /// ## Returns
    /// `None` if `random_u32()` fails to generate. This is highly unlikely, but possible.
    /// `Some(usize)` containing the number otherwise.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_with_floor;
    ///
    /// fn main() {
    ///    for n in 0..100000  {
    ///         let answ = random_with_floor(n);
    ///         let max_usize = usize::MAX;
    ///         println!("The random number between {} and {} is: {}", max_usize, n, answ.unwrap());
    ///         assert!(answ.is_some());
    ///    }
    /// }
    /// ```
    pub fn random_with_floor(floor: usize) -> Option<usize> {
        let max_usize = usize::MAX;
        random_from_range(floor, max_usize)
    }
}

