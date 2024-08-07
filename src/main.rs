use clap::Parser;
use std::process;

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

    let child = process::Command::new("unshare")
        .args([
            "--map-root-user",
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
            &[&config.root, "proc"].join("/"),
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
            &[&config.root, "dev"].join("/"),
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
            &[&config.root, "dev/shm"].join("/"),
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
            &[&config.root, "dev/mqueue"].join("/"),
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
            &[&config.root, "dev/pts"].join("/"),
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
            &[&config.root, "sys"].join("/"),
            "--options",
            "noexec,nosuid,nodev,rdonly",
        ])
        .output()
        .expect("Failed mounting file");

    process::Command::new("chmod")
        .args(["755", &[&config.root, "dev"].join("/")])
        .output()
        .expect("Failed changing file permissions");
    process::Command::new("chmod")
        .args(["1777", &[&config.root, "dev/shm"].join("/")])
        .output()
        .expect("Failed changing file permissions");
    process::Command::new("chmod")
        .args(["620", &[&config.root, "dev/pts"].join("/")])
        .output()
        .expect("Failed changing file permissions");

    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/null", "c", "1", "3"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/zero", "c", "1", "5"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/full", "c", "1", "7"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/tty", "c", "5", "0"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/random", "c", "1", "8"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");
    process::Command::new("mknod")
        .args(["--mode=0666", "debian_container/dev/urandom", "c", "1", "9"])
        .output()
        .expect("Failed making debian_container/dev subdirectory");

    process::Command::new("mount")
        .args(["/etc/", &[&config.root, "etc"].join("/")])
        .output()
        .expect("Failed to mount /etc folder");
    process::Command::new("ln")
        .args(["-s", "/dev/ptmx", &[&config.root, "dev/ptmx"].join("/")])
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
