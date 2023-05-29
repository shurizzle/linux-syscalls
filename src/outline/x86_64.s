// Syscall registers
// +-------+------+------+------+------+------+------+--------+
// | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
// +-------+------+------+------+------+------+------+--------+
// |  rax  | rdi  | rsi  | rdx  | r10  |  r8  |  r9  |  rax   |
// +-------+------+------+------+------+------+------+--------+
//
// sysv calling convention
// +----------+---------------------------+-------------------+
// |  return  |           params          | additional params |
// +----------+---------------------------+-------------------+
// | rax, rdx | rdi, rsi, rdx rcx, r8, r9 |      stack RTL    |
// +----------+---------------------------+-------------------+
//
//
// Registers are the same as the call convention params except for rax and r10
// So we can pass sysno as last param and move it on rax, same for r10.

  .intel_syntax noprefix

  .text

  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  mov     rax, rdi
  syscall
  ret

  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  mov     rax, rsi
  syscall
  ret

// useful for exit syscall exit(retcode);
  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mov     rax, rsi
  syscall
  ud2

  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  mov     rax, rdx
  syscall
  ret

  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  mov     rax, rcx
  syscall
  ret

  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  mov     rax, r8
  mov     r10, rcx
  syscall
  ret

  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  mov     rax, r9
  mov     r10, rcx
  syscall
  ret

.globl asmtest
asmtest:
  mov     rax, [rsp+0x8]
  ret

  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  mov     rax, [rsp+0x8]
  mov     r10, rcx
  syscall
  ret
