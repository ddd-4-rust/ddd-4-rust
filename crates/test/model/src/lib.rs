//! 本 crate 提供对应 Java 基线的 Rust 实现，并通过显式模块与定向重导出维护稳定公共 API。
//!
//! Java-compatible domain model shared by serializer parity suites.
#![expect(
    clippy::missing_errors_doc,
    reason = "non-published compatibility model preserves the terse Java bean API"
)]

mod duplicate_vendor_key_exception;
mod event_metadata;
mod model_error;
mod person;
mod person_created_event;
mod person_id;
mod person_id_jsonb_adapter;
mod person_name;
mod person_name_changed_event;
mod person_name_jsonb_adapter;
mod person_not_found_exception;
mod vendor;
mod vendor_created_event;
mod vendor_event_id;
mod vendor_id;
mod vendor_id_jsonb_adapter;
mod vendor_key;
mod vendor_key_jsonb_adapter;
mod vendor_key_str;
mod vendor_key_str_validator;
mod vendor_name;
mod vendor_name_jsonb_adapter;
mod vendor_name_str;
mod vendor_name_str_validator;
mod vendor_ref;

pub use duplicate_vendor_key_exception::DuplicateVendorKeyException;
pub use model_error::ModelError;
pub use person::Person;
pub use person_created_event::{PersonCreatedEvent, PersonCreatedEventBuilder};
pub use person_id::PersonId;
pub use person_id_jsonb_adapter::PersonIdJsonbAdapter;
pub use person_name::{PersonName, PersonNameError};
pub use person_name_changed_event::PersonNameChangedEvent;
pub use person_name_jsonb_adapter::PersonNameJsonbAdapter;
pub use person_not_found_exception::PersonNotFoundException;
pub use vendor::{ConstructorService, Vendor, VendorError};
pub use vendor_created_event::VendorCreatedEvent;
pub use vendor_event_id::VendorEventId;
pub use vendor_id::VendorId;
pub use vendor_id_jsonb_adapter::VendorIdJsonbAdapter;
pub use vendor_key::{VendorKey, VendorKeyError};
pub use vendor_key_jsonb_adapter::VendorKeyJsonbAdapter;
pub use vendor_key_str::VendorKeyStr;
pub use vendor_key_str_validator::VendorKeyStrValidator;
pub use vendor_name::{VendorName, VendorNameError};
pub use vendor_name_jsonb_adapter::VendorNameJsonbAdapter;
pub use vendor_name_str::VendorNameStr;
pub use vendor_name_str_validator::VendorNameStrValidator;
pub use vendor_ref::VendorRef;
