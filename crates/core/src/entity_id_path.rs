//! `entity_id_path` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! An ordered list of entity identifiers.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityIdPath`.

use crate::entity_id::EntityId;
use crate::entity_id_factory::EntityIdFactory;
use std::sync::Arc;
use thiserror::Error;

/// Validation failures for [`EntityIdPath`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// `EntityIdPathError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum EntityIdPathError {
    /// An entity path must contain at least one identifier.
    #[error("entity identifier path must not be empty")]
    Empty,
}

/// The separator between entity identifiers in serialized form.
///
/// `PATH_SEPARATOR` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
pub const PATH_SEPARATOR: &str = "/";

/// An ordered list of entity identifiers.
///
/// An aggregate root will be the first entry if it's contained in the list.
///
/// Uses `Arc<dyn EntityId>` for shared ownership, matching Java's reference semantics
/// in `rest()` and `parent()` methods.
///
/// Serialized as its string representation.
///
/// Java: `EntityIdPath extends AbstractStringValueObject implements Serializable`
#[derive(Debug, Clone)]
/// `EntityIdPath` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct EntityIdPath {
    entity_ids: Vec<Arc<dyn EntityId>>,
}

impl EntityIdPath {
    /// Creates a new entity identifier path from a list of entity IDs.
    ///
    /// Java: `new EntityIdPath(EntityId... entityIds)` / `new EntityIdPath(List<EntityId> ids)`
    ///
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(entity_ids: Vec<Arc<dyn EntityId>>) -> Result<Self, EntityIdPathError> {
        if entity_ids.is_empty() {
            return Err(EntityIdPathError::Empty);
        }
        Ok(Self { entity_ids })
    }

    /// Returns an iterator over the entity identifiers.
    ///
    /// Java: `iterator()`
    ///
    /// 执行 `iter` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn EntityId>> {
        self.entity_ids.iter()
    }

    /// Returns the number of entity IDs.
    ///
    /// 执行 `len` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn len(&self) -> usize {
        self.entity_ids.len()
    }

    /// Returns true if the path is empty (never — constructor prevents this).
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }

    /// Returns the first entity identifier in the path.
    ///
    /// Java: `first() -> <T extends EntityId> T`
    ///
    /// 执行 `first` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn first(&self) -> &Arc<dyn EntityId> {
        &self.entity_ids[0]
    }

    /// Returns the last entity identifier in the path.
    ///
    /// Java: `last() -> <T extends EntityId> T`
    ///
    /// 执行 `last` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn last(&self) -> &Arc<dyn EntityId> {
        &self.entity_ids[self.entity_ids.len() - 1]
    }

    /// Returns the path without the first entry, or None if only one element.
    ///
    /// Java: `rest() -> EntityIdPath` (returns null if only one element)
    ///
    /// 执行 `rest` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn rest(&self) -> Option<EntityIdPath> {
        if self.entity_ids.len() <= 1 {
            return None;
        }
        Some(EntityIdPath {
            entity_ids: self.entity_ids[1..].to_vec(),
        })
    }

    /// Returns the parent path without the last entry, or None if only one element.
    ///
    /// Java: `parent() -> EntityIdPath` (returns null if only one element)
    ///
    /// 执行 `parent` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn parent(&self) -> Option<EntityIdPath> {
        if self.entity_ids.len() <= 1 {
            return None;
        }
        Some(EntityIdPath {
            entity_ids: self.entity_ids[..self.entity_ids.len() - 1].to_vec(),
        })
    }

    /// Returns the number of elements in the path.
    ///
    /// Java: `size()`
    ///
    /// 执行 `size` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn size(&self) -> usize {
        self.entity_ids.len()
    }

    /// Returns the serialized form of the path.
    ///
    /// Java: `asBaseType()` / `toString()`
    ///
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_base_type(&self) -> String {
        self.entity_ids
            .iter()
            .map(|id| id.as_typed_string())
            .collect::<Vec<_>>()
            .join(PATH_SEPARATOR)
    }

    /// Converts a string into an EntityIdPath using the given factory.
    ///
    /// Java: `EntityIdPath.valueOf(EntityIdFactory factory, String str)`
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of(factory: &dyn EntityIdFactory, str: Option<&str>) -> Option<Self> {
        let str = str?;
        let entries: Vec<&str> = str.split(PATH_SEPARATOR).collect();
        let mut ids: Vec<Arc<dyn EntityId>> = Vec::new();
        for entry in entries {
            // Parse "Type id" format
            let p = entry.find(' ')?;
            let r#type = &entry[..p];
            let id = &entry[p + 1..];
            ids.push(Arc::from(factory.create_entity_id(r#type, id)?));
        }
        Some(Self { entity_ids: ids })
    }

    /// Validates that a string is a valid entity identifier path.
    ///
    /// Java: `EntityIdPath.isValid(EntityIdFactory factory, String value)`
    ///
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(factory: &dyn EntityIdFactory, value: Option<&str>) -> bool {
        let Some(value) = value else { return true };
        if value.is_empty() {
            return false;
        }
        let entries: Vec<&str> = value.split(PATH_SEPARATOR).collect();
        for entry in entries {
            if let Some(p) = entry.find(' ') {
                let r#type = &entry[..p];
                let id = &entry[p + 1..];
                if !factory.contains_type(r#type) || !factory.is_valid(r#type, id) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for EntityIdPath {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_base_type())
    }
}

// Serialize as the string representation (asBaseType)
impl serde::Serialize for EntityIdPath {
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_base_type().serialize(serializer)
    }
}
