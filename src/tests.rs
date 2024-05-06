use crate::prelude::*;
// Run with `cargo test -- --nocapture` for printout

#[test]
fn test_random_str() {
    for _ in 0..1000000 {
        let tmp = random_string();
        // println!("{:?}", tmp);
        assert!(tmp.is_some());
    }
    
}

#[test]
fn test_random_u8() {
    for _ in 0..1000000 {
        let answ = random_u8();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_u16() {
    for _ in 0..1000000 {
        let answ = random_u16();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_u64() {
    for _ in 0..1000000 {
        let answ = random_u64();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_f32() {
    for _ in 0..1000000 {
        let answ = random_f32();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_range() {
    let mut found0 = false;
    let mut found100 = false;
    for _ in 0..1000000 {
        let answ = random_from_range(0, 100);
        // println!("{:?}", answ);
        if answ.unwrap() == 100 {
            // println!("DID IT 100!!");
            found100 = true;
        }
        if answ.unwrap() == 0 {
            // println!("0! DID IT 0!");
            found0 = true;
        }
        assert!(answ.is_some());
    }
    assert!(found0 && found100);
}

#[test]
fn test_random_floor() {
    for n in 0..1000000 {
        let answ = random_with_floor(n);
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_ceiling() {
    for n in 1000000..2000000  {
        let answ = random_with_ceiling(n);
        // println!("The random number between 0 and {} is: {}", n, answ.unwrap());
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_index() {
    let collection = (0..1000).collect::<Vec<usize>>();
    let mut found999 = false;
    let mut found0 = false;
    for _ in 0..1000000 {
        let random_index = random_index(collection.len());
        assert!(random_index.is_some());
        // println!("{:?}", random_index);
        // These two tests test the same thing, I am more comfortable this way though.
        assert!(random_index.unwrap() < 1000);
        let _index_test = collection[random_index.unwrap()];
        if random_index.unwrap() == 0 {
            found0 = true;
        } else if random_index.unwrap() == 999 {
            found999 = true;
        }
    }
    assert!(found0 && found999);
}
