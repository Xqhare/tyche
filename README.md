# Tyche Yields Cryptographic High-Quality Entropy: A Rust randomness library

Tyche (pronounced TY-kee) is a Rust library designed for generating high-quality pseudorandom numbers. It provides various functionalities for different use cases, emphasizing security and ease of use.

Tyche is not a replacement for a crate like [rand](https://crates.io/crates/rand) but is significantly smaller and more focused on its feature set.

## Features

- Tested and developed on Linux only, could work on all UNIX systems like MacOS however.
- Secure Randomness: Tyche utilizes cryptographically secure pseudorandom number generators (CSPRNGs) to ensure the generated randomness is unpredictable and statistically sound.
- Ease of Use: Tyche provides a user-friendly API with intuitive functions for generating random values of different data types.
- Test driven development, meaning 100% test coverage, guaranteeing stability.
- 0 dependencies, all code in one file making it easily auditable and understandable.
- Fully documented with examples

## RNG's

- random u8
- random u32
- random u64
- random f32
- random in range of 2 usizes
- random index
- random usize with ceiling
- random usize with floor
- random String

## The Name: Tyche

The name Tyche is inspired by the Greek goddess of fortune, Tyche (Τύχη). In Greek mythology, Tyche personified luck, fortune, and fate. Just as Tyche's influence brought about both fortunate and unfortunate events, this library aims to provide a reliable source of randomness for various applications.

Just like the goddess was often depicted blindfolded, emphasizing the impartiality of fate, Tyche the library strives to deliver unpredictable and unbiased random numbers.

It is also a recursive acronym:

- Tyche
- Yields
- Cryptographic
- High-Quality
- Entropy

## Getting Started

To add Tyche to your project, include it in your Cargo.toml dependencies.

## Documentation

For detailed usage instructions and a comprehensive list of functionalities, refer to the library's inbuilt API documentation. You can generate it by running ```cargo doc```.

Examples have been written to be runable, small in scope and easily followable.

All functions return a `Option()`. This is because of the random number generator used on the backend. It can run out of entropy, something that is highly unlikely but possible, or the program can not open `/dev/urandom`. If you get a `None` back the second reason is the most likely canditate as I have not encountered one `None` value not caused by this exact reason.

### Basic Usage:

Import the functions you need, or just type `use tyche::prelude::*` to get it all.

```rust
use tyche::prelude::*;

```

