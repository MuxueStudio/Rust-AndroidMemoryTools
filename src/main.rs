mod memory_tool;

use crate::memory_tool::{read_point, read_val, DWORD};
use memory_tool::{get_pid, get_so_head};

fn main() {
    let pid = get_pid("com.cyhxzhdzy.kz").unwrap_or("None".to_string());
    let command = get_so_head(&pid, "libil2cpp.so");
    let addr = unsafe { read_point(&pid, command.unwrap()["Cd"]+0x4EF18, &[0x5C, 0xA0,0xC]) }; //quan xian wen ti
    unsafe {
        println!("{:#x}", addr);
    }
    unsafe {
        println!("{}", read_val::<DWORD>(&pid, addr));
    }
}
