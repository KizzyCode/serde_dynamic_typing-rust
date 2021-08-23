use serde_dynamic_typing::typing::AnyValue;


#[test]
fn integer() {
    let value = 7usize;
    let expected = AnyValue::Integer(7usize.into());

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize integer");
    assert_eq!(serialized, expected);

    let deserialized: usize = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize integer");
    assert_eq!(deserialized, value);
}
