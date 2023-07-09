use crate::Sysno;

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

pub use raw_syscall0 as raw_syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") sysno as usize => ret,
        in("rdi") arg0,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "syscall",
        "ud2",
        in("rax") sysno as usize,
        in("rdi") arg0,
        options(noreturn)
    )
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall4_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall5_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
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
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall6_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
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
    ret
}

include!("_syscalls.rs");

#[inline(always)]
pub(crate) fn init() {}
