pub mod yadb;

use std::env;
use nix::sys::wait;
use nix::unistd::Pid;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = yadb::attach::attach(&args).unwrap();

    println!("Process PID is {pid}");

    match wait::waitpid(Pid::from_raw(pid), Option::None) {
        Err(err) => println!("waitpid failed {err}"),
        Ok(_) => {},
    }

    let mut buffer = String::new();
    let mut history = Vec::<String>::new();
    let stdin = io::stdin();
    while let result = stdin.read_line(&mut buffer) {
        match result {
            Ok(n) => {
                println!("read {n} characters: {buffer}");
                match n {
                    0 => break,
                    1 => { // use last history entry
                        history.last().map(|it|handle_command(pid, &it));
                    },
                    _ => { // use new entry
                        history.push(buffer.clone());
                        handle_command(pid, &buffer);
                    },
                }
                buffer.clear();
            },
            Err(err) => println!("read_line failed {err}"),
        }
    }
}

fn handle_command(pid: i32, command: &String) {
    println!("Handle command: {pid} - {command}");
}