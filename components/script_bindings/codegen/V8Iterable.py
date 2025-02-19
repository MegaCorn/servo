from CodegenRust import (
    CGGeneric,
    CGSpecializedMethod,
)

def findGetFunc(descriptors):
    funcName = ""
    for descriptor in descriptors:
        for member in descriptor.interface.members:
            if member.isMethod() and not member.isStatic():
                for idx, (rettype, arguments) in enumerate(member.signatures()):
                    for argument in arguments:
                        if str(argument.identifier).endswith("index"):
                            funcName = CGSpecializedMethod.makeNativeName(descriptor, member)
    return funcName

def v8Iterable(cgthings, descriptors):
    for descriptor in descriptors:
        for member in descriptor.interface.members:
            # step1 检查是否需要支持迭代器访问
            if member.isMaplikeOrSetlikeOrIterable() and not member.isMaplikeOrSetlike():
                funcName = findGetFunc(descriptors)
                # step2 生成迭代器代码
                if member.isValueIterator():
                    value = str(member.valueType)
                    if value == "String":
                        pass
                    elif value == "StringOrNull":
                        code = CGGeneric(f"""
    let index_getter =
        |scope: &mut v8::HandleScope,
        index: u32,
        args: v8::PropertyCallbackArguments,
        mut rv: v8::ReturnValue<v8::Value>| {{
            log::error!("index_getter");
            let this = args.this();
            let data = this.get_internal_field(scope, 0).unwrap();
            let value: v8::Local<v8::External> = data.try_into().unwrap();
            let raw = value.value() as *const crate::dom::types::{descriptor.name};
            let ret_ = unsafe {{ (*raw).{funcName}(index) }};
            if ret_.is_none() {{
                return v8::Intercepted::No;
            }}
            let ret_ = ret_.unwrap();
            let object = v8::String::new(scope, ret_.str()).unwrap();
            rv.set(object.into());
            v8::Intercepted::Yes
        }};
    template.set_indexed_property_handler(v8::IndexedPropertyHandlerConfiguration::new().getter(index_getter));
""")
                        cgthings.append(code)
                    elif value == "Node (Wrapper)OrNull":
                        code = CGGeneric(f"""
    let index_getter =
        |scope: &mut v8::HandleScope,
        index: u32,
        args: v8::PropertyCallbackArguments,
        mut rv: v8::ReturnValue<v8::Value>| {{
            log::error!("index_getter");
            let this = args.this();
            let data = this.get_internal_field(scope, 0).unwrap();
            let value: v8::Local<v8::External> = data.try_into().unwrap();
            let raw = value.value() as *const crate::dom::types::{descriptor.name};
            let ret_ = unsafe {{ (*raw).{funcName}(index) }};
            if ret_.is_none() {{
                return v8::Intercepted::No;
            }}
            let ret_ = ret_.unwrap();
            let template = crate::dom::virtualmethods::node_downcast_template1(unsafe {{ ret_.value.ptr.as_ref() }}, scope);
            template.set_internal_field_count(1);
            let object = template.new_instance(scope).unwrap();
            let raw_ = ret_.value.ptr.as_ptr() as *mut std::ffi::c_void;
            object.set_internal_field(0, v8::External::new(scope, raw_).into());
            rv.set(object.into());
            v8::Intercepted::Yes
        }};
    template.set_indexed_property_handler(v8::IndexedPropertyHandlerConfiguration::new().getter(index_getter));
""")
                        cgthings.append(code)
                    elif value == "XRInputSource (Wrapper)":
                        pass
                elif member.isPairIterator():
                    pass