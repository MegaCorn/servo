use js::jsapi::{
    JSContext, JSObject, RegExpFlags, Value,
    JSString, jsid, JSClass,
};
use js::rust::{
    MutableHandleValue, MutableHandle, Handle, HandleObject,
};

pub fn JS_GetPendingException(
    cx: *mut JSContext,
    vp: &mut MutableHandleValue,
) -> bool {
    false
}

pub fn CheckRegExpSyntax(
    cx: *mut JSContext,
    chars: *const u16,
    length: usize,
    flags: RegExpFlags,
    error: &mut MutableHandle<Value>,
) -> bool {
    true
}

pub fn ExecuteRegExpNoStatics(
    cx: *mut JSContext,
    reobj: Handle<*mut JSObject>,
    chars: *const u16,
    length: usize,
    indexp: *mut usize,
    test: bool,
    rval: &mut MutableHandle<Value>,
) -> bool {
    true
}

pub fn ObjectIsRegExp(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    isRegExp: *mut bool,
) -> bool {
    true
}

pub fn JS_DefineProperty(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    value: Handle<Value>,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn JS_DefineProperty3(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    value: Handle<*mut JSObject>,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn JS_DefineProperty4(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    value: Handle<*mut JSString>,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn JS_DefineProperty5(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    value: i32,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn JS_DefinePropertyById5(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    id: Handle<jsid>,
    value: Handle<*mut JSObject>,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn JS_FireOnNewGlobalObject(cx: *mut JSContext, global: HandleObject) {}

pub fn JS_LinkConstructorAndPrototype(
    cx: *mut JSContext,
    ctor: Handle<*mut JSObject>,
    proto: Handle<*mut JSObject>,
) -> bool {
    true
}

pub fn JS_NewObjectWithGivenProto(
    cx: *mut JSContext,
    clasp: *const JSClass,
    proto: Handle<*mut JSObject>,
) -> *mut JSObject {
    std::ptr::null_mut()
}