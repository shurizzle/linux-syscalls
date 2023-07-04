# linux-syscalls

![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/shurizzle/linux-syscalls/unit-tests.yml?branch=master&style=for-the-badge)
[![Crates.io](https://img.shields.io/crates/v/linux-syscalls?style=for-the-badge)](https://crates.io/crates/linux-syscalls)
[![docs.rs](https://img.shields.io/docsrs/linux-syscalls?style=for-the-badge)](https://docs.rs/linux-syscalls)
![Crates.io](https://img.shields.io/crates/l/linux-syscalls?style=for-the-badge)

A library to easily deal with linux system calls.

![One does not simply make a syscall](https://i.imgflip.com/7ndzqd.jpg "memino")

As a side effect this library offers other utilities.

#### uname

This library invoke `uname` syscall in initialization and cache its result.

#### kernel version

Parse kernel version from `uname` in initialization (panics if it's invalid)
and offers a macro `kversion!` to deal with it.

#### auxiliary vector

Detect auxiliary vector (if present) in initialization and offer a Rust
interface to query its values. Useful for hardware/kernel features
like `scv` in ppc64.

#### vDSO

Detect vDSO and parse it (if present) in initialization and store its result.
Useful for vsyscalls on any platform and `__kernel_vsyscall` for faster
syscalls on x86 machines.

### Feature flags

- `std`: enable std support.
- `bare`: do not try to automatically initialize the library.
- `libc-compat`: forward this feature to `linux-errnos`.

### `#![no_std]`

Enable `#![no_std]` support by disabling the default `std` feature:

```toml
[dependencies]
linux-syscalls = { version = "*", default-features = false }
```

### Supported platforms

#### Tier 1

- aarch64-unknown-linux-gnu
- i686-unknown-linux-gnu
- x86_64-unknown-linux-gnu

#### Tier 2

- aarch64-unknown-linux-musl
- arm-unknown-linux-gnueabi
- arm-unknown-linux-gnueabihf
- arm-unknown-linux-musleabi
- arm-unknown-linux-musleabihf
- armv5te-unknown-linux-gnueabi
- armv5te-unknown-linux-musleabi
- armv7-unknown-linux-gnueabi
- armv7-unknown-linux-gnueabihf
- armv7-unknown-linux-musleabi
- armv7-unknown-linux-musleabihf
- i586-unknown-linux-gnu
- i586-unknown-linux-musl
- i686-unknown-linux-musl
- loongarch64-unknown-linux-gnu
- mips-unknown-linux-gnu
- mips-unknown-linux-musl
- mips64-unknown-linux-gnuabi64
- mips64-unknown-linux-muslabi64
- mips64el-unknown-linux-gnuabi64
- mips64el-unknown-linux-muslabi64
- mipsel-unknown-linux-gnu
- mipsel-unknown-linux-musl
- powerpc-unknown-linux-gnu
- powerpc64-unknown-linux-gnu
- powerpc64le-unknown-linux-gnu
- riscv64gc-unknown-linux-gnu
- s390x-unknown-linux-gnu
- thumbv7neon-unknown-linux-gnueabihf
- x86_64-unknown-linux-musl

### MSRV

1.40.0
