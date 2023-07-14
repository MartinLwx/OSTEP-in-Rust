use nix::libc;
use nix::sys::wait::wait;
use nix::unistd::execvp;
use nix::unistd::{fork, write, ForkResult};
use std::ffi::CString;
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

            // use exec()
            let args = vec![
                CString::new("wc").unwrap(),
                CString::new("./src/bin/p3.rs").unwrap(),
            ];
            let args = args.into_boxed_slice();
            execvp(&args[0], &args).unwrap();

            write(
                libc::STDOUT_FILENO,
                "this shouldn't print out".to_string().as_bytes(),
            )
            .ok();
        }
        Ok(ForkResult::Parent { child }) => {
            let rc_wait = wait().unwrap();
            println!(
                "hello, I am parent of {} (rc_wait: {}) (pid: {})",
                child,
                rc_wait.pid().unwrap(),
                process::id()
            );
        }
    }
}
