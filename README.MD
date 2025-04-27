# 开发环境
系统版本：Ubuntu 24.04.2 LTS

NDK版本：android-ndk-r25b

# 编译运行
`注意执行路径`
```bash
 bash /buildscript/build_all.sh
```
# 方法介绍
获取帮助:沐雪社区:https://www.muxue.pro
视频教程:https://v.douyin.com/I-xKYusrX5M/
哔哩哔哩:https://space.bilibili.com/3546693165386233
抖音:
```rust
// 获取指定包名的进程id
get_pid(package_name)
// 获取指定so的头部地址
get_so_head(pid,so_name) 

//返回["Cd"] 和 ["Xa"]头的地址：
let command = get_so_head(&pid, "libil2cpp.so");
//这样获取
command.unwrap()["Cd"]
//读取值
//T泛型 传入GG修改器的类型，不懂别玩
/*
类型
DWORD 
FLOAT 
WORD 
DOUBLE 
QWORD
*/
 */
read_val<T>(pid: &str, addr: off64_t);
//获取地址的指针
rpoint(pid: &str, address: c_long);
//读取锁链地址
read_point(pid: &str, address: off64_t, offsets: &[off64_t]);
//写入值
write_val<T>(pid: &str, address: off64_t, val: T);
```

# 完整代码示例

```rust
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
        write_val::<DWORD>(&pid, addr, 6666);
        println!("第二次读取DWORD：{}", read_val::<DWORD>(&pid, addr));
        write_val::<FLOAT>(&pid, addr, 666.2);
        println!("第一次读取FLOAT：{}", read_val::<FLOAT>(&pid, addr));
    }
}
```
