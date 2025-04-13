mod memory_tool;
use memory_tool::{get_pid, get_so_head};
fn main() {
    let pid = get_pid("com.cyhxzhdzy.kz").unwrap_or("None".to_string());
    let command = get_so_head(&pid, "libil2cpp.so");
    println!("{:?}", command);
}
