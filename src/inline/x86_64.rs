use crate::{Errno, Sysno};

use core::arch::asm;

#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

pub use syscall0 as syscall0_readonly;

#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "syscall",
        "ud2",
        in("rax") sysno as usize,
        in("rdi") arg0,
        options(noreturn)
    )
}

#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall4_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall5_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}
#[inline]
pub unsafe fn syscall6_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
pub(crate) fn init() {}
