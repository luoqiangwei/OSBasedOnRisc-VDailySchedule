// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer
// Step 1. Complete the `capitalize_first` function to pass the first two cases
// Step 2. Apply the `capitalize_first` function to a vector of strings, ensuring that it returns a vector of strings as well
// Step 3. Apply the `capitalize_first` function again to a list, but try and ensure it returns a single string
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut result = String::new();
    while let Some(first) = c.next() {
        if (result.len() == 0) {
            result.push(first.to_ascii_uppercase());
        } else {
            result.push(first);
        }
    }
    return result;
    // match c.next() {
    //     None => String::new(),
    //     Some(first) => first.collect::<String>() + c.as_str(),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = words.iter().map(|x| capitalize_first(x)).collect(); // TODO
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let mut capitalized_words = String::new(); // TODO
        for word in words.iter() {
            capitalized_words.push_str(&capitalize_first(word));
        }
        assert_eq!(capitalized_words, "Hello World");
    }
}
