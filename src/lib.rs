//! # Tyche Yields Cryptographic High-Quality Entropy: A Rust Randomness Library
//!
//! The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate.
//! Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.
//!
//! This project was tested and developed on linux, but should / could work on MacOS. Noone has tried yet.
//!
//! No windows support at the moment.
//!
//! ## Returns
//!
//! All functions return a `Result()`. This is because of the random number generator used on the backend. It can run out of entropy, something that is highly unlikely but possible, or the program can not open `/dev/urandom`. If you get a `Err()` back the second reason is the most likely canditate as I have not encountered one `Err()` value not caused by this or improper calling by supplying bad arguments. All `Error`'s are `std::io::Error` types.
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
//!     let chosen_element_u64 = random_from_u64range(0, 100).unwrap();
//!     println!("Chosen element {chosen_element_u64}, in range 0-100");
//!
//!     let chosen_element_f32 = random_from_f32range(0.1, 100.1).unwrap();
//!     println!("Chosen element {chosen_element_f32}, in range 0.1-100.1");
//!
//!     let chosen_element_i32 = random_from_i32range(-100, 100).unwrap();
//!     println!("Chosen element {chosen_element_i32}, in range -100, 100");
//!
//!     let collection = (0..100).collect::<Vec<usize>>();
//!     let random_index = random_index(collection.len()).unwrap();
//!     println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
//! 
//!     let random_ceiling = random_with_ceiling(100);
//!     println!("The random number between 0 and 100 is: {}", random_ceiling.unwrap());
//!
//!     let random_floor = random_with_floor(0);
//!     let max_usize = usize::MAX;
//!     println!("The random number between 0 and {} is: {}", max_usize, random_floor.unwrap());
//! }
//! ```

#![allow(clippy::needless_doctest_main)]
#[cfg(test)]
mod tests;

pub mod prelude {
    use std::{fs::File, io::{Read, Error}, ops::{Sub, Add}};

