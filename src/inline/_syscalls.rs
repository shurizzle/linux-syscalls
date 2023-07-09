use crate::Errno;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall0(sysno))
}

pub use syscall0 as syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall1(sysno, arg0))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall1_readonly(sysno, arg0))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall2(sysno, arg0, arg1))
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall2_readonly(sysno, arg0, arg1))
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
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall3_readonly(sysno, arg0, arg1, arg2))
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
pub unsafe fn syscall4_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall4_readonly(sysno, arg0, arg1, arg2, arg3))
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
pub unsafe fn syscall5_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall5_readonly(sysno, arg0, arg1, arg2, arg3, arg4))
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

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall6_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall6_readonly(
        sysno, arg0, arg1, arg2, arg3, arg4, arg5,
    ))
}

#[cfg(target_arch = "mips")]
#[allow(clippy::missing_safety_doc)]
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

#[cfg(target_arch = "mips")]
#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall7_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> Result<usize, Errno> {
    Errno::from_ret(raw_syscall7_readonly(
        sysno, arg0, arg1, arg2, arg3, arg4, arg5, arg6,
    ))
}
