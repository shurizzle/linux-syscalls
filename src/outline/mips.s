# Syscall registers
# +-------+------+------+------+------+-------+-------+--------+
# | sysno | arg0 | arg1 | arg2 | arg3 | arg4  | arg5  | return |
# +-------+------+------+------+------+-------+-------+--------+
# |  $2   |  $4  |  $5  |  $6  |  $7  | stack | stack |   $2   |
# +-------+------+------+------+------+-------+-------+--------+
# $7 is set to 0 in case of success, 1 in case of failure
# For pipe(2) a second fd is returned in $2 (just use pipe2)
#
# sysv calling convention
# +----------+----------------+-------------------+
# |  return  |     params     | additional params |
# +----------+----------------+-------------------+
# |    $2    | $4, $5, $6, $7 |    stack RTL      |
# +----------+----------------+-------------------+
#
# We can pass syscall's parameters as a normal call and move the last
# parameter to $2.


  .section .mdebug.abi32
  .previous
  .abicalls
  .set      mips4
  .set      nomips16
  .set      nomicromips
  .set      noreorder
  .set      nomacro

  .text
  .align  2

  .globl linux_syscalls_rs_syscall0
  .type  linux_syscalls_rs_syscall0,@function
linux_syscalls_rs_syscall0:
  move    $2, $4
  syscall
  negu    $8, $2     # negate res in temp reg
  jr      $31        # return
  # we call it before return because in mips we have a branch delay
  movn    $2, $8, $7 # move negated res in res if $7 is error

  .globl linux_syscalls_rs_syscall1
  .type  linux_syscalls_rs_syscall1,@function
linux_syscalls_rs_syscall1:
  move    $2, $5
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall1_noreturn
  .type  linux_syscalls_rs_syscall1_noreturn,@function
linux_syscalls_rs_syscall1_noreturn:
  move    $2, $5
  syscall
  teq     $zero, $zero # it's a trap

  .globl linux_syscalls_rs_syscall2
  .type  linux_syscalls_rs_syscall2,@function
linux_syscalls_rs_syscall2:
  move    $2, $6
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall3
  .type  linux_syscalls_rs_syscall3,@function
linux_syscalls_rs_syscall3:
  move    $2, $7
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall4
  .type  linux_syscalls_rs_syscall4,@function
linux_syscalls_rs_syscall4:
  lw      $2, 16($sp)
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall5
  .type  linux_syscalls_rs_syscall5,@function
linux_syscalls_rs_syscall5:
  lw      $2, 20($sp)
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall6
  .type  linux_syscalls_rs_syscall6,@function
linux_syscalls_rs_syscall6:
  lw      $2, 24($sp)
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

  .globl linux_syscalls_rs_syscall7
  .type  linux_syscalls_rs_syscall7,@function
linux_syscalls_rs_syscall7:
  lw      $2, 28($sp)
  syscall
  negu    $8, $2
  jr      $31
  movn    $2, $8, $7

