#[cfg(feature = "dry-run")]
use nozomi::{DeleteEvent, EventSink};

#[cfg(feature = "dry-run")]
#[derive(Default)]
struct TestSink {
    events: Vec<DeleteEvent>,
}

#[cfg(feature = "dry-run")]
impl EventSink for TestSink {
    fn emit(&mut self, event: DeleteEvent) {
        self.events.push(event);
    }
}

#[cfg(all(feature = "dry-run", not(feature = "error-stack")))]
mod test_dry_run {
    use super::TestSink;
    use nozomi::{DeleteEvent, DeleteMethod, DeleteRequest, Method, Result};
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn dry_run_does_not_modify_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_test.tmp");

        let original = b"HELLO_WORLD";
        {
            let mut file = File::create(&path).unwrap();
            file.write_all(original).unwrap();
            file.sync_all().unwrap();
        }

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run()?;

        let mut content = Vec::new();
        File::open(&path)
            .unwrap()
            .read_to_end(&mut content)
            .unwrap();
        assert_eq!(content, original);

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_does_not_remove_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_remove.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run()?;

        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    #[cfg(feature = "verify")]
    fn dry_run_skips_verify() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_verify.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        let result = request.run();

        assert!(result.is_ok());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_emits_events() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_event.tmp");
        File::create(&path).unwrap();

        let mut sink = TestSink::default();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run_with(&mut sink)?;

        assert_eq!(
            sink.events,
            vec![
                #[cfg(feature = "dry-run")]
                DeleteEvent::DryRunStarted { path: path.clone() },
                DeleteEvent::DeletionStarted { path: path.clone() },
                DeleteEvent::EntryOverwritePass {
                    path: path.clone(),
                    pass: 1,
                    total_passes: 1,
                },
                #[cfg(feature = "verify")]
                DeleteEvent::VerificationStarted { path: path.clone() },
                #[cfg(feature = "verify")]
                DeleteEvent::VerificationCompleted { path: path.clone() },
                DeleteEvent::EntryDeleted { path: path.clone() },
                DeleteEvent::DeletionFinished { path: path.clone() },
                #[cfg(feature = "dry-run")]
                DeleteEvent::DryRunCompleted { path: path.clone() },
            ]
        );

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_empty_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_empty.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        let result = request.run();

        assert!(result.is_ok());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn normal_run_modifies_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_normal_run.tmp");

        {
            let mut file = File::create(&path).unwrap();
            file.write_all(b"ABC").unwrap();
            file.sync_all().unwrap();
        }

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .build()?;

        request.run()?;

        assert!(!path.exists());
        Ok(())
    }
}

#[cfg(all(feature = "dry-run", feature = "error-stack"))]
#[allow(deprecated)]
mod test_dry_run {
    use super::TestSink;
    use nozomi::{DeleteEvent, DeleteMethod, DeleteRequest, Method, Result};
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn dry_run_does_not_modify_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_test.tmp");

        let original = b"HELLO_WORLD";
        {
            let mut file = File::create(&path).unwrap();
            file.write_all(original).unwrap();
            file.sync_all().unwrap();
        }

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run()?;

        let mut content = Vec::new();
        File::open(&path)
            .unwrap()
            .read_to_end(&mut content)
            .unwrap();
        assert_eq!(content, original);

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_does_not_remove_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_remove.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run()?;

        assert!(path.exists());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    #[cfg(feature = "verify")]
    fn dry_run_skips_verify() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_verify.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        let result = request.run();

        assert!(result.is_ok());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_emits_events() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_event.tmp");
        File::create(&path).unwrap();

        let mut sink = TestSink::default();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        request.run_with(&mut sink)?;

        assert_eq!(
            sink.events,
            vec![
                #[cfg(feature = "dry-run")]
                DeleteEvent::DryRunStarted { path: path.clone() },
                DeleteEvent::DeletionStarted { path: path.clone() },
                DeleteEvent::EntryOverwritePass {
                    path: path.clone(),
                    pass: 1,
                    total_passes: 1,
                },
                #[cfg(feature = "verify")]
                DeleteEvent::VerificationStarted { path: path.clone() },
                #[cfg(feature = "verify")]
                DeleteEvent::VerificationCompleted { path: path.clone() },
                DeleteEvent::EntryDeleted { path: path.clone() },
                DeleteEvent::DeletionFinished { path: path.clone() },
                #[cfg(feature = "dry-run")]
                DeleteEvent::DryRunCompleted { path: path.clone() },
            ]
        );

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn dry_run_empty_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_run_empty.tmp");
        File::create(&path).unwrap();

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .dry_run(true)
            .build()?;

        let result = request.run();

        assert!(result.is_ok());

        std::fs::remove_file(path).unwrap();
        Ok(())
    }

    #[test]
    fn normal_run_modifies_file() -> Result<()> {
        let mut path = std::env::temp_dir();
        path.push("nozomi_test_dry_normal_run.tmp");

        {
            let mut file = File::create(&path).unwrap();
            file.write_all(b"ABC").unwrap();
            file.sync_all().unwrap();
        }

        let request = DeleteRequest::builder()
            .method(DeleteMethod::BuiltIn(Method::PseudoRandom))
            .path(&path)
            .build()?;

        request.run()?;

        assert!(!path.exists());
        Ok(())
    }
}
