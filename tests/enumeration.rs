use serde_derive::{ Serialize, Deserialize };
use serde_dynamic_typing::typing::{ AnyValue, Enumeration, Utf8String, Map };
use std::iter::FromIterator;


#[test]
fn enum_unit() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum UnitEnum {
        VariantA,
        VariantB
    }

    let value = UnitEnum::VariantA;
    let expected = AnyValue::Enum(
        Enumeration::from("VariantA")
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize enum");
    assert_eq!(serialized, expected);

    let deserialized: UnitEnum = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize enum");
    assert_eq!(deserialized, value);
}


#[test]
fn enum_newtype() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum UnitVariant {
        VariantA(String),
        VariantB(String)
    }

    let value = UnitVariant::VariantA("ValueA".to_string());
    let expected = AnyValue::Enum(
        Enumeration::with_value(
            "VariantA",
            Utf8String::from("ValueA")
        )
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize enum");
    assert_eq!(serialized, expected);

    let deserialized: UnitVariant = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize enum");
    assert_eq!(deserialized, value);
}


#[test]
fn enum_tuple() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum TupleVariant {
        VariantA(String, u64),
        VariantB(String, u64)
    }

    let value = TupleVariant::VariantA("ValueA".to_string(), 0);
    let expected = AnyValue::Enum(
        Enumeration::with_value(
            "VariantA",
            AnyValue::Sequence([
                AnyValue::Utf8String("ValueA".into()),
                AnyValue::Integer(0u64.into())
            ].into())
        )
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize enum");
    assert_eq!(serialized, expected);

    let deserialized: TupleVariant = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize enum");
    assert_eq!(deserialized, value);
}


#[test]
fn enum_struct() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum StructVariant {
        VariantA { value_name: String, value_index: u64 },
        VariantB { value_name: String, value_index: u64 }
    }

    let value = StructVariant::VariantA { value_name: "ValueA".to_string(), value_index: 0 };
    let expected = AnyValue::Enum(
        Enumeration::with_value(
            "VariantA",
            AnyValue::Map(
                Map::from_iter([
                    (
                        AnyValue::Utf8String("value_name".into()),
                        AnyValue::Utf8String("ValueA".into())
                    ),
                    (
                        AnyValue::Utf8String("value_index".into()),
                        AnyValue::Integer(0u64.into())
                    )
                ])
            )
        )
    );

    let serialized = serde_dynamic_typing::to_typed(&value).expect("Failed to serialize enum");
    assert_eq!(serialized, expected);

    let deserialized: StructVariant = serde_dynamic_typing::from_typed(serialized).expect("Failed to deserialize enum");
    assert_eq!(deserialized, value);
}
