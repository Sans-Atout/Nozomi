#[cfg(feature = "analyze")]
mod test_analyse {
    use nozomi::{AnalysisReport, Method, PassInfo, PassKind};
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    #[test]
    fn analyse_default() {
        let method = Method::default();

        let report = method.analyze();

        assert_eq!(report.pass_count, 1);
        assert_eq!(report.passes.len(), 1);

        assert_eq!(
            report,
            AnalysisReport {
                pass_count: 1,
                passes: vec![PassInfo {
                    kind: PassKind::Random
                },],
            }
        )
    }

    #[test]
    fn analyze_has_no_side_effect() {
        let path = PathBuf::from("analyze_no_side_effect.tmp");

        {
            let mut file = File::create(&path).unwrap();
            file.write_all(b"SAFE").unwrap();
            file.sync_all().unwrap();
        }

        let method = Method::PseudoRandom;
        let _ = method.analyze();

        let mut content = Vec::new();
        File::open(&path)
            .unwrap()
            .read_to_end(&mut content)
            .unwrap();

        assert_eq!(content, b"SAFE");

        std::fs::remove_file(path).unwrap();
    }
}
