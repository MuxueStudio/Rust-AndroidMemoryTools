
use std::ffi::c_long;
use std::os::windows::fs::OpenOptionsExt;
use std::process::Command;
use std::fs::OpenOptions;
use libc::{c_void, off_t, O_RDWR};
extern "C" {
    fn pread(fd: i32, buf: *mut c_void, count: usize, offset: off_t) -> isize;
}
pub fn get_pid(package_name: &str) -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!(
            "ps -A | grep {} | awk '{{print $2}}'",
            package_name
        ))
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if pid.is_empty() {
        None
    } else {
        Some(pid)
    }
}

// libil2cpp.so
pub fn get_so_head(pid: &str, so_name: &str) -> Option<Vec<(String, String)>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("cat /proc/{}/maps | grep -F '{}'", pid, so_name))
        .output()
        .ok()?;
    if !output.status.success() {
        println!("错误输出: {:?}", String::from_utf8_lossy(&output.stderr));
        return None;
    }

    let map = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let mut vec: Vec<(String, String)> = Vec::new();

    if map.is_empty() {
        None
    } else {
        let arr = map.split("\n");
        let mut memory;
        for val in arr {
            if val.contains("rw-p") {
                memory = "Cd"
            } else {
                memory = "Xa"
            }
            vec.push((
                memory.parse().unwrap(),
                val.split("-").next().unwrap().to_string(),
            ));
        }
        Some(vec)
    }
}

pub fn read_point(addr: c_long) -> c_long {
    let path = format!("/proc/{}/mem", addr);
    let mut result;
    let mem_file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_RDWR as u32)
        .open(path);
    unsafe {
        pread(
            mem_file.as_raw_fd(),
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
            offset
        )
    }
}
