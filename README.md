# substr_rs

### Description
```
This Rust library provides a Substring struct with methods 
to extract substrings from a given string based on specified 
delimiters, regular expressions, or index ranges.
```

### Features
- Extract substrings using `character or string delimiters`.
- Extract substrings using `regular expressions`.
- Extract substrings using specified `start and end indices`.

### Setup
To use the Substring struct, 
- Add substr_rs to your Rust project by using cargo
```
cargo add substr_rs
```

- Or include substr_rs in your Cargo.toml
```toml
substr_rs = "0.3.0"
```

### Usage

- Import the substr_rs crate to your source file
```rust
use substr_rs::Substring;
```

### Methods

#### `from`
```
Extracts a substring from strval starting after the first occurrence 
of start and ending before the first occurrence of end.
```

```rust
pub fn from<T: ToString>(strval: &str, start: T, end: T) -> Option<String>
```

#### Parameters
- `strval`: The input string from which the substring will be extracted.
- `start`: The starting delimiter, where the substring begins.
- `end`: The ending delimiter, where the substring ends.

#### Returns
```
Option<String> containing the extracted substring if both delimiters 
are found; otherwise, None.
```

#### `from_regex`
```
Extracts a substring from strval starting after the first match of the 
start regex and ending before the first match of the end regex.
```

```rust
pub fn from_regex(strval: &str, start: &str, end: &str) -> Option<String>
```

#### Parameters
- `strval`: The input string from which the substring will be extracted.
- `start`: A regex pattern marking where the substring begins.
- `end`: A regex pattern marking where the substring ends.

#### Returns
```
Option<String> containing the extracted substring if both regex patterns 
match in the correct order; otherwise, None.
```

#### `from_index`
```
Extracts a substring from strval starting at the given start index and 
ending at the given end index.
```

```rust
pub fn from_index(strval: &str, start: usize, end: usize) -> Option<String>
```

#### Parameters
- `strval`: The input string from which the substring will be extracted.
- `start`: The starting index where the substring begins.
- `end`: The ending index where the substring ends.

#### Returns
```
Option<String> containing the extracted substring if the indices are valid; 
otherwise, None.
```

### Examples
- Extract a substring between two chars (yes, they can be the same):
```rust
let string_value = "SomeSubstringData";
let start_char: char = 'e';
let end_char: char = 'D';

if let Some(substr) = Substring::from(string_value, start_char, end_char) {
    println!(
        "The substring of `{}` between the characters `{}` and `{}` is {}", 
        string_value, start_char, end_char,
        substr
    );
}

// Output:
//  The substring of `SomeSubstringData` between the characters `e` and `D` is Substring
```

- Extract a substring between regex patterns
``` rust
let string_value = "<script id='main'>console.log('Hello, World!');</script>";
let start_pattern = "<script[^>]*>";
let end_pattern = "</script";

if let Some(substr) = Substring::from_regex(string_value, start_pattern, end_pattern) {
    println!(
        "The substring of `{}` between the patterns `{}` and `{}` is {}", 
        string_value, start_pattern, end_pattern,
        substr
    );
}

// Output:
//  The substring of `<script id='main'>console.log('Hello, World!');</script>` between the 
//  patterns `<script[^>]*>` and `</script` is console.log('Hello, World!');
```

## License
This project is published under the [MIT](https://github.com/f42h/substr_rs/blob/main/LICENSE) License. See the LICENSE file for more details.