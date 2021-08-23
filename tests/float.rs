use serde_dynamic_typing::typing::AnyValue;


#[test]
fn float() {
    let value = 0.478f64;
    let expected = AnyValue::Float(0.478f64.into());

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize float");
    assert_eq!(serialized, expected);

    let deserialized: f64 = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize float");
    assert_eq!(deserialized, value);
}
