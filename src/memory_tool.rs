use libc::{c_int, c_void, lseek64, off_t, open, read, O_RDWR, SEEK_SET};
use std::ffi::{c_long, CString};
use std::process;
use std::process::Command;

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

pub fn read_point(pid: c_long, addr: c_long) -> c_int {
    let mut val: c_int = 0;

    // 1. 打开进程内存文件
    let path = CString::new(format!("/proc/{}/mem", pid)).expect("CString::new failed");
    let mem_file = unsafe { open(path.as_ptr(), O_RDWR) };

    // 检查文件描述符是否有效
    if mem_file == -1 {
        eprintln!("Failed to open /proc/{}/mem", pid);
        process::exit(1);
    }

    // 2. 读取内存
    unsafe {
        if lseek64(mem_file, addr as i64, SEEK_SET) == -1 {
            eprintln!("Failed to seek to address 0x{:x}", addr);
            process::exit(1);
        }

        let bytes_read = read(
            mem_file,
            &mut val as *mut c_int as *mut c_void,
            size_of::<c_int>(),
        );

        if bytes_read != size_of::<c_int>() as isize {
            eprintln!("Failed to read memory at 0x{:x}", addr);
            process::exit(1);
        }
    }

    val
}
