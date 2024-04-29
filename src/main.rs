use std::{env, process, thread, time};
use std::os::unix::fs;
// use std::io;

fn main() -> std::io::Result<()> {

//    let args: Vec<String> = env::args().collect();
//    dbg!(args);

    fs::chroot("debian_container")?;
    env::set_current_dir("debian_container")?;

    println!("pid is {}", process::id());

    let waittime = time::Duration::from_secs(3000);
    thread::sleep(waittime);
    Ok(())
}