    /// Generates a cryptographically secure pseudorandom `u8`. Leveraging the inbuilt Linux or MacOSX CSPRING.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns:
    /// Returns `Ok(u8)` otherwise.
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
    pub fn random_u8() -> Result<u8, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 1];
        rng.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    /// Generates a cryptographically secure pseudorandom `u16` by casting 2 random bytes as a `u16`,
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(u16)` with the random `u16` number.
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
    pub fn random_u16() -> Result<u16, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 2];
        rng.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    
    /// Generates a cryptographically secure pseudorandom `u32` by casting 4 random bytes as a `u32`,
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(u32)` with the random `u32` number.
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
    pub fn random_u32() -> Result<u32, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 4];
        rng.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    /// Generates a cryptographically secure pseudorandom `u64` by casting 8 random bytes as a `u64`,
    /// combining their bytes.
    /// The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// Please note that this function needs a 64bit system for obvious reasons.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(u64)` with the random `u64` number.
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
    pub fn random_u64() -> Result<u64, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 8];
        rng.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    /// Generates a cryptographically secure pseudorandom `i8` by casting a random byte as a `i8`. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(i8)` with the random `i8` number.
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
    pub fn random_i8() -> Result<i8, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 1];
        rng.read_exact(&mut buffer)?;
        Ok(i8::from_le_bytes(buffer))
    }

    /// Generates a cryptographically secure pseudorandom `i32` by casting 4 random bytes as a `i32`,
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(i32)` with the random `i32` number.
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
    pub fn random_i32() -> Result<i32, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 4];
        rng.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    /// Generates a cryptographically secure pseudorandom `f32` by casting 4 random bytes as a `f32`,
    /// combining their bytes. The little Endian is used for that here, mainly because it is better
    /// optimised for x86 and ARM processors.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(f32)` with the random `f32` number.
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
    pub fn random_f32() -> Result<f32, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 4];
        rng.read_exact(&mut buffer)?;
        let out = f32::from_le_bytes(buffer);
        if out.is_nan() {
            random_f32()
        } else {
            Ok(out)
        }
    }

    /// This generates a random character. Because I am mapping random noise as `utf8` characters,
    /// some wierd output is to be expected and will propably be needed to be sanatised. So while this does work, take care if you end up using
    /// it. This is literally the most unsafe and non-standart way of doing things and really isn't
    /// suited for all usecases.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(String)` with the random `String`.
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
    pub fn random_string() -> Result<String, Error> {
        let mut rng = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 10];
        rng.read_exact(&mut buffer)?;
        let mut out: String = Default::default();
        // This is probably the most black magic, unsafe and non standart way of doing
        // someting I have ever done.
        let black_magic: Vec<_> = buffer.bytes().map(|c| String::from_utf8(vec![c.unwrap()])).collect();
        if let Some(entry) = black_magic.into_iter().flatten().next() {
            out.push_str(&entry);
        } else {
            return random_string();
        }
        Ok(out)
}
    
    /// Call with the start and end of the range (both `usize`).
    /// The range is inclusive on both ends.
    /// 
    /// Uses a 32bit seeded rng, for 64bit seeded rng please use `random_from_u64range`.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// Will return `Ok(usize)` wrapping a number inside the given range.
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
    pub fn random_from_range(start: usize, end: usize) -> Result<usize, Error> {
        if start < end {
            let range_size = (end).saturating_sub(start).saturating_add(1);
            let rng = random_u32()?;
            let random_index = rng as usize % range_size;
            Ok(start.saturating_add(random_index))
        } else if start == end {
            Ok(start)
        } else {
            Err(Error::other(format!("Start '{}' is larger than end '{}'!", start, end)))
        }
    }

    /// Call with the start and end of the range (both `u64`).
    /// The range is inclusive on both ends.
    /// 
    /// Please note that this function needs a 64bit system for obvious reasons.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// Will return `Ok(u64)` wrapping a number inside the given range.
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
    pub fn random_from_u64range(start: u64, end: u64) -> Result<u64, Error> {
        if start < end {
            let range_size = (end - start).saturating_add(1);
            let rng = random_u64()?;
            let random_index = rng % range_size;
            Ok(start.saturating_add(random_index))
        } else if start == end {
            Ok(start)
        } else {
            Err(Error::other(format!("Start '{}' is larger than end '{}'!", start, end)))
        }
    }

    /// Call with the start and end of the range (both `f32`).
    /// The range is inclusive on start, and never quite reaches end (At least it was never
    /// observed during testing).
    /// 
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// Will return `Ok(f32)` wrapping a number inside the given range.
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
    pub fn random_from_f32range(start: f32, end: f32) -> Result<f32, Error> {
        if start < end {
            // I still believe this to have an off by one error, however it is infinitly small
            // because of f32.
            // As further reading did not help in the slightes but confirm that floating point
            // numbers are weird I will have to live with it. It seems to grow towards end, and
            // never reaching it. I now suspect maths shinanigans.
            let range_size = end.sub(start);//.add(1.0);
            let rng = random_f32()?;
                if rng.is_sign_positive() {
                    let random_index = rng % range_size;
                    Ok(start.add(random_index))
                } else {
                    let random_index = (rng * -1.0) % range_size;
                    Ok(start.add(random_index))
                }
        } else if start == end {
            Ok(start)
        } else {
            Err(Error::other(format!("Start '{}' is larger than end '{}'!", start, end)))
        }
    }

    /// Call with the start and end of the range (both `i32`).
    /// The range is inclusive on both ends.
    /// 
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// Will return `Ok(i32)` wrapping a number inside the given range.
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
    pub fn random_from_i32range(start: i32, end: i32) -> Result<i32, Error> {
        if start < end {
            let range_size = end.sub(start).add(1);
            let rng = random_i32()?;
            if rng.is_positive() {
                let random_index = rng % range_size;
                Ok(start.add(random_index))
            } else {
                let random_index = -rng % range_size;
                Ok(start.add(random_index))
            }
        } else if start == end {
            Ok(start)
        } else {
            Err(Error::other(format!("Start '{}' is larger than end '{}'!", start, end)))

        }
    }

    /// Takes in the length of a collection, like a vector, and retruns a valid, random, index for
    /// it.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(usize)` containing the index.
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
    pub fn random_index(collection_length: usize) -> Result<usize, Error> {
        if collection_length >= 1 {
            random_with_ceiling(collection_length.saturating_sub(1))
        } else {
            Err(Error::other(format!("collection length '{}' is less than 1!", collection_length)))
        }
    }

    /// Computes a random number between 0 and the `ceiling` argument.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(usize)` containing the number.
    ///
    /// ## Example:
    /// ```
    /// use tyche::prelude::random_with_ceiling;
    ///
    /// fn main() {
    ///    for n in 100000..200000  {
    ///         let answ = random_with_ceiling(n);
    ///         println!("The random number between 0 and {} is: {}", n, answ.unwrap());
    ///    }
    /// }
    /// ```
    pub fn random_with_ceiling(ceiling: usize) -> Result<usize, Error> {
        let min_usize = usize::MIN;
        random_from_range(min_usize, ceiling)
    }

    /// Computes a random number between `usize::MAX` and the `floor` argument.
    ///
    /// ## Errors
    /// All `Error`'s are `std::io::Error` types.
    /// If if the CSPRNG has no entropy available. This is highly unlikely, but possible.      
    /// Or if the program cannot access `/dev/urandom`. This is most likely.
    ///
    /// ## Returns
    /// `Ok(usize)` containing the number.
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
    ///    }
    /// }
    /// ```
    pub fn random_with_floor(floor: usize) -> Result<usize, Error> {
        let max_usize = usize::MAX;
        random_from_range(floor, max_usize)
    }
}

