mod core;
use clap::{self, value_parser, Arg};

fn parse_args() -> (u32, u64) {
    let matches = clap::Command::new("still")
        .about(
            "Still is a simple process alarm. It make specified process sleep now, and wake up after a period of time.",
        )
        .arg(
            Arg::new("pid")
                .required(true)
                .value_parser(value_parser!(u32))
                .help("the process id to be still.")
                .index(1),
        )
        .arg(
            Arg::new("duration")
                .required(true).help("the duration to be still, such as 10s, 1m, 2h.")
                .index(2),
        )
        .get_matches();
    let pid = *matches.get_one::<u32>("pid").unwrap();
    let duration = matches.get_one::<String>("duration").unwrap();
    let duration = core::parse_duration(duration.as_str()).unwrap();
    (pid, duration)
}

#[cfg(target_os = "windows")]
fn main() {
    use std::env;
    use std::os::windows::process::CommandExt;
    use std::process::{Command, Stdio};

    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let (pid, duration) = parse_args();
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    Command::new(format!(r"{}/core.exe", exe_dir.display()))
        .arg(pid.to_string())
        .arg(duration.to_string())
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
    use daemonize::Daemonize;

    let (pid, duration) = parse_args();

    let daemon = Daemonize::new();
    match daemon.start() {
        Ok(_) => core::still(pid, duration),
        Err(e) => {
            eprintln!("failed to start daemon process: {}", e);
        }
    }
}
