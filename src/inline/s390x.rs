use crate::Sysno;

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        lateout("r2") ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

pub use raw_syscall0 as raw_syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1(sysno: Sysno, arg0: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "svc 0",
        "trap2",
        in("r1") sysno as usize,
        in("r2") arg0,
        options(noreturn)
    )
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
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
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        in("r5") arg3,
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
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        in("r5") arg3,
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
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        in("r5") arg3,
        in("r6") arg4,
        options(preserves_flags)
    );
    ret
}

pub use raw_syscall5 as raw_syscall5_readonly;

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
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        in("r5") arg3,
        in("r6") arg4,
        in("r7") arg5,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

pub use raw_syscall6 as raw_syscall6_readonly;

include!("_syscalls.rs");

#[inline(always)]
pub(crate) unsafe fn init() {}
