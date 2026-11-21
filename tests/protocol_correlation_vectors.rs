// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-831 / TC-2113 qualification of the vectors, as DATA. No reader consumes
//! `protocol_correlation_occurrences` yet (the existing strict reader refuses it
//! as `unknown_member`), so nothing here executes a reader decision, an
//! admission or a refusal of the carrier, and no test is tagged with a criterion
//! that states one (FR-831-AC-1 to AC-5, AC-7, AC-8, AC-12 stay pending the IR
//! reader, the FSL producer and a consumer). What these tests execute is:
//!
//! - the complete packages and every case edit against the V2 schema;
//! - the vectors' rows against their package: full structural key order,
//!   per-group ordinals, `source_map` lookup by the full key, coverage of every
//!   canonical position, all derived from the package and never hard-coded;
//! - a charge model recomputed from the package and compared with every budget
//!   expectation (counter, first unpaid amount, locus, limit); it is a model of
//!   the contract's charges, not a reader.

use ix_trace_rs::trace;
use serde_json::{Value, json};
use std::cmp::Ordering;

#[path = "support/operand_slots.rs"]
mod operand_slots;

const VECTORS: &str =
    include_str!("../proposals/checked-package-v2/protocol-correlation-vectors.json");
const BASE: &str = include_str!("../proposals/checked-package-v2/fixtures/protocol-node-base.json");
const DEP: &str =
    include_str!("../proposals/checked-package-v2/fixtures/protocol-correlation-dependency.json");
const SCHEMA: &str = include_str!("../proposals/checked-package-v2/schema.json");
const MEMBER: &str = "/protocol_correlation_occurrences";
const ELIGIBLE: [&str; 6] = ["send", "receive", "attempt", "effect", "event", "recommend"];

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

fn case_package(vectors: &Value, case: &Value) -> Value {
    apply(&package(vectors, case["package"].as_str().unwrap()), case["edit"].as_array().unwrap())
}

/// The member rows a case's edit adds, if it adds the member as an array.
fn rows_of(case: &Value) -> Option<Vec<Value>> {
    case["edit"]
        .as_array()?
        .iter()
        .find(|op| op["op"] == "add" && op["path"] == MEMBER)
        .and_then(|op| op["value"].as_array().cloned())
}

fn cases<'a>(vectors: &'a Value, group: &str) -> &'a Vec<Value> {
    vectors[group].as_array().unwrap_or_else(|| panic!("{group}"))
}

fn case_named<'a>(vectors: &'a Value, group: &str, id: &str) -> &'a Value {
    cases(vectors, group).iter().find(|c| c["id"] == id).unwrap_or_else(|| panic!("{id}"))
}

// ---- package facts, all read from the package ----

fn text_len(segment: &Value) -> u64 {
    segment.as_str().map_or_else(|| segment.to_string().len() as u64, |s| s.len() as u64)
}

/// The byte length of a NodeRef's package text and every path segment text
/// (FR-831 "Reader checks": the bytes a row charge or a node charge pays for).
fn ref_bytes(reference: &Value) -> u64 {
    reference["package"].as_str().unwrap().len() as u64
        + reference["path"].as_array().unwrap().iter().map(text_len).sum::<u64>()
}

fn operator(node: &Value) -> &str {
    node["body"]["operation"]["member"]["operator"].as_str().unwrap_or("")
}

fn is_control(node: &Value) -> bool {
    node["node_tag"] == "protocol" && node["semantic_form"] == "control"
}

fn related_len(node: &Value) -> u64 {
    node["body"]["arguments"]
        .as_array()
        .and_then(|a| a.iter().find(|a| a["name"] == "related"))
        .and_then(|a| a["value"]["members"].as_array())
        .map_or(0, |m| m.len() as u64)
}

fn declaration_occurrences(node: &Value) -> u64 {
    node["occurrences"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["role"] == "declaration")
        .count() as u64
}

fn is_eligible(node: &Value) -> bool {
    is_control(node) && ELIGIBLE.contains(&operator(node))
}

/// Total order of FR-322 NodeRef: package, then path segment by segment (a
/// shorter path first), then ordinal.
fn cmp_ref(a: &Value, b: &Value) -> Ordering {
    let seg = |r: &Value| -> Vec<String> {
        r["path"].as_array().unwrap().iter().map(|s| s.as_str().map_or_else(|| s.to_string(), str::to_owned)).collect()
    };
    a["package"]
        .as_str()
        .cmp(&b["package"].as_str())
        .then_with(|| seg(a).cmp(&seg(b)))
        .then_with(|| a["ordinal"].as_u64().cmp(&b["ordinal"].as_u64()))
}

