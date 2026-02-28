use nozomi::*;

use crate::events::MockSink;
use crate::events::temp_test_file;
use pretty_assertions::assert_eq;

#[cfg(not(feature = "error-stack"))]
use nozomi::Result;

#[test]
#[cfg(not(feature = "error-stack"))]
fn events_are_emitted_in_deterministic_order() -> Result<()> {
    let path = temp_test_file("order");

    let mut sink = MockSink::default();

    let request = DeleteRequest::builder()
        .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
        .path(&path)
        .build()?;

    request.run_with(&mut sink)?;

    assert_eq!(
        sink.events,
        vec![
            DeleteEvent::DeletionStarted { path: path.clone() },
            DeleteEvent::EntryOverwritePass {
                path: path.clone(),
                pass: 1,
                total_passes: 1,
            },
            DeleteEvent::EntryDeleted { path: path.clone() },
            DeleteEvent::DeletionFinished { path },
        ]
    );

    Ok(())
}

#[cfg(feature = "error-stack")]
use nozomi::Result;

#[test]
#[cfg(feature = "error-stack")]
fn events_are_emitted_in_deterministic_order() -> Result<()> {
    let path = temp_test_file("order");

    let mut sink = MockSink::default();

    let request = DeleteRequest::builder()
        .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
        .path(&path)
        .build()?;

    request.run_with(&mut sink)?;

    assert_eq!(
        sink.events,
        vec![
            DeleteEvent::DeletionStarted { path: path.clone() },
            DeleteEvent::EntryOverwritePass {
                path: path.clone(),
                pass: 1,
                total_passes: 1,
            },
            DeleteEvent::EntryDeleted { path: path.clone() },
            DeleteEvent::DeletionFinished { path },
        ]
    );

    Ok(())
}
