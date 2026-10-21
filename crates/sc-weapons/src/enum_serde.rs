//! serde adapters for the generated item enums, which carry no serde of
//! their own: each is stored as its DCB string (`as_dcb_str` /
//! `from_dcb_str`), as `sc_items` stores them.

use sc_extract::generated::{EItemSubType, EItemType};
use serde::{Deserialize, Deserializer, Serializer};

pub mod item_type {
    use super::*;
    pub fn serialize<S: Serializer>(v: &EItemType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemType, D::Error> {
        Ok(EItemType::from_dcb_str(&String::deserialize(d)?))
    }
}

pub mod item_sub_type {
    use super::*;
    pub fn serialize<S: Serializer>(v: &EItemSubType, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(v.as_dcb_str())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<EItemSubType, D::Error> {
        Ok(EItemSubType::from_dcb_str(&String::deserialize(d)?))
    }
}