/// The full row key of FR-831: control NodeRef, occurrence ordinal (every row
/// that reaches the key has role `declaration`), then `source_ordinal`.
fn cmp_key(a: &Value, b: &Value) -> Ordering {
    cmp_ref(&a["control"], &b["control"])
        .then_with(|| a["occurrence"]["ordinal"].as_u64().cmp(&b["occurrence"]["ordinal"].as_u64()))
        .then_with(|| a["source_ordinal"].as_u64().cmp(&b["source_ordinal"].as_u64()))
}

fn node_ref(node: &Value) -> &Value {
    &node["node_id"]
}

fn find_node<'a>(pkg: &'a Value, control: &Value) -> Option<&'a Value> {
    if control["package"] != pkg["name"] {
        return None;
    }
    pkg["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|n| node_ref(n) == control)
}

fn source_entry<'a>(pkg: &'a Value, control: &Value, occurrence: &Value) -> Option<&'a Value> {
    pkg["source_map"].as_array()?.iter().find(|e| {
        e["node_id"] == *control && e["role"] == occurrence["role"] && e["ordinal"] == occurrence["ordinal"]
    })
}

/// Protocol/control nodes in ascending structural node-id order.
fn control_nodes(pkg: &Value) -> Vec<&Value> {
    let mut nodes: Vec<&Value> =
        pkg["semantic_graph"]["nodes"].as_array().unwrap().iter().filter(|n| is_control(n)).collect();
    nodes.sort_by(|a, b| cmp_ref(node_ref(a), node_ref(b)));
    nodes
}

/// Data consistency of rows against their package; not a reader. Rows are in
/// strictly ascending full-key order, each joins a node of the package, an
/// eligible declaration occurrence with a `source_map` entry of the full key and
/// a canonical position below `m`, ordinals run 0..k-1 per (control,
/// occurrence), and every canonical position of every eligible control is named.
fn check_rows(pkg: &Value, rows: &[Value]) -> Result<(), String> {
    let mut previous: Option<&Value> = None;
    for (i, row) in rows.iter().enumerate() {
        let node = find_node(pkg, &row["control"]).ok_or(format!("row {i}: control names no node of the package"))?;
        if !is_eligible(node) {
            return Err(format!("row {i}: ineligible operator"));
        }
        if row["occurrence"]["role"] != "declaration"
            || source_entry(pkg, &row["control"], &row["occurrence"]).is_none()
        {
            return Err(format!("row {i}: no admitted source_map entry of the full key"));
        }
        if row["canonical_index"].as_u64().unwrap() >= related_len(node) {
            return Err(format!("row {i}: canonical_index at or above m"));
        }
        let same_group = previous.is_some_and(|p| {
            cmp_ref(&p["control"], &row["control"]) == Ordering::Equal && p["occurrence"] == row["occurrence"]
        });
        if let Some(p) = previous {
            if cmp_key(p, row) != Ordering::Less {
                return Err(format!("row {i}: key not strictly after its predecessor"));
            }
        }
        let expected = if same_group { previous.unwrap()["source_ordinal"].as_u64().unwrap() + 1 } else { 0 };
        if row["source_ordinal"].as_u64().unwrap() != expected {
            return Err(format!("row {i}: source_ordinal {} expected {expected}", row["source_ordinal"]));
        }
        previous = Some(row);
    }
    match first_uncovered(pkg, rows) {
        Some((control, _, index)) => Err(format!("uncovered canonical_index {index} of {control}")),
        None => Ok(()),
    }
}

/// The first eligible declaration occurrence, in ascending node-id order, with a
/// canonical position no row names: its control, occurrence and position.
fn first_uncovered(pkg: &Value, rows: &[Value]) -> Option<(Value, Value, u64)> {
    for node in control_nodes(pkg).into_iter().filter(|n| is_eligible(n)) {
        for occurrence in node["occurrences"].as_array().unwrap().iter().filter(|o| o["role"] == "declaration") {
            for index in 0..related_len(node) {
                let named = rows.iter().any(|r| {
                    r["control"] == *node_ref(node)
                        && r["occurrence"] == *occurrence
                        && r["canonical_index"].as_u64() == Some(index)
                });
                if !named {
                    return Some((node_ref(node).clone(), occurrence.clone(), index));
                }
            }
        }
    }
    None
}

