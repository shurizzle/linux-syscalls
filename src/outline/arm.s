// Syscall registers
// +-------+------+------+------+------+------+------+--------+
// | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
// +-------+------+------+------+------+------+------+--------+
// |  r7   |  r0  |  r1  |  r2  |  r3  |  r4  |  r5  |   r0   |
// +-------+------+------+------+------+------+------+--------+
//
// sysv calling convention
// +----------+----------------+-------------------+
// |  return  |     params     | additional params |
// +----------+----------------+-------------------+
// |    r0    | r0, r1, r2, r3 |    stack RTL      |
// +----------+----------------+-------------------+
//
// r0-r3 are the same of sysv calling convention so we can reuse them.


  .arch armv4

  .text

  .globl linux_syscalls_rs_syscall0
  .type  linux_syscalls_rs_syscall0,%function
linux_syscalls_rs_syscall0:
  .fnstart
  .cantunwind
  push {r7, lr}
  mov  r7, r0
  svc  #0
  pop  {r7, pc}
  .fnend

  .globl linux_syscalls_rs_syscall1
  .type  linux_syscalls_rs_syscall1,%function
linux_syscalls_rs_syscall1:
  .fnstart
  .cantunwind
  push {r7, lr}
  mov  r7, r1
  svc  #0
  pop  {r7, pc}
  .fnend

  .globl linux_syscalls_rs_syscall1_noreturn
  .type  linux_syscalls_rs_syscall1_noreturn,%function
linux_syscalls_rs_syscall1_noreturn:
  .fnstart
  .cantunwind
  mov  r7, r1
  svc  #0
  udf  #16
  .fnend

  .globl linux_syscalls_rs_syscall2
  .type  linux_syscalls_rs_syscall2,%function
linux_syscalls_rs_syscall2:
  .fnstart
  .cantunwind
  push {r7, lr}
  mov  r7, r2
  svc  #0
  pop  {r7, pc}
  .fnend

  .globl linux_syscalls_rs_syscall3
  .type  linux_syscalls_rs_syscall3,%function
linux_syscalls_rs_syscall3:
  .fnstart
  .cantunwind
  push {r7, lr}
  mov  r7, r3
  svc  #0
  pop  {r7, pc}
  .fnend

  .globl linux_syscalls_rs_syscall4
  .type  linux_syscalls_rs_syscall4,%function
linux_syscalls_rs_syscall4:
  .fnstart
  .cantunwind
  push {r7, lr}
  ldr  r7, [sp, #8]
  svc  #0
  pop  {r7, pc}
  .fnend

// r11 can be the fp, save and restore it
  .globl linux_syscalls_rs_syscall5
  .type  linux_syscalls_rs_syscall5,%function
linux_syscalls_rs_syscall5:
  .fnstart
  .cantunwind
  push {r4, r7, r11, lr}
  ldr  r4, [sp, #16]
  ldr  r7, [sp, #20]
  svc  #0
  pop  {r4, r7, r11, pc}
  .fnend

  .globl linux_syscalls_rs_syscall6
  .type  linux_syscalls_rs_syscall6,%function
linux_syscalls_rs_syscall6:
  .fnstart
  .cantunwind
  push {r4, r5, r7, lr}
  add  r7, sp, #16
  ldm  r7, {r4, r5, r7}
  svc  #0
  pop  {r4, r5, r7, pc}
  .fnend

