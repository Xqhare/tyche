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

    Secure Randomness: Tyche utilizes cryptographically secure pseudorandom number generators (CSPRNGs) to ensure the generated randomness is unpredictable and statistically sound.
    Variety of Distributions: The library offers functions to generate random numbers following various probability distributions, including uniform, normal, exponential, and more.
    Ease of Use: Tyche provides a user-friendly API with intuitive functions for generating random values of different data types.

## Getting Started

To add Tyche to your project, include it in your Cargo.toml dependencies.

## Usage:

```rust
use tyche::random;

fn main() {
  let random_number: u32 = random();
  println!("Generated random u32: {}", random_number);
}
```

## Documentation

For detailed usage instructions and a comprehensive list of functionalities, refer to the library's inbuilt API documentation. You can generate it by running ```cargo doc```.
