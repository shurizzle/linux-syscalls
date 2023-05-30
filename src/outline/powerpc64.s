# Syscall registers
# +-------+------+------+------+------+------+------+--------+
# | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
# +-------+------+------+------+------+------+------+--------+
# |  r0   |  r3  |  r4  |  r5  |  r6  |  r7  |  r8  |   r3   |
# +-------+------+------+------+------+------+------+--------+
#
# sysv calling convention
# +----------+---------------------------------+-------------------+
# |  return  |             params              | additional params |
# +----------+---------------------------------+-------------------+
# |    r3    | r3, r4, r5, r6, r7, r8, r9, r10 |    stack RTL      |
# +----------+---------------------------------+-------------------+
#
# We can pass syscall's parameters as a normal call and move the last
# parameter to r0.

  .section .text

# HACK: "old" assemblers (and clang) don't recognize scv mnemonic
.macro SCV0
  .byte 0x44
  .byte 0x00
  .byte 0x00
  .byte 0x01
.endm

  .globl linux_syscalls_rs_scvsyscall0
linux_syscalls_rs_scvsyscall0:
  mr   0, 3
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall1
linux_syscalls_rs_scvsyscall1:
  mr   0, 4
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall1_noreturn
linux_syscalls_rs_scvsyscall1_noreturn:
  mr   0, 4
  SCV0
  trap

  .globl linux_syscalls_rs_scvsyscall2
linux_syscalls_rs_scvsyscall2:
  mr   0, 5
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall3
linux_syscalls_rs_scvsyscall3:
  mr   0, 6
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall4
linux_syscalls_rs_scvsyscall4:
  mr   0, 7
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall5
linux_syscalls_rs_scvsyscall5:
  mr   0, 8
  SCV0
  blr

  .globl linux_syscalls_rs_scvsyscall6
linux_syscalls_rs_scvsyscall6:
  mr   0, 9
  SCV0
  blr

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  mr    0, 3
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  mr    0, 4
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mr    0, 4
  sc
  trap

  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  mr    0, 5
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  mr    0, 6
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  mr    0, 7
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  mr    0, 8
  sc
  bnslr
  neg   3, 3
  blr

  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  mr    0, 9
  sc
  bnslr
  neg   3, 3
  blr
