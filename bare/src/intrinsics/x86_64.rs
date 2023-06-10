use core::cmp::Ordering;

#[no_mangle]
unsafe extern "C" fn memcpy(mut dest: *mut u8, mut src: *const u8, n: usize) {
    let end = dest.add(n);
    while (dest as usize) < (end as usize) {
        *dest = *src;
        dest = dest.add(1);
        src = src.add(1);
    }
}

#[no_mangle]
unsafe extern "C" fn memcmp(mut dest: *const u8, mut src: *const u8, n: usize) -> i32 {
    let end = dest.add(n);
    while (dest as usize) < (end as usize) {
        match ((*dest as i16) - (*end as i16)).cmp(&0) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            _ => (),
        }

        dest = dest.add(1);
        src = src.add(1);
    }
    0
}

#[no_mangle]
unsafe extern "C" fn memset(dest: *mut u8, value: core::ffi::c_int, num: usize) -> *const u8 {
    let mut ptr = dest;
    let end = dest.add(num);

    while (ptr as usize) < (end as usize) {
        *ptr = value as u8;
        ptr = dest.add(1);
    }

    dest
}
