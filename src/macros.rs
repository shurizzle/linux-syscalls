#[cfg(any(
    all(target_arch = "x86_64", target_endian = "little"),
    all(target_arch = "aarch64", target_pointer_width = "64"),
    all(target_arch = "arm", target_pointer_width = "32"),
    all(
        target_arch = "riscv64",
        target_endian = "little",
        target_pointer_width = "64"
    ),
    all(
        target_arch = "riscv32",
        target_endian = "little",
        target_pointer_width = "32"
    ),
    all(target_arch = "mips64", target_pointer_width = "64"),
    all(
        target_arch = "s390x",
        target_endian = "big",
        target_pointer_width = "64"
    ),
    all(
        target_arch = "loongarch64",
        target_endian = "little",
        target_pointer_width = "64"
    ),
    all(
        target_arch = "x86",
        target_endian = "little",
        target_pointer_width = "32"
    ),
    all(
        target_arch = "powerpc",
        target_endian = "big",
        target_pointer_width = "32"
    ),
    all(target_arch = "powerpc64", target_pointer_width = "64"),
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

#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
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
