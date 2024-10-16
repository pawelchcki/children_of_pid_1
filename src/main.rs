use std::{process::exit, time::Duration};

use nix::{errno::errno, libc::{waitpid, ECHILD}, unistd::{self, setsid}};

fn do_fork<F>(child_action: F) -> unistd::Pid
where
    F: FnOnce(unistd::Pid),
{
    let res = unsafe {
        unistd::fork().unwrap()
    };

    let pid: unistd::Pid = unistd::Pid::this();

    match res {
        unistd::ForkResult::Parent { child } => {
            println!("Parent {}: Child PID is {}", pid, child);
            child
        }
        unistd::ForkResult::Child => {
            let child = unistd::Pid::this();

            println!("Child {}: I'm a new child process", child);
            child_action(child);
            exit(0);
        }
    }
}

fn main() {
    let pid: unistd::Pid = unistd::Pid::this();
    let now = std::time::SystemTime::now();
    println!("{pid} -> Hello, world!");


    do_fork(|_| {
        let sid = unistd::setsid().unwrap();
        println!("Sid: {sid}");
        // double fork
        do_fork(|pid|{
            println!("Child {pid}: I'm going to sleep for 10 seconds");
            unistd::sleep(10);
            println!("Child {pid}: I'm done sleeping");
        });
    });

    println!("Parent {pid}: Waiting for children to finish");
    let mut status: i32 = 0;
    loop {
        println!("loop");
        unsafe {
            waitpid(-1, &mut status as *mut i32 , 0);
            let e = nix::errno::errno();
            if e == ECHILD {
                break; // no more children to wait for
            }
        };
    }
    let elapsed = now.elapsed().unwrap().as_secs();
    println!("Parent {pid}: exiting after {elapsed} seconds");
}
