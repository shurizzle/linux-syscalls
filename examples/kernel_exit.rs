extern crate linux_syscalls;

use linux_syscalls::*;

fn main() {
    init();
    #[cfg(target_arch = "x86")]
    println!("x86");
    #[cfg(target_arch = "x86_64")]
    println!("x86_64");
    #[cfg(target_arch = "aarch64")]
    println!("aarch64");
    #[cfg(target_arch = "arm")]
    println!("arm");
    println!("{}", env::kernel_version());
    unsafe { syscall!([!] Sysno::exit, 69) }
}
