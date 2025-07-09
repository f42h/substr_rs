# substr_rs

Extract substrings either by characters or by strings

### Examples
```rust
if let Some(substring_between_chars) = Substring::from("SomeSubstringData", 'e', 'D') {
    println!(
        "The substring of \"SomeSubstringData\" between the characters 'e' and 'D' is {}", 
        substring_between_chars
    );
    // Output: The substring of "SomeSubstringData" between the characters 'e' and 'D' is Substring
}

// OR

if let Some(substring_between_strings) = Substring::from("SomeSubstringData", "Some", "Data") {
    println!(
        "The substring of \"SomeSubstringData\" between the strings \"Some\" and \"Data\" is {}", 
        substring_between_strings
    );
    // Output: The substring of "SomeSubstringData" between the strings "Some" and "Data" is Substring
}
```