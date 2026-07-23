//! `value_object_template` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.codegen.processor.ValueObjectTemplate`.

#[derive(Debug, Clone, PartialEq, Eq)]
/// `ValueObjectTemplate` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct ValueObjectTemplate {
    /// 保存 `type_name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub type_name: String,
    /// 保存 `description` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub description: String,
}
impl ValueObjectTemplate {
    /// 执行 `render_header` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub(crate) fn render_header(&self) -> String {
        format!("/// {}\npub struct {};", self.description, self.type_name)
    }
}
