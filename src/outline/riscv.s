# Syscall registers
# +-------+------+------+------+------+------+------+--------+
# | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
# +-------+------+------+------+------+------+------+--------+
# |  a7   |  a0  |  a1  |  a2  |  a3  |  a4  |  a5  |   a0   |
# +-------+------+------+------+------+------+------+--------+
#
# sysv calling convention
# +----------+--------------------------------+-------------------+
# |  return  |            params              | additional params |
# +----------+--------------------------------+-------------------+
# |  a0, a1  | a0, a1, a2, a3, a4, a5, a6, a7 |    stack RTL      |
# +----------+--------------------------------+-------------------+
#
# Registers are the same, we just need to copy last argument (sysno) in a7.

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  mv    a7, a0
  ecall
  ret

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  mv    a7, a1
  ecall
  ret

  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mv    a7, a1
  ecall
  unimp

  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  mv    a7, a2
  ecall
  ret

  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  mv    a7, a3
  ecall
  ret

  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  mv    a7, a4
  ecall
  ret

  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  mv    a7, a5
  ecall
  ret

  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  mv    a7, a6
  ecall
  ret
