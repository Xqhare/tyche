
pub mod prelude {
    use std::{fs::File, io::Read};
    
    pub fn random() -> Option<u8> {
        let mut rng = File::open("/dev/urandom");
        if rng.is_ok() {
            let mut buffer = [0u8; 1];
            if rng.unwrap().read_exact(&mut buffer).is_ok() {
                return Some(buffer[0]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}


