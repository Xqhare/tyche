//! # Tyche Yields Cryptographic High-Quality Entropy: A Rust Randomness Library
//!
//! The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate.
//! Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.
//!
//! This project was tested and developed on linux, but should / could work on `MacOS`. Noone has tried yet.
//!
//! No windows support at the moment.
//!
//! ## Returns
//!
//! All functions return a `Result()`. This is because of the random number generator used on the backend. It can run out of entropy, something that is highly unlikely but possible, or the program can not open `/dev/urandom`. If you get a `Err()` back the second reason is the most likely candidate as I have not encountered one `Err()` value not caused by this or improper calling by supplying bad arguments.

#![allow(clippy::needless_doctest_main)]
#[cfg(test)]
mod tests;
#[cfg(test)]
mod examples;

use athena::rng_api::{RngApi, RngError, RngResult};
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::ops::{Add, Sub};

/// A CSPRNG implementation using `/dev/urandom`.
pub struct Tyche(File);

impl Tyche {
    /// Create a new Tyche instance.
    ///
    /// ## Errors
    /// Returns `IoError` if `/dev/urandom` cannot be opened.
    pub fn new() -> Result<Self, IoError> {
        Ok(Tyche(File::open("/dev/urandom")?))
    }
}

impl RngApi for Tyche {
    fn random_u8(&mut self) -> RngResult<u8> {
        let mut buffer = [0u8; 1];
        self.0.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    fn random_u16(&mut self) -> RngResult<u16> {
        let mut buffer = [0u8; 2];
        self.0.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    fn random_u32(&mut self) -> RngResult<u32> {
        let mut buffer = [0u8; 4];
        self.0.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn random_u64(&mut self) -> RngResult<u64> {
        let mut buffer = [0u8; 8];
        self.0.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    fn random_usize(&mut self) -> RngResult<usize> {
        #[cfg(target_pointer_width = "64")]
        {
            Ok(self.random_u64()? as usize)
        }
        #[cfg(target_pointer_width = "32")]
        {
            Ok(self.random_u32()? as usize)
        }
    }

    fn random_i8(&mut self) -> RngResult<i8> {
        let mut buffer = [0u8; 1];
        self.0.read_exact(&mut buffer)?;
        Ok(i8::from_le_bytes(buffer))
    }

    fn random_i16(&mut self) -> RngResult<i16> {
        let mut buffer = [0u8; 2];
        self.0.read_exact(&mut buffer)?;
        Ok(i16::from_le_bytes(buffer))
    }

    fn random_i32(&mut self) -> RngResult<i32> {
        let mut buffer = [0u8; 4];
        self.0.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    fn random_i64(&mut self) -> RngResult<i64> {
        let mut buffer = [0u8; 8];
        self.0.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }

    fn random_f32(&mut self) -> RngResult<f32> {
        let mut buffer = [0u8; 4];
        self.0.read_exact(&mut buffer)?;
        let out = f32::from_le_bytes(buffer);
        if out.is_nan() {
            self.random_f32()
        } else {
            Ok(out)
        }
    }

    fn random_f64(&mut self) -> RngResult<f64> {
        let mut buffer = [0u8; 8];
        self.0.read_exact(&mut buffer)?;
        let out = f64::from_le_bytes(buffer);
        if out.is_nan() {
            self.random_f64()
        } else {
            Ok(out)
        }
    }

    fn random_bytes(&mut self, len: usize) -> RngResult<Vec<u8>> {
        let mut buffer = vec![0u8; len];
        self.0.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    fn random_string(&mut self, len: usize) -> RngResult<String> {
        let mut out = String::with_capacity(len);
        for _ in 0..len {
            out.push(self.random_ascii_char()?);
        }
        Ok(out)
    }

    fn random_latin_char(&mut self, uppercase: bool) -> RngResult<char> {
        let chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let idx = self.random_index(chars.len())?;
        let chosen_char = chars[idx];
        if uppercase {
            Ok(chosen_char.to_ascii_uppercase())
        } else {
            Ok(chosen_char)
        }
    }

    fn random_ascii_char(&mut self) -> RngResult<char> {
        let val = self.random_from_range_inclusive(32, 126)?;
        Ok(val as u8 as char)
    }

    fn random_bool(&mut self) -> RngResult<bool> {
        let b = self.random_u8()?;
        Ok((b & 1) == 1)
    }

    fn random_from_range_inclusive(&mut self, min: usize, max: usize) -> RngResult<usize> {
        if min < max {
            let range_size = max.saturating_sub(min).saturating_add(1);
            let rnd = self.random_usize()?;
            Ok(min.saturating_add(rnd % range_size))
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_range(&mut self, min: usize, max: usize) -> RngResult<usize> {
        if min < max {
            let range_size = max.saturating_sub(min);
            let rnd = self.random_usize()?;
            Ok(min.saturating_add(rnd % range_size))
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_u64_range(&mut self, min: u64, max: u64) -> RngResult<u64> {
        if min < max {
            let range_size = max.saturating_sub(min);
            let rnd = self.random_u64()?;
            Ok(min.saturating_add(rnd % range_size))
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_i_range(&mut self, min: isize, max: isize) -> RngResult<isize> {
        if min < max {
            let range_size = (max - min) as usize;
            let rnd = self.random_usize()?;
            Ok(min + (rnd % range_size) as isize)
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_i64_range(&mut self, min: i64, max: i64) -> RngResult<i64> {
        if min < max {
            let range_size = (max - min) as u64;
            let rnd = self.random_u64()?;
            Ok(min + (rnd % range_size) as i64)
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_i32_range(&mut self, min: i32, max: i32) -> RngResult<i32> {
        if min < max {
            let range_size = (max - min) as u32;
            let rnd = self.random_u32()?;
            Ok(min + (rnd % range_size) as i32)
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_f32_range(&mut self, min: f32, max: f32) -> RngResult<f32> {
        if min < max {
            let range_size = max.sub(min);
            let rng = self.random_f32()?;
            let random_index = rng.abs() % range_size;
            Ok(min.add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_from_f64_range(&mut self, min: f64, max: f64) -> RngResult<f64> {
        if min < max {
            let range_size = max.sub(min);
            let rng = self.random_f64()?;
            let random_index = rng.abs() % range_size;
            Ok(min.add(random_index))
        } else if min == max {
            Ok(min)
        } else {
            Err(RngError::Generic(format!(
                "Min '{min}' is larger than max '{max}'!"
            )))
        }
    }

    fn random_index(&mut self, collection_length: usize) -> RngResult<usize> {
        if collection_length >= 1 {
            self.random_with_ceiling(collection_length.saturating_sub(1))
        } else {
            Err(RngError::Generic(format!(
                "collection length '{collection_length}' is less than 1!"
            )))
        }
    }

    fn random_with_ceiling(&mut self, max: usize) -> RngResult<usize> {
        self.random_from_range_inclusive(usize::MIN, max)
    }

    fn random_with_floor(&mut self, min: usize) -> RngResult<usize> {
        self.random_from_range_inclusive(min, usize::MAX)
    }
}

pub mod prelude {
    use super::Tyche;
    use athena::rng_api::{RngApi, RngError};
    use std::io::Error as IoError;

    fn to_io_error(e: RngError) -> IoError {
        match e {
            RngError::Io(io) => io,
            RngError::Generic(s) => IoError::other(s),
        }
    }

    pub fn random_u8() -> Result<u8, IoError> {
        Tyche::new()?.random_u8().map_err(to_io_error)
    }

    pub fn random_u16() -> Result<u16, IoError> {
        Tyche::new()?.random_u16().map_err(to_io_error)
    }

    pub fn random_u32() -> Result<u32, IoError> {
        Tyche::new()?.random_u32().map_err(to_io_error)
    }

    pub fn random_u64() -> Result<u64, IoError> {
        Tyche::new()?.random_u64().map_err(to_io_error)
    }

    pub fn random_i8() -> Result<i8, IoError> {
        Tyche::new()?.random_i8().map_err(to_io_error)
    }

    pub fn random_i32() -> Result<i32, IoError> {
        Tyche::new()?.random_i32().map_err(to_io_error)
    }

    pub fn random_f32() -> Result<f32, IoError> {
        Tyche::new()?.random_f32().map_err(to_io_error)
    }

    pub fn random_string() -> Result<String, IoError> {
        // Compatibility: the old random_string was weirdly specific,
        // but let's just return a random string of length 10.
        Tyche::new()?.random_string(10).map_err(to_io_error)
    }

    pub fn random_latin_char(uppercase: bool) -> Result<char, IoError> {
        Tyche::new()?.random_latin_char(uppercase).map_err(to_io_error)
    }

    pub fn random_bool() -> Result<bool, IoError> {
        Tyche::new()?.random_bool().map_err(to_io_error)
    }

    pub fn random_from_range(start: usize, end: usize) -> Result<usize, IoError> {
        // The old implementation was inclusive on both ends for random_from_range
        Tyche::new()?.random_from_range_inclusive(start, end).map_err(to_io_error)
    }

    pub fn random_from_u64range(start: u64, end: u64) -> Result<u64, IoError> {
        // The old implementation was inclusive on both ends
        let mut t = Tyche::new()?;
        if start < end {
            let range_size = end.saturating_sub(start).saturating_add(1);
            let rnd = t.random_u64().map_err(to_io_error)?;
            Ok(start.saturating_add(rnd % range_size))
        } else if start == end {
            Ok(start)
        } else {
            Err(IoError::other(format!("Start '{start}' is larger than end '{end}'!")))
        }
    }

    pub fn random_from_f32range(start: f32, end: f32) -> Result<f32, IoError> {
        Tyche::new()?.random_from_f32_range(start, end).map_err(to_io_error)
    }

    pub fn random_from_i32range(start: i32, end: i32) -> Result<i32, IoError> {
        // The old implementation was inclusive on both ends
        let mut t = Tyche::new()?;
        if start < end {
            let range_size = (end as i64 - start as i64 + 1) as u64;
            let rnd = t.random_u32().map_err(to_io_error)? as u64;
            Ok((start as i64 + (rnd % range_size) as i64) as i32)
        } else if start == end {
            Ok(start)
        } else {
            Err(IoError::other(format!("Start '{start}' is larger than end '{end}'!")))
        }
    }

    pub fn random_index(collection_length: usize) -> Result<usize, IoError> {
        Tyche::new()?.random_index(collection_length).map_err(to_io_error)
    }

    pub fn random_with_ceiling(ceiling: usize) -> Result<usize, IoError> {
        Tyche::new()?.random_with_ceiling(ceiling).map_err(to_io_error)
    }

    pub fn random_with_floor(floor: usize) -> Result<usize, IoError> {
        Tyche::new()?.random_with_floor(floor).map_err(to_io_error)
    }
}
