//! `event_vo` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Rust token golden translated from `codegen/processor/src/main/resources/EventVO.java`.

use ddd_4_rust_codegen_processor::EventValueObject;

#[derive(Debug, Clone, PartialEq, Eq, EventValueObject)]
/// `GeneratedEventVo` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct GeneratedEventVo(pub String);
