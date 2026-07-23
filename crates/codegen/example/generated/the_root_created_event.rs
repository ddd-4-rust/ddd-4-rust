//! `the_root_created_event` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Generated from Java `TheRootCreatedEvent.java`.

use ddd_4_rust_codegen_processor::EventValueObject;
use serde::{Deserialize, Serialize};

/// Generated root-created event payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EventValueObject)]
/// `TheRootCreatedEvent` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct TheRootCreatedEvent {
    /// Root identifier.
    ///
    /// 保存 `root_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub root_id: uuid::Uuid,
    /// Initial root name.
    ///
    /// 保存 `root_name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub root_name: String,
}
