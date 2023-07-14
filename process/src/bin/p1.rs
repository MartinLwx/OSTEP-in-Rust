use nix::libc;
use nix::unistd::{fork, write, ForkResult};
use std::process;

fn main() {
    println!("hello world (pid:{})", process::id());
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork failed");
            process::exit(1);
        }
        Ok(ForkResult::Child) => {
            write(
                libc::STDERR_FILENO,
                format!("hello, I am child (pid:{})\n", process::id()).as_bytes(),
            )
            .ok();
        }
        Ok(ForkResult::Parent { child }) => {
            println!("hello, I am parent of {} (pid: {})", child, process::id());
        }
    }
}
