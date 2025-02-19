from CodegenRust import (
    CGGeneric,
    CGSpecializedMethod,
)

def v8Array(cgthings, descriptors):
    # step1 检查是否需要支持下标访问,比如hTMLCollection[0]
    isArray = False
    for des in descriptors:
        if des.interface.getExtendedAttribute("LegacyUnenumerableNamedProperties"):
            isArray = True
    if not isArray:
        return

    # step2 找到入参为index的函数,该函数用于支持下标访问
    funcName = ""
    className = ""
    for descriptor in descriptors:
        for member in descriptor.interface.members:
            if member.isMethod() and not member.isStatic():
                for idx, (rettype, arguments) in enumerate(member.signatures()):
                    for argument in arguments:
                        if str(argument.identifier).endswith("index"):
                            className = descriptor.name
                            funcName = CGSpecializedMethod.makeNativeName(descriptor, member)
                            if funcName == "__indexedgetter": # 默认的get函数名字为啥和trait定义的不一样
                                funcName = "IndexedGetter"

    # step3 生成迭代器代码
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
            let raw = value.value() as *const crate::dom::types::{className};
            let ret_ = unsafe {{ (*raw).{funcName}(index) }};
            if ret_.is_none() {{
                return v8::Intercepted::No;
            }}
            let ret_ = ret_.unwrap();
            let template = ret_.new_template(scope);
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