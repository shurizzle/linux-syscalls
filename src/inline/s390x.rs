use crate::{Errno, Sysno};

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        lateout("r2") ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

pub use syscall0 as syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
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
#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        options(nostack, preserves_flags)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "svc 0",
        in("r1") sysno as usize,
        inlateout("r2") arg0 => ret,
        in("r3") arg1,
        in("r4") arg2,
        options(nostack, preserves_flags, readonly)
    );
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
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
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall4_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
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
    Errno::from_ret(ret)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
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
    Errno::from_ret(ret)
}

pub use syscall5 as syscall5_readonly;

#[allow(clippy::missing_safety_doc)]
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
    Errno::from_ret(ret)
}

pub use syscall6 as syscall6_readonly;

#[inline(always)]
pub(crate) unsafe fn init() {}
