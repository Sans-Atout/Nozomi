use nozomi::*;

use crate::events::temp_test_file;

struct PanicSink;

impl EventSink for PanicSink {
    fn emit(&mut self, _: DeleteEvent) {
        panic!("sink panic");
    }
}

#[test]
fn sink_panic_does_not_abort_deletion() -> Result<()> {
    let path = temp_test_file("panic_sink");

    let mut sink = PanicSink;

    let request = DeleteRequest::builder()
        .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
        .path(&path)
        .build()?;

    // Should not panic even if sink panics internally
    let result = request.run_with(&mut sink);

    assert!(result.is_ok());

    Ok(())
}
