# Serde 序列化边界设计

**日期**: 2026-07-23
**作用范围**: `ddd-4-rust-serde`。
**类型**: 长期架构事实。

---

## 1. 设计原则

Java 的 Jackson、JSON-B、JAXB 三套来源文件按一对一登记在
`docs/migration/file_mapping.csv` 中（共 81+77+76=234 个文件），
但 Rust 生产实现统一建立在 Serde 上：

| Java 技术 | Rust 实现 | 说明 |
|---|---|---|
| Jackson | `serde` + `serde_json` | 完整替换 Jackson 运行时 |
| JSON-B | `serde` + `serde_json` | 保留 JSON-B 线格式兼容入口 |
| JAXB | `serde` + `quick-xml` | `xml` feature 可选启用 |

## 2. crate 内部布局

```text
crates/serde/src/
├── lib.rs
├── json.rs
├── json/
│   ├── serde.rs        # 主 JSON API
│   ├── serde/
│   ├── jackson.rs      # 弃用兼容门面
│   ├── jackson/
│   ├── jsonb.rs        # JSON-B 线格式门面
│   └── jsonb/
├── xml.rs
└── xml/
    ├── jaxb.rs         # JAXB XML API
    └── jaxb/
```

`json` 是默认 feature，`xml` 为可选 feature。

## 3. API 入口

主入口为 `ddd_4_rust_serde::json::serde`。
`json::jackson` 仅作为标记弃用的 Java 命名兼容门面，所有行为委托给
Serde 实现，**不引入** Jackson 依赖。

`ddd_4_rust_serde::AbstractEvent` 等既有入口通过定向 `pub use` 保持
兼容；任何 glob 重导出（`pub use *`）一律禁止。

## 4. 自定义适配器

`serde` 必须提供的自定义适配器：

- `EntityIdAdapter`：强类型 ID 序列化，保持 typed-id 的 `as_string()` 形式。
- `EntityIdPathAdapter`：实体 ID 路径的字符串编码与解码。
- `AggregateVersionAdapter`：聚合版本号（i64）的人类可读序列化。
- `DddSerdeModule`：聚合上述适配器的 `serde::Module`，统一注册到
  `serde_json` 自定义适配器链。

适配器不得修改 typed ID、聚合版本、异常数据、事件字段名、空值策略、
相关/因果 ID、XML 标签和事件时间等 Java 0.7.0 线格式规则。

## 5. 线格式不变性

迁移过程必须严格保持：

- typed ID（如 `PersonId`）的字符串形式；
- 聚合版本号格式；
- 异常数据的 `data-type` 字段；
- 事件字段名（snake_case 等价于 Java camelCase 映射）；
- 空值（`null` ↔ `Option::None`）；
- 相关/因果 ID（correlation_id / causation_id）；
- XML 标签名；
- 事件时间（含 IANA 时区）。

任意一项偏离都会破坏与现有 Java 实现的跨语言事件互通能力。

## 6. 异常序列化

异常数据 `ExceptionData` 必须支持：

- 序列化字段 `data-type`、`message`、`params`；
- 嵌套异常（`cause` 链）保留原顺序；
- 自定义参数类型通过 `Into<JsonValue>` 暴露。

不得把异常消息改写为人类可读文本后丢失原 `data-type`；下游解析器
依赖 `data-type` 区分异常种类。