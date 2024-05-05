use crate::prelude::*;
// Run with `cargo test -- --nocapture` for printout

#[test]
fn test_random_u8() {
    for _ in 0..100000 {
        let answ = random();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_u64() {
    for _ in 0..100000 {
        let answ = random_u64();
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_range() {
    let mut found0 = false;
    let mut found100 = false;
    for _ in 0..100000 {
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
    for n in 0..100000 {
        let answ = random_with_floor(n);
        // println!("{:?}", answ);
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_ceiling() {
    for n in 100000..200000  {
        let answ = random_with_ceiling(n);
        // println!("The random number between 0 and {} is: {}", n, answ.unwrap());
        assert!(answ.is_some());
    }
}

#[test]
fn test_random_index() {
    let collection = (0..100).collect::<Vec<usize>>();
    let mut found99 = false;
    let mut found0 = false;
    for _ in 0..100000 {
        let random_index = random_index(collection.len());
        assert!(random_index.is_some());
        // println!("{:?}", random_index);
        // These two tests test the same thing, I am more comfortable this way though.
        assert!(random_index.unwrap() < 100);
        let _index_test = collection[random_index.unwrap()];
        if random_index.unwrap() == 0 {
            found0 = true;
        } else if random_index.unwrap() == 99 {
            found99 = true;
        }
    }
    assert!(found0 && found99);
}
