pub struct Substring<T: ToString> {
    pub strval: String,
    pub start: T,
    pub end: T,
}

impl<T: ToString> Substring<T> {
    pub fn from(strval: &str, start: T, end: T) -> Option<String> {
        let start_pos_idx = strval.find(&start.to_string());
        let end_pos_idx = strval.find(&end.to_string());

        match (start_pos_idx, end_pos_idx) {
            (Some(start_idx), Some(end_idx)) if start_idx < end_idx => {
                let substring = &strval[(start_idx + start.to_string().len())..end_idx];
                return Some(String::from(substring));
            }
            _ => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_substring_between_chars() {
        Substring::from("SomeSubstringData", 'e', 'D')
            .map(|s| assert_eq!(s, "Substring"));
    }

    #[test]
    fn get_substring_between_strings() {
        Substring::from("SomeSubstringData", "Some", "Data")
            .map(|s| assert_eq!(s, "Substring"));
    }
}