use crate::{Errno, Sysno};

extern "C" {
    fn linux_syscalls_rs_syscall0(sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall1(arg0: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall1_noreturn(arg0: usize, sysno: usize) -> !;
    fn linux_syscalls_rs_syscall2(arg0: usize, arg1: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall3(arg0: usize, arg1: usize, arg2: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall4(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_syscall5(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_syscall6(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        sysno: usize,
    ) -> usize;

    #[cfg(target_arch = "mips")]
    fn linux_syscalls_rs_syscall7(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        arg6: usize,
        sysno: usize,
    ) -> usize;
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    Errno::from_ret(linux_syscalls_rs_syscall0(sysno as usize))
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    Errno::from_ret(linux_syscalls_rs_syscall1(arg0, sysno as usize))
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    linux_syscalls_rs_syscall1_noreturn(arg0, sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    Errno::from_ret(linux_syscalls_rs_syscall2(arg0, arg1, sysno as usize))
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(linux_syscalls_rs_syscall3(arg0, arg1, arg2, sysno as usize))
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
    Errno::from_ret(linux_syscalls_rs_syscall4(
        arg0,
        arg1,
        arg2,
        arg3,
        sysno as usize,
    ))
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
    Errno::from_ret(linux_syscalls_rs_syscall5(
        arg0,
        arg1,
        arg2,
        arg3,
        arg4,
        sysno as usize,
    ))
}

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
    Errno::from_ret(linux_syscalls_rs_syscall6(
        arg0,
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        sysno as usize,
    ))
}

#[cfg(target_arch = "mips")]
#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn syscall7(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(linux_syscalls_rs_syscall7(
        arg0,
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        sysno as usize,
    ))
}

#[cfg(target_arch = "mips")]
pub use syscall7 as syscall7_readonly;
pub use {
    syscall0 as syscall0_readonly, syscall1 as syscall1_readonly, syscall2 as syscall2_readonly,
    syscall3 as syscall3_readonly, syscall4 as syscall4_readonly, syscall5 as syscall5_readonly,
    syscall6 as syscall6_readonly,
};

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn init() {}
