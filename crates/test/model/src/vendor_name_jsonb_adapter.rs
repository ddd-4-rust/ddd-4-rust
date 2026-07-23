//! `vendor_name_jsonb_adapter` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use crate::{VendorName, vendor_name::VendorNameError};

/// Converts `VendorName` to and from its JSON-B string representation.
///
/// `VendorNameJsonbAdapter` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorNameJsonbAdapter;
impl VendorNameJsonbAdapter {
    #[must_use]
    /// 执行 `adapt_to_json` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn adapt_to_json(value: Option<&VendorName>) -> Option<&str> {
        value.map(VendorName::as_str)
    }
    /// 执行 `adapt_from_json` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn adapt_from_json(value: Option<&str>) -> Result<Option<VendorName>, VendorNameError> {
        VendorName::value_of(value)
    }
}
