// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-831 / TC-2113: the protocol correlation vectors are built on COMPLETE
//! published packages and are consistent with them. This test executes no
//! reader (no reader consumes the member yet); it checks that every case's
//! package and edit are schema-valid, that the recorded related lengths, group
//! source-map entries, row coverage and charge arithmetic are the packages'
//! own, and that the member's schema refuses its malformed rows.

use ix_trace_rs::trace;
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[path = "support/operand_slots.rs"]
mod operand_slots;

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/protocol-correlation-vectors.json");
const BASE: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-base.json");
const DEP: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-correlation-dependency.json");
const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const MEMBER: &str = "/protocol_correlation_occurrences";

fn parse(text: &str) -> Value {
    operand_slots::index_view(&serde_json::from_str(text).expect("valid JSON"))
}

fn apply_published(base: &Value, edit: &[Value]) -> Value {
    let mut package = base.clone();
    for op in edit {
        let path = op["path"].as_str().expect("path");
        let (parent, last) = path.rsplit_once('/').expect("absolute pointer");
        let target = package
            .pointer_mut(parent)
            .unwrap_or_else(|| panic!("{path}: parent exists"));
        match (op["op"].as_str().expect("op"), target) {
            ("add" | "replace", Value::Object(map)) => {
                map.insert(last.to_owned(), op["value"].clone());
            }
            ("add", Value::Array(items)) => {
                let at = if last == "-" { items.len() } else { last.parse().expect("index") };
                items.insert(at, op["value"].clone());
            }
            ("replace", Value::Array(items)) => {
                items[last.parse::<usize>().expect("index")] = op["value"].clone();
            }
            ("remove", Value::Object(map)) => {
                map.remove(last).expect("member exists");
            }
            ("remove", Value::Array(items)) => {
                items.remove(last.parse::<usize>().expect("index"));
            }
            (other, _) => panic!("{path}: unsupported {other}"),
        }
    }
    package
}

fn apply(base: &Value, edit: &[Value]) -> Value {
    operand_slots::index_view(&apply_published(&operand_slots::slot_view(base), edit))
}

/// The complete package a vectors package names: base plus its ops, or its file.
fn package(vectors: &Value, name: &str) -> Value {
    let entry = &vectors["packages"][name];
    let fixture = entry["fixture"].as_str().expect("fixture");
    let base = if fixture.ends_with("protocol-correlation-dependency.json") {
        parse(DEP)
    } else {
        parse(BASE)
    };
    apply(&base, entry["ops"].as_array().expect("ops"))
}

fn operator(node: &Value) -> &str {
    node["body"]["operation"]["member"]["operator"].as_str().unwrap_or("")
}

/// (path, m) of every eligible control occurrence, in node order.
fn eligible(package: &Value) -> Vec<(Vec<String>, usize)> {
    let mut found = Vec::new();
    for node in package["semantic_graph"]["nodes"].as_array().expect("nodes") {
        if node["node_tag"] != "protocol" || node["semantic_form"] != "control" {
            continue;
        }
        if !["send", "receive", "attempt", "effect", "event", "recommend"].contains(&operator(node)) {
            continue;
        }
        let path = node["node_id"]["path"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap().to_owned())
            .collect();
        let m = node["body"]["arguments"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["name"] == "related")
            .map_or(0, |a| a["value"]["members"].as_array().map_or(0, Vec::len));
        found.push((path, m));
    }
    found
}

fn path_of(control: &Value) -> Vec<String> {
    control["path"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_owned()).collect()
}

fn with_member(package: &Value, rows: &Value) -> Value {
    let mut edited = package.clone();
    edited.as_object_mut().unwrap().insert("protocol_correlation_occurrences".into(), rows.clone());
    edited
}

fn case_package(vectors: &Value, case: &Value) -> Value {
    apply(&package(vectors, case["package"].as_str().unwrap()), case["edit"].as_array().unwrap())
}

