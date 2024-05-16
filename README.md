# Tyche Yields Cryptographic High-Quality Entropy: A Rust randomness library

Tyche (pronounced TY-kee) is a Rust library designed for generating high-quality pseudorandom numbers. It provides various functionalities for different use cases, emphasizing security and ease of use.

Tyche is stable, feature complete (for now) and production ready.

Tyche is not a replacement for a crate like [rand](https://crates.io/crates/rand) but is significantly smaller and more focused on its feature set.

Tyche is a lightweigt wrapper around the inbuilt CSPRNG in UNIX systems, providing convinient access to it with `random_u8()` and converting the entropy into several useful datatypes.

## Features

- Tested and developed on Linux only, could work on all UNIX systems like MacOS however.
- Secure Randomness: Tyche utilizes cryptographically secure pseudorandom number generators (CSPRNGs) to ensure the generated randomness is unpredictable and statistically sound.
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
- random in range of 2 usizes
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

Then run `cargo update` to download it from the github repo.

[Learn more.](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

## Documentation

For detailed usage instructions and a comprehensive list of functionalities, refer to the library's inbuilt API documentation. You can generate it by running ```cargo doc```.

Examples have been written to be runable, small in scope and easily followable.

All functions return a `Option()`. This is because of the random number generator used on the backend. It can run out of entropy, something that is highly unlikely but possible, or the program can not open `/dev/urandom`. If you get a `None` back the second reason is the most likely canditate as I have not encountered one `None` value not caused by this.

### Basic Usage:

Import the functions you need, or just 
```rust
use tyche::prelude::*;

```
to get it all and take a look.

I would advice to start with the full example in the docs (or start of lib.rs). I would, I wrote it after all.

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

But what does that mean? (Explained by a non lawyer)
- If you change the code and use it somewhere the source code has to be made available.
- If you simply use it, and keep it in a seperate unchanged file (using it as a dependency for example) you are NOT required to make source code available.
- Include a copy of the full license text and the original copyright notice.
- License any derivative works of the library under the same or later version of the EPL.
- If you commecially distribute a product tat includes an EPL-licensed program, you are required to defend that program's contributers from any lawsuits/legal damages that may arise involving your product.
