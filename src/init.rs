use core::sync::atomic::{AtomicU32, Ordering};

use crate::{syscall, Errno, Sysno};

const FUTEX_WAIT_BITSET: i32 = 9;
const FUTEX_WAKE: i32 = 1;
const FUTEX_PRIVATE_FLAG: i32 = 128;

const INITIAL: u32 = 0x0;
const RUNNING: u32 = 0x1;
const COMPLETE: u32 = 0x2;
const PANICKED: u32 = 0xffffffff;

static mut LOCK: AtomicU32 = AtomicU32::new(INITIAL);

unsafe fn real_init() {
    crate::env::aux::init();
    crate::env::kernel::init();
    crate::env::vdso::init();
    crate::arch::init();
}

pub fn init() {
    unsafe {
        loop {
            match LOCK.compare_exchange_weak(INITIAL, RUNNING, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => {
                    let _guard = Guard;
                    real_init();
                    LOCK.store(COMPLETE, Ordering::Release);
                    syscall!(
                        Sysno::futex,
                        &LOCK as *const AtomicU32,
                        FUTEX_WAKE | FUTEX_PRIVATE_FLAG,
                        0x7fffffff,
                    )
                    .unwrap();
                    break;
                }
                Err(x) if x == RUNNING => loop {
                    let res = syscall!(
                        Sysno::futex,
                        &LOCK as *const AtomicU32,
                        FUTEX_WAIT_BITSET | FUTEX_PRIVATE_FLAG,
                        x,
                        core::ptr::null::<u8>(),
                        core::ptr::null::<u32>(),
                        !0u32,
                    );
                    if res != Err(Errno::EINTR) {
                        break;
                    }
                },
                Err(x) if x == PANICKED => panic!("syscall locker has been poisoned"),
                Err(_) => break,
            }
        }
    }
}

struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        unsafe {
            if LOCK.load(Ordering::Relaxed) == RUNNING {
                LOCK.store(PANICKED, Ordering::Relaxed);
            }
        }
    }
}
