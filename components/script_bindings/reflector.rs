/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use js::jsapi::{Heap, JSObject};
use js::rust::HandleObject;
use malloc_size_of_derive::MallocSizeOf;
use crate::codegen::InheritTypes::TopTypeId;
use crate::codegen::PrototypeList;
use crate::codegen::PrototypeList::MAX_PROTO_CHAIN_LENGTH;

/// A struct to store a reference to the reflector of a DOM object.
#[cfg_attr(crown, allow(crown::unrooted_must_root))]
#[derive(MallocSizeOf)]
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
// If you're renaming or moving this field, update the path in plugins::reflector as well
pub struct Reflector {
    #[ignore_malloc_size_of = "defined and measured in rust-mozjs"]
    object: Heap<*mut JSObject>,

    #[ignore_malloc_size_of = "defined and measured in rust-mozjs"]
    pub object1: Heap<*mut JSObject>,

    #[ignore_malloc_size_of = "v8"]
    my_type_id: *mut TopTypeId,

    #[ignore_malloc_size_of = "v8"]
    my_object: Box<i32>,

    #[ignore_malloc_size_of = "v8"]
    my_interface_chain: *mut [PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH],
}

unsafe impl js::gc::Traceable for Reflector {
    unsafe fn trace(&self, _: *mut js::jsapi::JSTracer) {}
}

#[cfg_attr(crown, allow(crown::unrooted_must_root))]
impl PartialEq for Reflector {
    fn eq(&self, other: &Reflector) -> bool {
        std::ptr::eq(&*self.my_object, &*other.my_object)
    }
}

impl Reflector {
    /// Get the reflector.
    #[inline]
    pub fn get_jsobject(&self) -> HandleObject {
        // We're rooted, so it's safe to hand out a handle to object in Heap
        unsafe { HandleObject::from_raw(self.object.handle()) }
    }

    /// Initialize the reflector. (May be called only once.)
    ///
    /// # Safety
    ///
    /// The provided [`JSObject`] pointer must point to a valid [`JSObject`].
    pub unsafe fn set_jsobject(&self, object: *mut JSObject) {
        // assert!(self.object.get().is_null());
        // assert!(!object.is_null());
        self.object1.set(object);
    }

    /// Return a pointer to the memory location at which the JS reflector
    /// object is stored. Used to root the reflector, as
    /// required by the JSAPI rooting APIs.
    pub fn rootable(&self) -> &Heap<*mut JSObject> {
        &self.object
    }

    /// Create an uninitialized `Reflector`.
    // These are used by the bindings and do not need `default()` functions.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Reflector {
        Reflector {
            object: Heap::default(),
            object1: Heap::default(),
            my_object: Box::new(0),
            my_type_id: Box::into_raw(Box::new(TopTypeId { abstract_: () })),
            my_interface_chain: Box::into_raw(Box::new([PrototypeList::ID::Last; MAX_PROTO_CHAIN_LENGTH])),
        }
    }
}

/// A trait to provide access to the `Reflector` for a DOM object.
pub trait DomObject: js::gc::Traceable + 'static {
    /// Returns the receiver's reflector.
    fn reflector(&self) -> &Reflector;
    fn set_type_id(&self, id: TopTypeId);
    fn get_type_id(&self) -> &TopTypeId;
    fn set_interface_chain(&self, new_chain: [PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH]);
    fn get_interface_chain(&self) -> &[PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH];
}

impl DomObject for Reflector {
    fn reflector(&self) -> &Self {
        self
    }
    fn set_type_id(&self, id: TopTypeId) {
        unsafe { *(self.my_type_id) = id };
    }
    fn get_type_id(&self) -> &TopTypeId {
        unsafe { &*(self.my_type_id) }
    }
    fn set_interface_chain(&self, new_chain: [PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH]) {
        unsafe { *(self.my_interface_chain) = new_chain };
    }
    fn get_interface_chain(&self) -> &[PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH] {
        unsafe { &*(self.my_interface_chain) }
    }
}

/// A trait to initialize the `Reflector` for a DOM object.
pub trait MutDomObject: DomObject {
    /// Initializes the Reflector
    ///
    /// # Safety
    ///
    /// The provided [`JSObject`] pointer must point to a valid [`JSObject`].
    unsafe fn init_reflector(&self, obj: *mut JSObject);
}

impl MutDomObject for Reflector {
    unsafe fn init_reflector(&self, obj: *mut JSObject) {
        self.set_jsobject(obj)
    }
}
