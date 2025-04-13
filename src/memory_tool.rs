use libc::{c_double, c_float, c_int, c_short, c_void, off64_t, pread64, O_RDONLY};
use std::ffi::{c_long, CString};
use std::mem::MaybeUninit;
use std::process::Command;

pub type DWORD = c_int;
pub type FLOAT = c_float;
pub type WORD = c_short;
pub type DOUBLE = c_double;
pub type QWORD = c_long;

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

pub unsafe fn read_val<T>(pid: i32, addr: off64_t) -> T{
    // 1. 打开内存文件
    let path = CString::new(format!("/proc/{}/mem", pid)).unwrap();
    let fd = libc::open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
         panic!("{}",format!("open failed: {}", std::io::Error::last_os_error()));
    }
    // 2. 使用pread64读取
    let mut val = MaybeUninit::<T>::uninit();
    let bytes_read = pread64(
        fd,
        val.as_mut_ptr() as *mut c_void,
        size_of::<T>(),
        addr
    );

    libc::close(fd); // 记得关闭文件描述符

    // 3. 检查结果
    if bytes_read == -1 {
        panic!("{}",format!("pread64 failed at {:#x}: {}", addr, std::io::Error::last_os_error()))
    } else if bytes_read != size_of::<T>() as isize {
        panic!("{}",format!("Incomplete read at {:#x}", addr))
    } else {
        val.assume_init()
    }
}