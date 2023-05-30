use crate::{env::aux::HardwareCapabilities2, Errno, Sysno};
use core::sync::atomic::{AtomicPtr, Ordering};

extern "C" {
    fn linux_syscalls_rs_syscall0(sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall1(arg0: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall1_noreturn(arg0: usize, sysno: usize) -> !;
    fn linux_syscalls_rs_syscall2(arg0: usize, arg1: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall3(arg0: usize, arg1: usize, arg2: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_syscall4(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_syscall5(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_syscall6(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        sysno: usize,
    ) -> usize;

    // scvsyscalls
    fn linux_syscalls_rs_scvsyscall0(sysno: usize) -> usize;
    fn linux_syscalls_rs_scvsyscall1(arg0: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_scvsyscall1_noreturn(arg0: usize, sysno: usize) -> !;
    fn linux_syscalls_rs_scvsyscall2(arg0: usize, arg1: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_scvsyscall3(arg0: usize, arg1: usize, arg2: usize, sysno: usize) -> usize;
    fn linux_syscalls_rs_scvsyscall4(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_scvsyscall5(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        sysno: usize,
    ) -> usize;
    fn linux_syscalls_rs_scvsyscall6(
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
        sysno: usize,
    ) -> usize;
}

static mut SYSCALL0: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall0 as *mut ());
static mut SYSCALL1: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall1 as *mut ());
static mut SYSCALL1_NORETURN: AtomicPtr<()> =
    AtomicPtr::new(linux_syscalls_rs_syscall1_noreturn as *mut ());
static mut SYSCALL2: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall2 as *mut ());
static mut SYSCALL3: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall3 as *mut ());
static mut SYSCALL4: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall4 as *mut ());
static mut SYSCALL5: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall5 as *mut ());
static mut SYSCALL6: AtomicPtr<()> = AtomicPtr::new(linux_syscalls_rs_syscall6 as *mut ());

#[inline]
pub unsafe fn syscall0(sysno: Sysno) -> Result<usize, Errno> {
    let f = core::mem::transmute::<*mut (), extern "C" fn(usize) -> usize>(
        SYSCALL0.load(Ordering::SeqCst),
    );
    Errno::from_ret(f(sysno as usize))
}

#[inline]
pub unsafe fn syscall1(sysno: Sysno, arg0: usize) -> Result<usize, Errno> {
    let f = core::mem::transmute::<*mut (), extern "C" fn(usize, usize) -> usize>(
        SYSCALL1.load(Ordering::SeqCst),
    );
    Errno::from_ret(f(arg0, sysno as usize))
}

#[inline]
pub unsafe fn syscall1_noreturn(sysno: Sysno, arg0: usize) -> ! {
    let f = core::mem::transmute::<*mut (), extern "C" fn(usize, usize) -> !>(
        SYSCALL1_NORETURN.load(Ordering::SeqCst),
    );
    f(arg0, sysno as usize)
}

#[inline]
pub unsafe fn syscall2(sysno: Sysno, arg0: usize, arg1: usize) -> Result<usize, Errno> {
    let f = core::mem::transmute::<*mut (), extern "C" fn(usize, usize, usize) -> usize>(
        SYSCALL2.load(Ordering::SeqCst),
    );
    Errno::from_ret(f(arg0, arg1, sysno as usize))
}

#[inline]
pub unsafe fn syscall3(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, Errno> {
    let f = core::mem::transmute::<*mut (), extern "C" fn(usize, usize, usize, usize) -> usize>(
        SYSCALL3.load(Ordering::SeqCst),
    );
    Errno::from_ret(f(arg0, arg1, arg2, sysno as usize))
}

#[inline]
pub unsafe fn syscall4(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, Errno> {
    let f = core::mem::transmute::<
        *mut (),
        extern "C" fn(usize, usize, usize, usize, usize) -> usize,
    >(SYSCALL4.load(Ordering::SeqCst));
    Errno::from_ret(f(arg0, arg1, arg2, arg3, sysno as usize))
}

#[inline]
pub unsafe fn syscall5(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, Errno> {
    let f = core::mem::transmute::<
        *mut (),
        extern "C" fn(usize, usize, usize, usize, usize, usize) -> usize,
    >(SYSCALL5.load(Ordering::SeqCst));
    Errno::from_ret(f(arg0, arg1, arg2, arg3, arg4, sysno as usize))
}

#[inline]
pub unsafe fn syscall6(
    sysno: Sysno,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    let f = core::mem::transmute::<
        *mut (),
        extern "C" fn(usize, usize, usize, usize, usize, usize, usize) -> usize,
    >(SYSCALL6.load(Ordering::SeqCst));
    Errno::from_ret(f(arg0, arg1, arg2, arg3, arg4, arg5, sysno as usize))
}

pub use {
    syscall0 as syscall0_readonly, syscall1 as syscall1_readonly, syscall2 as syscall2_readonly,
    syscall3 as syscall3_readonly, syscall4 as syscall4_readonly, syscall5 as syscall5_readonly,
    syscall6 as syscall6_readonly,
};

const PPC_FEATURE2_SCV: usize = 0x00100000;

#[inline(always)]
pub(crate) unsafe fn init() {
    if crate::env::getauxval::<HardwareCapabilities2>()
        .map_or(false, |flag| flag & PPC_FEATURE2_SCV != 0)
    {
        SYSCALL0.store(linux_syscalls_rs_scvsyscall0 as *mut (), Ordering::SeqCst);
        SYSCALL1.store(linux_syscalls_rs_scvsyscall1 as *mut (), Ordering::SeqCst);
        SYSCALL1_NORETURN.store(
            linux_syscalls_rs_scvsyscall1_noreturn as *mut (),
            Ordering::SeqCst,
        );
        SYSCALL2.store(linux_syscalls_rs_scvsyscall2 as *mut (), Ordering::SeqCst);
        SYSCALL3.store(linux_syscalls_rs_scvsyscall3 as *mut (), Ordering::SeqCst);
        SYSCALL4.store(linux_syscalls_rs_scvsyscall4 as *mut (), Ordering::SeqCst);
        SYSCALL5.store(linux_syscalls_rs_scvsyscall5 as *mut (), Ordering::SeqCst);
        SYSCALL6.store(linux_syscalls_rs_scvsyscall6 as *mut (), Ordering::SeqCst);
    }
}
