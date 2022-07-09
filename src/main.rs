use std::{thread::sleep, time::Duration};

fn main() {
    let mut prev_num: u128 = 1;
    let mut num: u128 = 1;
    loop {
        let result = prev_num + num;
        println!("{:?}", result);
        prev_num = num;
        num = result;
        sleep(Duration::from_secs(1));
    }
}
