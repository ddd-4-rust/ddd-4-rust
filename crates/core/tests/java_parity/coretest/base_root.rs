//! `base_root` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
/// Shared aggregate state used by the Java-compatible root fixtures.
#[derive(Debug, Default)]
/// `BaseRoot` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct BaseRoot {
    /// 保存 `applied_events` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub applied_events: Vec<String>,
}