// ---- the charge model (FR-831 "Reader checks"), recomputed from the package ----

#[derive(Debug, PartialEq, Clone)]
struct Charge {
    counter: &'static str,
    /// `None` is the symbolic view block G.
    amount: Option<u64>,
    locus: String,
    label: String,
}

fn row_charge(row: &Value) -> u64 {
    1 + ref_bytes(&row["control"]) + row["alias"].as_str().unwrap().len() as u64
}

/// The ordered charges of the strict step for a package and an optional member.
fn charges(pkg: &Value, rows: Option<&[Value]>) -> Vec<Charge> {
    let Some(rows) = rows else { return Vec::new() };
    let mut out = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        out.push(Charge { counter: "work", amount: Some(row_charge(row)), locus: format!("{MEMBER}/{i}"), label: format!("row {i}") });
    }
    let groups = rows
        .iter()
        .enumerate()
        .filter(|(i, r)| *i == 0 || cmp_ref(&rows[i - 1]["control"], &r["control"]) != Ordering::Equal || rows[i - 1]["occurrence"] != r["occurrence"])
        .count();
    if groups > 0 {
        out.push(Charge { counter: "read_storage", amount: None, locus: MEMBER.to_owned(), label: "view".to_owned() });
    }
    for node in control_nodes(pkg) {
        let mut amount = 1 + ref_bytes(node_ref(node));
        if is_eligible(node) {
            amount += declaration_occurrences(node) + related_len(node);
        }
        let name = node_ref(node)["path"].as_array().unwrap().iter().map(|s| s.as_str().unwrap()).collect::<Vec<_>>().join("/");
        out.push(Charge { counter: "work", amount: Some(amount), locus: MEMBER.to_owned(), label: format!("scan of {name}") });
        if is_eligible(node) && related_len(node) >= 1 {
            out.push(Charge { counter: "read_storage", amount: Some(related_len(node)), locus: MEMBER.to_owned(), label: format!("flags of {name}") });
        }
    }
    out
}

/// A limit spelled as a number or as `G`, `G - k`, `G + k`, evaluated for `g`.
fn limit_value(limit: &Value, g: u64) -> u64 {
    if let Some(n) = limit.as_u64() {
        return n;
    }
    let text = limit.as_str().unwrap().replace(' ', "");
    let rest = text.strip_prefix('G').unwrap_or_else(|| panic!("limit {text}"));
    match rest.chars().next() {
        None => g,
        Some('+') => g + rest[1..].parse::<u64>().unwrap(),
        Some('-') => g - rest[1..].parse::<u64>().unwrap(),
        Some(_) => panic!("limit {text}"),
    }
}

/// The first charge a limit denies, if any: the counter's cumulative total plus
/// the amount passes the limit. The other counter has no limit here.
fn first_unpaid(charges: &[Charge], counter: &str, limit: u64, g: u64, prior_work: u64) -> Option<Charge> {
    let mut used = if counter == "work" { prior_work } else { 0 };
    for charge in charges.iter().filter(|c| c.counter == counter) {
        let amount = charge.amount.unwrap_or(g);
        if used + amount > limit {
            return Some(charge.clone());
        }
        used += amount;
    }
    None
}

fn expected_amount(value: &Value, g: u64) -> u64 {
    value.as_u64().unwrap_or(g)
}

// ---- tests ----

/// FR-831-AC-10: every complete package and every case edit is schema-valid, so
/// each case would reach a reader's carrier check rather than the schema.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn every_case_package_is_complete_and_schema_valid() {
    let vectors = parse(VECTORS);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema");
    for name in ["PC-BASE", "PC-REL", "PC-DEP", "PC-ROOT"] {
        let pkg = package(&vectors, name);
        let first = validator.iter_errors(&pkg).next().map(|e| format!("{} {}", e.instance_path(), e));
        assert!(first.is_none(), "{name} is schema-valid: {first:?}");
        let mut with_member = pkg.clone();
        with_member.as_object_mut().unwrap().insert("protocol_correlation_occurrences".into(), json!([]));
        assert!(validator.is_valid(&with_member), "{name} with []");
    }
    let mut defects = Vec::new();
    for group in ["positive", "reader_refusals", "budget_cases", "consumer_cases"] {
        let list: Vec<Value> = match group {
            "budget_cases" => cases(&vectors["budget"], "cases").clone(),
            "consumer_cases" => cases(&vectors["consumer"], "cases").clone(),
            _ => cases(&vectors, group).clone(),
        };
        for case in &list {
            if let Some(error) = validator.iter_errors(&case_package(&vectors, case)).next() {
                defects.push(format!("{}: {}", case["id"], error.instance_path()));
            }
        }
    }
    assert!(defects.is_empty(), "{defects:#?}");
    assert_eq!(cases(&vectors, "positive").len(), 12);
    assert_eq!(cases(&vectors, "reader_refusals").len(), 21);
    assert_eq!(cases(&vectors["budget"], "cases").len(), 15);
    assert_eq!(cases(&vectors["consumer"], "cases").len(), 4);
}

