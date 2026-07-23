//! `event_vo_template_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `codegen/processor/src/test/java/org/fuin/ddd4j/codegen/processor/EventVOTemplateTest.java`.

use super::common::assert_golden;

#[test]
/// 执行 `java_scenario_1` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_1() -> Result<(), syn::Error> {
    assert_golden("event", 1)
}

#[test]
/// 执行 `java_scenario_2` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_2() -> Result<(), syn::Error> {
    assert_golden("event", 2)
}

#[test]
/// 执行 `java_scenario_3` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_3() -> Result<(), syn::Error> {
    assert_golden("event", 3)
}
