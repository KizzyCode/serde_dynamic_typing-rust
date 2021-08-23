use serde_dynamic_typing::typing::{ AnyValue, Integer, Map, Utf8String };
use std::{ collections::HashMap, iter::FromIterator };


#[test]
fn map() {
    let value = HashMap::from_iter(vec![
        ("Key0".to_string(), 0),
        ("Key1".to_string(), 1)
    ]);
    let expected = AnyValue::Map(
        Map::from_iter(vec![
            (Utf8String::new("Key0").into(), Integer::from(0).into()),
            (Utf8String::new("Key1").into(), Integer::from(1).into())
        ])
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize map");
    assert_eq!(serialized, expected);

    let deserialized: HashMap<String, i32> = serde_dynamic_typing::from_typed(serialized)
        .expect("Failed to deserialize map");
    assert_eq!(deserialized, value);
}
