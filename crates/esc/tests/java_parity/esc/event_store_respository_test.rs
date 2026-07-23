//! `event_store_respository_test` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
use super::{
    architecture_test::ESC_DEPENDENCY_BOUNDARY,
    base_test::{StoreMode, VendorAggregate, block_on, boxed_store},
    vendor_repository::VendorRepository,
};
use ddd_4_rust_core::{AggregateError, AggregateRootUuid, EntityId};

#[test]
/// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn reads_existing_aggregate() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(ESC_DEPENDENCY_BOUNDARY, "esc -> core");
    let id = AggregateRootUuid::new("Vendor")?;
    let repository = VendorRepository::new(boxed_store(StoreMode::ReadOne), "Vendor");
    let aggregate = block_on(repository.read(&id, |_| VendorAggregate::new()))?;
    assert_eq!(aggregate.id.entity_type().as_string(), "Vendor");
    Ok(())
}

#[test]
/// 执行 `maps_missing_stream_to_aggregate_not_found` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn maps_missing_stream_to_aggregate_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let id = AggregateRootUuid::new("Vendor")?;
    let repository = VendorRepository::new(boxed_store(StoreMode::ReadMissing), "Vendor");
    let error = block_on(repository.read(&id, |_| VendorAggregate::new())).err();
    assert!(matches!(error, Some(AggregateError::AggregateNotFound(_))));
    Ok(())
}

#[test]
/// 执行对应状态变更，并保持版本、并发或注册约束；失败不会通过生产路径 panic 表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn appends_new_aggregate_and_maps_conflict() -> Result<(), Box<dyn std::error::Error>> {
    let aggregate = VendorAggregate::new()?;
    let successful = VendorRepository::new(boxed_store(StoreMode::AppendOk), "Vendor");
    block_on(successful.add(&aggregate))?;
    let conflicting = VendorRepository::new(boxed_store(StoreMode::AppendConflict), "Vendor");
    assert!(matches!(
        block_on(conflicting.add(&aggregate)).err(),
        Some(AggregateError::AggregateAlreadyExists(_))
    ));
    Ok(())
}

#[test]
/// 执行 `updates_and_deletes_with_expected_version` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn updates_and_deletes_with_expected_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut aggregate = VendorAggregate::new()?;
    aggregate.version = 1;
    let repository = VendorRepository::new(boxed_store(StoreMode::DeleteOk), "Vendor");
    block_on(repository.update(&aggregate))?;
    block_on(repository.delete(&aggregate.id, 1))?;
    Ok(())
}

#[test]
/// 读取或转换对应数据，不隐藏缺失值和底层失败，具体所有权由返回类型表达。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn reads_at_version_and_maps_empty_or_deleted_streams() -> Result<(), Box<dyn std::error::Error>> {
    let id = AggregateRootUuid::new("Vendor")?;
    let repository = VendorRepository::new(boxed_store(StoreMode::ReadOne), "Vendor");
    assert!(std::ptr::eq(
        repository.event_store(),
        repository.event_store()
    ));
    let aggregate = block_on(repository.read_at_version(&id, 0, |_| VendorAggregate::new()))?;
    assert_eq!(aggregate.id.entity_type().as_string(), "Vendor");

    let empty = VendorRepository::new(boxed_store(StoreMode::ReadEmpty), "Vendor");
    assert!(matches!(
        block_on(empty.read_at_version(&id, u32::MAX, |_| VendorAggregate::new())).err(),
        Some(AggregateError::AggregateVersionNotFound(_))
    ));
    let deleted = VendorRepository::new(boxed_store(StoreMode::ReadDeleted), "Vendor");
    assert!(matches!(
        block_on(deleted.read(&id, |_| VendorAggregate::new())).err(),
        Some(AggregateError::AggregateDeleted(_))
    ));
    Ok(())
}

#[test]
/// 执行 `maps_backend_read_failures` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn maps_backend_read_failures() -> Result<(), Box<dyn std::error::Error>> {
    let id = AggregateRootUuid::new("Vendor")?;
    for mode in [
        StoreMode::ReadConnection,
        StoreMode::ReadSerialization,
        StoreMode::ReadOther,
    ] {
        let repository = VendorRepository::new(boxed_store(mode), "Vendor");
        assert!(matches!(
            block_on(repository.read(&id, |_| VendorAggregate::new())).err(),
            Some(AggregateError::Other(_))
        ));
        assert!(matches!(
            block_on(repository.read_at_version(&id, 1, |_| VendorAggregate::new())).err(),
            Some(AggregateError::Other(_))
        ));
    }
    Ok(())
}

#[test]
/// 执行 `maps_update_and_delete_failures` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
fn maps_update_and_delete_failures() -> Result<(), Box<dyn std::error::Error>> {
    let mut aggregate = VendorAggregate::new()?;
    aggregate.version = 1;
    for (mode, expected) in [
        (StoreMode::AppendConflict, "version"),
        (StoreMode::AppendMissing, "not found"),
        (StoreMode::AppendDeleted, "deleted"),
        (StoreMode::AppendConnection, "offline"),
    ] {
        let repository = VendorRepository::new(boxed_store(mode), "Vendor");
        let message = block_on(repository.update(&aggregate))
            .err()
            .map(|error| error.to_string())
            .ok_or("expected update error")?;
        assert!(message.to_lowercase().contains(expected));
    }
    let conflict = VendorRepository::new(boxed_store(StoreMode::DeleteConflict), "Vendor");
    assert!(matches!(
        block_on(conflict.delete(&aggregate.id, 1)).err(),
        Some(AggregateError::AggregateVersionConflict(_))
    ));
    let connection = VendorRepository::new(boxed_store(StoreMode::DeleteConnection), "Vendor");
    assert!(matches!(
        block_on(connection.delete(&aggregate.id, 1)).err(),
        Some(AggregateError::Other(_))
    ));
    Ok(())
}
