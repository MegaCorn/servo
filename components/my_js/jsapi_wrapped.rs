use js::jsapi::{
    JSContext, JSObject, RegExpFlags, Value,
    JSString, jsid, JSClass, HandleValueArray, ESClass, MutableHandleIdVector,
    HandleId, PropertyDescriptor, HandleObjectVector, ReadOnlyCompileOptions,
    SourceText, JSFunction, JSScript, JSStructuredCloneData, StructuredCloneScope,
    CloneDataPolicy, JSStructuredCloneCallbacks, Symbol,
};
use js::rust::{
    MutableHandleValue, MutableHandle, Handle, HandleObject, MutableHandleObject,
    HandleValue, MutableHandleId,
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

pub fn JS_ParseJSON(
    cx: *mut JSContext,
    chars: *const u16,
    len: u32,
    vp: MutableHandle<Value>,
) -> bool {
    true
}

pub fn GetPromiseIsHandled(promise: HandleObject) -> bool {
    true
}

pub fn JS_CallFunctionName(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    args: *const HandleValueArray,
    rval: MutableHandle<Value>,
) -> bool {
    true
}

pub fn JS_GetProperty(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    vp: MutableHandleValue,
) -> bool {
    true
}

pub fn JS_HasOwnProperty(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const ::std::os::raw::c_char,
    foundp: *mut bool,
) -> bool {
    true
}

pub fn GetBuiltinClass(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    cls: *mut ESClass,
) -> bool {
    true
}

pub fn GetPropertyKeys(
    cx: *mut JSContext,
    obj: HandleObject,
    flags: ::std::os::raw::c_uint,
    props: MutableHandleIdVector,
) -> bool {
    true
}

pub fn JS_GetOwnPropertyDescriptorById(
    cx: *mut JSContext,
    obj: HandleObject,
    id: HandleId,
    desc: MutableHandle<PropertyDescriptor>,
    isNone: *mut bool,
) -> bool {
    true
}

pub fn JS_GetPropertyById(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    id: Handle<jsid>,
    vp: MutableHandleValue,
) -> bool {
    true
}

pub fn JS_IdToValue(
    cx: *mut JSContext,
    id: jsid,
    vp: MutableHandle<Value>,
) -> bool {
    true
}

pub fn JS_ValueToSource(
    cx: *mut JSContext,
    v: Handle<Value>,
) -> *mut JSString {
    std::ptr::null_mut()
}

pub fn Construct1(
    cx: *mut JSContext,
    fun: Handle<Value>,
    args: *const HandleValueArray,
    objp: MutableHandle<*mut JSObject>,
) -> bool {
    true
}

pub fn SameValue(
    cx: *mut JSContext,
    v1: Handle<Value>,
    v2: Handle<Value>,
    same: *mut bool,
) -> bool {
    true
}

pub fn JS_GetPromiseResult(promise: HandleObject, dest: MutableHandleValue) {}

pub fn CompileFunction(
    cx: *mut JSContext,
    envChain: HandleObjectVector,
    options: *const ReadOnlyCompileOptions,
    name: *const ::std::os::raw::c_char,
    nargs: ::std::os::raw::c_uint,
    argnames: *const *const ::std::os::raw::c_char,
    srcBuf: *mut SourceText<u16>,
) -> *mut JSFunction {
    std::ptr::null_mut()
}

pub fn JS_ExecuteScript(
    cx: *mut JSContext,
    script: Handle<*mut JSScript>,
    rval: MutableHandle<Value>,
) -> bool {
    true
}

pub fn JS_GetScriptPrivate(script: *mut JSScript, dest: MutableHandleValue) {}

pub fn Call(
    cx: *mut JSContext,
    thisv: Handle<Value>,
    fun: Handle<Value>,
    args: *const HandleValueArray,
    rval: MutableHandle<Value>,
) -> bool {
    true
}

pub fn DetachArrayBuffer(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
) -> bool {
    true
}

pub fn JS_WrapObject(cx: *mut JSContext, objp: MutableHandleObject)-> bool {
    true
}

pub fn JS_ReadStructuredClone(
    cx: *mut JSContext,
    data: *const JSStructuredCloneData,
    version: u32,
    scope: StructuredCloneScope,
    vp: MutableHandleValue,
    cloneDataPolicy: *const CloneDataPolicy,
    optionalCallbacks: *const JSStructuredCloneCallbacks,
    closure: *mut ::std::os::raw::c_void,
) -> bool {
    true
}

pub fn JS_WriteStructuredClone(
    cx: *mut JSContext,
    v: HandleValue,
    data: *mut JSStructuredCloneData,
    scope: StructuredCloneScope,
    cloneDataPolicy: *const CloneDataPolicy,
    optionalCallbacks: *const JSStructuredCloneCallbacks,
    closure: *mut ::std::os::raw::c_void,
    transferable: HandleValue,
) -> bool {
    true
}

pub fn JS_DefineUCProperty2(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    name: *const u16,
    namelen: usize,
    value: Handle<Value>,
    attrs: ::std::os::raw::c_uint,
) -> bool {
    true
}

pub fn RUST_SYMBOL_TO_JSID(sym: *mut Symbol, id: MutableHandleId) {}

pub fn JS_CallFunctionValue(
    cx: *mut JSContext,
    obj: Handle<*mut JSObject>,
    fval: Handle<Value>,
    args: *const HandleValueArray,
    rval: MutableHandle<Value>,
) -> bool {
    true
}

pub fn JS_WrapValue(cx: *mut JSContext, vp: MutableHandleValue) -> bool {
    true
}
