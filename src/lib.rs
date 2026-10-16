#[doc = include_str!("../README.md")]
#[cfg(test)]
mod examples;
#[cfg(test)]
mod tests;

#[cfg(target_family = "windows")]
mod ffi {
    #[link(name = "bcryptprimitives")]
    unsafe extern "system" {
        pub fn ProcessPrng(pbData: *mut u8, cbData: usize) -> i32;
    }
}
#[cfg(target_family = "unix")]
use std::{fs::File, io::Read};

pub use athena::rng_api::{RngApi, RngError, RngResult};
use std::ops::{Add, Sub};

/// A CSPRNG implementation using `/dev/urandom`.
pub struct Tyche {
    pool: [u8; 256],
    idx: usize,
}

impl Tyche {
    /// Create a new Tyche instance.
    ///
    /// ## Errors
    /// Returns `IoError` if `/dev/urandom` cannot be opened.
    pub fn new() -> RngResult<Tyche> {
        let pool = [0u8; 256];
        let mut tyche = Tyche { pool, idx: 0 };
        tyche.regen_pool()?;
        Ok(tyche)
    }
    fn adv_idx(&mut self) -> RngResult<()> {
        let next_idx = self.idx.wrapping_add(1);
        if next_idx >= self.pool.len() {
            self.idx = 0;
            self.regen_pool()?;
        } else {
            self.idx = next_idx;
        }
        Ok(())
    }
    #[cfg(target_family = "unix")]
    fn regen_pool(&mut self) -> RngResult<()> {
        let mut f = File::open("/dev/urandom")?;
        f.read_exact(&mut self.pool)?;
        Ok(())
    }
    #[cfg(target_family = "windows")]
    fn regen_pool(&mut self) -> RngResult<()> {
        unsafe {
            ffi::ProcessPrng(self.pool.as_mut_ptr(), 256);
            Ok(())
        }
    }
}

impl RngApi for Tyche {
    fn random_u8(&mut self) -> RngResult<u8> {
        let out = self.pool[self.idx];
        self.adv_idx()?;
        Ok(out)
    }

    fn random_u16(&mut self) -> RngResult<u16> {
        let buffer = [self.random_u8()?, self.random_u8()?];
        Ok(u16::from_le_bytes(buffer))
    }

    fn random_u32(&mut self) -> RngResult<u32> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
        Ok(u32::from_le_bytes(buffer))
    }

    fn random_u64(&mut self) -> RngResult<u64> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
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
        let buffer = [self.random_u8()?; 1];
        Ok(i8::from_le_bytes(buffer))
    }

    fn random_i16(&mut self) -> RngResult<i16> {
        let buffer = [self.random_u8()?, self.random_u8()?];
        Ok(i16::from_le_bytes(buffer))
    }

    fn random_i32(&mut self) -> RngResult<i32> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
        Ok(i32::from_le_bytes(buffer))
    }

    fn random_i64(&mut self) -> RngResult<i64> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
        Ok(i64::from_le_bytes(buffer))
    }

    fn random_f32(&mut self) -> RngResult<f32> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
        let out = f32::from_le_bytes(buffer);
        if out.is_nan() {
            self.random_f32()
        } else {
            Ok(out)
        }
    }

    fn random_f64(&mut self) -> RngResult<f64> {
        let buffer = [
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
            self.random_u8()?,
        ];
        let out = f64::from_le_bytes(buffer);
        if out.is_nan() {
            self.random_f64()
        } else {
            Ok(out)
        }
    }

    fn random_bytes(&mut self, len: usize) -> RngResult<Vec<u8>> {
        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(self.random_u8()?);
        }
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
    use athena::rng_api::{RngApi, RngError, RngResult};

    pub fn random_u8() -> RngResult<u8> {
        Tyche::new()?.random_u8()
    }

    pub fn random_u16() -> RngResult<u16> {
        Tyche::new()?.random_u16()
    }

    pub fn random_u32() -> RngResult<u32> {
        Tyche::new()?.random_u32()
    }

    pub fn random_u64() -> RngResult<u64> {
        Tyche::new()?.random_u64()
    }

    pub fn random_i8() -> RngResult<i8> {
        Tyche::new()?.random_i8()
    }

    pub fn random_i32() -> RngResult<i32> {
        Tyche::new()?.random_i32()
    }

    pub fn random_f32() -> RngResult<f32> {
        Tyche::new()?.random_f32()
    }

    pub fn random_string() -> RngResult<String> {
        // Compatibility: the old random_string was weirdly specific,
        // but let's just return a random string of length 10.
        Tyche::new()?.random_string(10)
    }

    pub fn random_latin_char(uppercase: bool) -> RngResult<char> {
        Tyche::new()?.random_latin_char(uppercase)
    }

    pub fn random_bool() -> RngResult<bool> {
        Tyche::new()?.random_bool()
    }

    pub fn random_from_range(start: usize, end: usize) -> RngResult<usize> {
        // The old implementation was inclusive on both ends for random_from_range
        Tyche::new()?.random_from_range_inclusive(start, end)
    }

    pub fn random_from_u64range(start: u64, end: u64) -> RngResult<u64> {
        // The old implementation was inclusive on both ends
        let mut t = Tyche::new()?;
        if start < end {
            let range_size = end.saturating_sub(start).saturating_add(1);
            let rnd = t.random_u64()?;
            Ok(start.saturating_add(rnd % range_size))
        } else if start == end {
            Ok(start)
        } else {
            Err(RngError::Generic(format!(
                "Start '{start}' is larger than end '{end}'!"
            )))
        }
    }

    pub fn random_from_f32range(start: f32, end: f32) -> RngResult<f32> {
        Tyche::new()?.random_from_f32_range(start, end)
    }

    pub fn random_from_i32range(start: i32, end: i32) -> RngResult<i32> {
        // The old implementation was inclusive on both ends
        let mut t = Tyche::new()?;
        if start < end {
            let range_size = (end as i64 - start as i64 + 1) as u64;
            let rnd = t.random_u32()? as u64;
            Ok((start as i64 + (rnd % range_size) as i64) as i32)
        } else if start == end {
            Ok(start)
        } else {
            Err(RngError::Generic(format!(
                "Start '{start}' is larger than end '{end}'!"
            )))
        }
    }

    pub fn random_index(collection_length: usize) -> RngResult<usize> {
        Tyche::new()?.random_index(collection_length)
    }

    pub fn random_with_ceiling(ceiling: usize) -> RngResult<usize> {
        Tyche::new()?.random_with_ceiling(ceiling)
    }

    pub fn random_with_floor(floor: usize) -> RngResult<usize> {
        Tyche::new()?.random_with_floor(floor)
    }
}
