/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Base classes to work with IDL callbacks.

use std::default::Default;
use std::ffi::CString;
use std::mem::drop;
use std::ptr;
use std::rc::Rc;

use js::jsapi::{
    AddRawValueRoot, EnterRealm, Heap, IsCallable, JSObject, LeaveRealm, Realm, RemoveRawValueRoot,
};
use js::jsval::{JSVal, ObjectValue, UndefinedValue};
use js::rust::wrappers::{JS_GetProperty, JS_WrapObject};
use js::rust::{MutableHandleObject, Runtime};

use crate::dom::bindings::codegen::Bindings::WindowBinding::Window_Binding::WindowMethods;
use crate::dom::bindings::error::{report_pending_exception, Error, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::bindings::settings_stack::{AutoEntryScript, AutoIncumbentScript};
use crate::dom::bindings::utils::AsCCharPtrPtr;
use crate::dom::globalscope::GlobalScope;
use crate::dom::window::Window;
use crate::realms::{enter_realm, InRealm};
use crate::script_runtime::{CanGc, JSContext};

/// The exception handling used for a call.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ExceptionHandling {
    /// Report any exception and don't throw it to the caller code.
    Report,
    /// Throw any exception to the caller code.
    Rethrow,
}

/// A common base class for representing IDL callback function and
/// callback interface types.
#[derive(JSTraceable)]
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
pub(crate) struct CallbackObject {
    /// The underlying `JSObject`.
    callback: Heap<*mut JSObject>,
    permanent_js_root: Heap<JSVal>,

    /// The ["callback context"], that is, the global to use as incumbent
    /// global when calling the callback.
    ///
    /// Looking at the WebIDL standard, it appears as though there would always
    /// be a value here, but [sometimes] callback functions are created by
    /// hand-waving without defining the value of the callback context, and
    /// without any JavaScript code on the stack to grab an incumbent global
    /// from.
    ///
    /// ["callback context"]: https://heycam.github.io/webidl/#dfn-callback-context
    /// [sometimes]: https://github.com/whatwg/html/issues/2248
    incumbent: Option<Dom<GlobalScope>>,

    #[no_trace]
    pub v8_func: Option<v8::Global<v8::Function>>,
}

impl CallbackObject {
    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    // These are used by the bindings and do not need `default()` functions.
    #[allow(clippy::new_without_default)]
    fn new() -> CallbackObject {
        CallbackObject {
            callback: Heap::default(),
            permanent_js_root: Heap::default(),
            incumbent: GlobalScope::incumbent().map(|i| Dom::from_ref(&*i)),
            v8_func: None,
        }
    }

    pub fn new_v8(callback: v8::Global<v8::Function>) -> CallbackObject {
        CallbackObject {
            callback: Heap::default(),
            permanent_js_root: Heap::default(),
            incumbent: GlobalScope::incumbent().map(|i| Dom::from_ref(&*i)),
            v8_func: Some(callback),
        }
    }

    pub(crate) fn get(&self) -> *mut JSObject {
        self.callback.get()
    }

    #[allow(unsafe_code)]
    unsafe fn init(&mut self, cx: JSContext, callback: *mut JSObject) {
        self.callback.set(callback);
        self.permanent_js_root.set(ObjectValue(callback));
        assert!(AddRawValueRoot(
            *cx,
            self.permanent_js_root.get_unsafe(),
            b"CallbackObject::root\n".as_c_char_ptr()
        ));
    }
}

impl Drop for CallbackObject {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        println!("------------------------------------------ jinguoen CallbackObject Drop");
        unsafe {
            if let Some(cx) = Runtime::get() {
                RemoveRawValueRoot(cx.as_ptr(), self.permanent_js_root.get_unsafe());
            }
        }
    }
}

impl PartialEq for CallbackObject {
    fn eq(&self, other: &CallbackObject) -> bool {
        self.callback.get() == other.callback.get()
    }
}

/// A trait to be implemented by concrete IDL callback function and
/// callback interface types.
pub(crate) trait CallbackContainer {
    /// Create a new CallbackContainer object for the given `JSObject`.
    unsafe fn new(cx: JSContext, callback: *mut JSObject) -> Rc<Self>;
    /// Returns the underlying `CallbackObject`.
    fn callback_holder(&self) -> &CallbackObject;
    /// Returns the underlying `JSObject`.
    fn callback(&self) -> *mut JSObject {
        self.callback_holder().get()
    }
    /// Returns the ["callback context"], that is, the global to use as
    /// incumbent global when calling the callback.
    ///
    /// ["callback context"]: https://heycam.github.io/webidl/#dfn-callback-context
    fn incumbent(&self) -> Option<&GlobalScope> {
        self.callback_holder().incumbent.as_deref()
    }
}

/// A common base class for representing IDL callback function types.
#[derive(JSTraceable, PartialEq)]
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
pub(crate) struct CallbackFunction {
    pub object: CallbackObject,
}

impl CallbackFunction {
    /// Create a new `CallbackFunction` for this object.
    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    // These are used by the bindings and do not need `default()` functions.
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> CallbackFunction {
        CallbackFunction {
            object: CallbackObject::new(),
        }
    }

