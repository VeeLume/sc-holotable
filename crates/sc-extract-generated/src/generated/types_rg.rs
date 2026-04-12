// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::*;

/// DCB type: `RGB8`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGB8 {
    /// DCB field: `r` (Byte)
    #[serde(default)]
    pub r: u32,
    /// DCB field: `g` (Byte)
    #[serde(default)]
    pub g: u32,
    /// DCB field: `b` (Byte)
    #[serde(default)]
    pub b: u32,
}

impl Pooled for RGB8 {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_rg.rgb8 }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_rg.rgb8 }
}

impl<'a> Extract<'a> for RGB8 {
    const TYPE_NAME: &'static str = "RGB8";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r: inst.get_u32("r").unwrap_or_default(),
            g: inst.get_u32("g").unwrap_or_default(),
            b: inst.get_u32("b").unwrap_or_default(),
        }
    }
}

/// DCB type: `RGBA`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGBA {
    /// DCB field: `r` (Single)
    #[serde(default)]
    pub r: f32,
    /// DCB field: `g` (Single)
    #[serde(default)]
    pub g: f32,
    /// DCB field: `b` (Single)
    #[serde(default)]
    pub b: f32,
    /// DCB field: `a` (Single)
    #[serde(default)]
    pub a: f32,
}

impl Pooled for RGBA {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_rg.rgba }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_rg.rgba }
}

impl<'a> Extract<'a> for RGBA {
    const TYPE_NAME: &'static str = "RGBA";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r: inst.get_f32("r").unwrap_or_default(),
            g: inst.get_f32("g").unwrap_or_default(),
            b: inst.get_f32("b").unwrap_or_default(),
            a: inst.get_f32("a").unwrap_or_default(),
        }
    }
}

/// DCB type: `RGB`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGB {
    /// DCB field: `r` (Single)
    #[serde(default)]
    pub r: f32,
    /// DCB field: `g` (Single)
    #[serde(default)]
    pub g: f32,
    /// DCB field: `b` (Single)
    #[serde(default)]
    pub b: f32,
}

impl Pooled for RGB {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.b_rg.rgb }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.b_rg.rgb }
}

impl<'a> Extract<'a> for RGB {
    const TYPE_NAME: &'static str = "RGB";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            r: inst.get_f32("r").unwrap_or_default(),
            g: inst.get_f32("g").unwrap_or_default(),
            b: inst.get_f32("b").unwrap_or_default(),
        }
    }
}

