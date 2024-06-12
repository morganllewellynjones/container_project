use std::{process};
use clap::Parser;

// Program for running a contained process
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    // Root of the container image filesystem to enter
    #[arg(short, long)]
    root: String,

    // Command to execute inside the container
    #[arg(default_values_t = [String::from("/bin/sh")])]
    command: Vec<String>,
}

fn setup_container(config: Config) -> process::Child {
    // Uses the runc container specification for setup.
    // Link: https://github.com/opencontainers/runc/blob/main/libcontainer/SPEC.md?plain=1
    // Additionally, we will set-up a NAT to the childs network namespace and connect it to the
    // internet
    
    
    let child = process::Command::new("unshare")
        .args([
            "--pid",
            "--mount",
            "--uts",
            "--cgroup",
            "--time",
            "--net",
            "--mount-proc",
            "--kill-child",
            &["--root", &config.root].join("="),
            &["--wd", &config.root].join("="),
        ])
        .arg(&config.command.join(" "))
        .spawn()
        .expect("Failed to create unshared command.");

    let ns = child.id().to_string();

    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "proc",
            "--source",
            "/proc",
            "--target",
            "debian_container/proc",
            "--options",
            "noexec,nosuid,nodev",
        ])
        .output()
        .expect("Failed mounting file");

    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "tmpfs",
            "--source",
            "/dev",
            "--target",
            "debian_container/dev",
            "--options",
            "noexec,strictatime",
        ])
        .output()
        .expect("Failed mounting file");
    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "tmpfs",
            "--source",
            "/dev/shm",
            "--target",
            "debian_container/dev/shm",
            "--options",
            "noexec,nosuid,nodev",
        ])
        .output()
        .expect("Failed mounting file");
    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "mqueue",
            "--source",
            "/dev/mqueue",
            "--target",
            "debian_container/dev/mqueue",
            "--options",
            "noexec,nosuid,nodev",
        ])
        .output()
        .expect("Failed mounting file");
    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "devpts",
            "--source",
            "/dev/pts",
            "--target",
            "debian_container/dev/pts",
            "--options",
            "noexec,nosuid",
        ])
        .output()
        .expect("Failed mounting file");
    process::Command::new("mount")
        .args([
            "--namespace",
            &ns,
            "--types",
            "sysfs",
            "--source",
            "/sys",
            "--target",
            "debian_container/sys",
            "--options",
            "noexec,nosuid,nodev,rdonly",
        ])
        .output()
        .expect("Failed mounting file");

    process::Command::new("chmod")
        .args(["755", "debian_container/dev"])
        .output()
        .expect("Failed changing file permissions");
    process::Command::new("chmod")
        .args(["1777", "debian_container/dev/shm"])
        .output()
        .expect("Failed changing file permissions");
    process::Command::new("chmod")
        .args(["620", "debian_container/dev/pts"])
        .output()
        .expect("Failed changing file permissions");

    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/null"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/zero"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/full"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/tty"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/random"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mkdir")
        .args(["--mode=0666", "debian_container/dev/urandom"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mount")
        .args(["/etc/", "debian_container/etc"])
        .output()
        .expect("Failed to mount /etc folder");

    process::Command::new("ln")
        .args(["-s", "/dev/ptmx", "debian_container/dev/ptmx"])
        .output()
        .expect("Failed to symlink /dev/ptmx");

    return child;
}

fn main() {

    let config: Config = Config::parse();

    let jail: process::Child = setup_container(config);

    jail.wait_with_output()
        .expect("Failed to execute unshared command.");
}
