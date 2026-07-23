//! `ddd_serde_module` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Serde marker for DDD wire types.

/// Marker for Serde-capable DDD types.
///
/// Serde discovers adapters through derive attributes or explicit
/// `Serialize`/`Deserialize` implementations, so no runtime registration is
/// required. This marker preserves the corresponding Java source concept.
///
/// `DddSerdeModule` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct DddSerdeModule;

impl DddSerdeModule {
    /// Creates a new DDD serde module.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new() -> Self {
        Self
    }
}

impl Default for DddSerdeModule {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn default() -> Self {
        Self::new()
    }
}
