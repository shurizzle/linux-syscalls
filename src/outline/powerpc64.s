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
#
# TODO:
# Support for scv 0
# https://docs.kernel.org/powerpc/syscall64-abi.html
# https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.html
# https://uclibc.org/docs/tls-ppc64.txt
# https://www.akkadia.org/drepper/tls.pdf

  .section .text

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
