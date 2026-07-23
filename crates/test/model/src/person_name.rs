//! `person_name` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// `PersonNameError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum PersonNameError {
    /// 表示姓名为空、超长或不满足测试模型约束，调用方应修正输入后重试。
    #[error("person name must contain 1 to 100 characters")]
    Invalid,
}

/// Validated person name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
/// `PersonName` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct PersonName(String);
impl PersonName {
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn new(value: impl Into<String>) -> Result<Self, PersonNameError> {
        let value = value.into();
        if Self::is_valid(Some(&value)) {
            Ok(Self(value))
        } else {
            Err(PersonNameError::Invalid)
        }
    }
    #[must_use]
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn as_str(&self) -> &str {
        &self.0
    }
    #[must_use]
    /// 判断当前值是否满足对应条件；该检查只读取状态，不修改领域对象。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn is_valid(value: Option<&str>) -> bool {
        value.is_none_or(|value| !value.is_empty() && value.chars().count() <= 100)
    }
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn value_of(value: Option<&str>) -> Result<Option<Self>, PersonNameError> {
        value.map(Self::new).transpose()
    }
}
impl std::fmt::Display for PersonName {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
