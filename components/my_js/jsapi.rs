use js::jsapi::{
    JSObject, Value, JSContext, JSFunction, JSNative, ExceptionStackBehavior,
    JSErrorReport, JSString, AutoRequireNoGC, Latin1Char,
    JSLinearString, JSAtom,
};
use js::rust::{
    HandleValue, HandleObject, MutableHandleValue, HandleId,
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

pub fn JS_ClearPendingException(cx: *mut JSContext) {}

pub fn JS_IsExceptionPending(cx: *mut JSContext) -> bool {
    false
}

pub fn JS_SetPendingException(
    cx: *mut JSContext,
    v: HandleValue,
    behavior: ExceptionStackBehavior,
) {}

pub fn JS_GetPendingException(
    cx: *mut JSContext,
    vp: MutableHandleValue,
) -> bool {
    false
}

pub fn JS_ErrorFromException(
    cx: *mut JSContext,
    obj: HandleObject,
) -> *mut JSErrorReport {
    std::ptr::null_mut()
}

pub fn DisableJitBackend() {}

pub fn JS_GetLatin1StringCharsAndLength(
    cx: *mut JSContext,
    nogc: *const AutoRequireNoGC,
    str_: *mut JSString,
    length: *mut usize,
) -> *const Latin1Char {
    std::ptr::null()
}

pub fn JS_GetTwoByteStringCharsAndLength(
    cx: *mut JSContext,
    nogc: *const AutoRequireNoGC,
    str_: *mut JSString,
    length: *mut usize,
) -> *const u16 {
    std::ptr::null()
}

pub fn JS_NewStringCopyN(
    cx: *mut JSContext,
    s: *const ::std::os::raw::c_char,
    n: usize,
) -> *mut JSString {
    std::ptr::null_mut()
}

pub fn JS_DeprecatedStringHasLatin1Chars(str_: *mut JSString) -> bool {
    true
}

pub fn AtomToLinearString(atom: *mut JSAtom) -> *mut JSLinearString {
    std::ptr::null_mut()
}

pub fn GetLinearStringCharAt(s: *mut JSLinearString, idx: usize) -> u16 {
    0
}

pub fn GetLinearStringLength(s: *mut JSLinearString) -> usize {
    0
}

pub fn GetNonCCWObjectGlobal(obj: *mut JSObject) -> *mut JSObject {
    std::ptr::null_mut()
}

pub fn JS_EnumerateStandardClasses(
    cx: *mut JSContext,
    obj: js::jsapi::HandleObject,
) -> bool {
    true
}

pub fn JS_FreezeObject(
    cx: *mut JSContext,
    obj: js::jsapi::Handle<*mut JSObject>,
) -> bool {
    true
}

pub fn JS_IsGlobalObject(obj: *mut JSObject) -> bool {
    true
}

pub fn JS_ResolveStandardClass(
    cx: *mut JSContext,
    obj: js::jsapi::HandleObject,
    id: js::jsapi::HandleId,
    resolved: *mut bool,
) -> bool {
    true
}

pub fn StringIsArrayIndex(str_: *mut JSLinearString, indexp: *mut u32) -> bool {
    true
}