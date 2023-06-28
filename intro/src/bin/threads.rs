use std::thread;

static mut COUNTER: i32 = 0;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: threads <value>");
        std::process::exit(1);
    }
    let loops: usize = args[1].parse().expect("Provide a number > 0");
    println!("Initial value: {}", unsafe { COUNTER });

    let handler1 = thread::spawn(unsafe {
        move || {
            for _ in 0..loops {
                COUNTER += 1;
            }
        }
    });
    let handler2 = thread::spawn(unsafe {
        move || {
            for _ in 0..loops {
                COUNTER += 1;
            }
        }
    });
    handler1.join().unwrap();
    handler2.join().unwrap();

    println!("Final value: {}", unsafe { COUNTER });
}
