#[macro_use]
mod macros;
use macros::Json;

#[allow(unused_imports)]
use std::collections::HashMap;

fn main() {
    let _students = json!([
        {
            "name": "Jim Blandy",
            "class_of": 1926,
            "major": "Tibetan throat singing"
        },
        {
            "name": "Jason Orendorff",
            "class_of": 1702,
            "major": "Knots"
        }
    ]);
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn json_string() {
    assert_eq!(json!("test"), Json::String(String::from("test")));
}

#[test]
fn json_number() {
    assert_eq!(json!(65), Json::Number(65_f64));
}

#[test]
fn json_array_with_json_element() {
    let macro_generated_value = json!(
        [
            {
                "pitch": 440.0
            }
        ]
    );

    let mut test_map = HashMap::new();
    test_map.insert(String::from("pitch"), Json::Number(440.0));

    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(test_map))]);

    assert_eq!(macro_generated_value, hand_coded_value);
}