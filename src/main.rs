pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // println!("{:?}", c);
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string(),
    }
}

fn main() {
    let text = capitalize_first("hello");
    println!("{:?}", text);
}
