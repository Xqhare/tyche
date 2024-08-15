# Tyche Yields Cryptographic High-Quality Entropy: A Rust randomness library

Tyche (pronounced TY-kee) is a Rust library designed for generating high-quality pseudo-random numbers. It provides various functionalities for different use cases, emphasizing security and ease of use.

Tyche is stable, feature complete (for now) and production ready.

Tyche is not a replacement for a crate like [rand](https://crates.io/crates/rand) but is significantly smaller and more focused on its feature set.

Tyche is a lightweight wrapper around the inbuilt CSPRNG in UNIX systems, providing convenient access to it with `random_u8()` and converting the entropy into several useful data-types.

This project was tested and developed on Linux, but should / could work on macOS. No one has tried yet though.

No windows support at the moment.

## Features

- Tested and developed on Linux only, could work on all UNIX systems like macOS, however.
- Secure Randomness: Tyche utilizes cryptographically secure pseudo-random number generators (CSPRNGs) to ensure the generated randomness is unpredictable and statistically sound.
- Ease of Use: Tyche provides a user-friendly API with intuitive functions for generating random values of different data types.
- Test driven development, meaning 100% test coverage, guaranteeing stability.
- 0 dependencies, all code in one file making it easily auditable and understandable.
- Fully documented with examples - more lines of documentation than lines of code!
- Several generators, providing unsigned or signed integers, floating point numbers, or characters.

## Random Number Generators

- random u8
- random u16
- random u32
- random u64
- random i8
- random i32
- random f32
- random String
- random latin char
- random bool
- random in range of 2 usize's
- random in range of 2 u64
- random in range of 2 f32
- random in range of 2 i32
- random index
- random usize with ceiling
- random usize with floor

## Getting Started

To add Tyche to your project, include it in your Cargo.toml dependencies.

```toml
tyche = { git = "https://github.com/Xqhare/tyche" }
```

Then run `cargo update` to download it from this GitHub repo.

[Learn more.](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

## Documentation

For detailed usage instructions and a comprehensive list of functionalities, refer to the library's inbuilt API documentation. You can generate it by running ```cargo doc```.

Examples have been written to be runable, small in scope and easily followable.

### Basic Usage:

Import the functions you need, or just 
```rust
use tyche::prelude::*;

```
to get it all and take a look.

## Function examples:

For a detailed explanation of a function, please refer to it's dedicated documentation page.

You can copy and run this code to see all functions of tyche in action!

```rust
use tyche::prelude::*;

fn main() {
    let random_number_u8: u8 = random_u8().unwrap();
    println!("Generated random u8: {}", random_number_u8);

    let random_number_u16: u16 = random_u16().unwrap();
    println!("Generated random u_u16: {}", random_number_u16);

    let random_number_u32: u32 = random_u32().unwrap();
    println!("Generated random u_u32: {}", random_number_u32);

    let random_number_u64: u64 = random_u64().unwrap();
    println!("Generated random u64: {}", random_number_u64);

    let random_number_i8: i8 = random_i8().unwrap();
    println!("Generated random i8: {}", random_number_i8);

    let random_number_i32: i32 = random_i32().unwrap();
    println!("Generated random i32: {}", random_number_i32);

    let random_number_f32: f32 = random_f32().unwrap();
    println!("Generated random f32: {}", random_number_f32);

    let random_string: String = random_string(true).unwrap();
    println!("Generated random String: {}", random_string);

    let random_latin_char: char = random_latin_char().unwrap();
    println!("Generated random latin char: {}", random_latin_char);

    let random_bool: bool = random_bool().unwrap();
    println!("Generated random bool: {}", random_bool);

    let chosen_element = random_from_range(0, 100).unwrap();
    println!("Chosen element {chosen_element}, in range 0-100");

    let chosen_element_u64 = random_from_u64range(0, 100).unwrap();
    println!("Chosen element {chosen_element_u64}, in range 0-100");

    let chosen_element_f32 = random_from_f32range(0.1, 100.1).unwrap();
    println!("Chosen element {chosen_element_f32}, in range 0.1-100.1");

    let chosen_element_i32 = random_from_i32range(-100, 100).unwrap();
    println!("Chosen element {chosen_element_i32}, in range -100, 100");

    let collection = (0..100).collect::<Vec<usize>>();
    let random_index = random_index(collection.len()).unwrap();
    println!("Chosen index {}; Number at index {}", random_index, collection[random_index]);

    let random_ceiling = random_with_ceiling(100);
    println!("The random number between 0 and 100 is: {}", random_ceiling.unwrap());

    let random_floor = random_with_floor(0);
    let max_usize = usize::MAX;
    println!("The random number between 0 and {} is: {}", max_usize, random_floor.unwrap());

    let random_bool = random_bool().unwrap();
    println!("Random bool: {}", random_bool);
}
```

## Technical Details

All functions return a `Result()`. This is because of the random number generator used on the backend. It can run out of entropy, something that is highly unlikely but possible, or the program can not open `/dev/urandom`. If you get a `Err()` back the second reason is the most likely candidate as I have not encountered one `Err()` value not caused by this.

To generate a cryptographically secure pseudo-random number, Tyche casts as many random bytes as needed to construct it, and then combines their bytes using little Endian byte ordering.
Mainly because it is better optimised for x86 and ARM processors.

The `random_string` function is considered unstable and may change in the future.
It generates a random character by mapping random noise as `utf8` characters, so weird output is to be expected and will need to be sanatised. This is an unsafe and non-standard way of doing things, but it was fun building it.

`random_from_range` uses a 32bit seeded RNG, for 64bit seeded RNG please use `random_from_u64range`.

## The Name: Tyche

The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate. Just as Tyche's influence brought about both fortunate and unfortunate events, this library aims to provide a reliable source of randomness for various applications.

Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.

It is also a recursive acronym:

- Tyche
- Yields
- Cryptographic
- High-Quality
- Entropy

## License

Tyche is available under the Eclipse Public License 2.0.
