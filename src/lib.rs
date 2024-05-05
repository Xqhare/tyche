#[cfg(test)]
mod tests;

pub mod prelude {
    use std::{fs::File, io::Read};
    
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


    /// Will return `None` if end is bigger than start, or if random() fails.
    /// Will return `Some` wrapping a number inside the given range.
    pub fn random_from_range(start: usize, end: usize) -> Option<usize> {
        let rng = File::open("/dev/urandom");
        if rng.is_ok() && start < end {
            let range_size = end.saturating_sub(start).saturating_add(1);
            let seed = random();
            if seed.is_some() {
                let random_index = seed.unwrap() as usize % range_size;
                Some(start + random_index)
            } else {
                None
            }
        } else {
            None
        }
    }
}


