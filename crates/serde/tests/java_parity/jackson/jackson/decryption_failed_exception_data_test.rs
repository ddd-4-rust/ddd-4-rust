//! `decryption_failed_exception_data_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
//! Java source: `jackson/src/test/java/org/fuin/ddd4j/jackson/DecryptionFailedExceptionDataTest.java`.

use super::common::run_scenario;

#[test]
/// 执行 `java_scenario_1` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_1() -> Result<(), Box<dyn std::error::Error>> {
    run_scenario("jackson", "DecryptionFailedExceptionDataTest", 1)
}

#[test]
/// 执行 `java_scenario_2` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_2() -> Result<(), Box<dyn std::error::Error>> {
    run_scenario("jackson", "DecryptionFailedExceptionDataTest", 2)
}

#[test]
/// 执行 `java_scenario_3` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_3() -> Result<(), Box<dyn std::error::Error>> {
    run_scenario("jackson", "DecryptionFailedExceptionDataTest", 3)
}

#[test]
/// 执行 `java_scenario_4` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn java_scenario_4() -> Result<(), Box<dyn std::error::Error>> {
    run_scenario("jackson", "DecryptionFailedExceptionDataTest", 4)
}
