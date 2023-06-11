use crate::{Errno, Sysno};

use core::arch::asm;

#[doc = include_str!("../../docs/syscall0_readonly.md")]
#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        lateout("r0") ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

pub use syscall0 as syscall0_readonly;

#[doc = include_str!("../../docs/syscall1.md")]
#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall1_readonly.md")]
#[inline]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall1_noreturn.md")]
#[inline]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "mov r7, {sysno}",
        "svc 0",
        "udf #16",
        sysno = in(reg) sysno as usize,
        in("r0") arg0,
        options(noreturn)
    )
}

#[doc = include_str!("../../docs/syscall2.md")]
#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall2_readonly.md")]
#[inline]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall3.md")]
#[inline]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall3_readonly.md")]
#[inline]
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret;
    asm!(
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall4.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall4_readonly.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall5.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        in("r4") arg4,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall5_readonly.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        in("r4") arg4,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall6.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        in("r4") arg4,
        in("r5") arg5,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall6_readonly.md")]
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
        "mov {tmp}, r7",
        "mov r7, {sysno}",
        "svc 0",
        "mov r7, {tmp}",
        sysno = in(reg) sysno as usize,
        tmp = out(reg) _,
        inlateout("r0") arg0 => ret,
        in("r1") arg1,
        in("r2") arg2,
        in("r3") arg3,
        in("r4") arg4,
        in("r5") arg5,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
pub(crate) fn init() {}
