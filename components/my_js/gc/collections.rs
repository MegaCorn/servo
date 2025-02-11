use std::ops::{Deref, DerefMut};
use js::gc::Traceable;
use js::jsapi::{Heap, HandleValueArray};
use js::jsval::JSVal;
use js::rust::Handle;
use js::gc::GCMethods;

pub struct RootableVec<T: Traceable> {
    v: Vec<T>,
}

impl<T: Traceable> RootableVec<T> {
    /// Create a vector of items of type T that can be rooted later.
    pub fn new_unrooted() -> RootableVec<T> {
        RootableVec { v: Vec::new() }
    }
}

pub struct RootedVec<'a, T: Traceable + 'static> {
    root: &'a mut RootableVec<T>,
}

impl From<&RootedVec<'_, JSVal>> for HandleValueArray {
    fn from(vec: &RootedVec<'_, JSVal>) -> HandleValueArray {
        HandleValueArray {
            length_: vec.root.v.len(),
            elements_: vec.root.v.as_ptr(),
        }
    }
}

impl<'a, T: Traceable + 'static> RootedVec<'a, T> {
    pub fn new(root: &'a mut RootableVec<T>) -> RootedVec<'a, T> {
        RootedVec { root }
    }

    pub fn from_iter<I>(root: &'a mut RootableVec<T>, iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        RootedVec { root }
    }
}

impl<'a, T: Traceable> Deref for RootedVec<'a, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.root.v
    }
}

impl<'a, T: Traceable> DerefMut for RootedVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.root.v
    }
}

pub struct RootedTraceableBox<T: Traceable + 'static> {
    ptr: *mut T,
}

impl<T: Traceable + 'static> RootedTraceableBox<T> {
    /// Root a JSTraceable thing for the life of this RootedTraceableBox
    pub fn new(traceable: T) -> RootedTraceableBox<T> {
        Self::from_box(Box::new(traceable))
    }

    /// Consumes a boxed JSTraceable and roots it for the life of this RootedTraceableBox.
    pub fn from_box(boxed_traceable: Box<T>) -> RootedTraceableBox<T> {
        let traceable = Box::into_raw(boxed_traceable);
        RootedTraceableBox { ptr: traceable }
    }

    /// Returns underlying pointer
    pub unsafe fn ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T: Traceable> Deref for RootedTraceableBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: Traceable> DerefMut for RootedTraceableBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> RootedTraceableBox<Heap<T>>
where
    Heap<T>: Traceable + 'static,
    T: GCMethods + Copy,
{
    pub fn handle(&self) -> Handle<T> {
        unsafe { Handle::from_raw((*self.ptr).handle()) }
    }
}
