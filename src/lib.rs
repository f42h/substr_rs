/*  
*  MIT License
*  
*  Copyright (c) 2025 f42h
*  
*  Permission is hereby granted, free of charge, to any person obtaining a copy
*  of this software and associated documentation files (the "Software"), to deal
*  in the Software without restriction, including without limitation the rights
*  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
*  copies of the Software, and to permit persons to whom the Software is
*  furnished to do so, subject to the following conditions:
*  
*  The above copyright notice and this permission notice shall be included in all
*  copies or substantial portions of the Software.
*  
*  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
*  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
*  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
*  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
*  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
*  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
*  SOFTWARE.
*/

use regex::Regex;

pub struct Substring;
impl Substring {
    /// Extracts a substring from `strval` starting after the first occurrence of `start`
    /// and ending before the first occurrence of `end`.
    ///
    /// # Parameters
    /// - `strval`: The input string from which the substring will be extracted.
    /// - `start`: The starting delim, where the substring begins.
    /// - `end`: The ending delim, where the substring ends.
    ///
    /// # Returns
    /// - `Option<String>` containing the extracted substring if both delimiters are found
    /// ; otherwise, `None`.
    #[must_use]
    pub fn from<T: ToString>(strval: &str, start: T, end: T) -> Option<String> {
        // Convert the start and end delimiters to strings
        let conv_start = start.to_string();
        let conv_end = end.to_string();

        // Find the first occurrence of the start delimiter in the input string
        let start_pos_idx = strval.find(&conv_start)?;

        // Calculate the next index after the start delim location
        let start_pos_from_idx = start_pos_idx + conv_start.len();

        // Find the next occurrence of the `end` delim, starting the search directly 
        // after the occurrence of the `start` delim
        let end_pos_to_idx = strval[start_pos_from_idx..]
            .find(&conv_end)
            .map(|idx| idx + start_pos_from_idx)?;

        if start_pos_idx < end_pos_to_idx {
            // Return the substring section
            return Some(String::from(&strval[start_pos_from_idx..end_pos_to_idx]));
        }

        None
    }

    /// Extracts a substring from `strval` starting after the first match of the `start` regex
    /// and ending before the first match of the `end` regex.
    ///
    /// # Parameters
    /// - `strval`: The input string from which the substring will be extracted.
    /// - `start`: A regex pattern marking where the substring begins.
    /// - `end`: A regex pattern marking where the substring ends.
    ///
    /// # Returns
    /// - `Option<String>` containing the extracted substring if both regex patterns match
    ///   in correct order; otherwise, `None`.
    #[must_use]
    pub fn from_regex(strval: &str, start: &str, end: &str) -> Option<String> {
        // Compile the regex patterns
        let start_re = Regex::new(start).ok()?;
        let end_re = Regex::new(end).ok()?;

        // Find the first match of the start pattern in the input string
        let start_match = start_re.find(strval)?;
        // Calculate the index immediately after the start match
        let start_pos_from_idx = start_match.end();

        // Find the first match of the end pattern starting from the calculated index
        let end_match = end_re.find_at(strval, start_pos_from_idx)?;
        // Determine the starting index of the end match
        let end_pos_to_idx = end_match.start();

        if start_match.start() < end_pos_to_idx {
            // Return the substring section
            return Some(strval[start_pos_from_idx..end_pos_to_idx].to_string());
        }

        None
    }

    /// Extracts a substring from `strval` starting at the given `start` index
    /// and ending at the given `end` index.
    ///
    /// # Parameters
    /// - `strval`: The input string from which the substring will be extracted.
    /// - `start`: The starting index where the substring begins.
    /// - `end`: The ending index where the substring ends.
    ///
    /// # Returns
    /// - `Option<String>` containing the extracted substring if the indices are valid;
    ///   otherwise, `None`.
    #[must_use]
    pub fn from_index(strval: &str, start: usize, end: usize) -> Option<String> {
        // Check if the indices are within bounds and valid
        if start < end && end <= strval.len() {
            // Return the substring section
            return Some(strval[start..end].to_string());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_substring_between_chars() {
        Substring::from("XKBLAYOUT=\"de\"", '"', '"')
            .map(|s| assert_eq!(s, "de"));
    }

    #[test]
    fn get_substring_between_strings() {
        Substring::from("SomeSubstringData", "Some", "Data")
            .map(|s| assert_eq!(s, "Substring"));
    }

    #[test]
    fn get_substring_regex() {
        let result = Substring::from_regex(
            "<script id='main'>console.log('Hello, World!');</script>",
            "<script[^>]*>",
            "</script"
        )
        .unwrap();

        assert_eq!(result, "console.log('Hello, World!');");
    }

    #[test]
    fn get_substring_index() {
        Substring::from_index("SomeSubstringData", 4, 13)
            .map(|s| assert_eq!(s, "Substring"));
    }
}
 
