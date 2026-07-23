use ddd_4_rust_codegen_processor::apply_event;
struct Event; struct Aggregate;
#[apply_event]
impl Aggregate { fn invalid(&self, _event: &Event) {} }
fn main() {}
