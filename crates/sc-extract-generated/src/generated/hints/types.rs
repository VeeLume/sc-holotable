// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Types for feature `hints`.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports)]
#![allow(clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};
use svarog_common::CigGuid;
use svarog_datacore::{Instance, Value};
use crate::{Builder, Extract, Handle, Pooled};

use super::super::*;

/// DCB type: `HintUIData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintUIData {
    /// `TitleText` (Locale)
    #[serde(default)]
    pub title_text: String,
    /// `BodyText` (Locale)
    #[serde(default)]
    pub body_text: String,
    /// `BodyTextGamePad` (Locale)
    #[serde(default)]
    pub body_text_game_pad: String,
    /// `BodyTextJoystick` (Locale)
    #[serde(default)]
    pub body_text_joystick: String,
    /// `TitleImagePath` (String)
    #[serde(default)]
    pub title_image_path: String,
    /// `BodyImagePath` (String)
    #[serde(default)]
    pub body_image_path: String,
}

impl Pooled for HintUIData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hints.hint_uidata }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hints.hint_uidata }
}

impl<'a> Extract<'a> for HintUIData {
    const TYPE_NAME: &'static str = "HintUIData";
    fn extract(inst: &Instance<'a>, _b: &mut Builder<'a>) -> Self {
        Self {
            title_text: inst.get_str("TitleText").map(String::from).unwrap_or_default(),
            body_text: inst.get_str("BodyText").map(String::from).unwrap_or_default(),
            body_text_game_pad: inst.get_str("BodyTextGamePad").map(String::from).unwrap_or_default(),
            body_text_joystick: inst.get_str("BodyTextJoystick").map(String::from).unwrap_or_default(),
            title_image_path: inst.get_str("TitleImagePath").map(String::from).unwrap_or_default(),
            body_image_path: inst.get_str("BodyImagePath").map(String::from).unwrap_or_default(),
        }
    }
}

