//! `compat` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Shared Serde representations behind the JSON and XML compatibility namespaces.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AbstractAggregateExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractAggregateExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AbstractVersionedAggregateExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AbstractVersionedAggregateExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
    /// 保存 `version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub version: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AggregateAlreadyExistsExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateAlreadyExistsExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
    /// 保存 `version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub version: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AggregateDeletedExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateDeletedExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AggregateNotFoundExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateNotFoundExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AggregateVersionConflictExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateVersionConflictExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
    /// 保存 `expected_version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub expected_version: i32,
    /// 保存 `actual_version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub actual_version: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `AggregateVersionNotFoundExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct AggregateVersionNotFoundExceptionData {
    /// 保存 `aggregate_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_type: String,
    /// 保存 `aggregate_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub aggregate_id: String,
    /// 保存 `version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub version: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `DecryptionFailedExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DecryptionFailedExceptionData {
    /// 保存 `message` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `DuplicateEncryptionKeyIdExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DuplicateEncryptionKeyIdExceptionData {
    /// 保存 `key_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub key_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `DuplicateEntityExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DuplicateEntityExceptionData {
    /// 保存 `parent_id_path` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub parent_id_path: Option<String>,
    /// 保存 `entity_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `EncryptionKeyIdUnknownExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EncryptionKeyIdUnknownExceptionData {
    /// 保存 `key_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub key_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `EncryptionKeyVersionUnknownExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EncryptionKeyVersionUnknownExceptionData {
    /// 保存 `key_version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub key_version: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `EntityNotFoundExceptionData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityNotFoundExceptionData {
    /// 保存 `parent_id_path` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub parent_id_path: Option<String>,
    /// 保存 `entity_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub entity_id: String,
}

/// `CompatAbstractEvent` 是迁移兼容层公开的稳定类型别名。
/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
pub type CompatAbstractEvent = crate::AbstractEvent;
/// `CompatAbstractDomainEvent` 是迁移兼容层公开的稳定类型别名。
/// 该别名不复制底层数据，用于保持 Java 来源类型与 Rust 公共路径的一一对应关系。
pub type CompatAbstractDomainEvent = crate::AbstractDomainEvent;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// `CompatEncryptedData` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct CompatEncryptedData {
    /// 保存 `key_id` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub key_id: String,
    /// 保存 `key_version` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub key_version: String,
    /// 保存 `data_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub data_type: String,
    /// 保存 `content_type` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub content_type: String,
    /// 保存 `encrypted_data` 对应的领域数据。
    /// 该字段的类型、可选性和序列化名称共同构成与 Java 基线兼容的公开契约。
    pub encrypted_data: Vec<u8>,
}

impl ddd_4_rust_core::EncryptedData for CompatEncryptedData {
    /// 执行 `key_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_id(&self) -> &str {
        &self.key_id
    }
    /// 执行 `key_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn key_version(&self) -> &str {
        &self.key_version
    }
    /// 执行 `data_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn data_type(&self) -> &str {
        &self.data_type
    }
    /// 执行 `content_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn content_type(&self) -> &str {
        &self.content_type
    }
    /// 执行 `encrypted_data` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn encrypted_data(&self) -> &[u8] {
        &self.encrypted_data
    }
}
