#[test]
fn license_check() {
    use std::fs;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn assert_file_starts_with_license(entry: &fs::DirEntry) {
        const EXPECTED_FIRST_LINE: &str = "// Copyright 2023 Memgraph Ltd.";

        let file = fs::File::open(entry.path()).expect("should be able to open file for reading");
        let first_line = io::BufReader::new(file).lines().next().unwrap().unwrap();
        assert_eq!(
            first_line,
            EXPECTED_FIRST_LINE,
            "file {:?} did not contain the expected license file",
            entry.path()
        );
    }

    fn visit(dir: &Path) {
        assert!(dir.is_dir());
        for entry in fs::read_dir(dir).expect("should be able to read directory") {
            let entry = entry.expect("should be able to read file in directory");
            let path = entry.path();
            if path.is_dir() {
                visit(&path);
            } else if path.extension().unwrap() == "rs" {
                assert_file_starts_with_license(&entry);
            }
        }
    }

    visit(&std::env::current_dir().unwrap().join("src"));
}
