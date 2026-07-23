//! `ddd4j_code_gen_utils` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.codegen.processor.Ddd4jCodeGenUtils`.

/// `Ddd4jCodeGenUtils` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct Ddd4jCodeGenUtils;
impl Ddd4jCodeGenUtils {
    /// 执行 `snake_case` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub(crate) fn snake_case(value: &str) -> String {
        value
            .chars()
            .enumerate()
            .fold(String::new(), |mut output, (index, character)| {
                if character.is_uppercase() && index != 0 {
                    output.push('_');
                }
                output.extend(character.to_lowercase());
                output
            })
    }
}
