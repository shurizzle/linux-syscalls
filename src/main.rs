extern crate linux_syscalls;
use linux_syscalls::*;

fn main() {
    init();
    unsafe {
        let mut k = env::kernel_version().to_string();
        k.push('\n');
        syscall!([ro] Sysno::write, 1, k.as_ptr(), k.len()).unwrap();
        syscall!([!] Sysno::exit, 69)
    }
}
