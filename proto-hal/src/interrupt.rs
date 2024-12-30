#[allow(dead_code)]
pub struct Vector(*const ());

impl Vector {
    pub const fn handler(f: unsafe extern "C" fn()) -> Self {
        Self(f as _)
    }

    pub const fn reserved() -> Self {
        Self(core::ptr::null())
    }
}

unsafe impl Sync for Vector {}
