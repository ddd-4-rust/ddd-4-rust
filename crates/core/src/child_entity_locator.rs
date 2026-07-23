//! `child_entity_locator` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.ChildEntityLocator`.

/// Typed child lookup contract represented by Java's `@ChildEntityLocator` marker.
///
/// `ChildEntityLocator` 定义该领域概念必须遵守的行为契约。
/// 实现方应保持 Java 0.7.0 对应接口的语义，并用 Rust 类型表达失败与可选值。
pub trait ChildEntityLocator<Id, Child> {
    /// Returns the child identified by `id` when it belongs to this parent.
    ///
    /// 执行 `child` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn child(&self, id: &Id) -> Option<&Child>;
}
