// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

use std::path::PathBuf;

use ix_trace_rs::trace;

#[test]
#[trace("TC-1642", "FR-660-AC-3")]
fn agent_ix_git_dependencies_name_a_release_tag() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).expect("read workspace manifest");
    for line in text
        .lines()
        .filter(|line| line.contains("git = \"https://github.com/agent-ix/"))
    {
        assert!(
            line.contains("tag = ") || line.contains("version = "),
            "{line}"
        );
        assert!(!line.contains("branch = "), "{line}");
        assert!(!line.contains("rev = "), "{line}");
    }
}
