use ddd_4_rust_codegen_processor::{AggregateRootUuidValueObject, EventValueObject, IntegerEntityIdValueObject, StringValueObject};
#[derive(StringValueObject)] struct Name(String);
#[derive(IntegerEntityIdValueObject)] struct PersonId(i32);
#[derive(AggregateRootUuidValueObject)] struct RootId(uuid::Uuid);
#[derive(EventValueObject)] struct Created;
fn main() { let _=Name::new("Ada").to_string(); let _=PersonId::new(1).as_i32(); let _=RootId::new().as_uuid(); let _=Created::EVENT_TYPE; }
