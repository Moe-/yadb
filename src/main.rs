pub mod yadb;

use std::env;
use nix::sys::wait;
use nix::unistd::Pid;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = yadb::attach::attach(&args).unwrap();

    println!("Process PID is {pid}");

    match wait::waitpid(Pid::from_raw(pid), Option::None) {
        Err(err) => println!("waitpid failed {err}"),
        Ok(_) => {},
    }

    // implement waitpid, see https://docs.rs/fork/latest/fork/
}

