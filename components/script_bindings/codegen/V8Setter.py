from CodegenRust import (
    CGGeneric,
    CGSpecializedSetter,
    argument_type,
)


def makeFuncCall(descriptor, member, argType):
    nativeName = CGSpecializedSetter.makeNativeName(descriptor, member)
    canGc = nativeName in descriptor.canGcMethods

    optionPre = ""
    optionTail = ""
    if "Option<" in argType:
        argType = argType[7:-1]
        optionPre = "Some("
        optionTail = ")"

    finitePre = ""
    finiteTail = ""
    if "Finite<" in argType:
        argType = argType[7:-1]
        finitePre = "Finite::wrap("
        finiteTail = ")"

    funcCall = f"""unsafe {{ (*raw).{nativeName}({optionPre}{finitePre}val{finiteTail}{optionTail}, """
    # mozjs独有, 切引擎后删掉
    if canGc:
        funcCall += "crate::script_runtime::CanGc::note(), "
    funcCall = funcCall[0:-2]
    funcCall += "); };"

    return argType, funcCall


def v8Setter(attr, descriptor, member):
    # 构造servo函数调用
    argType = argument_type(descriptor, member.type)
    argType, funcCall = makeFuncCall(descriptor, member, argType)

    # 将v8::Value对象转换为servo对象
    # 基本类型
    # USVString, DOMString, bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64
    if argType == "USVString":
        trans = f"""let val = USVString::from(value.to_rust_string_lossy(scope));"""
    elif argType == "DOMString":
        trans = f"""let val = DOMString::from(value.to_rust_string_lossy(scope));"""
    elif argType == "bool":
        trans = f"""let val = value.boolean_value(scope);"""
    elif argType == "i8":
        trans = f"""let val = value.int32_value(scope).unwrap() as i8;"""
    elif argType == "u8":
        trans = f"""let val = value.uint32_value(scope).unwrap() as u8;"""
    elif argType == "i16":
        trans = f"""let val = value.int32_value(scope).unwrap() as i16;"""
    elif argType == "u16":
        trans = f"""let val = value.uint32_value(scope).unwrap() as u16;"""
    elif argType == "i32":
        trans = f"""let val = value.int32_value(scope).unwrap();"""
    elif argType == "u32":
        trans = f"""let val = value.uint32_value(scope).unwrap();"""
    elif argType == "i64":
        trans = f"""let val = value.integer_value(scope).unwrap();"""
    elif argType == "u64":
        trans = f"""let val = value.integer_value(scope).unwrap() as u64;"""
    elif argType == "f32":
        trans = f"""let val = value.number_value(scope).unwrap() as f32;"""
    elif argType == "f64":
        trans = f"""let val = value.number_value(scope).unwrap();"""
    # callback
    elif argType == "Rc<EventHandlerNonNull>":
        trans = f"""use crate::dom::bindings::codegen::Bindings::EventHandlerBinding::EventHandlerNonNull;
            let val_ = v8::Local::<v8::Function>::try_from(value).unwrap();
            let global_ = v8::Global::new(scope, val_);
            let global_raw = global_.into_raw();
            log::error!("====================jignuoen setter todo========================");
            let val = unsafe {{ EventHandlerNonNull::new_v8(global_raw) }};"""
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
        trans = f"""let val_ = value.to_rust_string_lossy(scope);
            let val = {prefix}str_to_{argType}(val_.as_str());"""
    else:
        funcCall = ""
        trans = ""

    setterCode = CGGeneric(f"""
    let setter_{attr} =
        |scope: &mut v8::HandleScope,
        key: v8::Local<v8::Name>,
        value: v8::Local<v8::Value>,
        args: v8::PropertyCallbackArguments,
        _rv: v8::ReturnValue<()>| {{
            log::error!("setter {attr}");
            let this = args.this();
            let data = this.get_internal_field(scope, 0).unwrap();
            let value_: v8::Local<v8::External> = data.try_into().unwrap();
            let raw = value_.value() as *const crate::dom::types::{descriptor.name};
            {trans}
            {funcCall}
            // {argType}
        }};
""")
    return setterCode