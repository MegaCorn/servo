from CodegenRust import CGGeneric

def v8Extend(cgthings, descriptors):
    # step1 检查是否有父节点
    parent = ""
    for des in descriptors:
        name = des.interface.parent
        if name:
            parent = name.identifier.name

    # step2 生成template代码
    code = ""
    if parent != "" and parent != "GPUError": # todo: https://developer.mozilla.org/en-US/docs/Web/API/GPUError
        code = CGGeneric(f"""
    use crate::dom::bindings::inheritance::Castable;
    use crate::dom::{parent.lower()}::{parent};
    let parent = self.upcast::<{parent}>();
    let template = parent.new_template(scope);
""")
    else:
        code = CGGeneric(f"""
    let template = v8::ObjectTemplate::new(scope);
""")

    cgthings.append(code)