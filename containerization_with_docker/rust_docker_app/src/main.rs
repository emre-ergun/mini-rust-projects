use std::{thread, time::Duration};


fn main() {
    let mut i = 0;
    loop {
        println!("{i} Hello, world!");
        i += 1;
        if i > 10 {
            break;
        }
        thread::sleep(Duration::from_millis(1000));

    }
}
