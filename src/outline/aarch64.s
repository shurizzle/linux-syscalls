// Syscall registers
// +-------+------+------+------+------+------+------+--------+
// | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
// +-------+------+------+------+------+------+------+--------+
// |  x8   |  x0  |  x1  |  x2  |  x3  |  x4  |  x5  |   x0   |
// +-------+------+------+------+------+------+------+--------+
//
// sysv calling convention
// +----------+--------------------------------+-------------------+
// |  return  |            params              | additional params |
// +----------+--------------------------------+-------------------+
// |    x0    | x0, x1, x2, x3, x4, x5, x6, x7 |    stack RTL      |
// +----------+--------------------------------+-------------------+
//
// Registers are the same, we just need to copy last argument (sysno) with x8.

  .text

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  mov x8, x0
  svc #0
  ret

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  mov x8, x1
  svc #0
  ret

  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mov x8, x1
  svc #0
  brk #1

  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  mov x8, x2
  svc #0
  ret

  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  mov x8, x3
  svc #0
  ret

  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  mov x8, x4
  svc #0
  ret

  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  mov x8, x5
  svc #0
  ret

  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  mov x8, x6
  svc #0
  ret
