use serde_dynamic_typing::typing::{ AnyValue, Sequence, Utf8String };
use std::iter::FromIterator;


#[test]
fn sequence() {
    let value = vec![
        "Key0".to_string(),
        "Key1".to_string()
    ];
    let expected = AnyValue::Sequence(
        Sequence::from_iter(vec![
            Utf8String::from("Key0").into(),
            Utf8String::from("Key1").into()
        ])
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize sequence");
    assert_eq!(serialized, expected);

    let deserialized: Vec<String> = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize sequence");
    assert_eq!(deserialized, value);
}
