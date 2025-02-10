use js::jsapi::{JSContext, JSObject, RegExpFlags, Value};
use js::rust::{MutableHandleValue, MutableHandle, Handle};

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