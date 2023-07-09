#![no_std]
#![cfg(all(
    target_os = "linux",
    any(
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
        all(target_arch = "mips", target_pointer_width = "32"),
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
            target_arch = "powerpc",
            target_endian = "big",
            target_pointer_width = "32"
        ),
        all(
            target_arch = "x86",
            target_endian = "little",
            target_pointer_width = "32"
        ),
        all(target_arch = "powerpc64", target_pointer_width = "64"),
    )
))]
#![cfg_attr(asm_experimental_arch, feature(asm_experimental_arch))]
#![cfg_attr(docs_rs, feature(doc_cfg))]

mod bitflags;

pub use linux_errnos::Errno;
pub use linux_sysno::Sysno;

pub mod env;
mod init;
#[cfg_attr(
    all(target_arch = "mips", target_pointer_width = "32"),
    path = "macros/mips.rs"
)]
mod macros;

#[cfg(any(doc, not(feature = "bare")))]
pub use init::init;
#[cfg(any(doc, feature = "bare"))]
pub use init::{init_from_args, init_from_environ};

#[cfg(outline_syscalls)]
#[cfg_attr(
    not(any(
        all(
            target_arch = "x86",
            target_endian = "little",
            target_pointer_width = "32"
        ),
        all(target_arch = "powerpc64", target_pointer_width = "64"),
    )),
    path = "outline/common.rs"
)]
#[cfg_attr(
    all(
        target_arch = "x86",
        target_endian = "little",
        target_pointer_width = "32"
    ),
    path = "outline/x86.rs"
)]
#[cfg_attr(
    all(target_arch = "powerpc64", target_pointer_width = "64"),
    path = "outline/powerpc64.rs"
)]
mod arch;

#[cfg(not(outline_syscalls))]
#[cfg_attr(
    all(target_arch = "arm", target_pointer_width = "32", not(thumb_mode)),
    path = "inline/arm.rs"
)]
#[cfg_attr(
    all(target_arch = "arm", target_pointer_width = "32", thumb_mode),
    path = "inline/thumb.rs"
)]
#[cfg_attr(
    all(target_arch = "aarch64", target_pointer_width = "64"),
    path = "inline/aarch64.rs"
)]
#[cfg_attr(
    all(target_arch = "x86_64", target_endian = "little"),
    path = "inline/x86_64.rs"
)]
#[cfg_attr(
    all(
        target_arch = "x86",
        target_endian = "little",
        target_pointer_width = "32"
    ),
    path = "inline/x86.rs"
)]
#[cfg_attr(
    any(
        all(
            target_arch = "riscv32",
            target_endian = "little",
            target_pointer_width = "32"
        ),
        all(
            target_arch = "riscv64",
            target_endian = "little",
            target_pointer_width = "64"
        )
    ),
    path = "inline/riscv.rs"
)]
#[cfg_attr(
    all(
        target_arch = "powerpc",
        target_endian = "big",
        target_pointer_width = "32"
    ),
    path = "inline/powerpc.rs"
)]
#[cfg_attr(
    all(target_arch = "powerpc64", target_pointer_width = "64"),
    path = "inline/powerpc64.rs"
)]
#[cfg_attr(
    all(target_arch = "mips", target_pointer_width = "32"),
    path = "inline/mips.rs"
)]
#[cfg_attr(
    all(target_arch = "mips64", target_pointer_width = "64"),
    path = "inline/mips64.rs"
)]
#[cfg_attr(
    all(
        target_arch = "s390x",
        target_endian = "big",
        target_pointer_width = "64"
    ),
    path = "inline/s390x.rs"
)]
#[cfg_attr(
    all(
        target_arch = "loongarch64",
        target_endian = "little",
        target_pointer_width = "64"
    ),
    path = "inline/loongarch64.rs"
)]
mod arch;

#[cfg(all(target_arch = "powerpc64", target_pointer_width = "64"))]
pub use arch::has_scv;

/// Make a raw system call with 0 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall0;

/// Make a raw system call with 1 argument.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall1;

/// Make a raw system call with 2 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall2;

/// Make a raw system call with 3 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall3;

/// Make a raw system call with 4 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall4;

/// Make a raw system call with 5 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall5;

/// Make a raw system call with 6 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall6;

#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
/// Make a raw system call with 7 arguments.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall7;

/// Make a raw system call with 0 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall0_readonly;

/// Make a raw system call with 1 argument.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall1_readonly;

/// Make a raw system call with 7 arguments.
/// It's assured that it will not return.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall1_noreturn;

/// Make a raw system call with 2 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall2_readonly;

/// Make a raw system call with 3 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall3_readonly;

/// Make a raw system call with 4 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall4_readonly;

/// Make a raw system call with 5 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall5_readonly;

/// Make a raw system call with 6 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall6_readonly;

#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
/// Make a raw system call with 7 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a `Result<usize, Errno>`.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::syscall7_readonly;

/// Make a raw system call with 0 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall0;

/// Make a raw system call with 1 argument.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall1;

/// Make a raw system call with 2 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall2;

/// Make a raw system call with 3 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall3;

/// Make a raw system call with 4 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall4;

/// Make a raw system call with 5 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall5;

/// Make a raw system call with 6 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall6;

#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
/// Make a raw system call with 7 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall7;

/// Make a raw system call with 0 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall0_readonly;

/// Make a raw system call with 1 argument.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall1_readonly;

/// Make a raw system call with 2 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall2_readonly;

/// Make a raw system call with 3 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall3_readonly;

/// Make a raw system call with 4 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall4_readonly;

/// Make a raw system call with 5 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall5_readonly;

/// Make a raw system call with 6 arguments.
/// Like the non `_readonly` version but you declare that syscall does not mutate any memory.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall6_readonly;

#[cfg(all(target_arch = "mips", target_pointer_width = "32"))]
/// Make a raw system call with 7 arguments.
///
/// Returns a raw `usize`, doesn't not check for errors.
///
/// # Safety
///
/// A system call is unsafe by definition.
/// It's the caller's responsibility to ensure safety.
pub use arch::raw_syscall7_readonly;
