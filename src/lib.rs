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
}
