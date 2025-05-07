# colprint

[![Crates.io](https://img.shields.io/crates/v/colprint.svg)](https://crates.io/crates/colprint)
[![Docs.rs](https://docs.rs/colprint/badge.svg)](https://docs.rs/colprint)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/FreddyWordingham/colprint/blob/main/LICENSE)

A lightweight Rust library for neatly printing data in aligned columns with automatic width calculation and customizable separators.

## Features

- **Simple API**: Format complex data with an intuitive macro syntax
- **Automatic Width Calculation**: Columns automatically adjust to content size
- **Custom Column Widths**: Manually specify column widths when needed
- **Flexible Formatting**: Support for `Display`, `Debug`, and `PrettyDebug` formatters
- **Custom Separators**: Define any text as column separators
- **Multi-line Support**: Properly handles content with line breaks
- **Unicode Compatible**: Works correctly with multi-byte characters

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colprint = "0.0.0"
```

## Usage

### Basic Usage

```rust
use colprint::colprint;

fn main() {
    let name = "Alice";
    let age = 30;

    // Basic display formatting
    colprint!("{} | {}", name, age);
}
```

```txt
Alice | 30
```

### Display vs Debug Formatting

The `colprint!` macro is most useful when printing complex (multi-line) data structures:

```rust
use colprint::colprint;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    country: String,
    job: String,
    hobby: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nAge: {}\nCountry: {}\nJob {}\nHobby {}",
            self.name, self.age, self.country, self.job, self.hobby
        )
    }
}

fn main() {
    let bob = Person {
        name: "Bob".to_string(),
        age: 25,
        country: "Canada".to_string(),
        job: "Data Scientist".to_string(),
        hobby: "Photography".to_string(),
    };
    let jessica = Person {
        name: "Jessica".to_string(),
        age: 28,
        country: "USA".to_string(),
        job: "Software Engineer".to_string(),
        hobby: "Hiking".to_string(),
    };

    colprint!("{}\t{}", bob, jessica);
}
```

```txt
Name: Bob         	Name: Jessica
Age: 25           	Age: 28
Country: Canada   	Country: USA
Job Data Scientist	Job Software Engineer
Hobby Photography 	Hobby Hiking
```

Also supports debug formatting:

```rust
    colprint!("{:#?}\t{:#?}", bob, jessica);
```

```txt
Person {                  	Person {
    name: "Bob",          	    name: "Jessica",
    age: 25,              	    age: 28,
    country: "Canada",    	    country: "USA",
    job: "Data Scientist",	    job: "Software Engineer",
    hobby: "Photography", 	    hobby: "Hiking",
}                         	}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
