#[cfg(target_os = "windows")]
use std::env;
#[cfg(target_os = "windows")]
use std::env::args;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
use daemonize::Daemonize;

#[cfg(target_os = "linux")]
use std::fs::File;
mod core;

#[cfg(target_os = "windows")]
fn main() {
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let args: Vec<String> = args().collect();
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    Command::new(format!(r"{}/core.exe", exe_dir.display()))
        .arg(args[1].as_str())
        .arg(args[2].as_str())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NEW_PROCESS_GROUP)
        .creation_flags(DETACHED_PROCESS)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("Failed to spawn child process");
}

#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pid: u32 = args[1].parse().expect("pid must be positive integers!");
    let duration: u64 = core::parse_duration(args[2].as_str()).unwrap();
    let stdout = File::create(format!("/tmp/{pid}.out")).expect("failed to create file!");
    let stderr = File::create(format!("/tmp/{pid}.err")).expect("failed to create file!");
    let daemon = Daemonize::new().umask(0o644).stdout(stdout).stderr(stderr);

    match daemon.start() {
        Ok(_) => core::still(pid, duration),
        Err(e) => eprintln!("failed to start daemon process: {}", e),
    }
}
