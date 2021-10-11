use std::{
    ffi::{c_void, CStr},
    mem::MaybeUninit,
};

pub struct BinnObject(*mut binn_sys::binn);

macro_rules! setget {
    ($set_name:ident, $get_name:ident, $t:ty, $binn_enum:expr) => {
        pub fn $set_name(&mut self, key: &CStr, mut x: $t) {
            unsafe {
                binn_sys::binn_object_set(
                    self.0,
                    key.as_ptr(),
                    $binn_enum as i32,
                    &mut x as *mut $t as *mut c_void,
                    0,
                )
            };
        }

        pub fn $get_name(&self, key: &CStr) -> Option<$t> {
            let mut x: MaybeUninit<$t> = MaybeUninit::uninit();
            unsafe {
                (binn_sys::binn_object_get(
                    self.0 as *mut c_void,
                    key.as_ptr(),
                    $binn_enum as i32,
                    x.as_mut_ptr() as *mut c_void,
                    std::ptr::null_mut(),
                ) != 0)
                    .then(|| x.assume_init())
            }
        }
    };
}

impl BinnObject {
    pub fn new() -> Self {
        Self(unsafe { binn_sys::binn_object() })
    }

    setget!(set_i8, get_i8, i8, binn_sys::BINN_INT8);
    setget!(set_i16, get_i16, i16, binn_sys::BINN_INT16);
    setget!(set_i32, get_i32, i32, binn_sys::BINN_INT32);
    setget!(set_i64, get_i64, i64, binn_sys::BINN_INT64);
    setget!(set_u8, get_u8, u8, binn_sys::BINN_UINT8);
    setget!(set_u16, get_u16, u16, binn_sys::BINN_UINT16);
    setget!(set_u32, get_u32, u32, binn_sys::BINN_UINT32);
    setget!(set_u64, get_u64, u64, binn_sys::BINN_UINT64);
    setget!(set_f32, get_f32, f32, binn_sys::BINN_FLOAT32);
    setget!(set_f64, get_f64, f64, binn_sys::BINN_FLOAT64);
    setget!(set_bool, get_bool, bool, binn_sys::BINN_BOOL);
}

impl Drop for BinnObject {
    fn drop(&mut self) {
        unsafe { binn_sys::binn_free(self.0) };
    }
}

impl Default for BinnObject {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn create_object_test() {
        let _binn = BinnObject::new();
    }

    macro_rules! test_setget {
        ($test_name:ident, $setter:ident, $getter:ident, $val:expr) => {
            #[test]
            fn $test_name() {
                let mut binn = BinnObject::new();
                let key = CString::new("foo").unwrap();
                binn.$setter(&key, $val);
                assert_eq!(binn.$getter(&key), Some($val));
            }
        };
    }

    test_setget!(set_i8_test, set_i8, get_i8, 42);
    test_setget!(set_i16_test, set_i16, get_i16, 42);
    test_setget!(set_i32_test, set_i32, get_i32, 42);
    test_setget!(set_i64_test, set_i64, get_i64, 42);
    test_setget!(set_u8_test, set_u8, get_u8, 42);
    test_setget!(set_u16_test, set_u16, get_u16, 42);
    test_setget!(set_u32_test, set_u32, get_u32, 42);
    test_setget!(set_u64_test, set_u64, get_u64, 42);
    test_setget!(set_f32_test, set_f32, get_f32, 42.0);
    test_setget!(set_f64_test, set_f64, get_f64, 42.0);
    test_setget!(set_bool_test, set_bool, get_bool, true);
}
