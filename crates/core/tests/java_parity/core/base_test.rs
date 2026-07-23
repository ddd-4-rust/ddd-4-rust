//! `base_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{
    EntityId, EntityIdFactory, EntityType, EntityTypeError, StringBasedEntityType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// `TestEntityId` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct TestEntityId {
    entity_type: StringBasedEntityType,
    value: i32,
}

impl TestEntityId {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub(crate) fn new(entity_type: &str, value: i32) -> Result<Self, EntityTypeError> {
        Ok(Self {
            entity_type: StringBasedEntityType::new(entity_type)?,
            value,
        })
    }
}

impl std::fmt::Display for TestEntityId {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.value)
    }
}

impl EntityId for TestEntityId {
    /// 执行 `entity_type` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn entity_type(&self) -> &dyn EntityType {
        &self.entity_type
    }
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn as_string(&self) -> String {
        self.value.to_string()
    }
}

/// `TestEntityIdFactory` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub(crate) struct TestEntityIdFactory;

impl EntityIdFactory for TestEntityIdFactory {
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn contains_type(&self, entity_type: &str) -> bool {
        matches!(entity_type, "A" | "B" | "C")
    }
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn is_valid(&self, entity_type: &str, id: &str) -> bool {
        self.contains_type(entity_type) && id.parse::<i32>().is_ok()
    }
    /// 执行 `create_entity_id` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn create_entity_id(&self, entity_type: &str, id: &str) -> Option<Box<dyn EntityId>> {
        if !self.is_valid(entity_type, id) {
            return None;
        }
        let value = id.parse().ok()?;
        TestEntityId::new(entity_type, value)
            .ok()
            .map(|id| Box::new(id) as Box<dyn EntityId>)
    }
}
