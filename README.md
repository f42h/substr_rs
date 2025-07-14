# substr_rs

### Description
```
Extract substrings either by characters or by strings
```

### Usage
- Add substr_rs to Cargo.toml
```
cargo add substr_rs
```

- Import the crate
```rust
use substr_rs::Substring;
```

### Examples
- Extract a substring between two chars (yes, they can be the same):
```rust
if let Some(substring_between_chars) = Substring::from("SomeSubstringData", 'e', 'D') {
    println!(
        "The substring of \"SomeSubstringData\" between the characters 'e' and 'D' is {}", 
        substring_between_chars
    );
    // Output: The substring of "SomeSubstringData" between the characters 'e' and 'D' is Substring
}
```

- Extract a substring between two strings:
```rust
if let Some(substring_between_strings) = Substring::from("SomeSubstringData", "Some", "Data") {
    println!(
        "The substring of \"SomeSubstringData\" between the strings \"Some\" and \"Data\" is {}", 
        substring_between_strings
    );
    // Output: The substring of "SomeSubstringData" between the strings "Some" and "Data" is Substring
}
```