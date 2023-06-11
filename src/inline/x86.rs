use crate::{Errno, Sysno};

use core::arch::asm;

#[inline(always)]
unsafe fn callee() -> usize {
    (*crate::env::vdso::VDSO.get()).0.vsyscall as usize
}

#[inline(always)]
unsafe fn vsyscall0(sysno: Sysno) -> Result<usize, Errno> {
    let ret;
    asm!(
        "call {callee}",
        callee = in(reg) callee(),
        inlateout("rax") sysno as usize => ret,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "call {callee}",
        callee = in(reg) callee(),
        inlateout("eax") sysno as usize => ret,
        in("ebx") arg0,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    asm!(
        "call {callee}",
        "ud2",
        callee = in(reg) callee(),
        in("eax") sysno as usize,
        in("ebx") arg0,
        options(noreturn)
    )
}

#[inline(always)]
unsafe fn vsyscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "call {callee}",
        callee = in(reg) callee(),
        inlateout("eax") sysno as usize => ret,
        in("ebx") arg0,
        in("ecx") arg1,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall3(sysno: Sysno, arg0: usize, arg1: usize, arg2: usize) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "call {callee}",
        callee = in(reg) callee(),
        inlateout("eax") sysno as usize => ret,
        in("ebx") arg0,
        in("ecx") arg1,
        in("edx") arg2,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    let ret: usize;
    asm!(
        "xchg esi, {arg3}",
        "call edi",
        "xchg esi, {arg3}",
        arg3 = in(reg) arg3,
        in("edi") callee(),
        inlateout("eax") sysno as usize => ret,
        in("ebx") arg0,
        in("ecx") arg1,
        in("edx") arg2,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    // NOTE: We don't have enough registers to handle this and we can't
    // use stack so we pass a slice as a secondary stack.
    let ret: usize;
    asm!(
        // save register
        "push esi",

        // push syscall parameter
        "push DWORD PTR [eax]",
        "mov esi, DWORD PTR [eax+4]",
        "mov eax, DWORD PTR [eax+8]",
        "call DWORD PTR [esp]",

        // delete pushed parameter
        "pop esi",

        // restore register
        "pop esi",
        inout("eax") &[callee(), arg3, sysno as usize] => ret,
        in("ebx") arg0,
        in("ecx") arg1,
        in("edx") arg2,
        in("edi") arg4,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[inline(always)]
unsafe fn vsyscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    // NOTE: Same as syscall5 but we have to fight for ebp too.

    let ret: usize;
    asm!(
        // save registers
        "push ebp",
        "push esi",

        // push syscall parameter
        "push DWORD PTR [eax]",
        "mov esi, DWORD PTR [eax+4]",
        "mov ebp, DWORD PTR [eax+8]",
        "mov eax, DWORD PTR [eax+12]",
        "call DWORD PTR [esp]",

        // delete pushed parameter
        "pop esi",

        // restore registers
        "pop esi",
        "pop ebp",
        inout("eax") &[callee(), arg3, arg5, sysno as usize] => ret,
        in("ebx") arg0,
        in("ecx") arg1,
        in("edx") arg2,
        in("edi") arg4,
        options(preserves_flags)
    );
    Errno::from_ret(ret)
}

#[doc = include_str!("../../docs/syscall0_readonly.md")]
#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall0(sysno)
    }
}

pub use syscall0 as syscall0_readonly;

#[doc = include_str!("../../docs/syscall1.md")]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            options(nostack, preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall1(sysno, arg0)
    }
}

#[doc = include_str!("../../docs/syscall1_readonly.md")]
pub unsafe fn syscall1_readonly(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall1(sysno, arg0)
    }
}

#[doc = include_str!("../../docs/syscall1_noreturn.md")]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    if callee() == 0 {
        asm!(
            "int $$0x80",
            "ud2",
            in("eax") sysno as usize,
            in("ebx") arg0,
            options(noreturn)
        )
    } else {
        vsyscall1_noreturn(sysno, arg0)
    }
}

#[doc = include_str!("../../docs/syscall2.md")]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            options(nostack, preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall2(sysno, arg0, arg1)
    }
}

#[doc = include_str!("../../docs/syscall2_readonly.md")]
pub unsafe fn syscall2_readonly(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall2(sysno, arg0, arg1)
    }
}

#[doc = include_str!("../../docs/syscall3.md")]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            options(nostack, preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall3(sysno, arg0, arg1, arg2)
    }
}

#[doc = include_str!("../../docs/syscall3_readonly.md")]
pub unsafe fn syscall3_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "int $$0x80",
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall3(sysno, arg0, arg1, arg2)
    }
}

#[doc = include_str!("../../docs/syscall4.md")]
pub unsafe fn syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        // We need to put arg3 in esi but asm macro won't let us do it directly.
        // So swap it prior to interrupt and swap again afterward.
        asm!(
            "xchg esi, {arg3}",
            "int $$0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            options(nostack, preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall4(sysno, arg0, arg1, arg2, arg3)
    }
}

#[doc = include_str!("../../docs/syscall4_readonly.md")]
pub unsafe fn syscall4_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "xchg esi, {arg3}",
            "int $$0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall4(sysno, arg0, arg1, arg2, arg3)
    }
}

#[doc = include_str!("../../docs/syscall5.md")]
pub unsafe fn syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "xchg esi, {arg3}",
            "int $$0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
            options(nostack, preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall5(sysno, arg0, arg1, arg2, arg3, arg4)
    }
}

#[doc = include_str!("../../docs/syscall5_readonly.md")]
pub unsafe fn syscall5_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "xchg esi, {arg3}",
            "int $$0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") sysno as usize => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
            options(nostack, preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall5(sysno, arg0, arg1, arg2, arg3, arg4)
    }
}

#[doc = include_str!("../../docs/syscall6.md")]
pub unsafe fn syscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        // Similary to syscall4/5, we need to put arg3 in esi and arg5 in ebp but
        // asm macro doesn't allow us to do it and we can't alloc memory on stack
        // and there are no other registers free.
        // So we create a temporary slice and pass its pointer to eax, now we can
        // copy data from slice to stack. Don't forget to preserve ebp and esi regs.
        asm!(
            "push ebp",
            "push esi",
            "mov esi, DWORD PTR [eax]",
            "mov ebp, DWORD PTR [eax+4]",
            "mov eax, DWORD PTR [eax+8]",
            "int $$0x80",
            "pop esi",
            "pop ebp",
            inlateout("eax") &[arg3, arg5, sysno as usize] => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
            options(preserves_flags)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall6(sysno, arg0, arg1, arg2, arg3, arg4, arg5)
    }
}

#[doc = include_str!("../../docs/syscall6_readonly.md")]
pub unsafe fn syscall6_readonly(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    if callee() == 0 {
        let ret;
        asm!(
            "push ebp",
            "push esi",
            "mov esi, DWORD PTR [eax]",
            "mov ebp, DWORD PTR [eax+4]",
            "mov eax, DWORD PTR [eax+8]",
            "int $$0x80",
            "pop esi",
            "pop ebp",
            inlateout("eax") &[arg3, arg5, sysno as usize] => ret,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
            options(preserves_flags, readonly)
        );
        Errno::from_ret(ret)
    } else {
        vsyscall6(sysno, arg0, arg1, arg2, arg3, arg4, arg5)
    }
}

#[inline(always)]
pub(crate) fn x86_init(_: *const ()) {}

#[inline(always)]
pub(crate) fn init() {}
