//use crate::error::Error;
use crate::value::Value;
use core::fmt;
use serde::de::{Deserialize, MapAccess, SeqAccess, Visitor};

impl<'de> Deserialize<'de> for Value<'de> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value<'de>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value<'de>, E> {
                Ok(Value::Bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value<'de>, E> {
                Ok(Value::Number(value.into()))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value<'de>, E> {
                Ok(Value::Number(value.into()))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value<'de>, E> {
                Ok(Value::Number(value.into()))
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::Str(v))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value<'de>, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value<'de>, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value<'de>, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Array(vec))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
            where
                V: MapAccess<'de>,
            {
                use fnv::FnvHashMap;

                let mut values = FnvHashMap::default();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(Value::Object(values))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {

    use crate::Value;
    #[test]
    fn deserialize_json_test() {
        let json_obj = r#"
            {
                "bool": true,
                "string_key": "string_val",
                "float": 1.23,
                "i64": -123,
                "u64": 123
            }
       "#;

        let val: Value = serde_json::from_str(json_obj).unwrap();
        assert_eq!(val.get("bool"), &Value::Bool(true));
        assert_eq!(val.get("string_key"), &Value::Str("string_val"));
        assert_eq!(val.get("float"), &Value::Number(1.23.into()));
        assert_eq!(val.get("i64"), &Value::Number((-123i64).into()));
        assert_eq!(val.get("u64"), &Value::Number(123u64.into()));
    }
}
