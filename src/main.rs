use errno::errno;
use getopt::Opt;
use libc::*;
use std::env;
use std::ffi::CString;
use std::process::{Command, Stdio};

#[derive(Debug)]
struct Config {
    cpus_limit: u8,
    memory_limit: u64,
    pids_limit: u64,
    hostname: CString,
    username: CString,
    directory: String,
    ports_published: Vec<Port>,
    cmd: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cpus_limit: 0,
            memory_limit: 0,
            pids_limit: 0,
            hostname: CString::new("container").unwrap(),
            username: CString::new("").unwrap(),
            directory: "debian_container".to_string(),
            ports_published: vec![],
            cmd: "/usr/bin/bash".to_string(),
        }
    }
}

#[derive(Debug, Default)]
struct Port {
    host: u16,
    dest: u16,
}

#[derive(Debug, Default)]
struct BindMount {
    source: String,
    destination: String,
}

fn c_error_check(status: c_int, fn_name: &str) {
    if status != 0 {
        panic!("{fn_name}: {}", errno().to_string());
    }
}

fn jail(config: &Config) {
    let newhostname = &config.hostname;
    let newhostnameptr = newhostname.as_ptr();
    unsafe {
        _ = c_error_check(
            unshare(CLONE_NEWPID | CLONE_NEWNS | CLONE_NEWPID | CLONE_NEWUTS),
            "unshare",
        );
        _ = c_error_check(sethostname(newhostnameptr, 15), "sethostname");
    }
}

fn run(config: Config) {

    jail(&config);

    let newroot = Command::new("chroot")
        .arg(&config.directory)
        .arg(&config.cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

fn main() {
    let mut config: Config = Default::default();
    let mut args: Vec<String> = env::args().collect();
    let mut opts = getopt::Parser::new(&args, "h:u:d:");
    let usage = "rc [-u username] [-h hostname] [-d container_directory] command".to_string();

    loop {
        match opts.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('h', Some(arg)) => config.hostname = CString::new(arg.clone()).unwrap(),
                Opt('u', Some(arg)) => config.username = CString::new(arg.clone()).unwrap(),
                Opt('d', Some(arg)) => config.directory = arg.clone(),
                _ => {
                    println!("{usage}");
                    return;
                }
            },
        }
    }
    dbg!(&config);
    run(config);
}