/// FR-831-AC-10: PC-REL holds real related records on all six eligible forms,
/// the base none, and PC-DEP is a complete package of its own identity.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn the_related_package_carries_all_six_forms() {
    let vectors = parse(VECTORS);
    let rel = package(&vectors, "PC-REL");
    let mut forms: Vec<&str> = rel["semantic_graph"]["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|n| is_eligible(n) && related_len(n) >= 1)
        .map(operator)
        .collect();
    forms.sort_unstable();
    forms.dedup();
    assert_eq!(forms, ["attempt", "effect", "event", "receive", "recommend", "send"]);
    let total: u64 = rel["semantic_graph"]["nodes"].as_array().unwrap().iter().filter(|n| is_eligible(n)).map(related_len).sum();
    assert_eq!(total, 7);
    let base = parse(BASE);
    assert!(base["semantic_graph"]["nodes"].as_array().unwrap().iter().all(|n| related_len(n) == 0), "the base is related-empty");
    assert_eq!(package(&vectors, "PC-DEP")["name"], "acme/flow");
    assert_eq!(package(&vectors, "PC-ROOT")["name"], "example");
}

/// FR-831-AC-10: every positive member is consistent with its package (full-key
/// order, ordinals, source_map lookup, coverage) and every recorded group holds
/// the package's own whole `source_map` entry and the member's own clauses.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn positive_rows_are_consistent_with_their_package_and_borrow_its_source_map() {
    let vectors = parse(VECTORS);
    let mut seen_member = 0;
    for case in cases(&vectors, "positive") {
        let id = case["id"].as_str().unwrap();
        let Some(rows) = rows_of(case) else { continue };
        seen_member += 1;
        let pkg = package(&vectors, case["package"].as_str().unwrap());
        check_rows(&pkg, &rows).unwrap_or_else(|e| panic!("{id}: {e}"));
        let Some(groups) = case["expect"]["read"]["groups"].as_array() else { continue };
        let mut distinct: Vec<(&Value, &Value)> = Vec::new();
        for row in &rows {
            if !distinct.iter().any(|(c, o)| **c == row["control"] && **o == row["occurrence"]) {
                distinct.push((&row["control"], &row["occurrence"]));
            }
        }
        assert_eq!(groups.len(), distinct.len(), "{id}: one group per (control, occurrence)");
        for (group, (control, occurrence)) in groups.iter().zip(distinct) {
            assert_eq!(&group["control"], control, "{id}: group order is table order");
            assert_eq!(&group["occurrence"], occurrence, "{id}");
            let entry = source_entry(&pkg, control, occurrence).unwrap_or_else(|| panic!("{id}: entry"));
            assert_eq!(&group["source_map_entry"], entry, "{id}: the whole entry, every region");
            assert!(!entry["regions"].as_array().unwrap().is_empty(), "{id}: nonempty regions");
            let clauses: Vec<Value> = rows
                .iter()
                .filter(|r| r["control"] == *control && r["occurrence"] == *occurrence)
                .map(|r| json!({"source_ordinal": r["source_ordinal"], "canonical_index": r["canonical_index"], "alias": r["alias"]}))
                .collect();
            assert_eq!(group["clauses"], Value::Array(clauses), "{id}: clauses in source order");
        }
    }
    assert!(seen_member >= 9);
}

