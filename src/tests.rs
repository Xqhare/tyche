use crate::prelude::*;
// Run with `cargo test -- --nocapture` for printout

#[test]
fn test_random_str() {
    for _ in 0..1500000 {
        let tmp = random_string();
        // println!("{:?}, len: {:?}", tmp, tmp.clone().unwrap().len());
        assert!(tmp.is_ok());
    }
    
}

#[test]
fn test_random_i8() {
    for _ in 0..1500000 {
            let answ = random_i8();
            // println!("{:?}", answ);
            assert!(answ.is_ok());
        }
}

#[test]
fn test_random_i32() {
    for _ in 0..1500000 {
            let answ = random_i32();
            // println!("{:?}", answ);
            assert!(answ.is_ok());
        }
}

#[test]
fn test_random_u8() {
    for _ in 0..1500000 {
        let answ = random_u8();
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_u16() {
    for _ in 0..1500000 {
        let answ = random_u16();
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_u32() {
    for _ in 0..1500000 {
        let answ = random_u32();
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_u64() {
    for _ in 0..1500000 {
        let answ = random_u64();
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_f32() {
    for _ in 0..1500000 {
        let answ = random_f32();
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_range() {
    let mut found0 = false;
    let mut found100 = false;
    for _ in 0..1500000 {
        let answ = random_from_range(0, 100).unwrap();
        // println!("{:?}", answ);
        if answ == 100 {
            // println!("DID IT 100!!");
            found100 = true;
        }
        if answ == 0 {
            // println!("0! DID IT 0!");
            found0 = true;
        }
    }
    assert!(found0 && found100);
}

#[test]
fn test_random_i32range() {
    let mut found_neg100 = false;
    let mut found100 = false;
    for _ in 0..1500000 {
        let answ = random_from_i32range(-100, 100).unwrap();
        // println!("{:?}", answ);
        if answ == 100 {
            // println!("DID IT 100!!");
            found100 = true;
        }
        if answ == -100 {
            // println!("-100! DID IT -100!");
            found_neg100 = true;
        }
    }
    assert!(found_neg100 && found100);
}

#[test]
fn test_random_u64range() {
    let mut found0 = false;
    let mut found100 = false;
    for _ in 0..1500000 {
        let answ = random_from_u64range(0, 100).unwrap();
        // println!("{:?}", answ);
        if answ == 100 {
            // println!("DID IT 100!!");
            found100 = true;
        }
        if answ == 0 {
            // println!("0! DID IT 0!");
            found0 = true;
        }
    }
    assert!(found0 && found100);
}

#[test]
fn test_random_f32range() {
    let mut found0_1 = false;
    let mut found100_1 = false;
    //let mut tmpvec: Vec<f32> = Default::default();
    for _ in 0..1500000 {
        let answ = random_from_f32range(0.1, 100.1).unwrap();
        //println!("{:?}", answ);
        if !found0_1 && answ == 0.1 {
            found0_1 = true;
        } 
        if !found100_1 && answ > 100.09 {
            found100_1 = true;
        }
        assert!(answ >= 0.1 && answ <= 100.1);
        //tmpvec.push(answ.unwrap());
    }
    //println!("{:?}", tmpvec.into_iter().reduce(f32::max).unwrap());
    assert!(found0_1 && found100_1);
}

#[test]
fn test_random_floor() {
    for n in 0..1500000 {
        let answ = random_with_floor(n);
        // println!("{:?}", answ);
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_ceiling() {
    for n in 1000000..2500000  {
        let answ = random_with_ceiling(n);
        // println!("The random number between 0 and {} is: {}", n, answ.unwrap());
        assert!(answ.is_ok());
    }
}

#[test]
fn test_random_index() {
    let collection = (0..1000).collect::<Vec<usize>>();
    let mut found999 = false;
    let mut found0 = false;
    for _ in 0..1500000 {
        let random_index = random_index(collection.len());
        assert!(random_index.is_ok());
        // println!("{:?}", random_index);
        // These two tests test the same thing, I am more comfortable this way though.
        assert!(random_index.as_ref().unwrap() < &1000);
        let _index_test = collection[*random_index.as_ref().unwrap()];
        if random_index.as_ref().unwrap() == &0 {
            found0 = true;
        } else if random_index.unwrap() == 999 {
            found999 = true;
        }
    }
    assert!(found0 && found999);
}
