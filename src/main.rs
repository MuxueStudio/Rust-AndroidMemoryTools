mod memory_tool;

use crate::memory_tool::{read_point, read_val, write_val, DWORD, FLOAT};
use memory_tool::{get_pid, get_so_head};

fn main() {
    let pid = get_pid("com.cyhxzhdzy.kz").unwrap_or("None".to_string());
    let command = get_so_head(&pid, "libil2cpp.so");
    unsafe {
        let addr = read_point(&pid, command.unwrap()["Cd"] + 0x4EF18, &[0x5C, 0xA0, 0xC]);
        println!("地址{:#x}", addr);
        write_val::<DWORD>(&pid, addr, 7777);
        println!("第一次读取DWORD：{}", read_val::<DWORD>(&pid, addr));
        // write_val::<DWORD>(&pid, addr, 6666);
        // println!("第二次读取DWORD：{}", read_val::<DWORD>(&pid, addr));
        // write_val::<FLOAT>(&pid, addr, 666.2);
        // println!("第一次读取FLOAT：{}", read_val::<FLOAT>(&pid, addr));
    }
}
