//! `expected_entity_id_path_validator` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.ExpectedEntityIdPathValidator`.

use crate::{EntityIdPath, ExpectedEntityIdPath};

/// Validates the length and ordered entity types of an entity identifier path.
#[derive(Debug, Clone)]
/// `ExpectedEntityIdPathValidator` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct ExpectedEntityIdPathValidator {
    expected: ExpectedEntityIdPath,
}

impl ExpectedEntityIdPathValidator {
    /// Creates a validator for an ordered path contract.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(expected: ExpectedEntityIdPath) -> Self {
        Self { expected }
    }
    /// Returns `true` for absent paths and paths matching all expected types.
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(&self, value: Option<&EntityIdPath>) -> bool {
        let Some(path) = value else { return true };
        path.len() == self.expected.entity_types().len()
            && path
                .iter()
                .zip(self.expected.entity_types())
                .all(|(actual, expected)| actual.entity_type().as_string() == expected)
    }
}
