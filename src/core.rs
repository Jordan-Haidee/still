#[cfg(target_os = "linux")]
use libc::{kill, pid_t, SIGCONT, SIGSTOP};
#[cfg(target_os = "windows")]
use ntapi::ntpsapi::{NtResumeProcess, NtSuspendProcess};
#[cfg(target_os = "windows")]
use std::env::args;
#[cfg(target_os = "windows")]
use winapi::{
    shared::ntdef::NULL,
    um::{processthreadsapi::OpenProcess, winnt::PROCESS_SUSPEND_RESUME},
};

#[cfg(target_os = "windows")]
fn suspend_process(pid: u32) -> bool {
    let process_handle = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid) };
    if process_handle == NULL {
        return false;
    }
    let suspend_status = unsafe { NtSuspendProcess(process_handle) };
    if suspend_status != 0 {
        return false;
    }
    println!("Process {} has been suspended successfully!", pid);
    return true;
}

#[cfg(target_os = "windows")]
fn resume_process(pid: u32) -> bool {
    let process_handle = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, 0, pid) };
    dbg!(process_handle);

    if process_handle == NULL {
        return false;
    }
    let suspend_status = unsafe { NtResumeProcess(process_handle) };
    dbg!(suspend_status);
    if suspend_status != 0 {
        return false;
    }
    println!("Process {} has been resumed successfully!", pid);
    return true;
}

#[cfg(target_os = "linux")]
fn suspend_process(pid: u32) -> bool {
    let result = unsafe { kill(pid as pid_t, SIGSTOP) };
    result == 0
}
#[cfg(target_os = "linux")]
fn resume_process(pid: u32) -> bool {
    let result = unsafe { kill(pid as pid_t, SIGCONT) };
    result == 0
}

pub fn parse_duration(s: &str) -> Option<u64> {
    if s.is_empty() {
        return None;
    }
    if let Some(last_char) = s.chars().last() {
        match last_char {
            'h' | 'H' => {
                let num = s[..s.len() - 1].parse::<f64>().ok()?;
                Some((num * 3600.0) as u64)
            }
            'm' | 'M' => {
                let num = s[..s.len() - 1].parse::<f64>().ok()?;
                Some((num * 60.0) as u64)
            }
            's' | 'S' => {
                let num = s[..s.len() - 1].parse::<f64>().ok()?;
                Some(num as u64)
            }
            _ => s.parse::<u64>().ok(),
        }
    } else {
        None
    }
}

pub fn still(pid: u32, seconds: u64) {
    suspend_process(pid);
    let duration = std::time::Duration::from_secs(seconds);
    std::thread::sleep(duration);
    resume_process(pid);
}

#[cfg(target_os = "windows")]
fn main() {
    let args: Vec<String> = args().collect();
    let pid = args[1]
        .parse::<u32>()
        .expect("pid must be a positive integer!");
    let secs = parse_duration(args[2].as_str()).expect("uncorrect duration input!");
    still(pid, secs);
}

#[cfg(target_os = "linux")]
fn main() {}
