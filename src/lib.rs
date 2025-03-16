use deku::prelude::*;
use std::ptr;

#[repr(C)]
#[derive(DekuRead, DekuWrite, PartialEq)]
#[deku(endian = "big")]
pub struct Example {
    pub id: u32,
    pub value: Value,
}

#[repr(C)]
#[derive(DekuRead, DekuWrite, PartialEq)]
#[deku(ctx = "endian: deku::ctx::Endian", endian = "endian", id_type = "u8")]
pub enum Value {
    #[deku(id = 1)]
    Val1(u8),
    #[deku(id = 2)]
    Val2,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn example_write(obj: *const Example, out_size: *mut u16) -> *mut u8 {
    if obj.is_null() || out_size.is_null() {
        return ptr::null_mut();
    }

    let obj_ref = unsafe { &*obj };
    match obj_ref.to_bytes() {
        Ok(bytes) => {
            let len = bytes.len();
            unsafe { *out_size = len as u16 };
            let ptr = bytes.as_ptr();
            std::mem::forget(bytes);
            ptr as *mut u8
        }
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn example_from_bytes(data: *const u8, len: usize) -> *mut Example {
    if data.is_null() || len == 0 {
        return core::ptr::null_mut();
    }

    let bytes = unsafe { core::slice::from_raw_parts(data, len) };

    match Example::from_bytes((bytes, 0)) {
        Ok((_, obj)) => {
            let boxed = Box::new(obj);
            Box::into_raw(boxed)
        }
        Err(_) => core::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn example_free(obj: *mut Example) {
    if !obj.is_null() {
        unsafe { drop(Box::from_raw(obj)) };
    }
}
