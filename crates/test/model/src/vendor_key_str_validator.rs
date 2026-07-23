//! `vendor_key_str_validator` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
/// Validator for the stable `V99999` vendor-key wire format.
///
/// `VendorKeyStrValidator` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct VendorKeyStrValidator;
impl VendorKeyStrValidator {
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(value: Option<&str>) -> bool {
        value.is_none_or(|value| {
            value.len() == 6
                && value.starts_with('V')
                && value[1..].bytes().all(|byte| byte.is_ascii_digit())
        })
    }
}
