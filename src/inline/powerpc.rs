use crate::Sysno;

macro_rules! sys {
    ($($tt:tt)+) => {
        ::core::arch::asm!(
            "sc",
            "bns 0f",
            "neg 3, 3",
            "0:",
            $($tt)+
        );
    };
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall0(sysno: Sysno) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        lateout("r3") ret,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)

    );
    ret
}

pub use raw_syscall0 as raw_syscall0_readonly;

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall1_readonly(sysno: Sysno, arg0: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        lateout("r4") _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    ::core::arch::asm!(
        "sc",
        "trap",
        in("r0") sysno as usize,
        in("r3") arg0,
        options(noreturn)
    );
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        lateout("r5") _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags)
    );
    ret
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub unsafe fn raw_syscall3_readonly(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        lateout("r6") _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        lateout("r7") _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        inlateout("r7") arg4 => _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        inlateout("r7") arg4 => _,
        lateout("r8") _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        inlateout("r7") arg4 => _,
        inlateout("r8") arg5 => _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
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
    sys!(
        inlateout("r0") sysno as usize => _,
        inlateout("r3") arg0 => ret,
        inlateout("r4") arg1 => _,
        inlateout("r5") arg2 => _,
        inlateout("r6") arg3 => _,
        inlateout("r7") arg4 => _,
        inlateout("r8") arg5 => _,
        lateout("r9") _,
        lateout("r10") _,
        lateout("r11") _,
        lateout("r12") _,
        lateout("cr0") _,
        options(nostack, preserves_flags, readonly)
    );
    ret
}

include!("_syscalls.rs");

#[inline(always)]
pub(crate) unsafe fn init() {}
