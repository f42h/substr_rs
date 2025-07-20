use substr_rs::Substring;

fn main() {
    {
        /*************************************** Using chars **********************************************/
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
    }
    
    {
        /************************************** Using strings *********************************************/
        let string_value = "SomeSubstringData";
        let start_string = "Some";
        let end_string = "Data";

        if let Some(substr) = Substring::from(string_value, start_string, end_string) {
            println!(
                "The substring of `{}` between the strings `{}` and `{}` is {}", 
                string_value, start_string, end_string,
                substr
            );
        }

        // Output:
        //  The substring of `SomeSubstringData` between the strings `Some` and `Data` is Substring
    }
    
    {
        /*********************************** Using regex patterns *****************************************/
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
    }
    
    {
        /************************************** Using indexes *********************************************/
        let string_value = "SomeSubstringData";
        let start_pattern: usize = 4;
        let end_pattern: usize = 13;

        if let Some(substr) = Substring::from_index(string_value, start_pattern, end_pattern) {
            println!(
                "The substring of `{}` between the start index `{}` and the end index `{}` is {}", 
                string_value, start_pattern, end_pattern,
                substr
            );
        }

        // Output:
        //  The substring of `SomeSubstringData` between the start index `4` and the end index `13` is Substring
    }
}