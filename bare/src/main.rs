#![no_std]
#![no_main]
#![cfg_attr(not(test), feature(lang_items))]

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

extern crate linux_syscalls;

use core::{ffi::c_void, fmt::Write};

use linux_syscalls::*;

#[non_exhaustive]
pub struct RawFd(i32);

#[inline]
pub fn stdout() -> RawFd {
    RawFd(1)
}

#[inline]
pub fn stderr() -> RawFd {
    RawFd(2)
}

impl core::fmt::Write for RawFd {
    fn write_str(&mut self, mut s: &str) -> core::fmt::Result {
        unsafe {
            while !s.is_empty() {
                match syscall!([ro] Sysno::write, self.0, s.as_ptr(), s.len()) {
                    Ok(0) => return Err(core::fmt::Error),
                    Ok(n) => {
                        s = s.get_unchecked(n..);
                    }
                    Err(Errno::EINTR) => (),
                    Err(_) => return Err(core::fmt::Error),
                }
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        write!(stdout(), $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        writeln!(stdout(), $($args)*).unwrap()
    };
}

#[cfg_attr(target_arch = "x86_64", path = "intrinsics/x86_64.rs")]
#[cfg_attr(target_arch = "aarch64", path = "intrinsics/aarch64.rs")]
mod intrinsics;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = write!(stderr(), "{}", info);
    unsafe { syscall!([!] Sysno::exit, 1) }
}

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov rdi, rsp",
    "push rbp",
    "jmp __bare_start",
);

#[cfg(target_arch = "x86")]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov eax, esp",
    "push ebp",
    "push ebp",
    "push ebp",
    "push eax",
    "push ebp",
    "jmp __bare_start",
);

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov x0, sp",
    "mov x30, xzr",
    "b __bare_start",
);

#[cfg(target_arch = "arm")]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov r0, sp",
    "mov lr, #0",
    "b __bare_start",
);

#[no_mangle]
pub unsafe extern "C" fn __bare_start(stack: *const c_void) -> ! {
    let argc = *stack.cast::<usize>();
    let argv = stack.add(core::mem::size_of::<usize>()).cast::<*const u8>();

    main(argc, argv)
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: usize, argv: *const *const u8) -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        intrinsics::__aarch64_have_lse_atomics = false;
    }
    init_from_args(argc, argv);

    print_arch();

    println!("{}", env::kernel_version());

    unsafe { syscall!([!] Sysno::exit, 69) }
}

fn print_arch() {
    #[cfg(target_arch = "x86")]
    println!("x86");
    #[cfg(target_arch = "x86_64")]
    println!("x86_64");
    #[cfg(target_arch = "arm")]
    println!("arm");
    #[cfg(target_arch = "aarch64")]
    println!("aarch64");
    #[cfg(target_arch = "riscv64")]
    println!("riscv64");
    #[cfg(target_arch = "powerpc")]
    println!("powerpc");
    #[cfg(target_arch = "powerpc64")]
    println!("powerpc64");
    #[cfg(target_arch = "mips")]
    println!("mips");
    #[cfg(target_arch = "mips64")]
    println!("mips64");
    #[cfg(target_arch = "s390x")]
    println!("s390x");
    #[cfg(target_arch = "loongarch64")]
    println!("loongarch64");
}
