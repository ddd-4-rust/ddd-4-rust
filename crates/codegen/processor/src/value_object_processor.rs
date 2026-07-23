//! `value_object_processor` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.codegen.processor.ValueObjectProcessor`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// `TargetType` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub(crate) enum TargetType {
    String,
    IntegerEntityId,
    AggregateRootUuid,
    Event,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// `AnnotationTemplate` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct AnnotationTemplate {
    /// 保存 `target` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub target: TargetType,
    /// 保存 `type_name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub type_name: String,
}
/// `ValueObjectProcessor` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct ValueObjectProcessor;
impl ValueObjectProcessor {
    /// 执行 `derive_name` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub(crate) fn derive_name(target: TargetType) -> &'static str {
        match target {
            TargetType::String => "StringValueObject",
            TargetType::IntegerEntityId => "IntegerEntityIdValueObject",
            TargetType::AggregateRootUuid => "AggregateRootUuidValueObject",
            TargetType::Event => "EventValueObject",
        }
    }
}
