//! `string_vo` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `codegen/api/.../StringVO.java`.

use crate::SimpleValueObject;

/// Input model for generating a string value object.
#[derive(Debug, Clone, PartialEq, Eq)]
/// `StringVo` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct StringVo {
    /// Optional Rust module path.
    ///
    /// 保存 `module` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub module: String,
    /// Generated type name.
    ///
    /// 保存 `name` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub name: String,
    /// Business description.
    ///
    /// 保存 `description` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub description: String,
    /// Shared persistence and serialization switches.
    ///
    /// 保存 `targets` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub targets: SimpleValueObject,
    /// Generate `OpenAPI` metadata.
    ///
    /// 保存 `openapi` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub openapi: bool,
    /// Inclusive minimum UTF-8 byte length.
    ///
    /// 保存 `min_length` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub min_length: usize,
    /// Inclusive maximum UTF-8 byte length.
    ///
    /// 保存 `max_length` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub max_length: usize,
    /// Optional regular expression supplied to generated validation code.
    ///
    /// 保存 `pattern` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub pattern: String,
    /// Documentation/example value.
    ///
    /// 保存 `example` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub example: String,
}

impl StringVo {
    /// Java annotation default for the minimum length.
    ///
    /// `DEFAULT_MIN_LENGTH` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const DEFAULT_MIN_LENGTH: usize = 0;
    /// Java annotation default for the maximum length.
    ///
    /// `DEFAULT_MAX_LENGTH` 是该类型公开的稳定常量。
    /// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
    pub const DEFAULT_MAX_LENGTH: usize = 2_147_483_647;
}

#[cfg(test)]
mod tests {
    use super::StringVo;

    #[test]
    /// 执行 `defaults_match_java_annotation` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn defaults_match_java_annotation() {
        assert_eq!(StringVo::DEFAULT_MIN_LENGTH, 0);
        assert_eq!(StringVo::DEFAULT_MAX_LENGTH, 2_147_483_647);
    }
}
