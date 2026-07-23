use ddd_4_rust_codegen_processor::apply_event;
struct Created;
struct Aggregate;
#[apply_event]
impl Aggregate { fn created(&mut self, _event: &Created) -> Result<(), ddd_4_rust_core::AggregateError> { Ok(()) } }
fn main() {}
