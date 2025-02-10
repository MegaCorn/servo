use js::jsapi::{
    JSObject, Value, JSContext, JSFunction, JSNative, ExceptionStackBehavior,
    JSErrorReport, JSString, AutoRequireNoGC, Latin1Char,
    JSLinearString, JSAtom, ReadOnlyCompileOptions, SourceText, Handle,
    JSRuntime, ModuleDynamicImportHook, ModuleMetadataHook, ModuleResolveHook,
    ScriptPrivateReferenceHook, ModuleErrorBehaviour,
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
    obj: Handle<*mut JSObject>,
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

pub fn CompileModule1(
    cx: *mut JSContext,
    options: *const ReadOnlyCompileOptions,
    srcBuf: *mut SourceText<js::jsapi::mozilla::Utf8Unit>,
) -> *mut JSObject {
    std::ptr::null_mut()
}

pub fn FinishDynamicModuleImport(
    cx: *mut JSContext,
    evaluationPromise: Handle<*mut JSObject>,
    referencingPrivate: Handle<Value>,
    moduleRequest: Handle<*mut JSObject>,
    promise: Handle<*mut JSObject>,
) -> bool {
    true
}

pub fn GetModuleRequestSpecifier(
    cx: *mut JSContext,
    moduleRequestArg: Handle<*mut JSObject>,
) -> *mut JSString {
    std::ptr::null_mut()
}

pub fn GetRequestedModuleSpecifier(
    cx: *mut JSContext,
    moduleRecord: Handle<*mut JSObject>,
    index: u32,
) -> *mut JSString {
    std::ptr::null_mut()
}

pub fn GetRequestedModulesCount(
    cx: *mut JSContext,
    moduleRecord: Handle<*mut JSObject>,
) -> u32 {
    0
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

pub fn ModuleEvaluate(
    cx: *mut JSContext,
    moduleRecord: Handle<*mut JSObject>,
    rval: js::jsapi::MutableHandleValue,
) -> bool {
    true
}

pub fn ModuleLink(
    cx: *mut JSContext,
    moduleRecord: Handle<*mut JSObject>,
) -> bool {
    true
}

pub fn SetModuleDynamicImportHook(
    rt: *mut JSRuntime,
    func: ModuleDynamicImportHook,
) {}

pub fn SetModuleMetadataHook(
    rt: *mut JSRuntime,
    func: ModuleMetadataHook,
) {}

pub fn SetModulePrivate(module: *mut JSObject, value: *const Value) {}

pub fn SetModuleResolveHook(
    rt: *mut JSRuntime,
    func: ModuleResolveHook,
) {}

pub fn SetScriptPrivateReferenceHooks(
    rt: *mut JSRuntime,
    addRefHook: ScriptPrivateReferenceHook,
    releaseHook: ScriptPrivateReferenceHook,
) {}

pub fn ThrowOnModuleEvaluationFailure(
    cx: *mut JSContext,
    evaluationPromise: Handle<*mut JSObject>,
    errorBehaviour: ModuleErrorBehaviour,
) -> bool {
    true
}