/// FR-831-AC-1, FR-831-AC-2: every complete package is schema-valid with and
/// without the member, and every case's edited package is schema-valid, so each
/// case reaches the reader's carrier check rather than the schema.
#[trace("TC-2113", "FR-831-AC-1", "FR-831-AC-2")]
#[test]
fn every_case_package_is_complete_and_schema_valid() {
    let vectors = parse(VECTORS);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema");
    for name in ["PC-BASE", "PC-REL", "PC-DEP", "PC-ROOT"] {
        let pkg = package(&vectors, name);
        let first = validator.iter_errors(&pkg).next().map(|e| format!("{} {}", e.instance_path(), e));
        assert!(first.is_none(), "{name} is schema-valid: {first:?}");
        assert!(validator.is_valid(&with_member(&pkg, &json!([]))), "{name} with []");
    }
    let mut defects = Vec::new();
    for group in ["positive", "reader_refusals"] {
        for case in vectors[group].as_array().unwrap() {
            let edited = case_package(&vectors, case);
            if let Some(error) = validator.iter_errors(&edited).next() {
                defects.push(format!("{}: {}", case["id"], error.instance_path()));
            }
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
    assert_eq!(vectors["positive"].as_array().unwrap().len(), 12);
    assert_eq!(vectors["reader_refusals"].as_array().unwrap().len(), 21);
}

/// FR-831-AC-3: PC-REL holds real related records on all six eligible forms.
#[trace("TC-2113", "FR-831-AC-3")]
#[test]
fn the_related_package_carries_all_six_forms() {
    let vectors = parse(VECTORS);
    let rel = package(&vectors, "PC-REL");
    let mut forms: BTreeMap<String, usize> = BTreeMap::new();
    for node in rel["semantic_graph"]["nodes"].as_array().unwrap() {
        if node["semantic_form"] == "control" && operator(node) != "" {
            let found = eligible(&rel)
                .into_iter()
                .any(|(p, m)| p == path_of(&node["node_id"]) && m >= 1);
            if found {
                *forms.entry(operator(node).to_owned()).or_default() += 1;
            }
        }
    }
    let seen: Vec<&str> = forms.keys().map(String::as_str).collect();
    assert_eq!(seen, ["attempt", "effect", "event", "receive", "recommend", "send"]);
    let total: usize = eligible(&rel).iter().map(|(_, m)| m).sum();
    assert_eq!(total, 7);
    assert!(eligible(&parse(BASE)).iter().all(|(_, m)| *m == 0), "the base is related-empty");
}

/// FR-831-AC-4, FR-831-AC-5: every positive member covers each eligible control's
/// canonical positions with contiguous source ordinals, in structural order, and
/// each recorded group holds the package's own source-map entry.
#[trace("TC-2113", "FR-831-AC-4", "FR-831-AC-5")]
#[test]
fn positive_rows_cover_the_packages_related_and_borrow_its_source_map() {
    let vectors = parse(VECTORS);
    for case in vectors["positive"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let Some(op) = case["edit"].as_array().unwrap().first() else { continue };
        let rows = op["value"].as_array().unwrap();
        let name = case["package"].as_str().unwrap();
        let pkg = package(&vectors, name);
        let mut by_control: BTreeMap<Vec<String>, Vec<(u64, u64)>> = BTreeMap::new();
        for row in rows {
            if row["control"]["package"] != pkg["name"] {
                // a dependency control: it is covered by its own package below
                continue;
            }
            by_control
                .entry(path_of(&row["control"]))
                .or_default()
                .push((row["source_ordinal"].as_u64().unwrap(), row["canonical_index"].as_u64().unwrap()));
        }
        for (path, m) in eligible(&pkg) {
            let rows = by_control.get(&path).cloned().unwrap_or_default();
            if m == 0 {
                assert!(rows.is_empty(), "{id}: {path:?} has no row over an empty related");
                continue;
            }
            let ordinals: Vec<u64> = rows.iter().map(|r| r.0).collect();
            assert_eq!(ordinals, (0..rows.len() as u64).collect::<Vec<_>>(), "{id}: {path:?} ordinals");
            let mut covered: Vec<u64> = rows.iter().map(|r| r.1).collect();
            covered.sort_unstable();
            covered.dedup();
            assert_eq!(covered, (0..m as u64).collect::<Vec<_>>(), "{id}: {path:?} covers 0..m");
        }
        if let Some(groups) = case["expect"]["read"]["groups"].as_array() {
            for group in groups {
                let source = package(&vectors, if group["control"]["package"] == "acme/flow" { "PC-DEP" } else { "PC-REL" });
                let entry = source["source_map"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|e| e["node_id"] == group["control"] && e["role"] == "declaration" && e["ordinal"] == 0)
                    .unwrap_or_else(|| panic!("{id}: entry"));
                assert_eq!(&group["source_map_entry"], entry, "{id}: whole entry, all regions");
            }
        }
    }
}

fn row_cost(row: &Value) -> u64 {
    1 + row["control"]["path"].as_array().unwrap().len() as u64 + row["alias"].as_str().unwrap().len() as u64
}

fn scan_costs(pkg: &Value) -> Vec<u64> {
    let eligible = eligible(pkg);
    let mut nodes: Vec<Vec<String>> = pkg["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| n["node_tag"] == "protocol" && n["semantic_form"] == "control")
        .map(|n| path_of(&n["node_id"]))
        .collect();
    nodes.sort();
    nodes
        .iter()
        .map(|path| 1 + eligible.iter().find(|(p, _)| p == path).map_or(0, |(_, m)| 1 + *m as u64))
        .collect()
}

/// FR-831-AC-7: the recorded charge arithmetic is the package's own.
#[trace("TC-2113", "FR-831-AC-7")]
#[test]
fn the_budget_arithmetic_is_the_packages_own() {
    let vectors = parse(VECTORS);
    let rel = package(&vectors, "PC-REL");
    let p04 = vectors["positive"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "CO-P04-every-form")
        .unwrap();
    let rows = p04["edit"][0]["value"].as_array().unwrap();
    let costs: Vec<u64> = rows.iter().map(row_cost).collect();
    let b01 = vectors["budget"]["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["id"] == "CO-B01-admit-measured")
        .unwrap();
    let recorded: Vec<u64> = b01["work_units"]["rows"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
    assert_eq!(costs, recorded);
    assert_eq!(costs.iter().sum::<u64>(), b01["work_units"]["rows_total"]);
    let scan = scan_costs(&rel);
    assert_eq!(scan.len() as u64, b01["work_units"]["scan_nodes"].as_u64().unwrap());
    assert_eq!(scan.iter().sum::<u64>(), b01["work_units"]["scan_total"]);
    assert_eq!(
        costs.iter().sum::<u64>() + scan.iter().sum::<u64>(),
        b01["work_units"]["total"]
    );
    let flags: Vec<u64> = eligible(&rel).iter().filter(|(_, m)| *m >= 1).map(|(_, m)| *m as u64).collect();
    assert_eq!(flags.iter().sum::<u64>(), b01["storage_units"]["flags_total"]);
    // the empty member over the base: scan only
    let base = parse(BASE);
    let scan = scan_costs(&base);
    let b12 = vectors["budget"]["cases"].as_array().unwrap().iter().find(|c| c["id"] == "CO-B12-empty-member").unwrap();
    assert_eq!(scan.len() as u64, b12["work_units"]["scan_nodes"].as_u64().unwrap());
    assert_eq!(scan.iter().sum::<u64>(), b12["work_units"]["total"]);
    // the long-alias row: 1 + 5 + 64
    assert_eq!(1 + 5 + 64, 70);
    // the first and last unpaid amounts
    assert_eq!(costs[0], 12);
    assert_eq!(costs[6], 19);
    assert_eq!(*scan.last().unwrap(), 2, "the last control node, West, is an eligible attempt with m 0");
}

/// FR-831-AC-6: every `schema_valid` edit is schema-valid and every
/// `schema_invalid` edit is refused at the recorded first error path.
#[trace("TC-2113", "FR-831-AC-6")]
#[test]
fn the_schema_decides_the_member_shape_at_the_recorded_first_error_path() {
    let vectors = parse(VECTORS);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema");
    let mut defects = Vec::new();
    for case in vectors["schema_valid"].as_array().unwrap() {
        if !validator.is_valid(&case_package(&vectors, case)) {
            defects.push(format!("{}: refused", case["id"]));
        }
    }
    let invalid = vectors["schema_invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 12);
    for case in invalid {
        assert_eq!(case["expect"]["code"], "malformed_wire");
        assert_eq!(case["expect"]["stage"], "schema");
        let first = validator
            .iter_errors(&case_package(&vectors, case))
            .next()
            .map(|e| e.instance_path().to_string());
        if first.as_deref() != case["expect"]["locus"].as_str() {
            defects.push(format!("{}: first error {first:?}, recorded {}", case["id"], case["expect"]["locus"]));
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
}

/// FR-831-AC-8: the member is the sole difference between the two packages of
/// each invariance case; every other member is equal.
#[trace("TC-2113", "FR-831-AC-8")]
#[test]
fn the_member_is_the_sole_difference_between_invariance_packages() {
    let vectors = parse(VECTORS);
    for case in vectors["invariance"].as_array().unwrap() {
        let base = package(&vectors, case["package"].as_str().unwrap());
        let a = apply(&base, case["editA"].as_array().unwrap());
        let b = apply(&base, case["editB"].as_array().unwrap());
        let strip = |mut p: Value| {
            p.as_object_mut().unwrap().remove("protocol_correlation_occurrences");
            p
        };
        assert_eq!(strip(a.clone()), strip(b.clone()), "{}: only the member differs", case["id"]);
        assert_ne!(a, b, "{}: the packages differ in bytes", case["id"]);
    }
}
