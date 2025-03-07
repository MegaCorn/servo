from CodegenRust import (
    CGGeneric,
    CGSpecializedMethod,
    return_type,
    argument_type,
    needCx,
    returnTypeNeedsOutparam,
    outparamTypeFromReturnType,
)


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


def method_arguments(descriptorProvider, returnType, arguments, passJSBits=True, inRealm=False, canGc=False):
    if needCx(returnType, arguments, passJSBits):
        yield "SafeJSContext"

    for argument in arguments:
        ty = argument_type(descriptorProvider, argument.type, argument.optional,
                           argument.defaultValue, argument.variadic)
        yield ty

    if inRealm:
        yield "InRealm"

    if canGc:
        yield "CanGc"

    if returnTypeNeedsOutparam(returnType):
        yield outparamTypeFromReturnType(returnType),


def makeFuncArgs(descriptor, arguments, nativeName):
    funcCall = ""
    idx = 0
    support = True
    mut = "*const"
    for argument in arguments:
        argument = str(argument)

        optionPre = ""
        optionTail = ""
        if "Option<" in argument:
            argument = argument[7:-1]
            optionPre = "Some("
            optionTail = ")"

        finitePre = ""
        finiteTail = ""
        if "Finite<" in argument:
            argument = argument[7:-1]
            finitePre = "Finite::wrap("
            finiteTail = ")"

        # 基本类型
        # USVString, DOMString, bool, i32, u32, i64, u64, f32, f64
        if argument == "USVString":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}USVString(args.get({idx}).to_rust_string_lossy(scope)){finiteTail}{optionTail};"""
        elif argument == "DOMString":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}DOMString::from(args.get({idx}).to_rust_string_lossy(scope)){finiteTail}{optionTail};"""
        elif argument == "ByteString":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}ByteString::new(args.get({idx}).to_rust_string_lossy(scope).into()){finiteTail}{optionTail};"""
        elif argument == "bool":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).boolean_value(scope){finiteTail}{optionTail};"""
        elif argument == "i32":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).int32_value(scope).unwrap(){finiteTail}{optionTail};"""
        elif argument == "u32":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).uint32_value(scope).unwrap(){finiteTail}{optionTail};"""
        elif argument == "i64":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).integer_value(scope).unwrap(){finiteTail}{optionTail};"""
        elif argument == "u64":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).integer_value(scope).unwrap() as u64{finiteTail}{optionTail};"""
        elif argument == "f32":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).number_value(scope).unwrap() as f32{finiteTail}{optionTail};"""
        elif argument == "f64":
            funcCall += f"""
            let arg{idx} = {optionPre}{finitePre}args.get({idx}).number_value(scope).unwrap(){finiteTail}{optionTail};"""
        # callback
        elif argument == "Rc<crate::dom::bindings::codegen::Bindings::EventListenerBinding::EventListener>" and descriptor.name == "EventTarget":
            mut = "*mut"
            if nativeName == "AddEventListener":
                funcCall += f"""
            let val_ = v8::Local::<v8::Function>::try_from(args.get({idx}));
            return_if_err!(val_);
            let val_ = val_.unwrap();
            let global_ = v8::Global::new(scope, val_);
            let global_scope = unsafe {{ (*raw).global() }};
            let listener = crate::dom::bindings::codegen::Bindings::EventListenerBinding::EventListener::new_v8(global_, global_scope.value.ptr.as_ptr() as *mut js::jsapi::JSObject);
            unsafe {{ (*raw).add_cb_map(args.get(0).to_rust_string_lossy(scope), listener.clone()); }};
            let arg{idx} = {optionPre}{finitePre}listener{finiteTail}{optionTail};"""
            elif nativeName == "RemoveEventListener":
                funcCall += f"""
            let val_ = v8::Local::<v8::Function>::try_from(args.get({idx})).unwrap();
            let global_ = v8::Global::new(scope, val_);
            let arg{idx} = unsafe {{ (*raw).remove_cb_map(&args.get(0).to_rust_string_lossy(scope)) }};
            if arg{idx}.is_none() {{
                return
            }}"""
        elif argument.startswith("UnionTypes::"):
            argument = argument[12:]

            if (nativeName.endswith("getter") or
                nativeName.endswith("deleter") or
                nativeName == "Keys" or
                nativeName == "Entries" or
                nativeName == "Values" or
                nativeName == "Alert" or
                nativeName == "Has" or
                nativeName == "Remove" or
                nativeName.endswith("setter")): # todo
                continue

            if (argument == "StringOrElementCreationOptions" or
               argument == "AddEventListenerOptionsOrBoolean" or
               argument == "StringOrUnsignedLong" or
               argument == "UnsignedLongOrBoolean"):
               funcCall += f"""
            let opt = v8_to_{argument}(scope, args.get({idx}));
            return_if_none!(opt);
            let opt = opt.unwrap();
            let arg{idx} = {optionPre}{finitePre}opt{finiteTail}{optionTail};"""
            else:
                support = False
        elif argument == "CanGc":
            funcCall += f"""
            let arg{idx} = crate::script_runtime::CanGc::note();"""
        elif argument == "SafeJSContext":
            funcCall += f"""
            let arg{idx} = unsafe {{ SafeJSContext::from_ptr(std::ptr::null_mut()) }};"""
        elif argument == "InRealm":
            funcCall += f"""
            let arg{idx} = crate::realms::InRealm::mock();"""
        # node
        elif argument == "&Node" and descriptor.name != "NodeFilter":
            funcCall += f"""
            let jsobj{idx} = args.get({idx}).to_object(scope).unwrap();
            let data{idx} = jsobj{idx}.get_internal_field(scope, 0).unwrap();
            let value{idx}: v8::Local<v8::External> = data{idx}.try_into().unwrap();
            let arg{idx} = value{idx}.value() as *const crate::dom::types::Node;
            let arg{idx} = unsafe {{{optionPre}&*arg{idx}{optionTail}}};"""
        # Event
        elif argument == "&Event":
            funcCall += f"""
            let jsobj{idx} = args.get({idx}).to_object(scope).unwrap();
            let data{idx} = jsobj{idx}.get_internal_field(scope, 0).unwrap();
            let value{idx}: v8::Local<v8::External> = data{idx}.try_into().unwrap();
            let arg{idx} = value{idx}.value() as *const crate::dom::types::Event;
            let arg{idx} = unsafe {{{optionPre}&*arg{idx}{optionTail}}};"""
        else:
            support = False
        idx += 1

    return funcCall, support, mut


def v8Function(descriptor, cgthings):
    for member in descriptor.interface.members:
        if (descriptor.name.endswith("Iterator") or
            descriptor.name.endswith("Setlike")): # todo
            continue

        if member.isMethod() and not member.isStatic():
            method = member.identifier.name
            method = method.replace("-", "_") # rust不允许函数名出现-
            nativeName = CGSpecializedMethod.makeNativeName(descriptor, member)

            if nativeName == "ToJSON": # todo
                continue
            if (nativeName.endswith("getter") or
                nativeName.endswith("deleter") or
                nativeName == "Keys" or
                nativeName == "Entries" or
                nativeName == "Values" or
                nativeName == "Alert" or
                nativeName == "Has" or
                nativeName == "Remove" or
                nativeName.endswith("setter")): # todo
                continue

            cratePrefix = "crate::dom::types::"
            if descriptor.name == "NodeFilter": # NodeFilter定义在binding里
                cratePrefix = ""

            for idx, (returnType, arguments) in enumerate(member.signatures()):
                arguments = method_arguments(descriptor, returnType, arguments,
                                             inRealm=nativeName in descriptor.inRealmMethods,
                                             canGc=nativeName in descriptor.canGcMethods)
                infallible = 'infallible' in descriptor.getExtendedAttributes(member)
                returnType = return_type(descriptor, returnType, infallible)
            returnType, unwrap1, unwrap2 = makeUnwrapCode(returnType)
            arguments = list(arguments)

            if "DomRoot<WindowProxy>" in returnType:
                continue
            if returnType.startswith("Vec") or "GPU" in returnType or "ImageData" in returnType: # todo ImageData是火狐特有的, 非webidl官方定义
                continue

            # 构造servo函数调用
            funcCall, support, mut = makeFuncArgs(descriptor, arguments, nativeName)

            # 将函数返回值转换为v8对象
            jsw = ""
            if support:
                tmp = "rv.set(object.into());"
            else:
                tmp = f"""log::error!("============== unsupported function api {nativeName} ================");"""
            if support and returnType == "bool":
                jsw = f"""let object = v8::Boolean::new(scope, ret);"""
            elif support and returnType == "USVString":
                jsw = f"""let object = v8::String::new(scope, ret.as_ref()).unwrap();"""
            elif support and returnType == "DOMString":
                jsw = f"""let object = v8::String::new(scope, ret.str()).unwrap();"""
            elif support and returnType == "ByteString":
                jsw = f"""let object = v8::String::new(scope, ret.as_str().unwrap()).unwrap();"""
            elif support and "DomRoot<Element>" in returnType:
                jsw = f"""let template = crate::dom::virtualmethods::node_downcast_template(unsafe {{ ret.value.ptr.as_ref() }}, scope);
            template.set_internal_field_count(1);
            let object = template.new_instance(scope).unwrap();
            let raw_ = ret.value.ptr.as_ptr() as *mut std::ffi::c_void;
            object.set_internal_field(0, v8::External::new(scope, raw_).into());"""
            elif support and "DomRoot<Node>" in returnType:
                jsw = f"""let template = crate::dom::virtualmethods::node_downcast_template1(unsafe {{ ret.value.ptr.as_ref() }}, scope);
            template.set_internal_field_count(1);
            let object = template.new_instance(scope).unwrap();
            let raw_ = ret.value.ptr.as_ptr() as *mut std::ffi::c_void;
            object.set_internal_field(0, v8::External::new(scope, raw_).into());"""
            elif support and "DomRoot<" in returnType:
                jsw = f"""let template = ret.new_template(scope);
            template.set_internal_field_count(1);
            let object = template.new_instance(scope).unwrap();
            let raw_ = ret.value.ptr.as_ptr() as *mut std::ffi::c_void;
            object.set_internal_field(0, v8::External::new(scope, raw_).into());"""
            else:
                tmp = ""


            stub = ", ".join(f"arg{i}" for i in range(len(arguments)))

            if nativeName == "ScrollTo": # todo
                nativeName = "ScrollTo_"
            if nativeName == "ScrollBy":
                nativeName = "ScrollBy_"
            if nativeName == "Scroll":
                nativeName = "Scroll_"
            if nativeName == "Open" and descriptor.name == "Document":
                nativeName = "Open_"

            if support:
                funcCall += f"""
            let ret = unsafe {{ (*raw).{nativeName}({stub})}};"""

            trans_ = f"""log::error!("============== unsupported function api {nativeName} ================");"""
            if support:
                trans_ = funcCall
            else:
                unwrap1 = ""
                unwrap2 = ""

            if nativeName == "Open" and descriptor.name == "XMLHttpRequest":
                trans_ = f"""
            //log::error!("fn open {{}} {{}}", args.get(0).to_rust_string_lossy(scope), args.get(1).to_rust_string_lossy(scope));
            let arg0 = ByteString::new(args.get(0).to_rust_string_lossy(scope).into());
            let arg1 = USVString::from(args.get(1).to_rust_string_lossy(scope));
            let ret = unsafe {{ (*raw).Open(arg0, arg1)}};
                """
            if nativeName == "Send" and descriptor.name == "XMLHttpRequest":
                trans_ = f"""
            //log::error!("fn send {{}}", args.get(0).to_rust_string_lossy(scope));
            let ret = unsafe {{ (*raw).Send(Some(crate::dom::bindings::codegen::UnionTypes::DocumentOrBlobOrArrayBufferViewOrArrayBufferOrFormDataOrStringOrURLSearchParams::String(DOMString::from(args.get(0).to_rust_string_lossy(scope)))), crate::script_runtime::CanGc::note())}};
                """

            code = CGGeneric(f"""
    let fn_{method} = v8::FunctionTemplate::new(
        scope,
        |scope: &mut v8::HandleScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue<v8::Value>| {{
            //log::error!("fn {method}");
            let this = args.this();
            let data = this.get_internal_field(scope, 0).unwrap();
            let value: v8::Local<v8::External> = data.try_into().unwrap();
            let raw = value.value() as {mut} {cratePrefix}{descriptor.name};
            {trans_}
            {unwrap1}{unwrap2}
            {jsw}
            {tmp}
            // {nativeName}
            // {arguments} {support}
            // {returnType}
        }},
    );
    v8_template_add_fn!(scope, template, fn_{method}, {method});
""")
            cgthings.append(code)
