// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Top-level `DataPools` composing per-feature pool sub-structs.

#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataPools {
    #[serde(default)]
    pub core: super::core::CorePools,
    #[cfg(feature = "actor-actorledgegrabbing")]
    #[serde(default)]
    pub actor_actorledgegrabbing: super::actor_actorledgegrabbing::ActorActorledgegrabbingPools,
    #[cfg(feature = "actor-actortargetedparams")]
    #[serde(default)]
    pub actor_actortargetedparams: super::actor_actortargetedparams::ActorActortargetedparamsPools,
    #[cfg(feature = "actor-actorviewlimits")]
    #[serde(default)]
    pub actor_actorviewlimits: super::actor_actorviewlimits::ActorActorviewlimitsPools,
    #[cfg(feature = "actor-actorzerogtraversalparams")]
    #[serde(default)]
    pub actor_actorzerogtraversalparams: super::actor_actorzerogtraversalparams::ActorActorzerogtraversalparamsPools,
    #[cfg(feature = "actor-atls_geo_jumppackconfig")]
    #[serde(default)]
    pub actor_atls_geo_jumppackconfig: super::actor_atls_geo_jumppackconfig::ActorAtls_geo_jumppackconfigPools,
    #[cfg(feature = "actor-externalforceresponse")]
    #[serde(default)]
    pub actor_externalforceresponse: super::actor_externalforceresponse::ActorExternalforceresponsePools,
    #[cfg(feature = "actor-gforce")]
    #[serde(default)]
    pub actor_gforce: super::actor_gforce::ActorGforcePools,
    #[cfg(feature = "actor-headtrackinglimits")]
    #[serde(default)]
    pub actor_headtrackinglimits: super::actor_headtrackinglimits::ActorHeadtrackinglimitsPools,
    #[cfg(feature = "actor-inputdeflectiontime")]
    #[serde(default)]
    pub actor_inputdeflectiontime: super::actor_inputdeflectiontime::ActorInputdeflectiontimePools,
    #[cfg(feature = "actor-locomotionpersonality")]
    #[serde(default)]
    pub actor_locomotionpersonality: super::actor_locomotionpersonality::ActorLocomotionpersonalityPools,
    #[cfg(feature = "actor-lookahead")]
    #[serde(default)]
    pub actor_lookahead: super::actor_lookahead::ActorLookaheadPools,
    #[cfg(feature = "actor-playeranimatedinteractionconfig")]
    #[serde(default)]
    pub actor_playeranimatedinteractionconfig: super::actor_playeranimatedinteractionconfig::ActorPlayeranimatedinteractionconfigPools,
    #[cfg(feature = "actor-playerdefaultactionsconfig")]
    #[serde(default)]
    pub actor_playerdefaultactionsconfig: super::actor_playerdefaultactionsconfig::ActorPlayerdefaultactionsconfigPools,
    #[cfg(feature = "actor-quantumtravelcameraeffects")]
    #[serde(default)]
    pub actor_quantumtravelcameraeffects: super::actor_quantumtravelcameraeffects::ActorQuantumtravelcameraeffectsPools,
    #[cfg(feature = "actor-skeletonconfigs")]
    #[serde(default)]
    pub actor_skeletonconfigs: super::actor_skeletonconfigs::ActorSkeletonconfigsPools,
    #[cfg(feature = "actor-stanceinfo")]
    #[serde(default)]
    pub actor_stanceinfo: super::actor_stanceinfo::ActorStanceinfoPools,
    #[cfg(feature = "actor-targettrackingautozoom")]
    #[serde(default)]
    pub actor_targettrackingautozoom: super::actor_targettrackingautozoom::ActorTargettrackingautozoomPools,
    #[cfg(feature = "ads")]
    #[serde(default)]
    pub ads: super::ads::AdsPools,
    #[cfg(feature = "aianimationdata")]
    #[serde(default)]
    pub aianimationdata: super::aianimationdata::AianimationdataPools,
    #[cfg(feature = "aiglobalsettings")]
    #[serde(default)]
    pub aiglobalsettings: super::aiglobalsettings::AiglobalsettingsPools,
    #[cfg(feature = "aimotive")]
    #[serde(default)]
    pub aimotive: super::aimotive::AimotivePools,
    #[cfg(feature = "aiprofile")]
    #[serde(default)]
    pub aiprofile: super::aiprofile::AiprofilePools,
    #[cfg(feature = "aiwavecollection")]
    #[serde(default)]
    pub aiwavecollection: super::aiwavecollection::AiwavecollectionPools,
    #[cfg(feature = "ammoparams")]
    #[serde(default)]
    pub ammoparams: super::ammoparams::AmmoparamsPools,
    #[cfg(feature = "analytics")]
    #[serde(default)]
    pub analytics: super::analytics::AnalyticsPools,
    #[cfg(feature = "announcer")]
    #[serde(default)]
    pub announcer: super::announcer::AnnouncerPools,
    #[cfg(feature = "areaservices")]
    #[serde(default)]
    pub areaservices: super::areaservices::AreaservicesPools,
    #[cfg(feature = "audio")]
    #[serde(default)]
    pub audio: super::audio::AudioPools,
    #[cfg(feature = "awardservice")]
    #[serde(default)]
    pub awardservice: super::awardservice::AwardservicePools,
    #[cfg(feature = "cameras")]
    #[serde(default)]
    pub cameras: super::cameras::CamerasPools,
    #[cfg(feature = "capacitorassignment")]
    #[serde(default)]
    pub capacitorassignment: super::capacitorassignment::CapacitorassignmentPools,
    #[cfg(feature = "cargomanifest")]
    #[serde(default)]
    pub cargomanifest: super::cargomanifest::CargomanifestPools,
    #[cfg(feature = "character")]
    #[serde(default)]
    pub character: super::character::CharacterPools,
    #[cfg(feature = "chatchannelfilters")]
    #[serde(default)]
    pub chatchannelfilters: super::chatchannelfilters::ChatchannelfiltersPools,
    #[cfg(feature = "chatcommandfastaccess")]
    #[serde(default)]
    pub chatcommandfastaccess: super::chatcommandfastaccess::ChatcommandfastaccessPools,
    #[cfg(feature = "chatemoterecord")]
    #[serde(default)]
    pub chatemoterecord: super::chatemoterecord::ChatemoterecordPools,
    #[cfg(feature = "chatfilteroptions")]
    #[serde(default)]
    pub chatfilteroptions: super::chatfilteroptions::ChatfilteroptionsPools,
    #[cfg(feature = "chatmanager")]
    #[serde(default)]
    pub chatmanager: super::chatmanager::ChatmanagerPools,
    #[cfg(feature = "closecombat")]
    #[serde(default)]
    pub closecombat: super::closecombat::ClosecombatPools,
    #[cfg(feature = "commodityconfiguration")]
    #[serde(default)]
    pub commodityconfiguration: super::commodityconfiguration::CommodityconfigurationPools,
    #[cfg(feature = "commoditytypedatabase")]
    #[serde(default)]
    pub commoditytypedatabase: super::commoditytypedatabase::CommoditytypedatabasePools,
    #[cfg(feature = "commsnotifications")]
    #[serde(default)]
    pub commsnotifications: super::commsnotifications::CommsnotificationsPools,
    #[cfg(feature = "communicationatlconfig")]
    #[serde(default)]
    pub communicationatlconfig: super::communicationatlconfig::CommunicationatlconfigPools,
    #[cfg(feature = "communicationconfig")]
    #[serde(default)]
    pub communicationconfig: super::communicationconfig::CommunicationconfigPools,
    #[cfg(feature = "communicationname")]
    #[serde(default)]
    pub communicationname: super::communicationname::CommunicationnamePools,
    #[cfg(feature = "communicationsystem")]
    #[serde(default)]
    pub communicationsystem: super::communicationsystem::CommunicationsystemPools,
    #[cfg(feature = "communicationvariableconfig")]
    #[serde(default)]
    pub communicationvariableconfig: super::communicationvariableconfig::CommunicationvariableconfigPools,
    #[cfg(feature = "consumabletypesdatabase")]
    #[serde(default)]
    pub consumabletypesdatabase: super::consumabletypesdatabase::ConsumabletypesdatabasePools,
    #[cfg(feature = "contextualcommunicationconfig")]
    #[serde(default)]
    pub contextualcommunicationconfig: super::contextualcommunicationconfig::ContextualcommunicationconfigPools,
    #[cfg(feature = "contracts")]
    #[serde(default)]
    pub contracts: super::contracts::ContractsPools,
    #[cfg(feature = "conversation")]
    #[serde(default)]
    pub conversation: super::conversation::ConversationPools,
    #[cfg(feature = "crafting")]
    #[serde(default)]
    pub crafting: super::crafting::CraftingPools,
    #[cfg(feature = "crewmanifest")]
    #[serde(default)]
    pub crewmanifest: super::crewmanifest::CrewmanifestPools,
    #[cfg(feature = "damage")]
    #[serde(default)]
    pub damage: super::damage::DamagePools,
    #[cfg(feature = "densityclasses")]
    #[serde(default)]
    pub densityclasses: super::densityclasses::DensityclassesPools,
    #[cfg(feature = "dialoguecontentbank")]
    #[serde(default)]
    pub dialoguecontentbank: super::dialoguecontentbank::DialoguecontentbankPools,
    #[cfg(feature = "dialoguerealm")]
    #[serde(default)]
    pub dialoguerealm: super::dialoguerealm::DialoguerealmPools,
    #[cfg(feature = "dynamiccameraeffects")]
    #[serde(default)]
    pub dynamiccameraeffects: super::dynamiccameraeffects::DynamiccameraeffectsPools,
    #[cfg(feature = "dynamiclightingrig")]
    #[serde(default)]
    pub dynamiclightingrig: super::dynamiclightingrig::DynamiclightingrigPools,
    #[cfg(feature = "emotions")]
    #[serde(default)]
    pub emotions: super::emotions::EmotionsPools,
    #[cfg(feature = "entities-haulingentityclass")]
    #[serde(default)]
    pub entities_haulingentityclass: super::entities_haulingentityclass::EntitiesHaulingentityclassPools,
    #[cfg(feature = "entities-scitem-actormovables")]
    #[serde(default)]
    pub entities_scitem_actormovables: super::entities_scitem_actormovables::EntitiesScitemActormovablesPools,
    #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
    #[serde(default)]
    pub entities_scitem_mastermodeexclusionglobalparams: super::entities_scitem_mastermodeexclusionglobalparams::EntitiesScitemMastermodeexclusionglobalparamsPools,
    #[cfg(feature = "entities-scitem-operatormoderecords")]
    #[serde(default)]
    pub entities_scitem_operatormoderecords: super::entities_scitem_operatormoderecords::EntitiesScitemOperatormoderecordsPools,
    #[cfg(feature = "entities-scitem-ships")]
    #[serde(default)]
    pub entities_scitem_ships: super::entities_scitem_ships::EntitiesScitemShipsPools,
    #[cfg(feature = "entities-scitem-usables")]
    #[serde(default)]
    pub entities_scitem_usables: super::entities_scitem_usables::EntitiesScitemUsablesPools,
    #[cfg(feature = "entities-ui")]
    #[serde(default)]
    pub entities_ui: super::entities_ui::EntitiesUiPools,
    #[cfg(feature = "entities-vfx")]
    #[serde(default)]
    pub entities_vfx: super::entities_vfx::EntitiesVfxPools,
    #[cfg(feature = "entitlementpolicies")]
    #[serde(default)]
    pub entitlementpolicies: super::entitlementpolicies::EntitlementpoliciesPools,
    #[cfg(feature = "environments")]
    #[serde(default)]
    pub environments: super::environments::EnvironmentsPools,
    #[cfg(feature = "evagraph")]
    #[serde(default)]
    pub evagraph: super::evagraph::EvagraphPools,
    #[cfg(feature = "explosiveordnance")]
    #[serde(default)]
    pub explosiveordnance: super::explosiveordnance::ExplosiveordnancePools,
    #[cfg(feature = "factions")]
    #[serde(default)]
    pub factions: super::factions::FactionsPools,
    #[cfg(feature = "fidgetconfig")]
    #[serde(default)]
    pub fidgetconfig: super::fidgetconfig::FidgetconfigPools,
    #[cfg(feature = "foley")]
    #[serde(default)]
    pub foley: super::foley::FoleyPools,
    #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
    #[serde(default)]
    pub forcefeedback_forcefeedbackeffects: super::forcefeedback_forcefeedbackeffects::Forcefeedback_forcefeedbackeffectsPools,
    #[cfg(feature = "formation")]
    #[serde(default)]
    pub formation: super::formation::FormationPools,
    #[cfg(feature = "friendmanager")]
    #[serde(default)]
    pub friendmanager: super::friendmanager::FriendmanagerPools,
    #[cfg(feature = "fuelparams")]
    #[serde(default)]
    pub fuelparams: super::fuelparams::FuelparamsPools,
    #[cfg(feature = "gamemode")]
    #[serde(default)]
    pub gamemode: super::gamemode::GamemodePools,
    #[cfg(feature = "gamemodule")]
    #[serde(default)]
    pub gamemodule: super::gamemodule::GamemodulePools,
    #[cfg(feature = "globalarmarkerparams")]
    #[serde(default)]
    pub globalarmarkerparams: super::globalarmarkerparams::GlobalarmarkerparamsPools,
    #[cfg(feature = "globalcargoloadingparams")]
    #[serde(default)]
    pub globalcargoloadingparams: super::globalcargoloadingparams::GlobalcargoloadingparamsPools,
    #[cfg(feature = "globalcommsnotificationparams")]
    #[serde(default)]
    pub globalcommsnotificationparams: super::globalcommsnotificationparams::GlobalcommsnotificationparamsPools,
    #[cfg(feature = "globalcuttableshapeparams")]
    #[serde(default)]
    pub globalcuttableshapeparams: super::globalcuttableshapeparams::GlobalcuttableshapeparamsPools,
    #[cfg(feature = "globalinteractionparams")]
    #[serde(default)]
    pub globalinteractionparams: super::globalinteractionparams::GlobalinteractionparamsPools,
    #[cfg(feature = "globalinventoryparams")]
    #[serde(default)]
    pub globalinventoryparams: super::globalinventoryparams::GlobalinventoryparamsPools,
    #[cfg(feature = "globalquantumdriveparams")]
    #[serde(default)]
    pub globalquantumdriveparams: super::globalquantumdriveparams::GlobalquantumdriveparamsPools,
    #[cfg(feature = "globalshopparams")]
    #[serde(default)]
    pub globalshopparams: super::globalshopparams::GlobalshopparamsPools,
    #[cfg(feature = "globaltutorialparams")]
    #[serde(default)]
    pub globaltutorialparams: super::globaltutorialparams::GlobaltutorialparamsPools,
    #[cfg(feature = "grips")]
    #[serde(default)]
    pub grips: super::grips::GripsPools,
    #[cfg(feature = "handholdgripdatabase")]
    #[serde(default)]
    pub handholdgripdatabase: super::handholdgripdatabase::HandholdgripdatabasePools,
    #[cfg(feature = "hardwaremouse")]
    #[serde(default)]
    pub hardwaremouse: super::hardwaremouse::HardwaremousePools,
    #[cfg(feature = "harvestable")]
    #[serde(default)]
    pub harvestable: super::harvestable::HarvestablePools,
    #[cfg(feature = "hazardawarenessparams")]
    #[serde(default)]
    pub hazardawarenessparams: super::hazardawarenessparams::HazardawarenessparamsPools,
    #[cfg(feature = "hints")]
    #[serde(default)]
    pub hints: super::hints::HintsPools,
    #[cfg(feature = "hudparams")]
    #[serde(default)]
    pub hudparams: super::hudparams::HudparamsPools,
    #[cfg(feature = "ifcs")]
    #[serde(default)]
    pub ifcs: super::ifcs::IfcsPools,
    #[cfg(feature = "initialdamageoverrides")]
    #[serde(default)]
    pub initialdamageoverrides: super::initialdamageoverrides::InitialdamageoverridesPools,
    #[cfg(feature = "inputpromptconfig")]
    #[serde(default)]
    pub inputpromptconfig: super::inputpromptconfig::InputpromptconfigPools,
    #[cfg(feature = "instancedinterior")]
    #[serde(default)]
    pub instancedinterior: super::instancedinterior::InstancedinteriorPools,
    #[cfg(feature = "interactionconditions")]
    #[serde(default)]
    pub interactionconditions: super::interactionconditions::InteractionconditionsPools,
    #[cfg(feature = "intoxication")]
    #[serde(default)]
    pub intoxication: super::intoxication::IntoxicationPools,
    #[cfg(feature = "inventorycontainers")]
    #[serde(default)]
    pub inventorycontainers: super::inventorycontainers::InventorycontainersPools,
    #[cfg(feature = "item")]
    #[serde(default)]
    pub item: super::item::ItemPools,
    #[cfg(feature = "journalentry")]
    #[serde(default)]
    pub journalentry: super::journalentry::JournalentryPools,
    #[cfg(feature = "jumppoints")]
    #[serde(default)]
    pub jumppoints: super::jumppoints::JumppointsPools,
    #[cfg(feature = "landingpadsize")]
    #[serde(default)]
    pub landingpadsize: super::landingpadsize::LandingpadsizePools,
    #[cfg(feature = "lawsystem")]
    #[serde(default)]
    pub lawsystem: super::lawsystem::LawsystemPools,
    #[cfg(feature = "leangraph")]
    #[serde(default)]
    pub leangraph: super::leangraph::LeangraphPools,
    #[cfg(feature = "level")]
    #[serde(default)]
    pub level: super::level::LevelPools,
    #[cfg(feature = "loadoutkits")]
    #[serde(default)]
    pub loadoutkits: super::loadoutkits::LoadoutkitsPools,
    #[cfg(feature = "longtermpersistence")]
    #[serde(default)]
    pub longtermpersistence: super::longtermpersistence::LongtermpersistencePools,
    #[cfg(feature = "lootgeneration")]
    #[serde(default)]
    pub lootgeneration: super::lootgeneration::LootgenerationPools,
    #[cfg(feature = "megamap")]
    #[serde(default)]
    pub megamap: super::megamap::MegamapPools,
    #[cfg(feature = "mining")]
    #[serde(default)]
    pub mining: super::mining::MiningPools,
    #[cfg(feature = "missionbroker")]
    #[serde(default)]
    pub missionbroker: super::missionbroker::MissionbrokerPools,
    #[cfg(feature = "missiondata")]
    #[serde(default)]
    pub missiondata: super::missiondata::MissiondataPools,
    #[cfg(feature = "missionfailureconditions")]
    #[serde(default)]
    pub missionfailureconditions: super::missionfailureconditions::MissionfailureconditionsPools,
    #[cfg(feature = "missiongiver")]
    #[serde(default)]
    pub missiongiver: super::missiongiver::MissiongiverPools,
    #[cfg(feature = "missionscenarios")]
    #[serde(default)]
    pub missionscenarios: super::missionscenarios::MissionscenariosPools,
    #[cfg(feature = "missiontype")]
    #[serde(default)]
    pub missiontype: super::missiontype::MissiontypePools,
    #[cfg(feature = "motionstatemachine")]
    #[serde(default)]
    pub motionstatemachine: super::motionstatemachine::MotionstatemachinePools,
    #[cfg(feature = "moveviewrestrictionpenalties")]
    #[serde(default)]
    pub moveviewrestrictionpenalties: super::moveviewrestrictionpenalties::MoveviewrestrictionpenaltiesPools,
    #[cfg(feature = "musiclogic")]
    #[serde(default)]
    pub musiclogic: super::musiclogic::MusiclogicPools,
    #[cfg(feature = "personalinnerthoughtrules")]
    #[serde(default)]
    pub personalinnerthoughtrules: super::personalinnerthoughtrules::PersonalinnerthoughtrulesPools,
    #[cfg(feature = "planetdaynighttemperatureparams")]
    #[serde(default)]
    pub planetdaynighttemperatureparams: super::planetdaynighttemperatureparams::PlanetdaynighttemperatureparamsPools,
    #[cfg(feature = "posturedatabase")]
    #[serde(default)]
    pub posturedatabase: super::posturedatabase::PosturedatabasePools,
    #[cfg(feature = "procbreathing")]
    #[serde(default)]
    pub procbreathing: super::procbreathing::ProcbreathingPools,
    #[cfg(feature = "proceduralaimrigrecord")]
    #[serde(default)]
    pub proceduralaimrigrecord: super::proceduralaimrigrecord::ProceduralaimrigrecordPools,
    #[cfg(feature = "proceduralanimations")]
    #[serde(default)]
    pub proceduralanimations: super::proceduralanimations::ProceduralanimationsPools,
    #[cfg(feature = "procedurallandingsetup")]
    #[serde(default)]
    pub procedurallandingsetup: super::procedurallandingsetup::ProcedurallandingsetupPools,
    #[cfg(feature = "procedurallayout")]
    #[serde(default)]
    pub procedurallayout: super::procedurallayout::ProcedurallayoutPools,
    #[cfg(feature = "qteconfigs")]
    #[serde(default)]
    pub qteconfigs: super::qteconfigs::QteconfigsPools,
    #[cfg(feature = "radarsystem")]
    #[serde(default)]
    pub radarsystem: super::radarsystem::RadarsystemPools,
    #[cfg(feature = "rastar")]
    #[serde(default)]
    pub rastar: super::rastar::RastarPools,
    #[cfg(feature = "refinerynotificationconfiguration")]
    #[serde(default)]
    pub refinerynotificationconfiguration: super::refinerynotificationconfiguration::RefinerynotificationconfigurationPools,
    #[cfg(feature = "refiningprocess")]
    #[serde(default)]
    pub refiningprocess: super::refiningprocess::RefiningprocessPools,
    #[cfg(feature = "rendererpresets")]
    #[serde(default)]
    pub rendererpresets: super::rendererpresets::RendererpresetsPools,
    #[cfg(feature = "rentalnotificationparams")]
    #[serde(default)]
    pub rentalnotificationparams: super::rentalnotificationparams::RentalnotificationparamsPools,
    #[cfg(feature = "reputation")]
    #[serde(default)]
    pub reputation: super::reputation::ReputationPools,
    #[cfg(feature = "resourcetypedatabase")]
    #[serde(default)]
    pub resourcetypedatabase: super::resourcetypedatabase::ResourcetypedatabasePools,
    #[cfg(feature = "roomsystem")]
    #[serde(default)]
    pub roomsystem: super::roomsystem::RoomsystemPools,
    #[cfg(feature = "scarryableikinteractionlist")]
    #[serde(default)]
    pub scarryableikinteractionlist: super::scarryableikinteractionlist::ScarryableikinteractionlistPools,
    #[cfg(feature = "scitemcommscomponentsetup")]
    #[serde(default)]
    pub scitemcommscomponentsetup: super::scitemcommscomponentsetup::ScitemcommscomponentsetupPools,
    #[cfg(feature = "scitemdisplayscreenpreset")]
    #[serde(default)]
    pub scitemdisplayscreenpreset: super::scitemdisplayscreenpreset::ScitemdisplayscreenpresetPools,
    #[cfg(feature = "scitemmanufacturer")]
    #[serde(default)]
    pub scitemmanufacturer: super::scitemmanufacturer::ScitemmanufacturerPools,
    #[cfg(feature = "scuttableshapedefinition")]
    #[serde(default)]
    pub scuttableshapedefinition: super::scuttableshapedefinition::ScuttableshapedefinitionPools,
    #[cfg(feature = "seatcdikconfigs")]
    #[serde(default)]
    pub seatcdikconfigs: super::seatcdikconfigs::SeatcdikconfigsPools,
    #[cfg(feature = "servicebeacon")]
    #[serde(default)]
    pub servicebeacon: super::servicebeacon::ServicebeaconPools,
    #[cfg(feature = "sgeometryviewdistanceratiocategories")]
    #[serde(default)]
    pub sgeometryviewdistanceratiocategories: super::sgeometryviewdistanceratiocategories::SgeometryviewdistanceratiocategoriesPools,
    #[cfg(feature = "sglobalchargedrainbeamparams")]
    #[serde(default)]
    pub sglobalchargedrainbeamparams: super::sglobalchargedrainbeamparams::SglobalchargedrainbeamparamsPools,
    #[cfg(feature = "sglobalcrosshairparams")]
    #[serde(default)]
    pub sglobalcrosshairparams: super::sglobalcrosshairparams::SglobalcrosshairparamsPools,
    #[cfg(feature = "sglobalelectronparams")]
    #[serde(default)]
    pub sglobalelectronparams: super::sglobalelectronparams::SglobalelectronparamsPools,
    #[cfg(feature = "sglobalhealingbeamparams")]
    #[serde(default)]
    pub sglobalhealingbeamparams: super::sglobalhealingbeamparams::SglobalhealingbeamparamsPools,
    #[cfg(feature = "sglobalhitbehaviorparams")]
    #[serde(default)]
    pub sglobalhitbehaviorparams: super::sglobalhitbehaviorparams::SglobalhitbehaviorparamsPools,
    #[cfg(feature = "sglobalsalvagerepairbeamparams")]
    #[serde(default)]
    pub sglobalsalvagerepairbeamparams: super::sglobalsalvagerepairbeamparams::SglobalsalvagerepairbeamparamsPools,
    #[cfg(feature = "sglobaltractorbeamparams")]
    #[serde(default)]
    pub sglobaltractorbeamparams: super::sglobaltractorbeamparams::SglobaltractorbeamparamsPools,
    #[cfg(feature = "shipinsurancerecord")]
    #[serde(default)]
    pub shipinsurancerecord: super::shipinsurancerecord::ShipinsurancerecordPools,
    #[cfg(feature = "specialeventdatabase")]
    #[serde(default)]
    pub specialeventdatabase: super::specialeventdatabase::SpecialeventdatabasePools,
    #[cfg(feature = "squantumdriveeffecttagstemplate")]
    #[serde(default)]
    pub squantumdriveeffecttagstemplate: super::squantumdriveeffecttagstemplate::SquantumdriveeffecttagstemplatePools,
    #[cfg(feature = "sreputationglobalcontextbbparams")]
    #[serde(default)]
    pub sreputationglobalcontextbbparams: super::sreputationglobalcontextbbparams::SreputationglobalcontextbbparamsPools,
    #[cfg(feature = "ssolarsystem")]
    #[serde(default)]
    pub ssolarsystem: super::ssolarsystem::SsolarsystemPools,
    #[cfg(feature = "starmap")]
    #[serde(default)]
    pub starmap: super::starmap::StarmapPools,
    #[cfg(feature = "tacticalquery")]
    #[serde(default)]
    pub tacticalquery: super::tacticalquery::TacticalqueryPools,
    #[cfg(feature = "tagdatabase")]
    #[serde(default)]
    pub tagdatabase: super::tagdatabase::TagdatabasePools,
    #[cfg(feature = "targetselector")]
    #[serde(default)]
    pub targetselector: super::targetselector::TargetselectorPools,
    #[cfg(feature = "tintpalettes")]
    #[serde(default)]
    pub tintpalettes: super::tintpalettes::TintpalettesPools,
    #[cfg(feature = "trackview")]
    #[serde(default)]
    pub trackview: super::trackview::TrackviewPools,
    #[cfg(feature = "traversalcostconfig")]
    #[serde(default)]
    pub traversalcostconfig: super::traversalcostconfig::TraversalcostconfigPools,
    #[cfg(feature = "turret")]
    #[serde(default)]
    pub turret: super::turret::TurretPools,
    #[cfg(feature = "ui-animatedmarkers")]
    #[serde(default)]
    pub ui_animatedmarkers: super::ui_animatedmarkers::UiAnimatedmarkersPools,
    #[cfg(feature = "ui-armarkerconfiguration")]
    #[serde(default)]
    pub ui_armarkerconfiguration: super::ui_armarkerconfiguration::UiArmarkerconfigurationPools,
    #[cfg(feature = "ui-buildingblocks")]
    #[serde(default)]
    pub ui_buildingblocks: super::ui_buildingblocks::UiBuildingblocksPools,
    #[cfg(feature = "ui-digitalsignage")]
    #[serde(default)]
    pub ui_digitalsignage: super::ui_digitalsignage::UiDigitalsignagePools,
    #[cfg(feature = "ui-directrtt")]
    #[serde(default)]
    pub ui_directrtt: super::ui_directrtt::UiDirectrttPools,
    #[cfg(feature = "ui-dockingslotvisibility")]
    #[serde(default)]
    pub ui_dockingslotvisibility: super::ui_dockingslotvisibility::UiDockingslotvisibilityPools,
    #[cfg(feature = "ui-elementsounds_deprecated")]
    #[serde(default)]
    pub ui_elementsounds_deprecated: super::ui_elementsounds_deprecated::UiElementsounds_deprecatedPools,
    #[cfg(feature = "ui-flashobjectbindinggroups")]
    #[serde(default)]
    pub ui_flashobjectbindinggroups: super::ui_flashobjectbindinggroups::UiFlashobjectbindinggroupsPools,
    #[cfg(feature = "ui-flighthudmessages")]
    #[serde(default)]
    pub ui_flighthudmessages: super::ui_flighthudmessages::UiFlighthudmessagesPools,
    #[cfg(feature = "ui-frontend")]
    #[serde(default)]
    pub ui_frontend: super::ui_frontend::UiFrontendPools,
    #[cfg(feature = "ui-graphs")]
    #[serde(default)]
    pub ui_graphs: super::ui_graphs::UiGraphsPools,
    #[cfg(feature = "ui-holovehicleconfig")]
    #[serde(default)]
    pub ui_holovehicleconfig: super::ui_holovehicleconfig::UiHolovehicleconfigPools,
    #[cfg(feature = "ui-hudcolors_shipcolorpalettes")]
    #[serde(default)]
    pub ui_hudcolors_shipcolorpalettes: super::ui_hudcolors_shipcolorpalettes::UiHudcolors_shipcolorpalettesPools,
    #[cfg(feature = "ui-innerthought")]
    #[serde(default)]
    pub ui_innerthought: super::ui_innerthought::UiInnerthoughtPools,
    #[cfg(feature = "ui-itemkiosk")]
    #[serde(default)]
    pub ui_itemkiosk: super::ui_itemkiosk::UiItemkioskPools,
    #[cfg(feature = "ui-itempreview_config")]
    #[serde(default)]
    pub ui_itempreview_config: super::ui_itempreview_config::UiItempreview_configPools,
    #[cfg(feature = "ui-itemtypedefinition")]
    #[serde(default)]
    pub ui_itemtypedefinition: super::ui_itemtypedefinition::UiItemtypedefinitionPools,
    #[cfg(feature = "ui-localplayershoppingdata")]
    #[serde(default)]
    pub ui_localplayershoppingdata: super::ui_localplayershoppingdata::UiLocalplayershoppingdataPools,
    #[cfg(feature = "ui-marker_config")]
    #[serde(default)]
    pub ui_marker_config: super::ui_marker_config::UiMarker_configPools,
    #[cfg(feature = "ui-markertrackingvolumeconfig")]
    #[serde(default)]
    pub ui_markertrackingvolumeconfig: super::ui_markertrackingvolumeconfig::UiMarkertrackingvolumeconfigPools,
    #[cfg(feature = "ui-missilelockreticleconfig")]
    #[serde(default)]
    pub ui_missilelockreticleconfig: super::ui_missilelockreticleconfig::UiMissilelockreticleconfigPools,
    #[cfg(feature = "ui-mobiglas")]
    #[serde(default)]
    pub ui_mobiglas: super::ui_mobiglas::UiMobiglasPools,
    #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
    #[serde(default)]
    pub ui_objectdatabankentrymarkerconfig: super::ui_objectdatabankentrymarkerconfig::UiObjectdatabankentrymarkerconfigPools,
    #[cfg(feature = "ui-playerchoice_imconfig_playerchoiceim")]
    #[serde(default)]
    pub ui_playerchoice_imconfig_playerchoiceim: super::ui_playerchoice_imconfig_playerchoiceim::UiPlayerchoice_imconfig_playerchoiceimPools,
    #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
    #[serde(default)]
    pub ui_playerchoice_library_playerchoicelibrary: super::ui_playerchoice_library_playerchoicelibrary::UiPlayerchoice_library_playerchoicelibraryPools,
    #[cfg(feature = "ui-playerchoice_pitconfig_playerchoicepersonalthought")]
    #[serde(default)]
    pub ui_playerchoice_pitconfig_playerchoicepersonalthought: super::ui_playerchoice_pitconfig_playerchoicepersonalthought::UiPlayerchoice_pitconfig_playerchoicepersonalthoughtPools,
    #[cfg(feature = "ui-playerchoice_signalconfig_interactorsignalconfig")]
    #[serde(default)]
    pub ui_playerchoice_signalconfig_interactorsignalconfig: super::ui_playerchoice_signalconfig_interactorsignalconfig::UiPlayerchoice_signalconfig_interactorsignalconfigPools,
    #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
    #[serde(default)]
    pub ui_playerecggraph_config_playerecggraphconfig: super::ui_playerecggraph_config_playerecggraphconfig::UiPlayerecggraph_config_playerecggraphconfigPools,
    #[cfg(feature = "ui-pointofinterestdata")]
    #[serde(default)]
    pub ui_pointofinterestdata: super::ui_pointofinterestdata::UiPointofinterestdataPools,
    #[cfg(feature = "ui-popups")]
    #[serde(default)]
    pub ui_popups: super::ui_popups::UiPopupsPools,
    #[cfg(feature = "ui-radar3dpresets")]
    #[serde(default)]
    pub ui_radar3dpresets: super::ui_radar3dpresets::UiRadar3dpresetsPools,
    #[cfg(feature = "ui-radardisplay_config_radar")]
    #[serde(default)]
    pub ui_radardisplay_config_radar: super::ui_radardisplay_config_radar::UiRadardisplay_config_radarPools,
    #[cfg(feature = "ui-seatreticlearchetype")]
    #[serde(default)]
    pub ui_seatreticlearchetype: super::ui_seatreticlearchetype::UiSeatreticlearchetypePools,
    #[cfg(feature = "ui-transformationinterpolatorrecords")]
    #[serde(default)]
    pub ui_transformationinterpolatorrecords: super::ui_transformationinterpolatorrecords::UiTransformationinterpolatorrecordsPools,
    #[cfg(feature = "ui-uiconfig_starcitizen")]
    #[serde(default)]
    pub ui_uiconfig_starcitizen: super::ui_uiconfig_starcitizen::UiUiconfig_starcitizenPools,
    #[cfg(feature = "ui-uielements")]
    #[serde(default)]
    pub ui_uielements: super::ui_uielements::UiUielementsPools,
    #[cfg(feature = "ui-uimodes")]
    #[serde(default)]
    pub ui_uimodes: super::ui_uimodes::UiUimodesPools,
    #[cfg(feature = "ui-uistatedisplay")]
    #[serde(default)]
    pub ui_uistatedisplay: super::ui_uistatedisplay::UiUistatedisplayPools,
    #[cfg(feature = "ui-vehicleentrance")]
    #[serde(default)]
    pub ui_vehicleentrance: super::ui_vehicleentrance::UiVehicleentrancePools,
    #[cfg(feature = "ui-videocomms")]
    #[serde(default)]
    pub ui_videocomms: super::ui_videocomms::UiVideocommsPools,
    #[cfg(feature = "ui-visor")]
    #[serde(default)]
    pub ui_visor: super::ui_visor::UiVisorPools,
    #[cfg(feature = "ui-worlddisplay")]
    #[serde(default)]
    pub ui_worlddisplay: super::ui_worlddisplay::UiWorlddisplayPools,
    #[cfg(feature = "unittest_unittestb")]
    #[serde(default)]
    pub unittest_unittestb: super::unittest_unittestb::Unittest_unittestbPools,
    #[cfg(feature = "vehicle")]
    #[serde(default)]
    pub vehicle: super::vehicle::VehiclePools,
    #[cfg(feature = "vfx")]
    #[serde(default)]
    pub vfx: super::vfx::VfxPools,
    #[cfg(feature = "vibrations")]
    #[serde(default)]
    pub vibrations: super::vibrations::VibrationsPools,
    #[cfg(feature = "videocommschannels")]
    #[serde(default)]
    pub videocommschannels: super::videocommschannels::VideocommschannelsPools,
    #[cfg(feature = "voicebundle")]
    #[serde(default)]
    pub voicebundle: super::voicebundle::VoicebundlePools,
    #[cfg(feature = "voicechannelsettingsrecord")]
    #[serde(default)]
    pub voicechannelsettingsrecord: super::voicechannelsettingsrecord::VoicechannelsettingsrecordPools,
    #[cfg(feature = "voicesingle")]
    #[serde(default)]
    pub voicesingle: super::voicesingle::VoicesinglePools,
    #[cfg(feature = "weaponarmodifiers")]
    #[serde(default)]
    pub weaponarmodifiers: super::weaponarmodifiers::WeaponarmodifiersPools,
    #[cfg(feature = "weaponmisfiredef")]
    #[serde(default)]
    pub weaponmisfiredef: super::weaponmisfiredef::WeaponmisfiredefPools,
    #[cfg(feature = "weaponproceduralanimation")]
    #[serde(default)]
    pub weaponproceduralanimation: super::weaponproceduralanimation::WeaponproceduralanimationPools,
    #[cfg(feature = "weaponproceduralclip")]
    #[serde(default)]
    pub weaponproceduralclip: super::weaponproceduralclip::WeaponproceduralclipPools,
    #[cfg(feature = "weaponproceduralrecoil")]
    #[serde(default)]
    pub weaponproceduralrecoil: super::weaponproceduralrecoil::WeaponproceduralrecoilPools,
    #[cfg(feature = "zerogtraversalgraph")]
    #[serde(default)]
    pub zerogtraversalgraph: super::zerogtraversalgraph::ZerogtraversalgraphPools,
}
