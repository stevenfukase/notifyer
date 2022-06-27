use std::env;

fn main() {
    let args = env::args();
    if args.len() != 3 {
        panic!("invalid arg count")
    }
    println!("{:?}", args);
}
