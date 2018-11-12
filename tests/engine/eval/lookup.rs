use serde_json::json;

#[macro_export]
macro_rules! test_lookup_eq {
    ($expression:expr, $data:expr, $position:expr, $result:expr) => {{
        let engine = balena_temen::Engine::default();
        let mut context = balena_temen::Context::default();
        let position: balena_temen::ast::Expression = $position.parse().unwrap();

        assert_eq!(
            engine
                .eval($expression, &position.identifier().unwrap(), &$data, &mut context)
                .unwrap(),
            $result
        );
    }};
    ($expression:expr, $data:expr, $result:expr) => {{
        test_lookup_eq!($expression, $data, "this", $result);
    }};
}

#[macro_export]
macro_rules! test_lookup_err {
    ($expression:expr, $data:expr, $position:expr) => {{
        let engine = balena_temen::Engine::default();
        let mut context = balena_temen::Context::default();
        let position: balena_temen::ast::Expression = $position.parse().unwrap();

        assert!(engine
            .eval($expression, &position.identifier().unwrap(), &$data, &mut context)
            .is_err());
    }};
    ($expression:expr, $data:expr) => {{
        test_lookup_err!($expression, $data, "this");
    }};
}

#[test]
fn simple_identifier() {
    let data = json!({
        "string": "hallo",
        "integer": 10,
        "float": 3.2,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
    });

    test_lookup_eq!("string", data, json!("hallo"));
    test_lookup_eq!("integer", data, json!(10));
    test_lookup_eq!("float", data, json!(3.2));
    test_lookup_eq!("boolean", data, json!(true));
    test_lookup_eq!("array", data, json!(["a", "b"]));
    test_lookup_eq!("object", data, json!({"a": "b"}));
    test_lookup_eq!("null", data, json!(null));
}

#[test]
fn fail_on_unknown_identifier() {
    let data = json!({
        "string": "hallo",
        "integer": 10,
        "float": 3.2,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
    });

    test_lookup_err!("na", data);
}

#[test]
fn dotted_identifier() {
    let data = json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    });

    test_lookup_eq!("root.another.string", data, json!("hallo"));
    test_lookup_eq!("root.another.integer", data, json!(10));
    test_lookup_eq!("root.another.float", data, json!(3.2));
    test_lookup_eq!("root.another.boolean", data, json!(true));
    test_lookup_eq!("root.another.array", data, json!(["a", "b"]));
    test_lookup_eq!("root.another.object", data, json!({"a": "b"}));
    test_lookup_eq!("root.another.null", data, json!(null));
}

#[test]
fn fail_on_unknown_dotted_identifier() {
    let data = json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    });

    test_lookup_err!("root.another.na", data);
    test_lookup_err!("root.na", data);
    test_lookup_err!("na", data);
}

#[test]
fn dotted_identifier_integer_index() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_eq!("root.0", data, json!("hallo"));
    test_lookup_eq!("root.1", data, json!(10));
    test_lookup_eq!("root.2", data, json!(3.2));
    test_lookup_eq!("root.3", data, json!(true));
    test_lookup_eq!("root.4", data, json!(["a", "b"]));
    test_lookup_eq!("root.5", data, json!({"a": "b"}));
    test_lookup_eq!("root.6", data, json!(null));
}

#[test]
fn fail_on_dotted_identifier_integer_index_out_of_bounds() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_err!("root.7", data);
}

#[test]
fn dotted_identifier_integer_index_mixed() {
    let data = json!({
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]});

    test_lookup_eq!("people.0[`name`]", data, json!("Robert"));
    test_lookup_eq!("people.0.name", data, json!("Robert"));
    test_lookup_eq!("people.1[`name`]", data, json!("Cyryl"));
    test_lookup_eq!("people.1.name", data, json!("Cyryl"));
}

#[test]
fn square_bracket_string() {
    let data = json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    });

    test_lookup_eq!("root[`another`][`string`]", data, json!("hallo"));
    test_lookup_eq!("root[`another`][`integer`]", data, json!(10));
    test_lookup_eq!("root[`another`][`float`]", data, json!(3.2));
    test_lookup_eq!("root[`another`][`boolean`]", data, json!(true));
    test_lookup_eq!("root[`another`][`array`]", data, json!(["a", "b"]));
    test_lookup_eq!("root[`another`][`object`]", data, json!({"a": "b"}));
    test_lookup_eq!("root[`another`][`null`]", data, json!(null));
}

#[test]
fn fail_on_square_bracket_string_invalid_index() {
    let data = json!({
        "root": {
            "another": {
                "string": "hallo",
                "integer": 10,
                "float": 3.2,
                "boolean": true,
                "array": ["a", "b"],
                "object": {"a": "b"},
                "null": null,
            }
        }
    });

    test_lookup_err!("root[`another`][`dummy`]", data);
    test_lookup_err!("root[`dummy`]", data);
}

