use std::{
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, SystemTime},
};


fn main() {
    let mut stdout = stdout();

    loop {
        let now = SystemTime::now();
        print!("\rCurrent time: {:?}", now);
        
        stdout.flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
