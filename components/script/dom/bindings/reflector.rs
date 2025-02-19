/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! The `Reflector` struct.

use js::rust::HandleObject;

use crate::dom::bindings::codegen::PrototypeList;
use crate::dom::bindings::codegen::PrototypeList::MAX_PROTO_CHAIN_LENGTH;
use crate::dom::bindings::conversions::DerivedFrom;
use crate::dom::bindings::iterable::{Iterable, IterableIterator};
use crate::dom::bindings::root::{Dom, DomRoot, Root, MaybeUnreflectedDom};
use crate::dom::bindings::trace::JSTraceable;
use crate::dom::globalscope::GlobalScope;
use crate::realms::AlreadyInRealm;
use crate::script_runtime::{CanGc, JSContext};

/// Create the reflector for a new DOM object and yield ownership to the
/// reflector.
pub(crate) fn reflect_dom_object<T, U>(obj: Box<T>, global: &U, can_gc: CanGc) -> DomRoot<T>
where
    T: DomObject + DomObjectWrap,
    U: DerivedFrom<GlobalScope>,
{
    // let global_scope = global.upcast();
    // unsafe { T::WRAP(GlobalScope::get_cx(), global_scope, None, obj, can_gc) }
    unsafe {
        let raw = Root::new(MaybeUnreflectedDom::from_box(obj));
        let ptr = raw.as_ptr();
        drop(raw);
        let root = DomRoot::from_ref(&*ptr);
        let type_id = T::get_type_id_from_wrap();
        root.set_type_id(type_id);
        let chain = T::get_interface_chain_from_wrap();
        root.set_interface_chain(chain);
        DomRoot::from_ref(&*root)
    }
}

pub(crate) fn reflect_dom_object_with_proto<T, U>(
    obj: Box<T>,
    global: &U,
    proto: Option<HandleObject>,
    can_gc: CanGc,
) -> DomRoot<T>
where
    T: DomObject + DomObjectWrap,
    U: DerivedFrom<GlobalScope>,
{
    // let global_scope = global.upcast();
    // unsafe { T::WRAP(GlobalScope::get_cx(), global_scope, proto, obj, can_gc) }
    unsafe {
        let raw = Root::new(MaybeUnreflectedDom::from_box(obj));
        let ptr = raw.as_ptr();
        drop(raw);
        let root = DomRoot::from_ref(&*ptr);
        let type_id = T::get_type_id_from_wrap();
        root.set_type_id(type_id);
        let chain = T::get_interface_chain_from_wrap();
        root.set_interface_chain(chain);
        DomRoot::from_ref(&*root)
    }
}

pub trait V8Template: {
    fn new_template<'s>(scope: &mut v8::HandleScope<'s>) -> v8::Local<'s, v8::ObjectTemplate>;
}

pub fn new_template<'s, T>(
    scope: &mut v8::HandleScope<'s>
) -> v8::Local<'s, v8::ObjectTemplate>
where
    T: V8Template
{
    T::new_template(scope)
}

pub trait DomGlobal: DomObject {
    /// Returns the [`GlobalScope`] of the realm that the [`DomObject`] was created in.  If this
    /// object is a `Node`, this will be different from it's owning `Document` if adopted by. For
    /// `Node`s it's almost always better to use `NodeTraits::owning_global`.
    fn global(&self) -> DomRoot<GlobalScope>
    where
        Self: Sized,
    {
        let ptr = js::rust::Runtime::my_get_window() as *const crate::dom::window::Window;
        let global_scope = unsafe { (*ptr).as_global_scope() };
        DomRoot::from_ref(global_scope)
        // let realm = AlreadyInRealm::assert_for_cx(GlobalScope::get_cx());
        // GlobalScope::from_reflector(self, &realm)
    }
}

impl<T: DomObject> DomGlobal for T {}

pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};

/// A trait to provide a function pointer to wrap function for DOM objects.
pub(crate) trait DomObjectWrap: Sized + DomObject {
    /// Function pointer to the general wrap function type
    #[allow(clippy::type_complexity)]
    const WRAP: unsafe fn(
        JSContext,
        &GlobalScope,
        Option<HandleObject>,
        Box<Self>,
        CanGc,
    ) -> Root<Dom<Self>>;

    fn get_type_id_from_wrap() -> crate::dom::bindings::inheritance::TopTypeId;

    fn get_interface_chain_from_wrap() -> [PrototypeList::ID; MAX_PROTO_CHAIN_LENGTH];
}

/// A trait to provide a function pointer to wrap function for
/// DOM iterator interfaces.
pub(crate) trait DomObjectIteratorWrap: DomObjectWrap + JSTraceable + Iterable {
    /// Function pointer to the wrap function for `IterableIterator<T>`
    #[allow(clippy::type_complexity)]
    const ITER_WRAP: unsafe fn(
        JSContext,
        &GlobalScope,
        Option<HandleObject>,
        Box<IterableIterator<Self>>,
        CanGc,
    ) -> Root<Dom<IterableIterator<Self>>>;
}
