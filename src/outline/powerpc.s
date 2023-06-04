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

.macro SYSCALL reg
  mr      0, \reg
  sc
  bnslr
  neg     3, 3
  blr
.endm

  .section .text

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  SYSCALL 3

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  SYSCALL 4

  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mr    0, 4
  sc
  trap

  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  SYSCALL 5

  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  SYSCALL 6

  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  SYSCALL 7

  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  SYSCALL 8

  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  SYSCALL 9
