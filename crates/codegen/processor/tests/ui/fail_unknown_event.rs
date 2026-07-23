use ddd_4_rust_codegen_processor::apply_event;
struct Aggregate;
#[apply_event]
impl Aggregate { fn missing(&mut self, _event: &MissingEvent) -> Result<(), ddd_4_rust_core::AggregateError> { Ok(()) } }
fn main() {}
