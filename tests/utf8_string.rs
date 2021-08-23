use serde_dynamic_typing::typing::AnyValue;


#[test]
fn utf8_string() {
    let value = "Testolope".to_string();
    let expected = AnyValue::Utf8String("Testolope".into());

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize string");
    assert_eq!(serialized, expected);

    let deserialized: String = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize string");
    assert_eq!(deserialized, value);
}
