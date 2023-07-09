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
    #[allow(clippy::too_many_arguments)]
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
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    linux_syscalls_rs_syscall0(sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1(sysno: Sysno, arg0: usize) -> usize {
    linux_syscalls_rs_syscall1(arg0, sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    linux_syscalls_rs_syscall2(arg0, arg1, sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    linux_syscalls_rs_syscall3(arg0, arg1, arg2, sysno as usize)
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
    linux_syscalls_rs_syscall4(arg0, arg1, arg2, arg3, sysno as usize)
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
    linux_syscalls_rs_syscall5(arg0, arg1, arg2, arg3, arg4, sysno as usize)
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
    linux_syscalls_rs_syscall6(arg0, arg1, arg2, arg3, arg4, arg5, sysno as usize)
}

#[cfg(target_arch = "mips")]
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline(always)]
pub unsafe fn raw_syscall7(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    linux_syscalls_rs_syscall7(arg0, arg1, arg2, arg3, arg4, arg5, arg6, sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall0(sysno))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall1(sysno, arg0))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    linux_syscalls_rs_syscall1_noreturn(arg0, sysno as usize)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall2(sysno, arg0, arg1))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall3(sysno, arg0, arg1, arg2))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall4(sysno, arg0, arg1, arg2, arg3))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall5(sysno, arg0, arg1, arg2, arg3, arg4))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall6(sysno, arg0, arg1, arg2, arg3, arg4, arg5))
}

#[cfg(target_arch = "mips")]
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline(always)]
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
    Errno::from_ret(raw_syscall7(
        sysno, arg0, arg1, arg2, arg3, arg4, arg5, arg6,
    ))
}

pub use {
    raw_syscall0 as raw_syscall0_readonly, raw_syscall1 as raw_syscall1_readonly,
    raw_syscall2 as raw_syscall2_readonly, raw_syscall3 as raw_syscall3_readonly,
    raw_syscall4 as raw_syscall4_readonly, raw_syscall5 as raw_syscall5_readonly,
    raw_syscall6 as raw_syscall6_readonly, syscall0 as syscall0_readonly,
    syscall1 as syscall1_readonly, syscall2 as syscall2_readonly, syscall3 as syscall3_readonly,
    syscall4 as syscall4_readonly, syscall5 as syscall5_readonly, syscall6 as syscall6_readonly,
};
#[cfg(target_arch = "mips")]
pub use {raw_syscall7 as raw_syscall7_readonly, syscall7 as syscall7_readonly};

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn init() {}
