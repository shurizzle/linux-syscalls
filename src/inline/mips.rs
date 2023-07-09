use crate::{Errno, Sysno};

use core::arch::asm;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

pub use syscall0 as syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "syscall",
        in("$2") sysno as usize,
        in("$4") arg0,
        options(noreturn)
    )
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        lateout("$7") err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        "syscall",
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(nostack, preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        in(reg) arg5,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        in(reg) arg5,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "sw {}, 24($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        in(reg) arg5,
        in(reg) arg6,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
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
    let ret: usize;
    let err: usize;
    asm!(
        ".set noat",
        "subu $sp, 32",
        "sw {}, 16($sp)",
        "sw {}, 20($sp)",
        "sw {}, 24($sp)",
        "syscall",
        "addu $sp, 32",
        ".set at",
        in(reg) arg4,
        in(reg) arg5,
        in(reg) arg6,
        inlateout("$2") sysno as usize => ret,
        in("$4") arg0,
        in("$5") arg1,
        in("$6") arg2,
        inlateout("$7") arg3 => err,
        lateout("$8") _,
        lateout("$9") _,
        lateout("$10") _,
        lateout("$11") _,
        lateout("$12") _,
        lateout("$13") _,
        lateout("$14") _,
        lateout("$15") _,
        lateout("$24") _,
        lateout("$25") _,
        options(preserves_flags, readonly)
    );
    if err != 0 {
        Err(Errno::new(ret as i32))
    } else {
        Ok(ret)
    }
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    syscall0(sysno).unwrap_or(0)
}

pub use raw_syscall0 as raw_syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1(sysno: Sysno, arg0: usize) -> usize {
    syscall1(sysno, arg0).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    syscall1_readonly(sysno, arg0).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    syscall2(sysno, arg0, arg1).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    syscall2_readonly(sysno, arg0, arg1).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    syscall3(sysno, arg0, arg1, arg2).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    syscall3_readonly(sysno, arg0, arg1, arg2).unwrap_or(0)
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
    syscall4(sysno, arg0, arg1, arg2, arg3).unwrap_or(0)
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
    syscall4_readonly(sysno, arg0, arg1, arg2, arg3).unwrap_or(0)
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
    syscall5(sysno, arg0, arg1, arg2, arg3, arg4).unwrap_or(0)
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
    syscall5_readonly(sysno, arg0, arg1, arg2, arg3, arg4).unwrap_or(0)
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
    syscall6(sysno, arg0, arg1, arg2, arg3, arg4, arg5).unwrap_or(0)
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
    syscall6_readonly(sysno, arg0, arg1, arg2, arg3, arg4, arg5).unwrap_or(0)
}

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
    syscall7(sysno, arg0, arg1, arg2, arg3, arg4, arg5, arg6).unwrap_or(0)
}

#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline(always)]
pub unsafe fn raw_syscall7_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    syscall7_readonly(sysno, arg0, arg1, arg2, arg3, arg4, arg5, arg6).unwrap_or(0)
}

#[inline(always)]
pub(crate) unsafe fn init() {}
