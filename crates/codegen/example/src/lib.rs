//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! Generated output and input-marker compilation examples.

#[path = "../generated/the_root_created_event.rs"]
mod the_root_created_event;
mod the_root_created_event_gen;
#[path = "../generated/the_root_id.rs"]
mod the_root_id;
mod the_root_id_gen;
#[path = "../generated/the_root_name.rs"]
mod the_root_name;
mod the_root_name_gen;

pub use the_root_created_event::TheRootCreatedEvent;
pub use the_root_created_event_gen::TheRootCreatedEventGen;
pub use the_root_id::TheRootId;
pub use the_root_id_gen::TheRootIdGen;
pub use the_root_name::TheRootName;
pub use the_root_name_gen::TheRootNameGen;
