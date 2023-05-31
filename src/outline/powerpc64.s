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
# NOTE: Useful links:
# https://docs.kernel.org/powerpc/syscall64-abi.html
# https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.html
# https://uclibc.org/docs/tls-ppc64.txt
# https://www.akkadia.org/drepper/tls.pdf

# HACK: "old" assemblers (and clang) don't recognize scv mnemonic
.macro SCV0
  .byte 0x44
  .byte 0x00
  .byte 0x00
  .byte 0x01
.endm

.macro HAS_SCV reg else
  ld   \reg, .LC_has_scv@toc(2)
  lbz  \reg, 0(\reg)
  cmpi 0, 0, \reg, 0
  beq  \else
.endm

.macro SYSCALL reg
  mr      0, \reg
  HAS_SCV \reg, 0f
  SCV0
  blr
  nop

0:
  sc
  bnslr
  neg     3, 3
  blr
.endm

  .section .text

  .globl linux_syscalls_rs_get_scv
linux_syscalls_rs_get_scv:
  ld    3, .LC_has_scv@toc(2)
  lbz   3, 0(3)
  blr

  .globl linux_syscalls_rs_set_scv
linux_syscalls_rs_set_scv:
  ld    4, .LC_has_scv@toc(2)
  stb   3, 0(4)
  blr

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  SYSCALL 3

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  SYSCALL 4

  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mr    0, 4
  sc         # It's just for the exit syscall, who cares
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

  .section .bss
has_scv:
  .space 1
  .size  has_scv, 1

  .section ".toc","aw",@progbits
.LC_has_scv:
  .tc has_scv[TC],has_scv