#[test]
fn square_bracket_integer() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_eq!("root[0]", data, json!("hallo"));
    test_lookup_eq!("root[1]", data, json!(10));
    test_lookup_eq!("root[2]", data, json!(3.2));
    test_lookup_eq!("root[3]", data, json!(true));
    test_lookup_eq!("root[4]", data, json!(["a", "b"]));
    test_lookup_eq!("root[5]", data, json!({"a": "b"}));
    test_lookup_eq!("root[6]", data, json!(null));
}

#[test]
fn fail_on_square_bracket_integer_out_of_bounds() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_err!("root[7]", data);
}

#[test]
fn square_bracket_negative_integer() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_eq!("root[-7]", data, json!("hallo"));
    test_lookup_eq!("root[-6]", data, json!(10));
    test_lookup_eq!("root[-5]", data, json!(3.2));
    test_lookup_eq!("root[-4]", data, json!(true));
    test_lookup_eq!("root[-3]", data, json!(["a", "b"]));
    test_lookup_eq!("root[-2]", data, json!({"a": "b"}));
    test_lookup_eq!("root[-1]", data, json!(null));
}

#[test]
fn fail_on_square_bracket_negative_integer_out_of_bounds() {
    let data = json!({
        "root": [
            "hallo",
            10,
            3.2,
            true,
            ["a", "b"],
            {"a": "b"},
            null
        ]});

    test_lookup_err!("root[-8]", data);
}

#[test]
fn square_bracket_mixed() {
    let data = json!({
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]});

    test_lookup_eq!("people[0][`name`]", data, json!("Robert"));
    test_lookup_eq!("people[0].name", data, json!("Robert"));
    test_lookup_eq!("people[1][`name`]", data, json!("Cyryl"));
    test_lookup_eq!("people[1].name", data, json!("Cyryl"));
}

#[test]
fn square_bracket_indirect() {
    let data = json!({
        "first": 0,
        "second": 1,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]});

    test_lookup_eq!("people[first][`name`]", data, json!("Robert"));
    test_lookup_eq!("people[first].name", data, json!("Robert"));
    test_lookup_eq!("people[second][`name`]", data, json!("Cyryl"));
    test_lookup_eq!("people[second].name", data, json!("Cyryl"));
}

#[test]
fn fail_on_square_bracket_indirect_invalid_type() {
    let data = json!({
        "first": 0,
        "second": 1,
        "boolean": true,
        "array": ["a", "b"],
        "object": {"a": "b"},
        "null": null,
        "people": [
            {
                "name": "Robert"
            },
            {
                "name": "Cyryl"
            }
        ]});

    test_lookup_err!("people[boolean].name", data);
    test_lookup_err!("people[array].name", data);
    test_lookup_err!("people[object].name", data);
    test_lookup_err!("people[null].name", data);
}

#[test]
fn square_bracket_multiple_indirect() {
    let data = json!({
        "country": "Czech Republic",
        "city": "Hradec Kralove",
        "data": {
            "country": {
                "Czech Republic": {
                    "city": {
                        "Hradec Kralove": {
                            "rust-developers": 1
                        }
                    }
                }
            }
        }});

    test_lookup_eq!("data.country[country].city[city]['rust-developers']", data, json!(1));
}

#[test]
fn square_bracket_nested_indirect() {
    let data = json!({
        "name": "czech",
        "country": {
            "czech": "Czech Republic"
        },
        "data": {
            "country": {
                "Czech Republic": {
                    "rust-developers": 2
                }
            }
        }});

    test_lookup_eq!("data.country[country[name]]['rust-developers']", data, json!(2));
}

#[test]
fn relative_lookup() {
    let data = json!({
        "first": 0,
        "second": 1,
        "names": [
            "Robert",
            "Cyryl"
        ]});

    test_lookup_eq!("this", data.clone(), "first", json!(0));
    test_lookup_eq!("this.this.this", data.clone(), "first", json!(0));
    test_lookup_eq!("this == this.this", data.clone(), "first", json!(true));

    test_lookup_eq!("this.super", data.clone(), "names[0]", json!(["Robert", "Cyryl"]));
    test_lookup_eq!("super", data.clone(), "names[0]", json!(["Robert", "Cyryl"]));
    test_lookup_eq!("super == this.super", data.clone(), "names[0]", json!(true));
    test_lookup_eq!("this.super.super", data.clone(), "names[0]", data.clone());

    test_lookup_eq!("this[0]", data.clone(), "names", json!("Robert"));
    test_lookup_eq!("this[1]", data.clone(), "names", json!("Cyryl"));
    test_lookup_eq!("this[first]", data.clone(), "names", json!("Robert"));
    test_lookup_eq!("this[second]", data.clone(), "names", json!("Cyryl"));

    test_lookup_eq!("names[this]", data.clone(), "first", json!("Robert"));
    test_lookup_eq!("names[this]", data.clone(), "second", json!("Cyryl"));

    test_lookup_eq!("this == names[second]", data.clone(), "names[1]", json!(true));
}
