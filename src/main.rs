pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    // println!("{:?}", c);
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let text = capitalize_first("hello");
    println!("{:?}", text);
}
