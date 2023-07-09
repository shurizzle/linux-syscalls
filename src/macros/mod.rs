/// Make a syscall and returns a `Result<usize, Errno>`
///
/// Accept a [`Sysno`] as the first parameter and a variable number of arguments (0 to 6).
/// It calls syscallN under the hood where N is the number of arguments.
///
/// The are two variations:
/// - `[ro]`: use the `_readonly` version of `syscallN`.
/// - `[!]`: use the `_noreturn` version of `syscall1` (useful for [`exit`]).
///
/// Use the previous tags if you what are you doing, otherwise you can omit them.
///
/// # Example
///
/// ```
/// use linux_syscalls::{Errno, Sysno, syscall};
///
/// let mut buf: [u8; 1024] = [0; 1024];
/// let buf = loop {
///     match unsafe { syscall!(Sysno::read, 0, buf.as_mut_ptr(), buf.len()) } {
///         Ok(n) => break &buf[..n],
///         Err(Errno::EINTR) => (),
///         Err(err) => return Err(err),
///     }
/// };
///
/// // [ro] because write do not change the memory.
/// unsafe { syscall!([ro] Sysno::write, 1, buf.as_ptr(), buf.len())? };
///
/// // [!] because exit do not return.
/// unsafe { syscall!([!] Sysno::exit, 0) };
/// ```
///
/// [`Sysno`]: crate::Sysno
/// [`exit`]: crate::Sysno::exit
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

/// Make a syscall and returns a `usize`
///
/// Accept a [`Sysno`] as the first parameter and a variable number of arguments (0 to 6).
/// It calls syscallN under the hood where N is the number of arguments.
///
/// The are two variations:
/// - `[ro]`: use the `_readonly` version of `raw_syscallN`.
/// - `[!]`: use the `_noreturn` version of `syscall1` (useful for [`exit`]).
///
/// Use the previous tags if you what are you doing, otherwise you can omit them.
///
/// # Example
///
/// ```
/// use linux_syscalls::{Sysno, raw_syscall};
///
/// println!("{}", raw_syscall!([ro] Sysno::brk, 0));
///
/// // [!] because exit do not return.
/// unsafe { raw_syscall!([!] Sysno::exit, 0) };
/// ```
///
/// [`Sysno`]: crate::Sysno
/// [`exit`]: crate::Sysno::exit
#[macro_export]
macro_rules! raw_syscall {
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::raw_syscall6_readonly(
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
        $crate::raw_syscall5_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::raw_syscall4_readonly(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::raw_syscall3_readonly($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::raw_syscall2_readonly($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ([!] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::syscall1_noreturn($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr, $arg0:expr $(,)?) => {
        $crate::raw_syscall1_readonly($sysno, ($arg0) as usize)
    };
    ([ro] $sysno:expr $(,)?) => {
        $crate::raw_syscall0_readonly($sysno)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr $(,)?) => {
        $crate::raw_syscall6(
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
        $crate::raw_syscall5(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
            ($arg4) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr $(,)?) => {
        $crate::raw_syscall4(
            $sysno,
            ($arg0) as usize,
            ($arg1) as usize,
            ($arg2) as usize,
            ($arg3) as usize,
        )
    };
    ($sysno:expr, $arg0:expr, $arg1:expr, $arg2:expr $(,)?) => {
        $crate::raw_syscall3($sysno, ($arg0) as usize, ($arg1) as usize, ($arg2) as usize)
    };
    ($sysno:expr, $arg0:expr, $arg1:expr $(,)?) => {
        $crate::raw_syscall2($sysno, ($arg0) as usize, ($arg1) as usize)
    };
    ($sysno:expr, $arg0:expr $(,)?) => {
        $crate::raw_syscall1($sysno, ($arg0) as usize)
    };
    ($sysno:expr $(,)?) => {
        $crate::raw_syscall0($sysno)
    };
}
