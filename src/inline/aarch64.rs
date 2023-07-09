use crate::Sysno;

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        lateout("x0") ret,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "svc 0",
        "brk 1",
        in("x8") sysno as usize,
        in("x0") arg0,
        options(noreturn)
    )
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
        in("x4") arg4,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
        in("x4") arg4,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
        in("x4") arg4,
        in("x5") arg5,
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
        "svc 0",
        in("x8") sysno as usize,
        inlateout("x0") arg0 => ret,
        in("x1") arg1,
        in("x2") arg2,
        in("x3") arg3,
        in("x4") arg4,
        in("x5") arg5,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

include!("_syscalls.rs");

#[inline(always)]
pub(crate) fn init() {}