/// DCB type: `HintTriggerData`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintTriggerData {
    /// `Hint` (Reference)
    #[serde(default)]
    pub hint: Option<CigGuid>,
    /// `appLink` (Class)
    #[serde(default)]
    pub app_link: Option<Handle<SMobiGlasAppLink>>,
    /// `HintEntityComponentTrigger` (Boolean)
    #[serde(default)]
    pub hint_entity_component_trigger: bool,
    /// `TriggerEvent` (EnumChoice)
    #[serde(default)]
    pub trigger_event: String,
    /// `UntriggerEvent` (EnumChoice)
    #[serde(default)]
    pub untrigger_event: String,
    /// `NumEventsToTrigger` (Int32)
    #[serde(default)]
    pub num_events_to_trigger: i32,
    /// `TriggerInSCPersistentUniverse` (Boolean)
    #[serde(default)]
    pub trigger_in_scpersistent_universe: bool,
    /// `TriggerInS42Default` (Boolean)
    #[serde(default)]
    pub trigger_in_s42_default: bool,
    /// `TriggerInSCHanger` (Boolean)
    #[serde(default)]
    pub trigger_in_schanger: bool,
    /// `TriggerInEABattleRoyale` (Boolean)
    #[serde(default)]
    pub trigger_in_eabattle_royale: bool,
    /// `TriggerInEAFreeFlight` (Boolean)
    #[serde(default)]
    pub trigger_in_eafree_flight: bool,
    /// `TriggerInEASquadronBattle` (Boolean)
    #[serde(default)]
    pub trigger_in_easquadron_battle: bool,
    /// `TriggerInEASwarm` (Boolean)
    #[serde(default)]
    pub trigger_in_easwarm: bool,
    /// `TriggerInEAPirateSwarm` (Boolean)
    #[serde(default)]
    pub trigger_in_eapirate_swarm: bool,
    /// `TriggerInEAClassicRace` (Boolean)
    #[serde(default)]
    pub trigger_in_eaclassic_race: bool,
    /// `TriggerInEAGravRace` (Boolean)
    #[serde(default)]
    pub trigger_in_eagrav_race: bool,
    /// `TriggerInEAElimination` (Boolean)
    #[serde(default)]
    pub trigger_in_eaelimination: bool,
    /// `TriggerInEATeamElimination` (Boolean)
    #[serde(default)]
    pub trigger_in_eateam_elimination: bool,
    /// `TriggerInEAControl` (Boolean)
    #[serde(default)]
    pub trigger_in_eacontrol: bool,
    /// `TriggerInEATheatersOfWar` (Boolean)
    #[serde(default)]
    pub trigger_in_eatheaters_of_war: bool,
    /// `TriggerInEADuel` (Boolean)
    #[serde(default)]
    pub trigger_in_eaduel: bool,
    /// `TriggerInEAFPSGunGame` (Boolean)
    #[serde(default)]
    pub trigger_in_eafpsgun_game: bool,
    /// `TriggerInEAHorde` (Boolean)
    #[serde(default)]
    pub trigger_in_eahorde: bool,
    /// `TriggerInEAExperimentalModes` (Boolean)
    #[serde(default)]
    pub trigger_in_eaexperimental_modes: bool,
    /// `TriggerInSCFrontend` (Boolean)
    #[serde(default)]
    pub trigger_in_scfrontend: bool,
    /// `TriggerInFloorDemo` (Boolean)
    #[serde(default)]
    pub trigger_in_floor_demo: bool,
    /// `SuppressionEvent` (EnumChoice)
    #[serde(default)]
    pub suppression_event: String,
    /// `MouseHint` (Boolean)
    #[serde(default)]
    pub mouse_hint: bool,
    /// `KeyboardHint` (Boolean)
    #[serde(default)]
    pub keyboard_hint: bool,
    /// `GamepadHint` (Boolean)
    #[serde(default)]
    pub gamepad_hint: bool,
    /// `JoystickHint` (Boolean)
    #[serde(default)]
    pub joystick_hint: bool,
    /// `HintAllowedOnFoot` (Boolean)
    #[serde(default)]
    pub hint_allowed_on_foot: bool,
    /// `HintAllowedOnGroundVehicles` (Boolean)
    #[serde(default)]
    pub hint_allowed_on_ground_vehicles: bool,
    /// `HintAllowedOnSpaceships` (Boolean)
    #[serde(default)]
    pub hint_allowed_on_spaceships: bool,
    /// `HintIsDemoOnly` (Boolean)
    #[serde(default)]
    pub hint_is_demo_only: bool,
    /// `TriggerTimer` (Single)
    #[serde(default)]
    pub trigger_timer: f32,
    /// `DisplayTimeOverride` (Single)
    #[serde(default)]
    pub display_time_override: f32,
    /// `MaxRepeats` (Int32)
    #[serde(default)]
    pub max_repeats: i32,
    /// `TimeToRepeat` (Single)
    #[serde(default)]
    pub time_to_repeat: f32,
    /// `InfiniteTriggers` (Boolean)
    #[serde(default)]
    pub infinite_triggers: bool,
    /// `suppressible` (Boolean)
    #[serde(default)]
    pub suppressible: bool,
    /// `Priority` (Int32)
    #[serde(default)]
    pub priority: i32,
    /// `TriggerOnComplete` (Reference)
    #[serde(default)]
    pub trigger_on_complete: Option<CigGuid>,
    /// `TriggerInTutorial` (Boolean)
    #[serde(default)]
    pub trigger_in_tutorial: bool,
    /// `dialogueContexts` (Reference (array))
    #[serde(default)]
    pub dialogue_contexts: Vec<CigGuid>,
}

impl Pooled for HintTriggerData {
    fn pool(pools: &DataPools) -> &Vec<Option<Self>> { &pools.hints.hint_trigger_data }
    fn pool_mut(pools: &mut DataPools) -> &mut Vec<Option<Self>> { &mut pools.hints.hint_trigger_data }
}

