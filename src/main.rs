use errno::errno;
use libc::*;
use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::process::{Command, Stdio};
use std::{thread, time};

struct Config {
    cpus_limit: u8,
    memory_limit: u64,
    pids_limit: u64,
    hostname: String,
    username: String,
    ports_published: Vec<Port>,
}

struct Port {
    host_port: u16,
    dest_port: u16,
}

struct BindMount {
    source: String,
    destination: String,
}

fn c_error_check(status: c_int, fn_name: &str) {
    if status != 0 {
        panic!("{fn_name}: {}", errno().to_string());
    }
}

fn run() {
    let mut args: Vec<String> = env::args().collect();

    let newroot = Command::new("chroot")
        .arg("debian_container")
        .args(args.drain(1..))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

fn jail() {
    let newhostname = CString::new("debiancontainer").unwrap();
    let newhostnameptr = newhostname.as_ptr();
    unsafe {
        _ = c_error_check(
            unshare(CLONE_NEWPID | CLONE_NEWNET | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUTS),
            "unshare",
        );
        _ = c_error_check(sethostname(newhostnameptr, 15), "sethostname");
    }
    run();
}

fn main() {
    jail();
}
