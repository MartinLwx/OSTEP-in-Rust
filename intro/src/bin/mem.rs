use std::{thread, time::Duration};

fn main() {
    let mut i32_ptr = Box::new(0);
    println!(
        "({}) address pointed to by p: {:p}",
        std::process::id(),
        i32_ptr
    );
    loop {
        thread::sleep(Duration::from_secs(1));
        *i32_ptr += 1;
        println!("({}) p: {}", std::process::id(), *i32_ptr);
    }
}
