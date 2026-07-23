//! `the_root_id` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Generated from Java `TheRootId.java`.

use ddd_4_rust_codegen_processor::AggregateRootUuidValueObject;

/// Generated aggregate root identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AggregateRootUuidValueObject)]
/// `TheRootId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct TheRootId(uuid::Uuid);
