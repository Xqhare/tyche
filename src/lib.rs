//! # Tyche Yields Cryptographic High-Quality Entropy: A Rust Randomness Library
//!
//! The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate.
//! Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.
//!
//! This project was tested and developed on linux, but should / could work on MacOS. Noone has tried yet.
//!
//! No windows support at the moment.
//!
//! ## Function examples:
//!
//! For a detailed explanation of a function, please refer to it's dedicated documentation page.
//!
//! You can copy and run this code to see all functions of tyche in action!
//!
//! ```
//! use tyche::prelude::*;
//!
//! fn main() {
//!     let random_number_u8: u8 = random_u8().unwrap();
//!     println!("Generated random u8: {}", random_number_u8);
//!
//!     let random_number_u16: u16 = random_u16().unwrap();
//!     println!("Generated random u_u16: {}", random_number_u16);
//!    
//!     let random_number_u32: u32 = random_u32().unwrap();
//!     println!("Generated random u_u32: {}", random_number_u32);
//!     
//!     let random_number_u64: u64 = random_u64().unwrap();
//!     println!("Generated random u64: {}", random_number_u64);
//!
//!     let random_number_i8: i8 = random_i8().unwrap();
//!     println!("Generated random i8: {}", random_number_i8);
//!
//!     let random_number_i32: i32 = random_i32().unwrap();
//!     println!("Generated random i32: {}", random_number_i32);
//!     
//!     let random_number_f32: f32 = random_f32().unwrap();
//!     println!("Generated random f32: {}", random_number_f32);
//!
//!     let random_string: String = random_string().unwrap();
//!     println!("Generated random String: {}", random_string);
//! 
//!     let chosen_element = random_from_range(0, 100).unwrap();
//!     println!("Chosen element {chosen_element}, in range 0-100");
//!
//!     let chosen_element = random_from_f32range(0.1, 100.1).unwrap();
//!     println!("Chosen element {chosen_element}, in range 0.1-100.1");
//!
//!     let chosen_element = random_from_i32range(-100, 100).unwrap();
//!     println!("Chosen element {chosen_element}, in range -100, 100");
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
    use std::{fs::File, io::Read, ops::{Sub, Add}};

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

    /// Generates a cryptographically secure pseudorandom u16 by casting 2 random bytes as a u16,
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

    
    /// Generates a cryptographically secure pseudorandom u32 by casting 4 random bytes as a u32,
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

    /// Generates a cryptographically secure pseudorandom u64 by casting 8 random bytes as a u64,
    /// combining their bytes.
    /// The little Endian is used for that here, mainly because it is better
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

    /// Generates a cryptographically secure pseudorandom i8 by casting a random byte as a i8. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// Please note that this function needs a 64bit system for obvious reasons.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(i8)` with the random i8 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_i8;
    ///
    /// fn main() {
    ///   let random_number: i8 = random_i8().unwrap();
    ///   println!("Generated random i8: {}", random_number);
    /// }
    /// ```
    pub fn random_i8() -> Option<i8> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 1];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = i8::from_le_bytes(buffer);
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Generates a cryptographically secure pseudorandom i32 by casting 4 random bytes as a i32,
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// Please note that this function needs a 64bit system for obvious reasons.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(i32)` with the random i32 number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_i32;
    ///
    /// fn main() {
    ///   let random_number: i32 = random_i32().unwrap();
    ///   println!("Generated random i32: {}", random_number);
    /// }
    /// ```
    pub fn random_i32() -> Option<i32> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 4];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let out = i32::from_le_bytes(buffer);
                Some(out)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Generates a cryptographically secure pseudorandom f32 by casting 4 random bytes as a f32,
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
                if out.is_nan() {
                    return random_f32();
                } else {
                    Some(out)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// This generates a random character. Because I am mapping random noise as utf8 characters,
    /// some wierd output is to be expected and will propably be needed to be sanatised. So while this does work, take care if you end up using
    /// it.
    ///
    /// ## Returns
    /// `None` if the CSPRNG has no entropy available or there is no access to it. This is highly unlikely, but possible.
    /// `Some(String)` with the random String.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_string;
    ///
    /// fn main() {
    ///   let random_string: String = random_string().unwrap();
    ///   println!("Generated random String: {}", random_string);
    /// }
    /// ```
    pub fn random_string() -> Option<String> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 10];
            let temp = rng.unwrap().read_exact(&mut buffer);
            if temp.is_ok() {
                let mut out: String = Default::default();
                // This is probably the most black magic, unsafe and non standart way of doing
                // someting I have ever done.
                let black_magic: Vec<_> = buffer.bytes().map(|c| String::from_utf8(vec![c.unwrap()])).collect();
                for entry in black_magic {
                    if entry.is_ok() {
                        out.push_str(entry.unwrap().as_str());
                        break;
                    }
                }
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
    /// Uses u32 rng, for u64rng please use `random_from_u64range`.
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

    /// Call with the start and end of the range (both `u64`).
    /// The range is inclusive on both ends.
    /// 
    /// 64bit systems only.
    ///
    /// ## Returns
    /// Will return `None` if start is bigger than end, or if random() fails. This is highly unlikely, but possible.
    /// Will return `Some(u64)` wrapping a number inside the given range.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_from_u64range;
    ///
    /// fn main() {
    ///     let chosen_element = random_from_u64range(0, 100).unwrap();
    ///     println!("Chosen element {chosen_element}, in range 0-100");
    /// }
    /// ```
    pub fn random_from_u64range(start: u64, end: u64) -> Option<u64> {
        if start < end {
            let range_size = (end - start).saturating_add(1);
            let rng = random_u64();
            if rng.is_some() {
                let random_index = rng.unwrap() % range_size;
                Some(start + random_index)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Call with the start and end of the range (both `f32`).
    /// The range is inclusive on start, and never quite reaches end (At least it was never
    /// observed during testing).
    /// 
    /// ## Returns
    /// Will return `None` if start is bigger than end, or if random_f32() fails. This is highly unlikely, but possible.
    /// Will return `Some(f32)` wrapping a number inside the given range.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_from_f32range;
    ///
    /// fn main() {
    ///     let chosen_element = random_from_f32range(0.1, 100.1).unwrap();
    ///     println!("Chosen element {chosen_element}, in range 0.1-100.1");
    /// }
    /// ```
    pub fn random_from_f32range(start: f32, end: f32) -> Option<f32> {
        if start < end {
            // I still believe this to have an off by one error, however it is infinitly small
            // because of f32.
            // As further reading did not help in the slightes but confirm that floating point
            // numbers are weird I will have to live with it. It seems to grow towards end, and
            // never reaching it. I now suspect maths shinanigans.
            let range_size = end.sub(start);//.add(1.0);
            let rng = random_f32();
            if rng.is_some() {
                if rng.unwrap().is_sign_positive() {
                    let random_index = rng.unwrap() % range_size;
                    Some(start + random_index)
                } else {
                    let random_index = (rng.unwrap() * -1.0) % range_size;
                    Some(start + random_index)
                }
                
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Call with the start and end of the range (both `i32`).
    /// The range is inclusive on both ends.
    /// 
    /// ## Returns
    /// Will return `None` if start is bigger than end, or if random_i32() fails. This is highly unlikely, but possible.
    /// Will return `Some(i32)` wrapping a number inside the given range.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_from_i32range;
    ///
    /// fn main() {
    ///     let chosen_element = random_from_i32range(-100, 100).unwrap();
    ///     println!("Chosen element {chosen_element}, in range -100, 100");
    /// }
    /// ```
    pub fn random_from_i32range(start: i32, end: i32) -> Option<i32> {
        if start < end {
            let range_size = end.sub(start).add(1);
            let rng = random_i32();
            if rng.is_some() {
                if rng.unwrap().is_positive() {
                    let random_index = rng.unwrap() % range_size;
                    Some(start + random_index)
                } else {
                    let random_index = (rng.unwrap() * -1) % range_size;
                    Some(start + random_index)
                }
                
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

