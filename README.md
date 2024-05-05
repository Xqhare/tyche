# Tyche Yields Cryptographic High-Quality Entropy: A Rust randomness library

Tyche (pronounced TY-kee) is a Rust library designed for generating high-quality pseudorandom numbers. It provides various functionalities for different use cases, emphasizing security and ease of use.

## The Name: Tyche

The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate. Just as Tyche's influence brought about both fortunate and unfortunate events, this library aims to provide a reliable source of randomness for various applications.

Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.

It is also a recursive acronym:

- Tyche
- Yields
- Cryptographic
- High-Quality
- Entropy

## Features

- Tested and developed on Linux only, could work on all UNIX systems like MacOS however.
- Secure Randomness: Tyche utilizes cryptographically secure pseudorandom number generators (CSPRNGs) to ensure the generated randomness is unpredictable and statistically sound.
- Ease of Use: Tyche provides a user-friendly API with intuitive functions for generating random values of different data types.
- Test driven development, meaning 100% test coverage, guaranteeing stability.

## Getting Started

To add Tyche to your project, include it in your Cargo.toml dependencies.

## Documentation

For detailed usage instructions and a comprehensive list of functionalities, refer to the library's inbuilt API documentation. You can generate it by running ```cargo doc```.

Examples have been written to be runable, small in scope and easily followable.

### Basic Usage:

Import the functions you need, or just type `use tyche::prelude::*` to get it all.

#### Function overview

The most basic and often used function is `random()`. It returns an option of a `u8`, as the CSPRNG is not guaranteed to have enough entropy to produce a random number, this is highly unlikely, but possible.

```rust
use tyche::prelude::random;

fn main() {
  let random_number: u8 = random().unwrap();
  println!("Generated random u8: {}", random_number);
}
```

The probably most used and usefull function is `random_from_range(start, end)`. It takes in a start and end `usize` and returns a number in between them. The range is inclusive on both sides.

```rust
use tyche::prelude::random_from_range;

fn main() {
  let chosen_element = random_from_range(0, 100).unwrap();
  println!("Chosen element {chosen_element}, in range 0-100");
}
```