/// FR-831-AC-10: every refusal vector's rows are inconsistent with their
/// package, the coverage refusals retain the first uncovered control,
/// occurrence and canonical position the package computes, and CO-R17 carries
/// both a join defect and a key defect in one row with the join first.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn refusal_vectors_are_inconsistent_and_coverage_vectors_retain_the_first_uncovered_position() {
    let vectors = parse(VECTORS);
    for case in cases(&vectors, "reader_refusals") {
        let id = case["id"].as_str().unwrap();
        let rows = rows_of(case).unwrap_or_else(|| panic!("{id}: a refusal carries rows"));
        let pkg = package(&vectors, case["package"].as_str().unwrap());
        assert!(check_rows(&pkg, &rows).is_err(), "{id}: the rows are inconsistent with the package");
        assert_eq!(case["expect"]["decision"], "refuse", "{id}");
        if ["CO-R18", "CO-R19", "CO-R20", "CO-R21"].iter().any(|p| id.starts_with(p)) {
            let (control, occurrence, index) = first_uncovered(&pkg, &rows).unwrap_or_else(|| panic!("{id}: uncovered"));
            let retained = &case["expect"]["retained"];
            assert_eq!(retained["control"], control, "{id}: retained control");
            assert_eq!(retained["occurrence"], occurrence, "{id}: retained occurrence");
            assert_eq!(retained["canonical_index"].as_u64(), Some(index), "{id}: first uncovered canonical_index");
            assert_eq!(case["expect"]["locus"], MEMBER, "{id}");
        }
    }
    let r17 = case_named(&vectors, "reader_refusals", "CO-R17-join-before-order-in-one-row");
    let rows = rows_of(r17).unwrap();
    let pkg = package(&vectors, "PC-REL");
    assert!(find_node(&pkg, &rows[1]["control"]).is_none(), "row 1 has a join defect");
    assert_eq!(cmp_key(&rows[1], &rows[0]), Ordering::Less, "row 1 also sorts before row 0 (a key defect)");
    assert_eq!(r17["expect"]["locus"], format!("{MEMBER}/1/control"), "the join is reported, not the key");
}

/// FR-831-AC-10: mutating a key, an ordinal, an occurrence, a package or the
/// source map of a consistent member makes the consistency check fail, so the
/// check is sensitive to the full key and the context.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn the_consistency_check_fails_on_a_wrong_key_or_context() {
    let vectors = parse(VECTORS);
    let pkg = package(&vectors, "PC-REL");
    let rows = rows_of(case_named(&vectors, "positive", "CO-P04-every-form")).unwrap();
    check_rows(&pkg, &rows).expect("the unmutated member is consistent");
    let mutations: Vec<(&str, Vec<Value>)> = {
        let mut swapped = rows.clone();
        swapped.swap(0, 2);
        let mut ordinal = rows.clone();
        ordinal[1]["source_ordinal"] = json!(2);
        let mut occurrence = rows.clone();
        occurrence[3]["occurrence"]["ordinal"] = json!(1);
        let mut foreign = rows.clone();
        foreign[4]["control"]["package"] = json!("acme/flow");
        let mut index = rows.clone();
        index[0]["canonical_index"] = json!(2);
        let mut repeated = rows.clone();
        repeated.insert(1, rows[0].clone());
        vec![
            ("rows out of key order", swapped),
            ("ordinal gap", ordinal),
            ("occurrence without an entry", occurrence),
            ("control of another package", foreign),
            ("canonical_index at or above m", index),
            ("repeated key", repeated),
        ]
    };
    for (label, mutated) in mutations {
        assert!(check_rows(&pkg, &mutated).is_err(), "{label} must fail");
    }
    let mut no_entry = pkg.clone();
    no_entry["source_map"].as_array_mut().unwrap().retain(|e| e["node_id"]["path"].as_array().unwrap().last().unwrap() != "Requested");
    assert!(check_rows(&no_entry, &rows).is_err(), "a missing source_map entry must fail");
    let mut missing_row = rows.clone();
    missing_row.pop();
    assert!(check_rows(&pkg, &missing_row).is_err(), "an uncovered control must fail");
}

