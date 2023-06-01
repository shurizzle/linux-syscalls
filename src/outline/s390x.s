# Syscall registers
# +-------+------+------+------+------+------+------+--------+
# | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
# +-------+------+------+------+------+------+------+--------+
# |  r1   |  r2  |  r3  |  r4  |  r5  |  r6  |  r7  |   r2   |
# +-------+------+------+------+------+------+------+--------+
#
# sysv calling convention
# +----------+--------------------+-------------------+
# |  return  |       params       | additional params |
# +----------+--------------------+-------------------+
# |  r2, r3  | r2, r3, r4, r5, r6 |    stack RTL      |
# +----------+--------------------+-------------------+
#
# r2-r6 are the same of sysv calling convention so we can reuse them.

  .text

  .globl linux_syscalls_rs_syscall0
  .type  linux_syscalls_rs_syscall0,%function
linux_syscalls_rs_syscall0:
  lgr   %r1, %r2
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall1
  .type  linux_syscalls_rs_syscall1,%function
linux_syscalls_rs_syscall1:
  lgr   %r1, %r3
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall1_noreturn
  .type  linux_syscalls_rs_syscall1_noreturn,%function
linux_syscalls_rs_syscall1_noreturn:
  lgr   %r1, %r3
  svc   0
  trap2

  .globl linux_syscalls_rs_syscall2
  .type  linux_syscalls_rs_syscall2,%function
linux_syscalls_rs_syscall2:
  lgr   %r1, %r4
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall3
  .type  linux_syscalls_rs_syscall3,%function
linux_syscalls_rs_syscall3:
  lgr   %r1, %r5
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall4
  .type  linux_syscalls_rs_syscall4,%function
linux_syscalls_rs_syscall4:
  lgr   %r1, %r6
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall5
  .type  linux_syscalls_rs_syscall5,%function
linux_syscalls_rs_syscall5:
  lg    %r1, 160(%r6)
  svc   0
  br    %r14

  .globl linux_syscalls_rs_syscall6
  .type  linux_syscalls_rs_syscall6,%function
linux_syscalls_rs_syscall6:
  stmg  %r7, %r15, 56(%r15)
  lg    %r1, 168(%r15)
  lg    %r7, 160(%r15)
  svc   0
  lmg   %r7, %r15, 56(%r15)
  br    %r14
