// Syscall registers
// +-------+------+------+------+------+------+------+--------+
// | sysno | arg0 | arg1 | arg2 | arg3 | arg4 | arg5 | return |
// +-------+------+------+------+------+------+------+--------+
// |  eax  | ebx  | ecx  | edx  | esi  | edi  | ebp  |  eax   |
// +-------+------+------+------+------+------+------+--------+
//
// cdecl calling convention (bad)
// +--------+--------+-------------------+
// | return | params | additional params |
// +--------+--------+-------------------+
// |  eax   |        |     stack RTL     |
// +--------+--------+-------------------+
//
// fastcall calling convention
// +--------+----------+-------------------+
// | return |  params  | additional params |
// +--------+----------+-------------------+
// |  eax   | ecx, edx |     stack RTL     |
// +--------+----------+-------------------+
//
// We can use fastcall to populate ecx and edx registers, other parameters will
// be passed on the stack

  .intel_syntax noprefix

  .text

// syscall(sysno)
  .globl linux_syscalls_rs_syscall0
linux_syscalls_rs_syscall0:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  mov  eax, ecx
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  ret
0:
  mov  eax, ecx
  int  0x80
  ret

// syscall(arg0, sysno)
  .globl linux_syscalls_rs_syscall1
linux_syscalls_rs_syscall1:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebx
  mov  eax, edx
  mov  ebx, ecx
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  ebx
  ret
0:
  push ebx
  mov  eax, edx
  mov  ebx, ecx
  int  0x80
  pop  ebx
  ret


// syscall(arg0, sysno)
  .globl linux_syscalls_rs_syscall1_noreturn
linux_syscalls_rs_syscall1_noreturn:
  mov  eax, edx
  mov  ebx, ecx
  int  0x80
  ud2

// syscall(arg1, arg0, sysno)
  .globl linux_syscalls_rs_syscall2
linux_syscalls_rs_syscall2:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebx
  mov  ebx, edx
  mov  eax, [esp+0x8]
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  ebx
  ret  0x4
0:
  push ebx
  mov  ebx, edx
  mov  eax, [esp+0x8]
  int  0x80
  pop  ebx
  ret  0x4

// syscall(arg1, arg2, arg0, sysno)
  .globl linux_syscalls_rs_syscall3
linux_syscalls_rs_syscall3:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebx
  mov  ebx, [esp+0x8]
  mov  eax, [esp+0xc]
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  ebx
  ret  0x8
0:
  push ebx
  mov  ebx, [esp+0x8]
  mov  eax, [esp+0xc]
  int  0x80
  pop  ebx
  ret  0x8

// syscall(arg1, arg2, arg0, arg3, sysno)
  .globl linux_syscalls_rs_syscall4
linux_syscalls_rs_syscall4:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebx
  push esi
  mov  ebx, [esp+0xc]
  mov  esi, [esp+0x10]
  mov  eax, [esp+0x14]
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  esi
  pop  ebx
  ret  0xc
0:
  push ebx
  push esi
  mov  ebx, [esp+0xc]
  mov  esi, [esp+0x10]
  mov  eax, [esp+0x14]
  int  0x80
  pop  esi
  pop  ebx
  ret  0xc

// syscall(arg1, arg2, arg0, arg3, arg4, sysno)
  .globl linux_syscalls_rs_syscall5
linux_syscalls_rs_syscall5:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebx
  push edi
  push esi
  mov  ebx, [esp+0x10]
  mov  esi, [esp+0x14]
  mov  edi, [esp+0x18]
  mov  eax, [esp+0x1c]
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  esi
  pop  edi
  pop  ebx
  ret  0x10
0:
  push ebx
  push edi
  push esi
  mov  ebx, [esp+0x10]
  mov  esi, [esp+0x14]
  mov  edi, [esp+0x18]
  mov  eax, [esp+0x1c]
  int  0x80
  pop  esi
  pop  edi
  pop  ebx
  ret  0x10

// syscall(arg1, arg2, arg0, arg3, arg4, sysno)
  .globl linux_syscalls_rs_syscall6
linux_syscalls_rs_syscall6:
  mov  eax, [LINUX_SYSCALLS_RS_VSYSCALL]
  test eax, eax
  jz   0f
  push ebp
  push ebx
  push edi
  push esi
  mov  ebx, [esp+0x14]
  mov  esi, [esp+0x18]
  mov  edi, [esp+0x1c]
  mov  ebp, [esp+0x20]
  mov  eax, [esp+0x24]
  call [LINUX_SYSCALLS_RS_VSYSCALL]
  pop  esi
  pop  edi
  pop  ebx
  pop  ebp
  ret  0x14
0:
  push ebp
  push ebx
  push edi
  push esi
  mov  ebx, [esp+0x14]
  mov  esi, [esp+0x18]
  mov  edi, [esp+0x1c]
  mov  ebp, [esp+0x20]
  mov  eax, [esp+0x24]
  int  0x80
  pop  esi
  pop  edi
  pop  ebx
  pop  ebp
  ret  0x14

  .section .data

  .globl LINUX_SYSCALLS_RS_VSYSCALL
LINUX_SYSCALLS_RS_VSYSCALL: .zero 4
