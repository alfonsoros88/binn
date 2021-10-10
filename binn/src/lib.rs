pub struct BinnObject(*mut binn_sys::binn);

impl BinnObject {
    pub fn new() -> Self {
        Self(unsafe { binn_sys::binn_object() })
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
    use super::*;

    #[test]
    fn create_object_test() {
        let _binn = BinnObject::new();
    }
}