    /// Returns the underlying `CallbackObject`.
    pub(crate) fn callback_holder(&self) -> &CallbackObject {
        &self.object
    }

    /// Initialize the callback function with a value.
    /// Should be called once this object is done moving.
    pub(crate) unsafe fn init(&mut self, cx: JSContext, callback: *mut JSObject) {
        self.object.init(cx, callback);
    }

    pub unsafe fn init_v8(&mut self, callback: v8::Global<v8::Function>) {
        log::error!("jinguoen CallbackFunction::init_v8");
        self.object.v8_func = Some(callback);
    }
}

/// A common base class for representing IDL callback interface types.
#[derive(JSTraceable, PartialEq)]
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
pub(crate) struct CallbackInterface {
    pub object: CallbackObject,
}

impl CallbackInterface {
    /// Create a new CallbackInterface object for the given `JSObject`.
    // These are used by the bindings and do not need `default()` functions.
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> CallbackInterface {
        CallbackInterface {
            object: CallbackObject::new(),
        }
    }

    #[allow(clippy::new_without_default)]
    pub fn new_v8(callback: v8::Global<v8::Function>) -> CallbackInterface {
        CallbackInterface {
            object: CallbackObject::new_v8(callback),
        }
    }

    /// Returns the underlying `CallbackObject`.
    pub(crate) fn callback_holder(&self) -> &CallbackObject {
        &self.object
    }

    /// Initialize the callback function with a value.
    /// Should be called once this object is done moving.
    pub(crate) unsafe fn init(&mut self, cx: JSContext, callback: *mut JSObject) {
        self.object.init(cx, callback);
    }

    /// Returns the property with the given `name`, if it is a callable object,
    /// or an error otherwise.
    pub(crate) fn get_callable_property(&self, cx: JSContext, name: &str) -> Fallible<JSVal> {
        rooted!(in(*cx) let mut callable = UndefinedValue());
        rooted!(in(*cx) let obj = self.callback_holder().get());
        unsafe {
            let c_name = CString::new(name).unwrap();
            if !JS_GetProperty(*cx, obj.handle(), c_name.as_ptr(), callable.handle_mut()) {
                return Err(Error::JSFailed);
            }

            if !callable.is_object() || !IsCallable(callable.to_object()) {
                return Err(Error::Type(format!(
                    "The value of the {} property is not callable",
                    name
                )));
            }
        }
        Ok(callable.get())
    }
}

pub(crate) use script_bindings::callback::ThisReflector;

/// Wraps the reflector for `p` into the realm of `cx`.
pub(crate) fn wrap_call_this_object<T: ThisReflector>(
    cx: JSContext,
    p: &T,
    mut rval: MutableHandleObject,
) {
    rval.set(p.jsobject());
    assert!(!rval.get().is_null());

    unsafe {
        if !JS_WrapObject(*cx, rval) {
            rval.set(ptr::null_mut());
        }
    }
}

/// A class that performs whatever setup we need to safely make a call while
/// this class is on the stack. After `new` returns, the call is safe to make.
pub(crate) struct CallSetup {
    /// The global for reporting exceptions. This is the global object of the
    /// (possibly wrapped) callback object.
    exception_global: DomRoot<GlobalScope>,
    /// The `JSContext` used for the call.
    cx: JSContext,
    /// The realm we were in before the call.
    old_realm: *mut Realm,
    /// The exception handling used for the call.
    handling: ExceptionHandling,
    /// <https://heycam.github.io/webidl/#es-invoking-callback-functions>
    /// steps 8 and 18.2.
    entry_script: Option<AutoEntryScript>,
    /// <https://heycam.github.io/webidl/#es-invoking-callback-functions>
    /// steps 9 and 18.1.
    incumbent_script: Option<AutoIncumbentScript>,
}

impl CallSetup {
    /// Performs the setup needed to make a call.
    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn new<T: CallbackContainer>(
        callback: &T,
        handling: ExceptionHandling,
    ) -> CallSetup {
        let global = unsafe { GlobalScope::from_object(callback.callback()) };
        if let Some(window) = global.downcast::<Window>() {
            window.Document().ensure_safe_to_run_script_or_layout();
        }
        let cx = GlobalScope::get_cx();

        let aes = AutoEntryScript::new(&global);
        let ais = callback.incumbent().map(AutoIncumbentScript::new);
        CallSetup {
            exception_global: global,
            cx,
            old_realm: unsafe { EnterRealm(*cx, callback.callback()) },
            handling,
            entry_script: Some(aes),
            incumbent_script: ais,
        }
    }

    /// Returns the `JSContext` used for the call.
    pub(crate) fn get_context(&self) -> JSContext {
        self.cx
    }
}

impl Drop for CallSetup {
    fn drop(&mut self) {
        unsafe {
            LeaveRealm(*self.cx, self.old_realm);
            if self.handling == ExceptionHandling::Report {
                let ar = enter_realm(&*self.exception_global);
                report_pending_exception(*self.cx, true, InRealm::Entered(&ar), CanGc::note());
            }
            drop(self.incumbent_script.take());
            drop(self.entry_script.take().unwrap());
        }
    }
}
