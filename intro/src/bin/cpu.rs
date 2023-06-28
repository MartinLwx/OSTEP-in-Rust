use std::{thread, time::Duration};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: cpu <string>");
        std::process::exit(1);
    }
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", args[1]);
    }
}
