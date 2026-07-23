//! `has_entity_type_constant_validator` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `org.fuin.ddd4j.core.HasEntityTypeConstantValidator`.

use crate::{EntityTypeError, HasEntityTypeConstant, StringBasedEntityType};

/// Validates and extracts a type's associated entity type constant.
///
/// `HasEntityTypeConstantValidator` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct HasEntityTypeConstantValidator;

impl HasEntityTypeConstantValidator {
    /// Validates the associated constant using the same bounds as entity types.
    ///
    /// 执行 `extract_value` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn extract_value<T: HasEntityTypeConstant>()
    -> Result<StringBasedEntityType, EntityTypeError> {
        StringBasedEntityType::new(T::ENTITY_TYPE)
    }
}
