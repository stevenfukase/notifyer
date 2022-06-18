use std::vec;

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // println!("{:?}", c);
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| capitalize_first(word)).collect()
}

fn main() {
    let text = capitalize_first("hello");
    println!("{:?}", text);

    let words = vec!["hello", "steven"];
    let capitalized_vector = capitalize_words_vector(&words);
    println!("{:?}", capitalized_vector);
}
