use ddd_4_rust_codegen_processor::apply_event;
struct Event; struct Aggregate;
#[apply_event]
impl Aggregate {
    fn first(&mut self, _event: &Event) -> Result<(), ddd_4_rust_core::AggregateError> { Ok(()) }
    fn second(&mut self, _event: &Event) -> Result<(), ddd_4_rust_core::AggregateError> { Ok(()) }
}
fn main() {}
