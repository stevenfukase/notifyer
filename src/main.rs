use std::{thread, time};

fn main() {
    let times = 1_000_000_000;
    let mut count = 0;

    while count < times {
        println!("{}", count);
        count += 1;
        thread::sleep(time::Duration::from_secs(1));
    }
}
