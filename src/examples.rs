use crate::prelude::*;

#[test]
fn example_full_usage() {
    let random_number_u8: u8 = random_u8().expect("Failed to get u8");
    println!("Generated random u8: {}", random_number_u8);

    let random_number_u16: u16 = random_u16().expect("Failed to get u16");
    println!("Generated random u_u16: {}", random_number_u16);
   
    let random_number_u32: u32 = random_u32().expect("Failed to get u32");
    println!("Generated random u_u32: {}", random_number_u32);
    
    let random_number_u64: u64 = random_u64().expect("Failed to get u64");
    println!("Generated random u64: {}", random_number_u64);

    let random_number_i8: i8 = random_i8().expect("Failed to get i8");
    println!("Generated random i8: {}", random_number_i8);

    let random_number_i32: i32 = random_i32().expect("Failed to get i32");
    println!("Generated random i32: {}", random_number_i32);
    
    let random_number_f32: f32 = random_f32().expect("Failed to get f32");
    println!("Generated random f32: {}", random_number_f32);

    let random_string: String = random_string().expect("Failed to get string");
    println!("Generated random String: {}", random_string);

    let random_latin_char: char = random_latin_char(false).expect("Failed to get char");
    println!("Generated random latin char: {}", random_latin_char);

    let random_bool: bool = random_bool().expect("Failed to get bool");
    println!("Generated random bool: {}", random_bool);

    let chosen_element = random_from_range(0, 100).expect("Failed to get range");
    println!("Chosen element {chosen_element}, in range 0-100");

    let chosen_element_u64 = random_from_u64range(0, 100).expect("Failed to get u64 range");
    println!("Chosen element {chosen_element_u64}, in range 0-100");

    let chosen_element_f32 = random_from_f32range(0.1, 100.1).expect("Failed to get f32 range");
    println!("Chosen element {chosen_element_f32}, in range 0.1-100.1");

    let chosen_element_i32 = random_from_i32range(-100, 100).expect("Failed to get i32 range");
    println!("Chosen element {chosen_element_i32}, in range -100, 100");

    let collection = (0..100).collect::<Vec<usize>>();
    let random_index = random_index(collection.len()).expect("Failed to get index");
    println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);

    let random_ceiling = random_with_ceiling(100);
    println!("The random number between 0 and 100 is: {}", random_ceiling.expect("Failed to get ceiling"));

    let random_floor = random_with_floor(0);
    let max_usize = usize::MAX;
    println!("The random number between 0 and {} is: {}", max_usize, random_floor.expect("Failed to get floor"));
}

#[test]
fn example_random_u8() {
    let random_number: u8 = random_u8().unwrap();
    println!("Generated random u8: {}", random_number);
}

#[test]
fn example_random_u16() {
    let random_number: u16 = random_u16().unwrap();
    println!("Generated random u16: {}", random_number);
}

#[test]
fn example_random_u32() {
    let random_number: u32 = random_u32().unwrap();
    println!("Generated random u32: {}", random_number);
}

#[test]
fn example_random_u64() {
    let random_number: u64 = random_u64().unwrap();
    println!("Generated random u64: {}", random_number);
}

#[test]
fn example_random_i8() {
    let random_number: i8 = random_i8().unwrap();
    println!("Generated random i8: {}", random_number);
}

#[test]
fn example_random_i32() {
    let random_number: i32 = random_i32().unwrap();
    println!("Generated random i32: {}", random_number);
}

#[test]
fn example_random_f32() {
    let random_number: f32 = random_f32().unwrap();
    println!("Generated random f32: {}", random_number);
}

#[test]
fn example_random_string() {
    let random_string: String = random_string().unwrap();
    println!("Generated random String: {}", random_string);
}

#[test]
fn example_random_latin_char() {
    let random_char: char = random_latin_char(true).unwrap();
    println!("Generated random char: {}", random_char);
}

#[test]
fn example_random_bool() {
    let random_bool: bool = random_bool().unwrap();
    println!("Generated random bool: {}", random_bool);
}

#[test]
fn example_random_from_range() {
    let chosen_element = random_from_range(0, 100).unwrap();
    println!("Chosen element {chosen_element}, in range 0-100");
}

#[test]
fn example_random_from_u64range() {
    let chosen_element = random_from_u64range(0, 100).unwrap();
    println!("Chosen element {chosen_element}, in range 0-100");
}

#[test]
fn example_random_from_f32range() {
    let chosen_element = random_from_f32range(0.1, 100.1).unwrap();
    println!("Chosen element {chosen_element}, in range 0.1-100.1");
}

#[test]
fn example_random_from_i32range() {
    let chosen_element = random_from_i32range(-100, 100).unwrap();
    println!("Chosen element {chosen_element}, in range -100, 100");
}

#[test]
fn example_random_index() {
    let collection = (0..100).collect::<Vec<usize>>();
    let random_index = random_index(collection.len()).unwrap();
    println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);
}

#[test]
fn example_random_with_ceiling() {
    let answ = random_with_ceiling(100);
    println!("The random number between 0 and 100 is: {}", answ.unwrap());
}

#[test]
fn example_random_with_floor() {
    let answ = random_with_floor(0);
    let max_usize = usize::MAX;
    println!("The random number between 0 and {} is: {}", max_usize, answ.unwrap());
}
