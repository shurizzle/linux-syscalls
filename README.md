# linux-syscalls

![One does not simply make a syscall](https://i.imgflip.com/7ndzqd.jpg "memino")

### Platforms

**Tier 1**
- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- i686-unknown-linux-gnu

**Tier 2**
- arm-unknown-linux-gnueabi
- arm-unknown-linux-gnueabihf
- armv4t-unknown-linux-gnueabi
- armv7-unknown-linux-gnueabihf
- thumbv7neon-unknown-linux-gnueabihf
- riscv64gc-unknown-linux-gnu
- powerpc64-unknown-linux-gnu
- mips-unknown-linux-gnu
- mips64-unknown-linux-gnuabi64
- s390x-unknown-linux-gnu
- i586-unknown-linux-gnu
- i686-unknown-linux-gnu
- loongarch64-unknown-linux-gnu

**Tier 3**
- armv5te-unknown-linux-gnueabi
- armv7-unknown-linux-gnueabi

# Unsupported platforms

```bash
rustc +nightly --print target-list | \
    grep -P -- '-linux(-|$)' | \
    grep -vP '^((x86(_64)?|s390x|aarch64|i[56]86|mips|powerpc64|riscv64gc|loongarch64)-unknown-linux-gnu|(arm(v4t|v5te|v7)?|thumbv7neon)-unknown-linux-gnueabi(hf)?|mips64-unknown-linux-gnuabi64)$'
```

### MSRV

1.40.0
