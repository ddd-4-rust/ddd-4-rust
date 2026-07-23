//! `ddd4j_utils_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use ddd_4_rust_core::{Ddd4JUtils, EventType};

#[test]
/// 执行 `calculates_the_java_compatible_event_type_checksum` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn calculates_the_java_compatible_event_type_checksum() -> Result<(), Box<dyn std::error::Error>> {
    let event_types = [
        EventType::new("PersonDeletedEvent")?,
        EventType::new("PersonRenamedEvent")?,
        EventType::new("PersonCreatedEvent")?,
    ];
    assert_eq!(Ddd4JUtils::calculate_checksum(&event_types), 1_341_789_591);
    Ok(())
}
