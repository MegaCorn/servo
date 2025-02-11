use js::jsapi::{SourceText, JSContext, Realm, JSString};
use js::jsapi::mozilla::Utf8Unit;
use std::marker::PhantomData;
use js::rust::ScriptedCaller;
use js::rust::{MutableHandleValue, HandleValue};

pub fn transform_str_to_source_text(source: &str) -> SourceText<Utf8Unit> {
    SourceText {
        units_: source.as_ptr() as *const _,
        length_: source.len() as u32,
        ownsUnits_: false,
        _phantom_0: PhantomData,
    }
}

pub unsafe fn describe_scripted_caller(cx: *mut JSContext) -> Result<ScriptedCaller, ()> {
    let mut line = 0;
    let mut col = 0;
    Ok(ScriptedCaller {
        filename: String::new(),
        line,
        col,
    })
}

pub fn transform_u16_to_source_text(source: &[u16]) -> SourceText<u16> {
    SourceText {
        units_: source.as_ptr() as *const _,
        length_: source.len() as u32,
        ownsUnits_: false,
        _phantom_0: PhantomData,
    }
}

#[inline]
pub unsafe fn get_context_realm(cx: *mut JSContext) -> *mut Realm {
    std::ptr::null_mut()
}

#[inline]
pub unsafe fn maybe_wrap_value(cx: *mut JSContext, rval: MutableHandleValue) {}

#[inline]
pub unsafe fn ToString(cx: *mut JSContext, v: HandleValue) -> *mut JSString {
    std::ptr::null_mut()
}