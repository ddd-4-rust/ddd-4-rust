use ddd_4_rust_codegen_processor::child_locator;
struct Aggregate;
#[child_locator] impl Aggregate { fn invalid(&mut self) -> bool { false } }
fn main() {}
