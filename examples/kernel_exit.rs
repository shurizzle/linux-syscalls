extern crate linux_syscalls;

use linux_syscalls::*;

fn main() {
    init();
    print_arch();
    println!("{}", env::kernel_version());
    unsafe { syscall!([!] Sysno::exit, 69) }
}

fn print_arch() {
    #[cfg(target_arch = "x86")]
    println!("x86");
    #[cfg(target_arch = "x86_64")]
    println!("x86_64");
    #[cfg(all(target_arch = "arm"))]
    println!("arm");
    #[cfg(target_arch = "aarch64")]
    println!("aarch64");
    #[cfg(target_arch = "riscv64")]
    println!("riscv64");
    #[cfg(target_arch = "powerpc64")]
    println!("powerpc64");
    #[cfg(target_arch = "mips")]
    println!("mips");
    #[cfg(target_arch = "mips64")]
    println!("mips64");
}
