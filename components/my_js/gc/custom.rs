use std::ops::{Deref, DerefMut};
use js::jsapi::{JSTracer, JSObject, JSContext, Value};

pub unsafe trait CustomTrace {
    fn trace(&self, trc: *mut JSTracer);
}

unsafe impl CustomTrace for *mut JSObject {
    fn trace(&self, trc: *mut JSTracer) {}
}

unsafe impl CustomTrace for Value {
    fn trace(&self, trc: *mut JSTracer) {}
}

unsafe impl<T: CustomTrace> CustomTrace for Option<T> {
    fn trace(&self, trc: *mut JSTracer) {}
}

unsafe impl<T: CustomTrace> CustomTrace for Vec<T> {
    fn trace(&self, trc: *mut JSTracer) {}
}

#[repr(C)]
pub struct CustomAutoRooter<T> {
    data: T,
}

impl<T: CustomTrace> CustomAutoRooter<T> {
    pub fn new(data: T) -> Self {
        CustomAutoRooter {
            data,
        }
    }
}

pub struct CustomAutoRooterGuard<'a, T: 'a + CustomTrace> {
    rooter: &'a mut CustomAutoRooter<T>,
}

impl<'a, T: 'a + CustomTrace> CustomAutoRooterGuard<'a, T> {
    pub fn new(cx: *mut JSContext, rooter: &'a mut CustomAutoRooter<T>) -> Self {
        CustomAutoRooterGuard { rooter }
    }
}

impl<'a, T: 'a + CustomTrace> Deref for CustomAutoRooterGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.rooter.data
    }
}

impl<'a, T: 'a + CustomTrace> DerefMut for CustomAutoRooterGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.rooter.data
    }
}