use crate::{Errno, Sysno};

use core::arch::asm;

#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret;
    asm!(
        "ecall",
        in("a7") sysno as usize,
        lateout("a0") ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

pub use syscall0 as syscall0_readonly;

#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "ecall",
        "unimp",
        in("a7") sysno as usize,
        in("a0") arg0,
        options(noreturn)
    )
}

#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
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
        "ecall",
        in("a7") sysno as usize,
        inlateout("a0") arg0 => ret,
        in("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
pub(crate) fn init() {}
