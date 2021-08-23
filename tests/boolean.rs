use serde_dynamic_typing::typing::AnyValue;


#[test]
fn boolean() {
    let value = true;
    let expected = AnyValue::Bool(true.into());

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize boolean");
    assert_eq!(serialized, expected);

    let deserialized: bool = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize boolean");
    assert_eq!(deserialized, value);
}
