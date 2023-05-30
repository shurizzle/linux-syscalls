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
    #[cfg(target_arch = "riscv64")]
    println!("riscv64");
    #[cfg(target_arch = "powerpc64")]
    println!("powerpc64");
    println!("{}", env::kernel_version());
    println!("culo");
    unsafe { syscall!([!] Sysno::exit, 69) }
}
