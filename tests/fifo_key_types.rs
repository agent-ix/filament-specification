// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! FR-726 FIFO key types (TC-1770): the published key-type cases, decided by
//! the stated structural walk. A key is FIFO-key-stable when the walk through
//! option, collection, record, tuple and variant members reaches no IEEE
//! float leaf and no event leaf; a model-owned type's members are read from
//! the model document, because the package holds no node for them.

use ix_trace_rs::trace;
use serde_json::Value;

const VECTORS: &str = include_str!("../proposals/checked-package-v2/fifo-key-type-vectors.json");

/// The first leaf that makes `key` not FIFO-key-stable, if any.
fn unstable_leaf<'a>(key: &'a Value, document: &'a Value) -> Option<&'a str> {
    match key["kind"].as_str().expect("type form") {
        "scalar" => {
            let family = key["family"].as_str().expect("scalar family");
            matches!(family, "float32" | "float64").then_some(family)
        }
        "event_type" => Some("event_type"),
        "quantity" => None,
        "bounded_domain" => unstable_leaf(&key["base"], document),
        "option" => unstable_leaf(&key["of"], document),
        "collection" => unstable_leaf(&key["element"], document),
        "record" => fields(&key["fields"], document),
        "model_type" => {
            let name = key["declaration"].as_str().expect("declaration");
            let declaration = &document["declarations"][name];
            match declaration["form"].as_str().expect("model form") {
                "value_type" => unstable_leaf(&declaration["base"], document),
                "record_value_type" => fields(&declaration["fields"], document),
                "variant_type" => declaration["cases"]
                    .as_array()
                    .expect("cases")
                    .iter()
                    .filter(|case| !case["payload"].is_null())
                    .find_map(|case| unstable_leaf(&case["payload"], document)),
                other => panic!("{name}: unexpected model form {other}"),
            }
        }
        other => panic!("unexpected type form {other}"),
    }
}

fn fields<'a>(fields: &'a Value, document: &'a Value) -> Option<&'a str> {
    fields
        .as_array()
        .expect("fields")
        .iter()
        .find_map(|field| unstable_leaf(&field["type"], document))
}

#[trace("TC-1770", "FR-726-AC-2", "FR-726-AC-3")]
#[test]
fn tc_1770_fifo_key_types_follow_the_structural_walk() {
    let vectors: Value = serde_json::from_str(VECTORS).expect("FIFO key vectors JSON");
    let document = &vectors["model_document"];
    let (mut admitted, mut refused) = (0, 0);
    for case in vectors["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let leaf = unstable_leaf(&case["key"], document);
        match case["expected"].as_str().expect("expected") {
            "admitted" => {
                assert_eq!(leaf, None, "{id} is FIFO-key-stable");
                admitted += 1;
            }
            "refused" => {
                assert_eq!(leaf, case["leaf"].as_str(), "{id} is refused for its leaf");
                refused += 1;
            }
            other => panic!("{id}: unknown expectation {other}"),
        }
    }
    assert_eq!((admitted, refused), (10, 7));
    // FK-15's float leaf is reachable only through the model document: the
    // key names the declaration and carries no member of its own.
    let fk15 = &vectors["cases"][14];
    assert_eq!(fk15["id"], "FK-15");
    assert!(fk15["key"].get("fields").is_none());
    assert_eq!(vectors["refusal"]["code"], "ill_typed");
    assert_eq!(vectors["refusal"]["cause"], "operator-ineligible");
}
