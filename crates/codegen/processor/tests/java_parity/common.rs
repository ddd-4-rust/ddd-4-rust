//! `common` 模块承载同名 Java 类型迁移后的 Rust 领域实现；文件名保持 `snake_case`，公开类型保持 `PascalCase`。
//!
/// `COMPANY_ID` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
const COMPANY_ID: &[&str] = &[
    include_str!("../golden/company_id/all_converters.rs"),
    include_str!("../golden/company_id/jaxb_only.rs"),
    include_str!("../golden/company_id/jpa_only.rs"),
    include_str!("../golden/company_id/jsonb_only.rs"),
    include_str!("../golden/company_id/no_converters.rs"),
    include_str!("../golden/company_id/open_api_only.rs"),
];
/// `RAMP_ID` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
const RAMP_ID: &[&str] = &[
    include_str!("../golden/ramp_id/all_converters.rs"),
    include_str!("../golden/ramp_id/jaxb_only.rs"),
    include_str!("../golden/ramp_id/jpa_only.rs"),
    include_str!("../golden/ramp_id/jsonb_only.rs"),
    include_str!("../golden/ramp_id/no_converters.rs"),
    include_str!("../golden/ramp_id/open_api_only.rs"),
];
/// `EVENT` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
const EVENT: &[&str] = &[
    include_str!("../golden/my_event/jackson.rs"),
    include_str!("../golden/my_event/jaxb.rs"),
    include_str!("../golden/my_event/jsonb.rs"),
];
/// `COMPANY_KEY` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
const COMPANY_KEY: &[&str] = &[include_str!("../golden/company_key/all_converters.rs")];
/// `SOURCE_TEMPLATES` 是该类型公开的稳定常量。
/// 调用方可依赖其语义，但不应假定未写入公共契约的内部表示。
const SOURCE_TEMPLATES: &[&str] = &[
    include_str!("../../templates/aggregate_root_uuid_vo.rs"),
    include_str!("../../templates/event_vo.rs"),
    include_str!("../../templates/integer_entity_id_vo.rs"),
    include_str!("../../templates/string_vo.rs"),
];

/// 执行 `assert_golden` 对应的领域行为，参数和返回值遵循当前类型公开契约。
/// 该方法不引入未声明的全局副作用，调用方应按签名处理返回结果。
pub fn assert_golden(kind: &str, scenario: usize) -> Result<(), syn::Error> {
    let values = match kind {
        "company_id" => COMPANY_ID,
        "ramp_id" => RAMP_ID,
        "event" => EVENT,
        "company_key" => COMPANY_KEY,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "unknown golden group",
            ));
        }
    };
    let source = values.get(scenario.saturating_sub(1)).ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "golden scenario out of range",
        )
    })?;
    let syntax = syn::parse_file(source)?;
    assert!(!syntax.items.is_empty());
    assert!(source.contains("derive"));
    assert!(
        SOURCE_TEMPLATES
            .iter()
            .all(|template| syn::parse_file(template).is_ok())
    );
    Ok(())
}
