#![no_std]
#![no_main]
#![cfg_attr(not(test), feature(lang_items))]

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

extern crate linux_syscalls;
use core::fmt::Write;

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

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[no_mangle]
unsafe extern "C" fn memcpy(mut dest: *mut u8, mut src: *const u8, n: usize) {
    let end = dest.add(n);
    while (dest as usize) < (end as usize) {
        dest = dest.add(1);
        src = src.add(1);
    }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[no_mangle]
unsafe extern "C" fn memcmp(mut dest: *const u8, mut src: *const u8, n: usize) -> i32 {
    let end = dest.add(n);
    while (dest as usize) < (end as usize) {
        let res = (*dest as i16) - (*end as i16);

        if res < 0 {
            return -1;
        } else if res > 0 {
            return 1;
        }

        dest = dest.add(1);
        src = src.add(1);
    }
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = write!(stderr(), "{}", info);
    unsafe { syscall!([!] Sysno::exit, 1) }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov rdi, [rsp]",
    "mov rsi, rsp",
    "add rsi, 8",
    "call main",
);

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_pointer_width = "32"
))]
core::arch::global_asm!(
    ".globl _start",
    "_start:",
    "mov rdi, [rsp]",
    "mov rsi, rsp",
    "add rsi, 4",
    "call main",
);

#[no_mangle]
pub extern "C" fn main(argc: isize, argv: *const *const u8) -> isize {
    init_from_args(argc, argv);

    let mut stdout = stdout();
    writeln!(stdout, "{}", env::kernel_version()).unwrap();

    unsafe { syscall!([!] Sysno::exit, 69) }
}
