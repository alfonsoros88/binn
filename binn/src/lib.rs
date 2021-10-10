use std::ffi::{c_void, CStr};

pub struct BinnObject(*mut binn_sys::binn);

impl BinnObject {
    pub fn new() -> Self {
        Self(unsafe { binn_sys::binn_object() })
    }

    pub fn set_i32(&mut self, key: &CStr, x: i32) {
        unsafe {
            let mut y = x;
            binn_sys::binn_object_set(
                self.0,
                key.as_ptr(),
                binn_sys::BINN_INT32 as i32,
                &mut y as *mut i32 as *mut c_void,
                0,
            )
        };
    }

    pub fn get_i32(&self, key: &CStr) -> Option<i32> {
        let mut x = 0;
        unsafe {
            (binn_sys::binn_object_get(
                self.0 as *mut c_void,
                key.as_ptr(),
                binn_sys::BINN_INT32 as i32,
                &mut x as *mut i32 as *mut c_void,
                std::ptr::null_mut(),
            ) != 0)
                .then(|| x)
        }
    }
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

    #[test]
    fn binn_object_set_i32_test() {
        let mut binn = BinnObject::new();
        let key = CString::new("foo").unwrap();
        binn.set_i32(&key, 42);
        assert_eq!(binn.get_i32(&key), Some(42));
    }
}
