# Syscall registers
# +-------+------+------+------+------+------+------+--------+
# | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
# +-------+------+------+------+------+------+------+--------+
# |  $2   |  $4  |  $5  |  $6  |  $7  |  $8  |  $9  |   $2   |
# +-------+------+------+------+------+------+------+--------+
# $7 is set to 0 in case of success, 1 in case of failure
# For pipe(2) a second fd is returned in $2 (just use pipe2)
#
# sysv calling convention
# +----------+----------------------------------+-------------------+
# |  return  |              params              | additional params |
# +----------+----------------------------------+-------------------+
# |    $2    | $4, $5, $6, $7, $8, $9, $10, $11 |    stack RTL      |
# +----------+----------------------------------+-------------------+
#
# Basically mips with more registers

  .section    .mdebug.abi64
  .previous
  .abicalls
  .set         mips4
  .set         nomips16
  .set         nomicromips
  .set         noreorder
  .set         nomacro

  .globl linux_syscalls_rs_syscall0
  .type  linux_syscalls_rs_syscall0,@function
linux_syscalls_rs_syscall0:
  move    $2, $4
  syscall
  dnegu   $12, $2    # negate res in temp reg
  jr      $31        # return
  # we call it before return because in mips we have a branch delay
  movn    $2, $12, $7 # move negated res in res if $7 is error

  .globl linux_syscalls_rs_syscall1
  .type  linux_syscalls_rs_syscall1,@function
linux_syscalls_rs_syscall1:
  move    $2, $5
  syscall
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

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
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

  .globl linux_syscalls_rs_syscall3
  .type  linux_syscalls_rs_syscall3,@function
linux_syscalls_rs_syscall3:
  move    $2, $7
  syscall
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

  .globl linux_syscalls_rs_syscall4
  .type  linux_syscalls_rs_syscall4,@function
linux_syscalls_rs_syscall4:
  move    $2, $8
  syscall
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

  .globl linux_syscalls_rs_syscall5
  .type  linux_syscalls_rs_syscall5,@function
linux_syscalls_rs_syscall5:
  move    $2, $9
  syscall
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

  .globl linux_syscalls_rs_syscall6
  .type  linux_syscalls_rs_syscall6,@function
linux_syscalls_rs_syscall6:
  move    $2, $10
  syscall
  dnegu   $12, $2
  jr      $31
  movn    $2, $12, $7

