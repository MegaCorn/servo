use js::jsapi::{
    JSObject, Value, JSContext, JSFunction, JSNative,
};

pub fn JS_NewFunction(
    cx: *mut JSContext,
    call: JSNative,
    nargs: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
    name: *const ::std::os::raw::c_char,
) -> *mut JSFunction {
    std::ptr::null_mut()
}

pub fn NewFunctionWithReserved(
    cx: *mut JSContext,
    call: JSNative,
    nargs: ::std::os::raw::c_uint,
    flags: ::std::os::raw::c_uint,
    name: *const ::std::os::raw::c_char,
) -> *mut JSFunction {
    std::ptr::null_mut()
}

pub fn SetFunctionNativeReserved(
    fun: *mut JSObject,
    which: usize,
    val: *const Value,
) {}

pub fn GetFunctionNativeReserved(
    fun: *mut JSObject,
    which: usize,
) -> *const Value {
    std::ptr::null()
}

pub fn AddRawValueRoot(
    cx: *mut JSContext,
    vp: *mut Value,
    name: *const ::std::os::raw::c_char,
) -> bool {
    true
}

pub fn JS_GetFunctionObject(fun: *mut JSFunction) -> *mut JSObject {
    std::ptr::null_mut()
}

pub fn RemoveRawValueRoot(cx: *mut JSContext, vp: *mut Value) {}