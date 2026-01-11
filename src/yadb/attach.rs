use nix::sys::ptrace;
use nix::unistd::Pid;
use std::process::Command;

pub fn attach(args: &Vec<String>) -> Result<i32, &str> {
    let pid;
    if args.len() == 3 && args[1] == "-p" {
        pid = args[2].parse::<i32>().unwrap();
        if pid <= 0 {
            return Err("Invalid PID");
        }
        
    } else {
       let mut new_args = args.clone();
       new_args = new_args.drain(2..).collect();
       let child = Command::new(&args[1])
                .args(new_args)
                .spawn()
                .expect("failed to execute process");
        pid = child.id() as i32;
    }

    println!("Trying to attach to PID {pid}");
    return match ptrace::attach(Pid::from_raw(pid)) {
        Err(err) => {
            println!("Failed to attach with error {err}");
            return Err("Failed to attach");
        },
        Ok(_) => Ok(pid)
    };
}