impl<'a> Extract<'a> for HintTriggerData {
    const TYPE_NAME: &'static str = "HintTriggerData";
    fn extract(inst: &Instance<'a>, b: &mut Builder<'a>) -> Self {
        Self {
            hint: inst.get("Hint").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            app_link: match inst.get("appLink") {
                Some(Value::Class { struct_index, data }) => Some(b.alloc_nested::<SMobiGlasAppLink>(Instance::from_inline_data(b.db, struct_index, data), false)),
                Some(Value::ClassRef(r))
                | Some(Value::StrongPointer(Some(r)))
                | Some(Value::WeakPointer(Some(r))) => Some(b.alloc_nested::<SMobiGlasAppLink>(b.db.instance(r.struct_index, r.instance_index), true)),
                _ => None,
            },
            hint_entity_component_trigger: inst.get_bool("HintEntityComponentTrigger").unwrap_or_default(),
            trigger_event: inst.get_str("TriggerEvent").map(String::from).unwrap_or_default(),
            untrigger_event: inst.get_str("UntriggerEvent").map(String::from).unwrap_or_default(),
            num_events_to_trigger: inst.get_i32("NumEventsToTrigger").unwrap_or_default(),
            trigger_in_scpersistent_universe: inst.get_bool("TriggerInSCPersistentUniverse").unwrap_or_default(),
            trigger_in_s42_default: inst.get_bool("TriggerInS42Default").unwrap_or_default(),
            trigger_in_schanger: inst.get_bool("TriggerInSCHanger").unwrap_or_default(),
            trigger_in_eabattle_royale: inst.get_bool("TriggerInEABattleRoyale").unwrap_or_default(),
            trigger_in_eafree_flight: inst.get_bool("TriggerInEAFreeFlight").unwrap_or_default(),
            trigger_in_easquadron_battle: inst.get_bool("TriggerInEASquadronBattle").unwrap_or_default(),
            trigger_in_easwarm: inst.get_bool("TriggerInEASwarm").unwrap_or_default(),
            trigger_in_eapirate_swarm: inst.get_bool("TriggerInEAPirateSwarm").unwrap_or_default(),
            trigger_in_eaclassic_race: inst.get_bool("TriggerInEAClassicRace").unwrap_or_default(),
            trigger_in_eagrav_race: inst.get_bool("TriggerInEAGravRace").unwrap_or_default(),
            trigger_in_eaelimination: inst.get_bool("TriggerInEAElimination").unwrap_or_default(),
            trigger_in_eateam_elimination: inst.get_bool("TriggerInEATeamElimination").unwrap_or_default(),
            trigger_in_eacontrol: inst.get_bool("TriggerInEAControl").unwrap_or_default(),
            trigger_in_eatheaters_of_war: inst.get_bool("TriggerInEATheatersOfWar").unwrap_or_default(),
            trigger_in_eaduel: inst.get_bool("TriggerInEADuel").unwrap_or_default(),
            trigger_in_eafpsgun_game: inst.get_bool("TriggerInEAFPSGunGame").unwrap_or_default(),
            trigger_in_eahorde: inst.get_bool("TriggerInEAHorde").unwrap_or_default(),
            trigger_in_eaexperimental_modes: inst.get_bool("TriggerInEAExperimentalModes").unwrap_or_default(),
            trigger_in_scfrontend: inst.get_bool("TriggerInSCFrontend").unwrap_or_default(),
            trigger_in_floor_demo: inst.get_bool("TriggerInFloorDemo").unwrap_or_default(),
            suppression_event: inst.get_str("SuppressionEvent").map(String::from).unwrap_or_default(),
            mouse_hint: inst.get_bool("MouseHint").unwrap_or_default(),
            keyboard_hint: inst.get_bool("KeyboardHint").unwrap_or_default(),
            gamepad_hint: inst.get_bool("GamepadHint").unwrap_or_default(),
            joystick_hint: inst.get_bool("JoystickHint").unwrap_or_default(),
            hint_allowed_on_foot: inst.get_bool("HintAllowedOnFoot").unwrap_or_default(),
            hint_allowed_on_ground_vehicles: inst.get_bool("HintAllowedOnGroundVehicles").unwrap_or_default(),
            hint_allowed_on_spaceships: inst.get_bool("HintAllowedOnSpaceships").unwrap_or_default(),
            hint_is_demo_only: inst.get_bool("HintIsDemoOnly").unwrap_or_default(),
            trigger_timer: inst.get_f32("TriggerTimer").unwrap_or_default(),
            display_time_override: inst.get_f32("DisplayTimeOverride").unwrap_or_default(),
            max_repeats: inst.get_i32("MaxRepeats").unwrap_or_default(),
            time_to_repeat: inst.get_f32("TimeToRepeat").unwrap_or_default(),
            infinite_triggers: inst.get_bool("InfiniteTriggers").unwrap_or_default(),
            suppressible: inst.get_bool("suppressible").unwrap_or_default(),
            priority: inst.get_i32("Priority").unwrap_or_default(),
            trigger_on_complete: inst.get("TriggerOnComplete").and_then(|v| v.as_record_ref()).map(|r| r.guid),
            trigger_in_tutorial: inst.get_bool("TriggerInTutorial").unwrap_or_default(),
            dialogue_contexts: inst.get_array("dialogueContexts")
                .map(|arr| arr.filter_map(|v| if let Value::Reference(Some(r)) = v { Some(r.guid) } else { None }).collect())
                .unwrap_or_default(),
        }
    }
}

