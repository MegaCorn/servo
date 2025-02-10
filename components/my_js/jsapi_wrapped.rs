use js::jsapi::JSContext;
use js::rust::MutableHandleValue;

pub fn JS_GetPendingException(
    cx: *mut JSContext,
    vp: &mut MutableHandleValue,
) -> bool {
    false
}