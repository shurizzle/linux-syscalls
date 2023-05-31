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

unsafe fn real_init(env: *const ()) {
    crate::env::aux::init(env);
    crate::env::kernel::init();
    crate::env::vdso::init();
    crate::arch::init();
}

unsafe fn aux_from_environ(env: *const *const u8) -> *const () {
    let mut p = env;
    while !(*p).is_null() {
        p = p.add(1);
    }
    p.add(1) as *mut ()
}

#[cfg(not(feature = "bare"))]
unsafe fn aux_ptr() -> *const () {
    extern "C" {
        #[cfg(any(target_env = "gnu", target_env = "musl"))]
        static __environ: *const *const u8;
        #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
        static environ: *const *const u8;
    }
    #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
    let env = environ;
    #[cfg(any(target_env = "gnu", target_env = "musl"))]
    let env = __environ;
    aux_from_environ(env)
}

#[cfg(not(feature = "bare"))]
pub fn init() {
    unsafe { inner_init(aux_ptr()) }
}

#[cfg(feature = "bare")]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn init_from_environ(env: *const *const u8) {
    unsafe { inner_init(aux_from_environ(env)) }
}

#[cfg(feature = "bare")]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn init_from_args(argc: isize, argv: *const *const u8) {
    unsafe { inner_init(aux_from_environ(argv.add(argc as usize))) }
}

unsafe fn inner_init(auxv: *const ()) {
    loop {
        match LOCK.compare_exchange_weak(INITIAL, RUNNING, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                let _guard = Guard;
                real_init(auxv);
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
