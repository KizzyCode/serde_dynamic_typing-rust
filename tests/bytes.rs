#[macro_use] extern crate serde_derive;
use serde_dynamic_typing::typing::AnyValue;


#[test]
fn bytes() {
    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct ByteContainer(
        #[serde(with = "serde_bytes")]
        Vec<u8>
    );

    let value = ByteContainer(b"Testolope".to_vec());
    let expected = AnyValue::Bytes((*b"Testolope").into());

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize bytes");
    assert_eq!(serialized, expected);

    let deserialized: ByteContainer = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize bytes");
    assert_eq!(deserialized, value);
}
