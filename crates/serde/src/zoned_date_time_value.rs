//! `zoned_date_time_value` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// Parsing failure for a Java-compatible zoned date-time value.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// `ZonedDateTimeError` 汇总该领域操作可能产生的结构化状态或错误分支。
/// 调用方应显式匹配需要处理的分支，不能依赖字符串或不可达的 panic 表达业务结果。
pub enum ZonedDateTimeError {
    /// The input did not contain the required trailing IANA zone name.
    #[error("zoned date-time must end with an IANA zone in brackets")]
    MissingZone,
    /// The IANA zone name was unknown.
    #[error("unknown IANA time zone: {0}")]
    InvalidZone(String),
    /// The date-time prefix was invalid.
    #[error("invalid RFC 3339 date-time: {0}")]
    InvalidDateTime(String),
}

/// An instant paired with its original IANA time-zone name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// `ZonedDateTimeValue` 表示与同名 Java 类型对应的 Rust 领域对象。
/// 该对象封装迁移后的状态和不变式；构造、转换及失败语义以公开方法的签名为准。
pub struct ZonedDateTimeValue {
    instant: DateTime<Utc>,
    timezone: Tz,
}

impl ZonedDateTimeValue {
    /// Creates a zoned value without discarding the named time zone.
    #[must_use]
    /// 创建当前类型的新实例，并在构造边界建立该类型要求的不变式。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn new(instant: DateTime<Utc>, timezone: Tz) -> Self {
        Self { instant, timezone }
    }
    /// Creates a compatibility value for callers that only have UTC.
    #[must_use]
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn from_utc(instant: DateTime<Utc>) -> Self {
        Self::new(instant, chrono_tz::UTC)
    }
    /// Returns the absolute UTC instant.
    #[must_use]
    /// 执行 `instant` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn instant(&self) -> &DateTime<Utc> {
        &self.instant
    }
    /// Returns the preserved IANA time-zone identifier.
    #[must_use]
    /// 执行 `timezone` 对应的领域行为，参数和返回值遵循当前类型公开契约。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub const fn timezone(&self) -> Tz {
        self.timezone
    }
    /// Parses Java's `offset-date-time[Area/Location]` wire format.
    ///
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn parse(value: &str) -> Result<Self, ZonedDateTimeError> {
        let (date_time, zone) = value
            .rsplit_once('[')
            .ok_or(ZonedDateTimeError::MissingZone)?;
        let zone = zone
            .strip_suffix(']')
            .ok_or(ZonedDateTimeError::MissingZone)?;
        let timezone = zone
            .parse::<Tz>()
            .map_err(|_| ZonedDateTimeError::InvalidZone(zone.to_owned()))?;
        let instant = DateTime::parse_from_rfc3339(date_time)
            .map_err(|_| ZonedDateTimeError::InvalidDateTime(date_time.to_owned()))?
            .with_timezone(&Utc);
        Ok(Self { instant, timezone })
    }
    /// Formats the instant using the preserved zone and Java's bracketed zone suffix.
    #[must_use]
    /// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    pub fn to_java_string(&self) -> String {
        format!(
            "{}[{}]",
            self.instant.with_timezone(&self.timezone).to_rfc3339(),
            self.timezone
        )
    }
}
impl std::fmt::Display for ZonedDateTimeValue {
    /// 按稳定且可读的格式输出当前值，不改变对象内部状态。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.to_java_string())
    }
}
impl Serialize for ZonedDateTimeValue {
    /// 把领域值转换为稳定线格式，字段名称和空值行为与 Java 黄金样例保持一致。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_java_string())
    }
}
impl<'de> Deserialize<'de> for ZonedDateTimeValue {
    /// 从外部表示恢复领域值；格式错误或未知类型通过返回类型交给调用方处理。
    /// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}
