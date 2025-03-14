from CodegenRust import (
    CGGeneric,
    CGSpecializedGetter,
    return_type,
    typeNeedsCx,
    getRetvalDeclarationForType,
    returnTypeNeedsOutparam,
)


def makeFuncCall(descriptor, member):
    nativeName = CGSpecializedGetter.makeNativeName(descriptor, member)
    inRealm = nativeName in descriptor.inRealmMethods
    canGc = nativeName in descriptor.canGcMethods
    result = getRetvalDeclarationForType(member.type, descriptor)
    if member.type and returnTypeNeedsOutparam(member.type):
        rootType = result
    else:
        rootType = None

    funcCall = nativeName + "("
    if typeNeedsCx(member.type, member.readonly): # 这里的参数顺序是servo硬编码的
        funcCall += "crate::script_runtime::JSContext::from_ptr(std::ptr::null_mut()), "
    if inRealm:
        funcCall += "crate::realms::InRealm::mock(), "
    if canGc:
        funcCall += "crate::script_runtime::CanGc::note(), "
    if rootType:
        funcCall += "js::rust::MutableHandleValue::from_marked_location(std::ptr::null_mut()), "
    if funcCall.endswith(", "):
        funcCall = funcCall[0:-2]
    funcCall += ")"

    return funcCall, nativeName


def makeUnwrapCode(returnType):
    option = False
    fall = False
    if returnType.startswith("Fallible<"):
        fall = True
        returnType = returnType[9:-1]
    if returnType.startswith("Option<"):
        option = True
        returnType = returnType[7:-1]

    unwrap1 = ""
    if fall:
        unwrap1 = f"""
            return_if_err!(ret);
            let ret = ret.unwrap();
"""
    unwrap2 = ""
    if option:
        unwrap2 = f"""
            return_if_none!(ret);
            let ret = ret.unwrap();
"""

    return returnType, unwrap1, unwrap2


def v8Getter(attr, descriptor, member):
    # mozjs独有, 用于构造servo函数调用入参, 切引擎后删掉
    funcCall, nativeName = makeFuncCall(descriptor, member)

    # 检查函数返回值是否需要unwrap
    infallible = 'infallible' in descriptor.getExtendedAttributes(member, getter=True)
    returnType = return_type(descriptor, member.type, infallible)
    returnType, unwrap1, unwrap2 = makeUnwrapCode(returnType)

    # 将函数返回值转换为v8对象
    match = True
    # 基本类型
    # USVString, DOMString, ByteString, bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, Finite<f32>, Finite<f64>
    if returnType == "USVString":
        trans = f"""let ret = v8::String::new(scope, ret.as_ref()).unwrap();"""
    elif returnType == "DOMString":
        trans = f"""let ret = v8::String::new(scope, ret.str()).unwrap();"""
    elif returnType == "ByteString":
        trans = f"""let ret = v8::String::new(scope, ret.as_str().unwrap()).unwrap();"""
    elif returnType == "bool":
        trans = f"""let ret = v8::Boolean::new(scope, ret);"""
    elif returnType == "i8":
        trans = f"""let ret = v8::Integer::new(scope, ret as i32);"""
    elif returnType == "u8":
        trans = f"""let ret = v8::Integer::new_from_unsigned(scope, ret as u32);"""
    elif returnType == "i16":
        trans = f"""let ret = v8::Integer::new(scope, ret as i32);"""
    elif returnType == "u16":
        trans = f"""let ret = v8::Integer::new_from_unsigned(scope, ret as u32);"""
    elif returnType == "i32":
        trans = f"""let ret = v8::Integer::new(scope, ret);"""
    elif returnType == "u32":
        trans = f"""let ret = v8::Integer::new_from_unsigned(scope, ret);"""
    elif returnType == "i64":
        trans = f"""let ret = v8::Integer::new(scope, ret as i32);"""
    elif returnType == "u64":
        trans = f"""let ret = v8::Integer::new_from_unsigned(scope, ret as u32);"""
    elif returnType == "f32":
        trans = f"""let ret = v8::Number::new(scope, ret as f64);"""
    elif returnType == "f64":
        trans = f"""let ret = v8::Number::new(scope, ret);"""
    elif returnType == "Finite<f32>":
        trans = f"""let ret = v8::Number::new(scope, *ret as f64);"""
    elif returnType == "Finite<f64>":
        trans = f"""let ret = v8::Number::new(scope, *ret);"""
    # 枚举类型
    elif member.type.isEnum():
        prefix = ""
        if descriptor.name == "OffscreenCanvasRenderingContext2D":
            prefix = "crate::dom::types::CanvasGradient::"
        elif descriptor.name == "PaintRenderingContext2D":
            prefix = "crate::dom::types::CanvasGradient::"
        elif descriptor.name == "XRJointSpace":
            prefix = "crate::dom::types::XRHand::"
        else:
            prefix = "Self::"
        trans = f"""let ret = v8::String::new(scope, {prefix}{returnType}_to_str(&ret)).unwrap();"""
    # object
    elif "DomRoot<" in returnType and "WindowProxy" not in returnType and "GPU" not in returnType and "WGSLLanguageFeatures" not in returnType: # todo WindowProxy GPU
        trans = f"""let template = ret.new_template(scope);
            let raw = ret.value.ptr.as_ptr() as *mut std::ffi::c_void;
            template.set_internal_field_count(1);
            let ret = template.new_instance(scope).unwrap();
            ret.set_internal_field(0, v8::External::new(scope, raw).into());"""
    # callback
    elif returnType == "Rc<crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull>":
        trans = f"""let func_ = ret.get_v8();
            return_if_none!(func_);
            let func_ = func_.unwrap();
            let ret = v8::Local::new(scope, func_);"""
    else:
        match = False
        trans = ""

    tmp = f"""log::error!("============== unsupported getter api {nativeName} ================");"""
    if match:
        tmp = "rv.set(ret.into());"

    aa = CGSpecializedGetter.makeNativeName(descriptor, member)
    if aa == "Languages" and descriptor.name == "Navigator":
        tmp = f"""
            let array = v8::Array::new(scope, 1);
            let s1 = v8::String::new(scope, "zh-CN").unwrap();
            array.set_index(scope, 0, s1.into());
            rv.set(array.into());
        """
    elif aa == "Response" and descriptor.name == "XMLHttpRequest":
        tmp = f"""
            let ret_ = unsafe {{ (*raw).Response1(crate::script_runtime::JSContext::from_ptr(std::ptr::null_mut()), crate::script_runtime::CanGc::note(), js::rust::MutableHandleValue::from_marked_location(std::ptr::null_mut())) }};
            let ret = v8::String::new(scope, ret_.as_str()).unwrap();
            rv.set(ret.into());
        """

    # 构造代码段
    getterCode = CGGeneric(f"""
    let getter_{attr} =
        |scope: &mut v8::HandleScope,
        key: v8::Local<v8::Name>,
        args: v8::PropertyCallbackArguments,
        mut rv: v8::ReturnValue<v8::Value>| {{
            log::error!("getter {attr}");
            let this = args.this();
            let data = this.get_internal_field(scope, 0).unwrap();
            let value: v8::Local<v8::External> = data.try_into().unwrap();
            let raw = value.value() as *const crate::dom::types::{descriptor.name};
            let ret = unsafe {{ (*raw).{funcCall} }};
            {unwrap1}{unwrap2}
            {trans}
            {tmp}
            // {returnType} {match}
        }};
""")
    return getterCode