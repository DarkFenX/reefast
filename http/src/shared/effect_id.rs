use std::str::FromStr;

const D_PREFIX: &str = "d";
const SCSE_PREFIX: &str = "scse";
const SCPE_PREFIX: &str = "scpe";
const SCPT_PREFIX: &str = "scpt";
const SCSL_PREFIX: &str = "scsl";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(crate) enum HEffectId {
    Dogma(rc::DogmaEffectId),
    ScSystemEmitter(rc::ItemTypeId),
    ScProxyEffect(rc::ItemTypeId),
    ScProxyTrap(rc::ItemTypeId),
    ScShipLink(rc::ItemTypeId),
    Custom(rc::CustomEffectId),
}
impl From<&rc::EffectId> for HEffectId {
    fn from(core_effect_id: &rc::EffectId) -> Self {
        match core_effect_id {
            rc::EffectId::Dogma(id) => Self::Dogma(*id),
            rc::EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            rc::EffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            rc::EffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            rc::EffectId::ScShipLink(id) => Self::ScShipLink(*id),
            rc::EffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<rc::EffectId> for HEffectId {
    fn from(core_effect_id: rc::EffectId) -> Self {
        match core_effect_id {
            rc::EffectId::Dogma(id) => Self::Dogma(id),
            rc::EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id),
            rc::EffectId::ScProxyEffect(id) => Self::ScProxyEffect(id),
            rc::EffectId::ScProxyTrap(id) => Self::ScProxyTrap(id),
            rc::EffectId::ScShipLink(id) => Self::ScShipLink(id),
            rc::EffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&HEffectId> for rc::EffectId {
    fn from(h_effect_id: &HEffectId) -> Self {
        match h_effect_id {
            HEffectId::Dogma(id) => Self::Dogma(*id),
            HEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            HEffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            HEffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            HEffectId::ScShipLink(id) => Self::ScShipLink(*id),
            HEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for HEffectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Dogma(id) => format!("{D_PREFIX}{id}"),
            Self::ScSystemEmitter(id) => format!("{SCSE_PREFIX}{id}"),
            Self::ScProxyEffect(id) => format!("{SCPE_PREFIX}{id}"),
            Self::ScProxyTrap(id) => format!("{SCPT_PREFIX}{id}"),
            Self::ScShipLink(id) => format!("{SCSL_PREFIX}{id}"),
            Self::Custom(id) => format!("{C_PREFIX}{id}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> serde::Deserialize<'de> for HEffectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HEffectIdVisitor;

        impl<'de> serde::de::Visitor<'de> for HEffectIdVisitor {
            type Value = HEffectId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("effect type-prefixed int HEffectId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(D_PREFIX) {
                    let id = rc::DogmaEffectId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Dogma(id));
                }
                if let Some(id_str) = v.strip_prefix(SCSE_PREFIX) {
                    let id = rc::ItemTypeId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::ScSystemEmitter(id));
                }
                if let Some(id_str) = v.strip_prefix(SCPE_PREFIX) {
                    let id = rc::ItemTypeId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::ScProxyEffect(id));
                }
                if let Some(id_str) = v.strip_prefix(SCPT_PREFIX) {
                    let id = rc::ItemTypeId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::ScProxyTrap(id));
                }
                if let Some(id_str) = v.strip_prefix(SCSL_PREFIX) {
                    let id = rc::ItemTypeId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::ScShipLink(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = rc::CustomEffectId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg = format!(
                    "expected an int prefixed by \"{D_PREFIX}\", \"{SCSE_PREFIX}\", \"{SCPE_PREFIX}\", \
                    \"{SCPT_PREFIX}\", \"{SCSL_PREFIX}\", or \"{C_PREFIX}\", got \"{v}\""
                );
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(HEffectIdVisitor)
    }
}
