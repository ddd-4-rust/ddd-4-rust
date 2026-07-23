use ddd_4_rust_codegen_processor::{apply_event, EventValueObject};
#[derive(EventValueObject)] struct Created<T>(T);
struct Aggregate<T>(T);
#[apply_event]
impl<T: 'static> Aggregate<T> { fn created(&mut self, _event: &Created<T>) -> Result<(), ddd_4_rust_core::AggregateError> { Ok(()) } }
fn main() {}