/// FR-831-AC-10: every budget case's expectation equals the charge model
/// recomputed from the case's own package and rows: the counter, the first
/// unpaid amount and locus, and the limit's admission or denial; the recorded
/// work units are the model's.
#[trace("TC-2113", "FR-831-AC-10")]
#[test]
fn budget_expectations_equal_the_recomputed_charge_model() {
    let vectors = parse(VECTORS);
    let budget = &vectors["budget"];
    for case in cases(budget, "cases") {
        let id = case["id"].as_str().unwrap();
        let pkg = case_package(&vectors, case);
        let rows = pkg.get("protocol_correlation_occurrences").and_then(Value::as_array).cloned();
        let model = charges(&pkg, rows.as_deref());
        let prior = case["prior_work"].as_u64().unwrap_or(0);
        for g in [6_u64, 96, 1200] {
            let limit = limit_value(&case["limit_over_baseline"], g);
            let denied = first_unpaid(&model, case["counter"].as_str().unwrap(), limit, g, prior);
            let expect = &case["expect"];
            match (expect["decision"].as_str().unwrap(), &denied) {
                ("admit", None) => {}
                ("incomplete", Some(charge)) => {
                    let first = &expect["first_unpaid"];
                    assert_eq!(first["kind"], charge.counter, "{id}: counter");
                    assert_eq!(expected_amount(&first["amount"], g), charge.amount.unwrap_or(g), "{id}: first unpaid amount (G={g})");
                    assert_eq!(first["locus"], charge.locus, "{id}: locus");
                    if charge.label.starts_with("scan of") || charge.label.starts_with("flags of") {
                        assert!(first["charge"].as_str().unwrap().contains(&charge.label), "{id}: charge description {:?} vs {}", first["charge"], charge.label);
                    }
                    assert_eq!(expect["limit"], case["counter"], "{id}");
                    assert_eq!(expect["partial_view"], false, "{id}");
                }
                (decision, outcome) => panic!("{id}: expected {decision}, the model gives {outcome:?} (G={g})"),
            }
        }
    }
    // The recorded measured arithmetic of CO-B01 and CO-B12 is the model's.
    let b01 = case_named(budget, "cases", "CO-B01-admit-measured");
    let pkg = case_package(&vectors, b01);
    let rows = rows_of(b01).unwrap();
    let model = charges(&pkg, Some(&rows));
    let work: Vec<u64> = model.iter().filter(|c| c.counter == "work").map(|c| c.amount.unwrap()).collect();
    let recorded_rows: Vec<u64> = b01["work_units"]["rows"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
    assert_eq!(&work[..rows.len()], recorded_rows.as_slice());
    let recorded_scan: Vec<u64> = b01["work_units"]["scan"].as_array().unwrap().iter().map(|v| v["amount"].as_u64().unwrap()).collect();
    assert_eq!(&work[rows.len()..], recorded_scan.as_slice());
    assert_eq!(b01["work_units"]["total"].as_u64().unwrap(), work.iter().sum::<u64>());
    assert_eq!(b01["limit_over_baseline"].as_u64().unwrap(), work.iter().sum::<u64>());
    let flags: u64 = model.iter().filter(|c| c.label.starts_with("flags")).map(|c| c.amount.unwrap()).sum();
    assert_eq!(b01["storage_units"]["flags_total"].as_u64().unwrap(), flags);
    // The long-alias row and the long control text are paid by their bytes.
    let b07 = case_named(budget, "cases", "CO-B07-alias-payload");
    let alias_row = &rows_of(b07).unwrap()[0];
    assert_eq!(alias_row["alias"].as_str().unwrap().len(), 64);
    assert_eq!(row_charge(alias_row), 1 + ref_bytes(&alias_row["control"]) + 64);
    let b15 = case_named(budget, "cases", "CO-B15-control-bytes-are-charged");
    let long = &rows_of(b15).unwrap()[1];
    assert!(ref_bytes(&long["control"]) > 200, "the long control text is charged by its bytes");
    assert!(find_node(&case_package(&vectors, b15), &long["control"]).is_none(), "the long control names no node: charged, then refused by the join");
}

/// FR-831-AC-6: every `schema_valid` edit is schema-valid and every
/// `schema_invalid` edit is refused at the recorded first error path.
#[trace("TC-2113", "FR-831-AC-6")]
#[test]
fn the_schema_decides_the_member_shape_at_the_recorded_first_error_path() {
    let vectors = parse(VECTORS);
    let validator = jsonschema::validator_for(&parse(SCHEMA)).expect("schema");
    let mut defects = Vec::new();
    for case in cases(&vectors, "schema_valid") {
        if !validator.is_valid(&case_package(&vectors, case)) {
            defects.push(format!("{}: refused", case["id"]));
        }
    }
    let invalid = cases(&vectors, "schema_invalid");
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

/// FR-831-AC-11: the member is a separate top-level member: two packages of an
/// invariance case differ in it alone and in no node, node id, source_map entry
/// or protocol location. This compares packages as data; it is not the
/// producer invariance of FR-831-AC-8, which needs real source variants.
#[trace("TC-2113", "FR-831-AC-11")]
#[test]
fn the_member_is_the_sole_difference_between_invariance_packages() {
    let vectors = parse(VECTORS);
    for case in cases(&vectors, "invariance") {
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
