from CodegenRust import (
    CGGeneric,
    getEnumValueName,
)

def v8Enum(cgthings, enums):
    if len(enums) == 0:
        return
    for enum in enums:
        ident = enum.identifier.name
        pairs = ",\n        ".join([f'"{val}" => {ident}::{getEnumValueName(val)}'
                                for val in list(enum.values())])
        pairs += f""",\n        _ => {ident}::{getEnumValueName(list(enum.values())[0])}"""
        code = CGGeneric(f"""
pub fn str_to_{ident}(param: &str) -> {ident} {{
    match (param) {{
        {pairs}
    }}
}}
""")
        cgthings.append(code)

        pairs = ",\n        ".join([f'{ident}::{getEnumValueName(val)} => "{val}"'
                                for val in list(enum.values())])
        pairs += f""",\n        _ => \"{list(enum.values())[0]}\""""
        code = CGGeneric(f"""
pub fn {ident}_to_str(param: &{ident}) -> &'static str {{
    match (param) {{
        {pairs}
    }}
}}
""")
        cgthings.append(code)