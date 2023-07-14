use nix::libc;
use nix::sys::wait::wait;
use nix::unistd::execvp;
use nix::unistd::{dup2, fork, write, ForkResult};
use std::ffi::CString;
use std::fs;
use std::os::unix::io::IntoRawFd;
use std::process;

fn main() {
    println!("hello world (pid:{})", process::id());
    match unsafe { fork() } {
        Err(_) => {
            eprintln!("fork failed");
            process::exit(1);
        }
        Ok(ForkResult::Child) => {
            let file = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .read(true)
                .open("./p4.output")
                .unwrap();

            // hint: 1 means stdout
            dup2(file.into_raw_fd(), 1).unwrap();

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
