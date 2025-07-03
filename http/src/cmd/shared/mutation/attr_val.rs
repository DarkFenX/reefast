use std::str::FromStr;

use crate::shared::HMutaRoll;

const ROLL_PREFIX: &str = "r";
const ABS_PREFIX: &str = "a";

pub(in crate::cmd) enum HItemAttrMutationValue {
    Roll(HMutaRoll),
    // Absolute will be used by default
    Absolute(rc::AttrVal),
}
impl<'de> serde::Deserialize<'de> for HItemAttrMutationValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HItemAttrMutationValueVisitor;

        impl<'de> serde::de::Visitor<'de> for HItemAttrMutationValueVisitor {
            type Value = HItemAttrMutationValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("number or string with number with optional type prefix")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v as f64)))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v as f64)))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v as f64)))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v as f64)))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v as f64)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Absolute(rc::AttrVal::from(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(roll_str) = v.strip_prefix(ROLL_PREFIX) {
                    let roll = HMutaRoll::from_str(roll_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Roll(roll));
                }
                if let Some(abs_str) = v.strip_prefix(ABS_PREFIX) {
                    let abs = rc::AttrVal::from_str(abs_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Absolute(abs));
                }
                let prange = rc::AttrVal::from_str(v).map_err(|v| serde::de::Error::custom(v))?;
                Ok(Self::Value::Absolute(prange))
            }
        }
        deserializer.deserialize_any(HItemAttrMutationValueVisitor)
    }
}
