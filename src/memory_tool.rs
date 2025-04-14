use libc::{c_double, c_float, c_int, c_short, c_void, off64_t, pread64, O_RDONLY};
use std::collections::HashMap;
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

pub fn get_so_head(pid: &str, so_name: &str) -> Option<HashMap<String, off64_t>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("cat /proc/{}/maps | grep -F '{}'", pid, so_name))
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    let mut result: HashMap<String, off64_t> = HashMap::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 关键修复：用十六进制解析地址
        if let Some(addr_part) = trimmed.split('-').next() {
            if let Ok(addr) = u64::from_str_radix(addr_part.trim(), 16) {
                let mem_type = if trimmed.contains("rw-p") { "Cd" } else { "Xa" };
                result.insert(mem_type.to_string(), addr as off64_t);
            } else {
                eprintln!("Failed to parse (hex): {}", addr_part); // 调试用
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

pub unsafe fn read_val<T>(pid: &str, addr: off64_t) -> T {
    // 1. 打开内存文件
    let path = CString::new(format!("/proc/{}/mem", pid)).unwrap();
    let fd = libc::open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        panic!(
            "{}",
            format!("open failed: {}", std::io::Error::last_os_error())
        );
    }
    // 2. 使用pread64读取
    let mut val = MaybeUninit::<T>::uninit();
    let bytes_read = pread64(fd, val.as_mut_ptr() as *mut c_void, size_of::<T>(), addr);

    libc::close(fd); // 记得关闭文件描述符

    // 3. 检查结果
    if bytes_read == -1 {
        panic!(
            "{}",
            format!(
                "pread64 failed at {:#x}: {}",
                addr,
                std::io::Error::last_os_error()
            )
        )
    } else if bytes_read != size_of::<T>() as isize {
        panic!("{}", format!("Incomplete read at {:#x}", addr))
    } else {
        val.assume_init()
    }
}
pub unsafe fn rpoint(pid: &str, address: c_long) -> c_long {
    let path = CString::new(format!("/proc/{}/mem", pid)).unwrap();
    let fd = libc::open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        panic!(
            "Failed to open /proc/{}/mem: {}",
            pid,
            std::io::Error::last_os_error()
        );
    }

    // 关键修复：使用u32读取 + 精准类型转换
    let mut val: u32 = 0;
    let ret = pread64(
        fd,
        &mut val as *mut u32 as *mut _,
        4, // 明确读取4字节
        address as i64,
    );

    libc::close(fd);

    if ret != 4 {
        panic!("Read failed at {:#x} ({} bytes read)", address, ret);
    }
    val as i64
}
pub unsafe fn read_point(pid: &str, address: off64_t, offsets: &[off64_t]) -> c_long {
    //报错就是没有这个地址，可以拿gg偏移测试，该函数u没问题，明天写个异常处理
    println!("address:{:#x}", address);
    let mut p1 = rpoint(pid, address); //point addr is true
    println!("{:#x}", p1);
    let size = offsets.len();
    println!("size:{}", size);
    for i in 0..size - 1 {
        println!("i: {} p1:{:#x} offsets:{:#x}", i, p1, offsets[i]);
        p1 = rpoint(pid, p1 + offsets[i]);
        println!("new p1:{:#x}", p1);
    }
    println!("p1:{:#x} offsets:{:#x}", p1, offsets[size - 1]);
    p1 + offsets[size - 1]
    // 0
}

pub unsafe fn write_val<T>(pid: &str, address: off64_t, val: T) {
    let path = CString::new(format!("/proc/{}/mem", pid)).unwrap();
    let fd = libc::open(path.as_ptr(), libc::O_RDWR);
    if fd == -1 {
        panic!("Failed to open /proc/{}/mem", pid);
    }
    let bytes_written = libc::pwrite64(
        fd,
        &val as *const T as *const c_void,
        size_of::<T>(),
        address,
    );
    if bytes_written == -1 {
        panic!("Failed to write to {:#x}", address);
    }
    libc::close(fd);
}
