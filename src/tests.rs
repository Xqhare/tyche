use crate::prelude::*;
// Run with `cargo test -- --nocapture` for printout

#[test]
fn test_random_u8() {
    for _ in 0..10000 {
        let answ = random();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_range() {
    for _ in 0..10000 {
        let answ = random_from_range(0, 100);
        // println!("{:?}", answ);
        /* if answ.unwrap() == 100 {
            println!("DID IT 100!!");
        }
        if answ.unwrap() == 0 {
            println!("0! DID IT 0!");
        } */
        assert!(answ.is_some());
    }
}
