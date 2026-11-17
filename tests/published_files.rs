// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

use std::path::{Path, PathBuf};

use filament_specification::{published_file, published_files};
use ix_trace_rs::trace;

fn files_under(directory: &Path, root: &Path, files: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(directory).expect("read published directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            files_under(&path, root, files);
        } else if path.is_file() {
            let relative = path
                .strip_prefix(root)
                .expect("file stays under repository root")
                .to_str()
                .expect("published paths are UTF-8")
                .replace('\\', "/");
            files.push((relative, std::fs::read(path).expect("read published file")));
        }
    }
}

#[test]
#[trace("TC-1640", "FR-660-AC-1")]
fn published_file_returns_every_proposal_byte_and_lists_fixture_paths_in_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected = Vec::new();
    files_under(&root.join("proposals"), &root, &mut expected);
    expected.sort_by(|left, right| left.0.cmp(&right.0));

    for (path, bytes) in &expected {
        assert_eq!(published_file(path), Some(bytes.as_slice()), "{path}");
    }
    assert!(published_file("spec/functional/ecosystem/FR-660-version-the-wire-the-published-files-and-the-consumer-facades.md").is_none());
    assert!(published_file("proposals/checked-package-v2/fixtures/absent.json").is_none());
    assert!(published_file("spec/test-cases/absent.md").is_none());

    let listed: Vec<_> = published_files("proposals/checked-package-v2/fixtures/")
        .map(|(path, _)| path.to_owned())
        .collect();
    let mut fixture_files: Vec<_> = expected
        .iter()
        .filter(|(path, _)| path.starts_with("proposals/checked-package-v2/fixtures/"))
        .map(|(path, _)| path.clone())
        .collect();
    fixture_files.sort();
    assert_eq!(listed, fixture_files);
}

#[test]
#[trace("TC-1640", "FR-660-AC-6")]
fn published_file_returns_test_case_bytes_and_lists_paths_in_order() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected = Vec::new();
    files_under(&root.join("spec/test-cases"), &root, &mut expected);
    expected.sort_by(|left, right| left.0.cmp(&right.0));

    for (path, bytes) in &expected {
        assert_eq!(published_file(path), Some(bytes.as_slice()), "{path}");
    }

    let tc210 = "spec/test-cases/TC-210-finite-simulation-reproducibility.md";
    let tc210_bytes = std::fs::read(root.join(tc210)).expect("read TC-210");
    assert_eq!(published_file(tc210), Some(tc210_bytes.as_slice()));
    let listed: Vec<_> = published_files("spec/test-cases/")
        .map(|(path, _)| path.to_owned())
        .collect();
    let mut case_files: Vec<_> = expected
        .iter()
        .filter(|(path, _)| path.starts_with("spec/test-cases/"))
        .map(|(path, _)| path.clone())
        .collect();
    case_files.sort();
    assert_eq!(listed, case_files);
}
