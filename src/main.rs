mod memory_tool;

use crate::memory_tool::FLOAT;
use memory_tool::{get_pid, read_val};

fn main() {
    let pid = get_pid("com.cyhxzhdzy.kz").unwrap_or("None".to_string());
    // let command = get_so_head(&pid, "libil2cpp.so");
    unsafe { println!("{:?}", read_val::<FLOAT>(pid.parse().unwrap(), 0xb75c3000)); }
    
}
