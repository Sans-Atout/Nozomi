use nozomi::*;
use std::path::PathBuf;

use crate::events::MockSink;
use pretty_assertions::assert_eq;

#[test]
fn deletion_finished_is_emitted_even_on_error() {
	let path = PathBuf::from("this_file_should_not_exist");

	let mut sink = MockSink::default();

	let request = DeleteRequest::builder()
		.path(&path)
		.method(DeleteMethod::BuiltIn(Method::PseudoRandom))
		.build()
		.expect("builder should succeed");

	let result = request.run_with(&mut sink);

	assert!(result.is_err());

	assert_eq!(
		sink.events,
		vec![
			DeleteEvent::DeletionStarted { path: path.clone() },
			DeleteEvent::DeletionFinished { path },
		]
	);
}