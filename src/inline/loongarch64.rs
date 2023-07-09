use crate::Sysno;

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        lateout("$a0") ret,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "syscall 0",
        "break 1",
        in("$a7") sysno as usize,
        in("$a0") arg0,
        options(noreturn)
    )
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    asm!(
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        in("$a4") arg4,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        in("$a4") arg4,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        in("$a4") arg4,
        in("$a5") arg5,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
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
        "syscall 0",
        in("$a7") sysno as usize,
        inlateout("$a0") arg0 => ret,
        in("$a1") arg1,
        in("$a2") arg2,
        in("$a3") arg3,
        in("$a4") arg4,
        in("$a5") arg5,
        lateout("$t0") _,
        lateout("$t1") _,
        lateout("$t2") _,
        lateout("$t3") _,
        lateout("$t4") _,
        lateout("$t5") _,
        lateout("$t6") _,
        lateout("$t7") _,
        lateout("$t8") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

include!("_syscalls.rs");

#[inline(always)]
pub(crate) fn init() {}
