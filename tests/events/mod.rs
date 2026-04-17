use std::path::PathBuf;
#[allow(deprecated)]
mod emission_order;
#[allow(deprecated)]
mod error_behavior;
#[allow(deprecated)]
mod sink_isolation;

use nozomi::{DeleteEvent, EventSink};
pub fn temp_test_file(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("nozomi_test_{name}"));
    std::fs::write(&path, b"test-data").expect("failed to create test file");
    path
}

#[derive(Default)]
pub struct MockSink {
    pub events: Vec<DeleteEvent>,
}

impl EventSink for MockSink {
    fn emit(&mut self, event: DeleteEvent) {
        self.events.push(event);
    }
}
