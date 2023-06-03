#![no_std]
#![cfg_attr(asm_experimental_arch, feature(asm_experimental_arch))]

mod bitflags;

#[cfg(target_os = "linux")]
pub use linux_errnos::Errno;
#[cfg(target_os = "linux")]
pub use linux_sysno::Sysno;

#[cfg(target_os = "linux")]
pub mod env;

#[cfg(target_os = "linux")]
mod init;

#[cfg(all(target_os = "linux", not(feature = "bare")))]
pub use init::init;
#[cfg(all(target_os = "linux", feature = "bare"))]
pub use init::{init_from_args, init_from_environ};

#[cfg(all(target_os = "linux", outline_syscalls))]
#[cfg_attr(
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "s390x",
        target_arch = "loongarch64"
    ),
    path = "outline/common.rs"
)]
#[cfg_attr(target_arch = "x86", path = "outline/x86.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "outline/powerpc64.rs")]
mod arch;

#[cfg(all(target_os = "linux", not(outline_syscalls)))]
#[cfg_attr(all(target_arch = "arm", not(thumb_mode)), path = "inline/arm.rs")]
#[cfg_attr(all(target_arch = "arm", thumb_mode), path = "inline/thumb.rs")]
#[cfg_attr(target_arch = "aarch64", path = "inline/aarch64.rs")]
#[cfg_attr(target_arch = "x86_64", path = "inline/x86_64.rs")]
#[cfg_attr(target_arch = "x86", path = "inline/x86.rs")]
#[cfg_attr(target_arch = "riscv64", path = "inline/riscv64.rs")]
#[cfg_attr(target_arch = "powerpc64", path = "inline/powerpc64.rs")]
#[cfg_attr(target_arch = "mips", path = "inline/mips.rs")]
#[cfg_attr(target_arch = "mips64", path = "inline/mips64.rs")]
#[cfg_attr(target_arch = "s390x", path = "inline/s390x.rs")]
#[cfg_attr(target_arch = "loongarch64", path = "inline/loongarch64.rs")]
mod arch;

#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
pub use arch::has_scv;
#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv64",
        target_arch = "powerpc64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "s390x",
        target_arch = "loongarch64"
    )
))]
pub use arch::{
    syscall0, syscall0_readonly, syscall1, syscall1_noreturn, syscall1_readonly, syscall2,
    syscall2_readonly, syscall3, syscall3_readonly, syscall4, syscall4_readonly, syscall5,
    syscall5_readonly, syscall6, syscall6_readonly,
};
#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub use arch::{syscall7, syscall7_readonly};

#[cfg(all(
    target_os = "linux",
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "riscv64",
        target_arch = "powerpc64",
        target_arch = "mips64",
        target_arch = "s390x",
        target_arch = "loongarch64"
    )
))]
#[macro_export]
macro_rules! syscall {
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::syscall6_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr $(,)?) => {
        $crate::syscall5_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::syscall4_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::syscall3_readonly($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::syscall2_readonly($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ([!] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1_noreturn($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1_readonly($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr $(,)?) => {
        $crate::syscall0_readonly($sysno)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::syscall6(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr $(,)?) => {
        $crate::syscall5(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::syscall4(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::syscall3($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::syscall2($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ($sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1($sysno, ($arg0) as usize)
    };
    ($sysno:expr $(,)?) => {
        $crate::syscall0($sysno)
    };
}

#[cfg(all(target_os = "linux", target_arch = "mips"))]
#[macro_export]
macro_rules! syscall {
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr $(,)?) => {
        $crate::syscall7_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
            ($arg6) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::syscall6_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr $(,)?) => {
        $crate::syscall5_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::syscall4_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::syscall3_readonly($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::syscall2_readonly($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ([!] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1_noreturn($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1_readonly($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr $(,)?) => {
        $crate::syscall0_readonly($sysno)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr $(,)?) => {
        $crate::syscall7(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
            ($arg6) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::syscall6(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
            ($arg5) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr $(,)?) => {
        $crate::syscall5(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::syscall4(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::syscall3($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::syscall2($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ($sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1($sysno, ($arg0) as usize)
    };
    ($sysno:expr $(,)?) => {
        $crate::syscall0($sysno)
    };
}
