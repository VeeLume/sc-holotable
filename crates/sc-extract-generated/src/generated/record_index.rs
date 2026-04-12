// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

//! Top-level `RecordIndex` composing per-feature index sub-structs.

#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIndex {
    #[serde(default)]
    pub core: super::core::CoreIndex,
    #[cfg(feature = "actor-actorledgegrabbing")]
    #[serde(default)]
    pub actor_actorledgegrabbing: super::actor_actorledgegrabbing::ActorActorledgegrabbingIndex,
    #[cfg(feature = "actor-actortargetedparams")]
    #[serde(default)]
    pub actor_actortargetedparams: super::actor_actortargetedparams::ActorActortargetedparamsIndex,
    #[cfg(feature = "actor-actorviewlimits")]
    #[serde(default)]
    pub actor_actorviewlimits: super::actor_actorviewlimits::ActorActorviewlimitsIndex,
    #[cfg(feature = "actor-actorzerogtraversalparams")]
    #[serde(default)]
    pub actor_actorzerogtraversalparams: super::actor_actorzerogtraversalparams::ActorActorzerogtraversalparamsIndex,
    #[cfg(feature = "actor-atls_geo_jumppackconfig")]
    #[serde(default)]
    pub actor_atls_geo_jumppackconfig: super::actor_atls_geo_jumppackconfig::ActorAtls_geo_jumppackconfigIndex,
    #[cfg(feature = "actor-externalforceresponse")]
    #[serde(default)]
    pub actor_externalforceresponse: super::actor_externalforceresponse::ActorExternalforceresponseIndex,
    #[cfg(feature = "actor-gforce")]
    #[serde(default)]
    pub actor_gforce: super::actor_gforce::ActorGforceIndex,
    #[cfg(feature = "actor-headtrackinglimits")]
    #[serde(default)]
    pub actor_headtrackinglimits: super::actor_headtrackinglimits::ActorHeadtrackinglimitsIndex,
    #[cfg(feature = "actor-inputdeflectiontime")]
    #[serde(default)]
    pub actor_inputdeflectiontime: super::actor_inputdeflectiontime::ActorInputdeflectiontimeIndex,
    #[cfg(feature = "actor-locomotionpersonality")]
    #[serde(default)]
    pub actor_locomotionpersonality: super::actor_locomotionpersonality::ActorLocomotionpersonalityIndex,
    #[cfg(feature = "actor-lookahead")]
    #[serde(default)]
    pub actor_lookahead: super::actor_lookahead::ActorLookaheadIndex,
    #[cfg(feature = "actor-playeranimatedinteractionconfig")]
    #[serde(default)]
    pub actor_playeranimatedinteractionconfig: super::actor_playeranimatedinteractionconfig::ActorPlayeranimatedinteractionconfigIndex,
    #[cfg(feature = "actor-playerdefaultactionsconfig")]
    #[serde(default)]
    pub actor_playerdefaultactionsconfig: super::actor_playerdefaultactionsconfig::ActorPlayerdefaultactionsconfigIndex,
    #[cfg(feature = "actor-quantumtravelcameraeffects")]
    #[serde(default)]
    pub actor_quantumtravelcameraeffects: super::actor_quantumtravelcameraeffects::ActorQuantumtravelcameraeffectsIndex,
    #[cfg(feature = "actor-skeletonconfigs")]
    #[serde(default)]
    pub actor_skeletonconfigs: super::actor_skeletonconfigs::ActorSkeletonconfigsIndex,
    #[cfg(feature = "actor-stanceinfo")]
    #[serde(default)]
    pub actor_stanceinfo: super::actor_stanceinfo::ActorStanceinfoIndex,
    #[cfg(feature = "actor-targettrackingautozoom")]
    #[serde(default)]
    pub actor_targettrackingautozoom: super::actor_targettrackingautozoom::ActorTargettrackingautozoomIndex,
    #[cfg(feature = "ads")]
    #[serde(default)]
    pub ads: super::ads::AdsIndex,
    #[cfg(feature = "aianimationdata")]
    #[serde(default)]
    pub aianimationdata: super::aianimationdata::AianimationdataIndex,
    #[cfg(feature = "aiglobalsettings")]
    #[serde(default)]
    pub aiglobalsettings: super::aiglobalsettings::AiglobalsettingsIndex,
    #[cfg(feature = "aimotive")]
    #[serde(default)]
    pub aimotive: super::aimotive::AimotiveIndex,
    #[cfg(feature = "aiprofile")]
    #[serde(default)]
    pub aiprofile: super::aiprofile::AiprofileIndex,
    #[cfg(feature = "aiwavecollection")]
    #[serde(default)]
    pub aiwavecollection: super::aiwavecollection::AiwavecollectionIndex,
    #[cfg(feature = "ammoparams")]
    #[serde(default)]
    pub ammoparams: super::ammoparams::AmmoparamsIndex,
    #[cfg(feature = "analytics")]
    #[serde(default)]
    pub analytics: super::analytics::AnalyticsIndex,
    #[cfg(feature = "announcer")]
    #[serde(default)]
    pub announcer: super::announcer::AnnouncerIndex,
    #[cfg(feature = "areaservices")]
    #[serde(default)]
    pub areaservices: super::areaservices::AreaservicesIndex,
    #[cfg(feature = "audio")]
    #[serde(default)]
    pub audio: super::audio::AudioIndex,
    #[cfg(feature = "awardservice")]
    #[serde(default)]
    pub awardservice: super::awardservice::AwardserviceIndex,
    #[cfg(feature = "cameras")]
    #[serde(default)]
    pub cameras: super::cameras::CamerasIndex,
    #[cfg(feature = "capacitorassignment")]
    #[serde(default)]
    pub capacitorassignment: super::capacitorassignment::CapacitorassignmentIndex,
    #[cfg(feature = "cargomanifest")]
    #[serde(default)]
    pub cargomanifest: super::cargomanifest::CargomanifestIndex,
    #[cfg(feature = "character")]
    #[serde(default)]
    pub character: super::character::CharacterIndex,
    #[cfg(feature = "chatchannelfilters")]
    #[serde(default)]
    pub chatchannelfilters: super::chatchannelfilters::ChatchannelfiltersIndex,
    #[cfg(feature = "chatcommandfastaccess")]
    #[serde(default)]
    pub chatcommandfastaccess: super::chatcommandfastaccess::ChatcommandfastaccessIndex,
    #[cfg(feature = "chatemoterecord")]
    #[serde(default)]
    pub chatemoterecord: super::chatemoterecord::ChatemoterecordIndex,
    #[cfg(feature = "chatfilteroptions")]
    #[serde(default)]
    pub chatfilteroptions: super::chatfilteroptions::ChatfilteroptionsIndex,
    #[cfg(feature = "chatmanager")]
    #[serde(default)]
    pub chatmanager: super::chatmanager::ChatmanagerIndex,
    #[cfg(feature = "closecombat")]
    #[serde(default)]
    pub closecombat: super::closecombat::ClosecombatIndex,
    #[cfg(feature = "commodityconfiguration")]
    #[serde(default)]
    pub commodityconfiguration: super::commodityconfiguration::CommodityconfigurationIndex,
    #[cfg(feature = "commoditytypedatabase")]
    #[serde(default)]
    pub commoditytypedatabase: super::commoditytypedatabase::CommoditytypedatabaseIndex,
    #[cfg(feature = "commsnotifications")]
    #[serde(default)]
    pub commsnotifications: super::commsnotifications::CommsnotificationsIndex,
    #[cfg(feature = "communicationatlconfig")]
    #[serde(default)]
    pub communicationatlconfig: super::communicationatlconfig::CommunicationatlconfigIndex,
    #[cfg(feature = "communicationconfig")]
    #[serde(default)]
    pub communicationconfig: super::communicationconfig::CommunicationconfigIndex,
    #[cfg(feature = "communicationname")]
    #[serde(default)]
    pub communicationname: super::communicationname::CommunicationnameIndex,
    #[cfg(feature = "communicationsystem")]
    #[serde(default)]
    pub communicationsystem: super::communicationsystem::CommunicationsystemIndex,
    #[cfg(feature = "communicationvariableconfig")]
    #[serde(default)]
    pub communicationvariableconfig: super::communicationvariableconfig::CommunicationvariableconfigIndex,
    #[cfg(feature = "consumabletypesdatabase")]
    #[serde(default)]
    pub consumabletypesdatabase: super::consumabletypesdatabase::ConsumabletypesdatabaseIndex,
    #[cfg(feature = "contextualcommunicationconfig")]
    #[serde(default)]
    pub contextualcommunicationconfig: super::contextualcommunicationconfig::ContextualcommunicationconfigIndex,
    #[cfg(feature = "contracts")]
    #[serde(default)]
    pub contracts: super::contracts::ContractsIndex,
    #[cfg(feature = "conversation")]
    #[serde(default)]
    pub conversation: super::conversation::ConversationIndex,
    #[cfg(feature = "crafting")]
    #[serde(default)]
    pub crafting: super::crafting::CraftingIndex,
    #[cfg(feature = "crewmanifest")]
    #[serde(default)]
    pub crewmanifest: super::crewmanifest::CrewmanifestIndex,
    #[cfg(feature = "damage")]
    #[serde(default)]
    pub damage: super::damage::DamageIndex,
    #[cfg(feature = "densityclasses")]
    #[serde(default)]
    pub densityclasses: super::densityclasses::DensityclassesIndex,
    #[cfg(feature = "dialoguecontentbank")]
    #[serde(default)]
    pub dialoguecontentbank: super::dialoguecontentbank::DialoguecontentbankIndex,
    #[cfg(feature = "dialoguerealm")]
    #[serde(default)]
    pub dialoguerealm: super::dialoguerealm::DialoguerealmIndex,
    #[cfg(feature = "dynamiccameraeffects")]
    #[serde(default)]
    pub dynamiccameraeffects: super::dynamiccameraeffects::DynamiccameraeffectsIndex,
    #[cfg(feature = "dynamiclightingrig")]
    #[serde(default)]
    pub dynamiclightingrig: super::dynamiclightingrig::DynamiclightingrigIndex,
    #[cfg(feature = "emotions")]
    #[serde(default)]
    pub emotions: super::emotions::EmotionsIndex,
    #[cfg(feature = "entities-haulingentityclass")]
    #[serde(default)]
    pub entities_haulingentityclass: super::entities_haulingentityclass::EntitiesHaulingentityclassIndex,
    #[cfg(feature = "entities-scitem-actormovables")]
    #[serde(default)]
    pub entities_scitem_actormovables: super::entities_scitem_actormovables::EntitiesScitemActormovablesIndex,
    #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
    #[serde(default)]
    pub entities_scitem_mastermodeexclusionglobalparams: super::entities_scitem_mastermodeexclusionglobalparams::EntitiesScitemMastermodeexclusionglobalparamsIndex,
    #[cfg(feature = "entities-scitem-operatormoderecords")]
    #[serde(default)]
    pub entities_scitem_operatormoderecords: super::entities_scitem_operatormoderecords::EntitiesScitemOperatormoderecordsIndex,
    #[cfg(feature = "entities-scitem-ships")]
    #[serde(default)]
    pub entities_scitem_ships: super::entities_scitem_ships::EntitiesScitemShipsIndex,
    #[cfg(feature = "entities-scitem-usables")]
    #[serde(default)]
    pub entities_scitem_usables: super::entities_scitem_usables::EntitiesScitemUsablesIndex,
    #[cfg(feature = "entities-ui")]
    #[serde(default)]
    pub entities_ui: super::entities_ui::EntitiesUiIndex,
    #[cfg(feature = "entities-vfx")]
    #[serde(default)]
    pub entities_vfx: super::entities_vfx::EntitiesVfxIndex,
    #[cfg(feature = "entitlementpolicies")]
    #[serde(default)]
    pub entitlementpolicies: super::entitlementpolicies::EntitlementpoliciesIndex,
    #[cfg(feature = "environments")]
    #[serde(default)]
    pub environments: super::environments::EnvironmentsIndex,
    #[cfg(feature = "evagraph")]
    #[serde(default)]
    pub evagraph: super::evagraph::EvagraphIndex,
    #[cfg(feature = "explosiveordnance")]
    #[serde(default)]
    pub explosiveordnance: super::explosiveordnance::ExplosiveordnanceIndex,
    #[cfg(feature = "factions")]
    #[serde(default)]
    pub factions: super::factions::FactionsIndex,
    #[cfg(feature = "fidgetconfig")]
    #[serde(default)]
    pub fidgetconfig: super::fidgetconfig::FidgetconfigIndex,
    #[cfg(feature = "foley")]
    #[serde(default)]
    pub foley: super::foley::FoleyIndex,
    #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
    #[serde(default)]
    pub forcefeedback_forcefeedbackeffects: super::forcefeedback_forcefeedbackeffects::Forcefeedback_forcefeedbackeffectsIndex,
    #[cfg(feature = "formation")]
    #[serde(default)]
    pub formation: super::formation::FormationIndex,
    #[cfg(feature = "friendmanager")]
    #[serde(default)]
    pub friendmanager: super::friendmanager::FriendmanagerIndex,
    #[cfg(feature = "fuelparams")]
    #[serde(default)]
    pub fuelparams: super::fuelparams::FuelparamsIndex,
    #[cfg(feature = "gamemode")]
    #[serde(default)]
    pub gamemode: super::gamemode::GamemodeIndex,
    #[cfg(feature = "gamemodule")]
    #[serde(default)]
    pub gamemodule: super::gamemodule::GamemoduleIndex,
    #[cfg(feature = "globalarmarkerparams")]
    #[serde(default)]
    pub globalarmarkerparams: super::globalarmarkerparams::GlobalarmarkerparamsIndex,
    #[cfg(feature = "globalcargoloadingparams")]
    #[serde(default)]
    pub globalcargoloadingparams: super::globalcargoloadingparams::GlobalcargoloadingparamsIndex,
    #[cfg(feature = "globalcommsnotificationparams")]
    #[serde(default)]
    pub globalcommsnotificationparams: super::globalcommsnotificationparams::GlobalcommsnotificationparamsIndex,
    #[cfg(feature = "globalcuttableshapeparams")]
    #[serde(default)]
    pub globalcuttableshapeparams: super::globalcuttableshapeparams::GlobalcuttableshapeparamsIndex,
    #[cfg(feature = "globalinteractionparams")]
    #[serde(default)]
    pub globalinteractionparams: super::globalinteractionparams::GlobalinteractionparamsIndex,
    #[cfg(feature = "globalinventoryparams")]
    #[serde(default)]
    pub globalinventoryparams: super::globalinventoryparams::GlobalinventoryparamsIndex,
    #[cfg(feature = "globalquantumdriveparams")]
    #[serde(default)]
    pub globalquantumdriveparams: super::globalquantumdriveparams::GlobalquantumdriveparamsIndex,
    #[cfg(feature = "globalshopparams")]
    #[serde(default)]
    pub globalshopparams: super::globalshopparams::GlobalshopparamsIndex,
    #[cfg(feature = "globaltutorialparams")]
    #[serde(default)]
    pub globaltutorialparams: super::globaltutorialparams::GlobaltutorialparamsIndex,
    #[cfg(feature = "grips")]
    #[serde(default)]
    pub grips: super::grips::GripsIndex,
    #[cfg(feature = "handholdgripdatabase")]
    #[serde(default)]
    pub handholdgripdatabase: super::handholdgripdatabase::HandholdgripdatabaseIndex,
    #[cfg(feature = "hardwaremouse")]
    #[serde(default)]
    pub hardwaremouse: super::hardwaremouse::HardwaremouseIndex,
    #[cfg(feature = "harvestable")]
    #[serde(default)]
    pub harvestable: super::harvestable::HarvestableIndex,
    #[cfg(feature = "hazardawarenessparams")]
    #[serde(default)]
    pub hazardawarenessparams: super::hazardawarenessparams::HazardawarenessparamsIndex,
    #[cfg(feature = "hints")]
    #[serde(default)]
    pub hints: super::hints::HintsIndex,
    #[cfg(feature = "hudparams")]
    #[serde(default)]
    pub hudparams: super::hudparams::HudparamsIndex,
    #[cfg(feature = "ifcs")]
    #[serde(default)]
    pub ifcs: super::ifcs::IfcsIndex,
    #[cfg(feature = "initialdamageoverrides")]
    #[serde(default)]
    pub initialdamageoverrides: super::initialdamageoverrides::InitialdamageoverridesIndex,
    #[cfg(feature = "inputpromptconfig")]
    #[serde(default)]
    pub inputpromptconfig: super::inputpromptconfig::InputpromptconfigIndex,
    #[cfg(feature = "instancedinterior")]
    #[serde(default)]
    pub instancedinterior: super::instancedinterior::InstancedinteriorIndex,
    #[cfg(feature = "interactionconditions")]
    #[serde(default)]
    pub interactionconditions: super::interactionconditions::InteractionconditionsIndex,
    #[cfg(feature = "intoxication")]
    #[serde(default)]
    pub intoxication: super::intoxication::IntoxicationIndex,
    #[cfg(feature = "inventorycontainers")]
    #[serde(default)]
    pub inventorycontainers: super::inventorycontainers::InventorycontainersIndex,
    #[cfg(feature = "item")]
    #[serde(default)]
    pub item: super::item::ItemIndex,
    #[cfg(feature = "journalentry")]
    #[serde(default)]
    pub journalentry: super::journalentry::JournalentryIndex,
    #[cfg(feature = "jumppoints")]
    #[serde(default)]
    pub jumppoints: super::jumppoints::JumppointsIndex,
    #[cfg(feature = "landingpadsize")]
    #[serde(default)]
    pub landingpadsize: super::landingpadsize::LandingpadsizeIndex,
    #[cfg(feature = "lawsystem")]
    #[serde(default)]
    pub lawsystem: super::lawsystem::LawsystemIndex,
    #[cfg(feature = "leangraph")]
    #[serde(default)]
    pub leangraph: super::leangraph::LeangraphIndex,
    #[cfg(feature = "level")]
    #[serde(default)]
    pub level: super::level::LevelIndex,
    #[cfg(feature = "loadoutkits")]
    #[serde(default)]
    pub loadoutkits: super::loadoutkits::LoadoutkitsIndex,
    #[cfg(feature = "longtermpersistence")]
    #[serde(default)]
    pub longtermpersistence: super::longtermpersistence::LongtermpersistenceIndex,
    #[cfg(feature = "lootgeneration")]
    #[serde(default)]
    pub lootgeneration: super::lootgeneration::LootgenerationIndex,
    #[cfg(feature = "megamap")]
    #[serde(default)]
    pub megamap: super::megamap::MegamapIndex,
    #[cfg(feature = "mining")]
    #[serde(default)]
    pub mining: super::mining::MiningIndex,
    #[cfg(feature = "missionbroker")]
    #[serde(default)]
    pub missionbroker: super::missionbroker::MissionbrokerIndex,
    #[cfg(feature = "missiondata")]
    #[serde(default)]
    pub missiondata: super::missiondata::MissiondataIndex,
    #[cfg(feature = "missionfailureconditions")]
    #[serde(default)]
    pub missionfailureconditions: super::missionfailureconditions::MissionfailureconditionsIndex,
    #[cfg(feature = "missiongiver")]
    #[serde(default)]
    pub missiongiver: super::missiongiver::MissiongiverIndex,
    #[cfg(feature = "missionscenarios")]
    #[serde(default)]
    pub missionscenarios: super::missionscenarios::MissionscenariosIndex,
    #[cfg(feature = "missiontype")]
    #[serde(default)]
    pub missiontype: super::missiontype::MissiontypeIndex,
    #[cfg(feature = "motionstatemachine")]
    #[serde(default)]
    pub motionstatemachine: super::motionstatemachine::MotionstatemachineIndex,
    #[cfg(feature = "moveviewrestrictionpenalties")]
    #[serde(default)]
    pub moveviewrestrictionpenalties: super::moveviewrestrictionpenalties::MoveviewrestrictionpenaltiesIndex,
    #[cfg(feature = "musiclogic")]
    #[serde(default)]
    pub musiclogic: super::musiclogic::MusiclogicIndex,
    #[cfg(feature = "personalinnerthoughtrules")]
    #[serde(default)]
    pub personalinnerthoughtrules: super::personalinnerthoughtrules::PersonalinnerthoughtrulesIndex,
    #[cfg(feature = "planetdaynighttemperatureparams")]
    #[serde(default)]
    pub planetdaynighttemperatureparams: super::planetdaynighttemperatureparams::PlanetdaynighttemperatureparamsIndex,
    #[cfg(feature = "posturedatabase")]
    #[serde(default)]
    pub posturedatabase: super::posturedatabase::PosturedatabaseIndex,
    #[cfg(feature = "procbreathing")]
    #[serde(default)]
    pub procbreathing: super::procbreathing::ProcbreathingIndex,
    #[cfg(feature = "proceduralaimrigrecord")]
    #[serde(default)]
    pub proceduralaimrigrecord: super::proceduralaimrigrecord::ProceduralaimrigrecordIndex,
    #[cfg(feature = "proceduralanimations")]
    #[serde(default)]
    pub proceduralanimations: super::proceduralanimations::ProceduralanimationsIndex,
    #[cfg(feature = "procedurallandingsetup")]
    #[serde(default)]
    pub procedurallandingsetup: super::procedurallandingsetup::ProcedurallandingsetupIndex,
    #[cfg(feature = "procedurallayout")]
    #[serde(default)]
    pub procedurallayout: super::procedurallayout::ProcedurallayoutIndex,
    #[cfg(feature = "qteconfigs")]
    #[serde(default)]
    pub qteconfigs: super::qteconfigs::QteconfigsIndex,
    #[cfg(feature = "radarsystem")]
    #[serde(default)]
    pub radarsystem: super::radarsystem::RadarsystemIndex,
    #[cfg(feature = "rastar")]
    #[serde(default)]
    pub rastar: super::rastar::RastarIndex,
    #[cfg(feature = "refinerynotificationconfiguration")]
    #[serde(default)]
    pub refinerynotificationconfiguration: super::refinerynotificationconfiguration::RefinerynotificationconfigurationIndex,
    #[cfg(feature = "refiningprocess")]
    #[serde(default)]
    pub refiningprocess: super::refiningprocess::RefiningprocessIndex,
    #[cfg(feature = "rendererpresets")]
    #[serde(default)]
    pub rendererpresets: super::rendererpresets::RendererpresetsIndex,
    #[cfg(feature = "rentalnotificationparams")]
    #[serde(default)]
    pub rentalnotificationparams: super::rentalnotificationparams::RentalnotificationparamsIndex,
    #[cfg(feature = "reputation")]
    #[serde(default)]
    pub reputation: super::reputation::ReputationIndex,
    #[cfg(feature = "resourcetypedatabase")]
    #[serde(default)]
    pub resourcetypedatabase: super::resourcetypedatabase::ResourcetypedatabaseIndex,
    #[cfg(feature = "roomsystem")]
    #[serde(default)]
    pub roomsystem: super::roomsystem::RoomsystemIndex,
    #[cfg(feature = "scarryableikinteractionlist")]
    #[serde(default)]
    pub scarryableikinteractionlist: super::scarryableikinteractionlist::ScarryableikinteractionlistIndex,
    #[cfg(feature = "scitemcommscomponentsetup")]
    #[serde(default)]
    pub scitemcommscomponentsetup: super::scitemcommscomponentsetup::ScitemcommscomponentsetupIndex,
    #[cfg(feature = "scitemdisplayscreenpreset")]
    #[serde(default)]
    pub scitemdisplayscreenpreset: super::scitemdisplayscreenpreset::ScitemdisplayscreenpresetIndex,
    #[cfg(feature = "scitemmanufacturer")]
    #[serde(default)]
    pub scitemmanufacturer: super::scitemmanufacturer::ScitemmanufacturerIndex,
    #[cfg(feature = "scuttableshapedefinition")]
    #[serde(default)]
    pub scuttableshapedefinition: super::scuttableshapedefinition::ScuttableshapedefinitionIndex,
    #[cfg(feature = "seatcdikconfigs")]
    #[serde(default)]
    pub seatcdikconfigs: super::seatcdikconfigs::SeatcdikconfigsIndex,
    #[cfg(feature = "servicebeacon")]
    #[serde(default)]
    pub servicebeacon: super::servicebeacon::ServicebeaconIndex,
    #[cfg(feature = "sgeometryviewdistanceratiocategories")]
    #[serde(default)]
    pub sgeometryviewdistanceratiocategories: super::sgeometryviewdistanceratiocategories::SgeometryviewdistanceratiocategoriesIndex,
    #[cfg(feature = "sglobalchargedrainbeamparams")]
    #[serde(default)]
    pub sglobalchargedrainbeamparams: super::sglobalchargedrainbeamparams::SglobalchargedrainbeamparamsIndex,
    #[cfg(feature = "sglobalcrosshairparams")]
    #[serde(default)]
    pub sglobalcrosshairparams: super::sglobalcrosshairparams::SglobalcrosshairparamsIndex,
    #[cfg(feature = "sglobalelectronparams")]
    #[serde(default)]
    pub sglobalelectronparams: super::sglobalelectronparams::SglobalelectronparamsIndex,
    #[cfg(feature = "sglobalhealingbeamparams")]
    #[serde(default)]
    pub sglobalhealingbeamparams: super::sglobalhealingbeamparams::SglobalhealingbeamparamsIndex,
    #[cfg(feature = "sglobalhitbehaviorparams")]
    #[serde(default)]
    pub sglobalhitbehaviorparams: super::sglobalhitbehaviorparams::SglobalhitbehaviorparamsIndex,
    #[cfg(feature = "sglobalsalvagerepairbeamparams")]
    #[serde(default)]
    pub sglobalsalvagerepairbeamparams: super::sglobalsalvagerepairbeamparams::SglobalsalvagerepairbeamparamsIndex,
    #[cfg(feature = "sglobaltractorbeamparams")]
    #[serde(default)]
    pub sglobaltractorbeamparams: super::sglobaltractorbeamparams::SglobaltractorbeamparamsIndex,
    #[cfg(feature = "shipinsurancerecord")]
    #[serde(default)]
    pub shipinsurancerecord: super::shipinsurancerecord::ShipinsurancerecordIndex,
    #[cfg(feature = "specialeventdatabase")]
    #[serde(default)]
    pub specialeventdatabase: super::specialeventdatabase::SpecialeventdatabaseIndex,
    #[cfg(feature = "squantumdriveeffecttagstemplate")]
    #[serde(default)]
    pub squantumdriveeffecttagstemplate: super::squantumdriveeffecttagstemplate::SquantumdriveeffecttagstemplateIndex,
    #[cfg(feature = "sreputationglobalcontextbbparams")]
    #[serde(default)]
    pub sreputationglobalcontextbbparams: super::sreputationglobalcontextbbparams::SreputationglobalcontextbbparamsIndex,
    #[cfg(feature = "ssolarsystem")]
    #[serde(default)]
    pub ssolarsystem: super::ssolarsystem::SsolarsystemIndex,
    #[cfg(feature = "starmap")]
    #[serde(default)]
    pub starmap: super::starmap::StarmapIndex,
    #[cfg(feature = "tacticalquery")]
    #[serde(default)]
    pub tacticalquery: super::tacticalquery::TacticalqueryIndex,
    #[cfg(feature = "tagdatabase")]
    #[serde(default)]
    pub tagdatabase: super::tagdatabase::TagdatabaseIndex,
    #[cfg(feature = "targetselector")]
    #[serde(default)]
    pub targetselector: super::targetselector::TargetselectorIndex,
    #[cfg(feature = "tintpalettes")]
    #[serde(default)]
    pub tintpalettes: super::tintpalettes::TintpalettesIndex,
    #[cfg(feature = "trackview")]
    #[serde(default)]
    pub trackview: super::trackview::TrackviewIndex,
    #[cfg(feature = "traversalcostconfig")]
    #[serde(default)]
    pub traversalcostconfig: super::traversalcostconfig::TraversalcostconfigIndex,
    #[cfg(feature = "turret")]
    #[serde(default)]
    pub turret: super::turret::TurretIndex,
    #[cfg(feature = "ui-animatedmarkers")]
    #[serde(default)]
    pub ui_animatedmarkers: super::ui_animatedmarkers::UiAnimatedmarkersIndex,
    #[cfg(feature = "ui-armarkerconfiguration")]
    #[serde(default)]
    pub ui_armarkerconfiguration: super::ui_armarkerconfiguration::UiArmarkerconfigurationIndex,
    #[cfg(feature = "ui-buildingblocks")]
    #[serde(default)]
    pub ui_buildingblocks: super::ui_buildingblocks::UiBuildingblocksIndex,
    #[cfg(feature = "ui-digitalsignage")]
    #[serde(default)]
    pub ui_digitalsignage: super::ui_digitalsignage::UiDigitalsignageIndex,
    #[cfg(feature = "ui-directrtt")]
    #[serde(default)]
    pub ui_directrtt: super::ui_directrtt::UiDirectrttIndex,
    #[cfg(feature = "ui-dockingslotvisibility")]
    #[serde(default)]
    pub ui_dockingslotvisibility: super::ui_dockingslotvisibility::UiDockingslotvisibilityIndex,
    #[cfg(feature = "ui-elementsounds_deprecated")]
    #[serde(default)]
    pub ui_elementsounds_deprecated: super::ui_elementsounds_deprecated::UiElementsounds_deprecatedIndex,
    #[cfg(feature = "ui-flashobjectbindinggroups")]
    #[serde(default)]
    pub ui_flashobjectbindinggroups: super::ui_flashobjectbindinggroups::UiFlashobjectbindinggroupsIndex,
    #[cfg(feature = "ui-flighthudmessages")]
    #[serde(default)]
    pub ui_flighthudmessages: super::ui_flighthudmessages::UiFlighthudmessagesIndex,
    #[cfg(feature = "ui-frontend")]
    #[serde(default)]
    pub ui_frontend: super::ui_frontend::UiFrontendIndex,
    #[cfg(feature = "ui-graphs")]
    #[serde(default)]
    pub ui_graphs: super::ui_graphs::UiGraphsIndex,
    #[cfg(feature = "ui-holovehicleconfig")]
    #[serde(default)]
    pub ui_holovehicleconfig: super::ui_holovehicleconfig::UiHolovehicleconfigIndex,
    #[cfg(feature = "ui-hudcolors_shipcolorpalettes")]
    #[serde(default)]
    pub ui_hudcolors_shipcolorpalettes: super::ui_hudcolors_shipcolorpalettes::UiHudcolors_shipcolorpalettesIndex,
    #[cfg(feature = "ui-innerthought")]
    #[serde(default)]
    pub ui_innerthought: super::ui_innerthought::UiInnerthoughtIndex,
    #[cfg(feature = "ui-itemkiosk")]
    #[serde(default)]
    pub ui_itemkiosk: super::ui_itemkiosk::UiItemkioskIndex,
    #[cfg(feature = "ui-itempreview_config")]
    #[serde(default)]
    pub ui_itempreview_config: super::ui_itempreview_config::UiItempreview_configIndex,
    #[cfg(feature = "ui-itemtypedefinition")]
    #[serde(default)]
    pub ui_itemtypedefinition: super::ui_itemtypedefinition::UiItemtypedefinitionIndex,
    #[cfg(feature = "ui-localplayershoppingdata")]
    #[serde(default)]
    pub ui_localplayershoppingdata: super::ui_localplayershoppingdata::UiLocalplayershoppingdataIndex,
    #[cfg(feature = "ui-marker_config")]
    #[serde(default)]
    pub ui_marker_config: super::ui_marker_config::UiMarker_configIndex,
    #[cfg(feature = "ui-markertrackingvolumeconfig")]
    #[serde(default)]
    pub ui_markertrackingvolumeconfig: super::ui_markertrackingvolumeconfig::UiMarkertrackingvolumeconfigIndex,
    #[cfg(feature = "ui-missilelockreticleconfig")]
    #[serde(default)]
    pub ui_missilelockreticleconfig: super::ui_missilelockreticleconfig::UiMissilelockreticleconfigIndex,
    #[cfg(feature = "ui-mobiglas")]
    #[serde(default)]
    pub ui_mobiglas: super::ui_mobiglas::UiMobiglasIndex,
    #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
    #[serde(default)]
    pub ui_objectdatabankentrymarkerconfig: super::ui_objectdatabankentrymarkerconfig::UiObjectdatabankentrymarkerconfigIndex,
    #[cfg(feature = "ui-playerchoice_imconfig_playerchoiceim")]
    #[serde(default)]
    pub ui_playerchoice_imconfig_playerchoiceim: super::ui_playerchoice_imconfig_playerchoiceim::UiPlayerchoice_imconfig_playerchoiceimIndex,
    #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
    #[serde(default)]
    pub ui_playerchoice_library_playerchoicelibrary: super::ui_playerchoice_library_playerchoicelibrary::UiPlayerchoice_library_playerchoicelibraryIndex,
    #[cfg(feature = "ui-playerchoice_pitconfig_playerchoicepersonalthought")]
    #[serde(default)]
    pub ui_playerchoice_pitconfig_playerchoicepersonalthought: super::ui_playerchoice_pitconfig_playerchoicepersonalthought::UiPlayerchoice_pitconfig_playerchoicepersonalthoughtIndex,
    #[cfg(feature = "ui-playerchoice_signalconfig_interactorsignalconfig")]
    #[serde(default)]
    pub ui_playerchoice_signalconfig_interactorsignalconfig: super::ui_playerchoice_signalconfig_interactorsignalconfig::UiPlayerchoice_signalconfig_interactorsignalconfigIndex,
    #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
    #[serde(default)]
    pub ui_playerecggraph_config_playerecggraphconfig: super::ui_playerecggraph_config_playerecggraphconfig::UiPlayerecggraph_config_playerecggraphconfigIndex,
    #[cfg(feature = "ui-pointofinterestdata")]
    #[serde(default)]
    pub ui_pointofinterestdata: super::ui_pointofinterestdata::UiPointofinterestdataIndex,
    #[cfg(feature = "ui-popups")]
    #[serde(default)]
    pub ui_popups: super::ui_popups::UiPopupsIndex,
    #[cfg(feature = "ui-radar3dpresets")]
    #[serde(default)]
    pub ui_radar3dpresets: super::ui_radar3dpresets::UiRadar3dpresetsIndex,
    #[cfg(feature = "ui-radardisplay_config_radar")]
    #[serde(default)]
    pub ui_radardisplay_config_radar: super::ui_radardisplay_config_radar::UiRadardisplay_config_radarIndex,
    #[cfg(feature = "ui-seatreticlearchetype")]
    #[serde(default)]
    pub ui_seatreticlearchetype: super::ui_seatreticlearchetype::UiSeatreticlearchetypeIndex,
    #[cfg(feature = "ui-transformationinterpolatorrecords")]
    #[serde(default)]
    pub ui_transformationinterpolatorrecords: super::ui_transformationinterpolatorrecords::UiTransformationinterpolatorrecordsIndex,
    #[cfg(feature = "ui-uiconfig_starcitizen")]
    #[serde(default)]
    pub ui_uiconfig_starcitizen: super::ui_uiconfig_starcitizen::UiUiconfig_starcitizenIndex,
    #[cfg(feature = "ui-uielements")]
    #[serde(default)]
    pub ui_uielements: super::ui_uielements::UiUielementsIndex,
    #[cfg(feature = "ui-uimodes")]
    #[serde(default)]
    pub ui_uimodes: super::ui_uimodes::UiUimodesIndex,
    #[cfg(feature = "ui-uistatedisplay")]
    #[serde(default)]
    pub ui_uistatedisplay: super::ui_uistatedisplay::UiUistatedisplayIndex,
    #[cfg(feature = "ui-vehicleentrance")]
    #[serde(default)]
    pub ui_vehicleentrance: super::ui_vehicleentrance::UiVehicleentranceIndex,
    #[cfg(feature = "ui-videocomms")]
    #[serde(default)]
    pub ui_videocomms: super::ui_videocomms::UiVideocommsIndex,
    #[cfg(feature = "ui-visor")]
    #[serde(default)]
    pub ui_visor: super::ui_visor::UiVisorIndex,
    #[cfg(feature = "ui-worlddisplay")]
    #[serde(default)]
    pub ui_worlddisplay: super::ui_worlddisplay::UiWorlddisplayIndex,
    #[cfg(feature = "unittest_unittestb")]
    #[serde(default)]
    pub unittest_unittestb: super::unittest_unittestb::Unittest_unittestbIndex,
    #[cfg(feature = "vehicle")]
    #[serde(default)]
    pub vehicle: super::vehicle::VehicleIndex,
    #[cfg(feature = "vfx")]
    #[serde(default)]
    pub vfx: super::vfx::VfxIndex,
    #[cfg(feature = "vibrations")]
    #[serde(default)]
    pub vibrations: super::vibrations::VibrationsIndex,
    #[cfg(feature = "videocommschannels")]
    #[serde(default)]
    pub videocommschannels: super::videocommschannels::VideocommschannelsIndex,
    #[cfg(feature = "voicebundle")]
    #[serde(default)]
    pub voicebundle: super::voicebundle::VoicebundleIndex,
    #[cfg(feature = "voicechannelsettingsrecord")]
    #[serde(default)]
    pub voicechannelsettingsrecord: super::voicechannelsettingsrecord::VoicechannelsettingsrecordIndex,
    #[cfg(feature = "voicesingle")]
    #[serde(default)]
    pub voicesingle: super::voicesingle::VoicesingleIndex,
    #[cfg(feature = "weaponarmodifiers")]
    #[serde(default)]
    pub weaponarmodifiers: super::weaponarmodifiers::WeaponarmodifiersIndex,
    #[cfg(feature = "weaponmisfiredef")]
    #[serde(default)]
    pub weaponmisfiredef: super::weaponmisfiredef::WeaponmisfiredefIndex,
    #[cfg(feature = "weaponproceduralanimation")]
    #[serde(default)]
    pub weaponproceduralanimation: super::weaponproceduralanimation::WeaponproceduralanimationIndex,
    #[cfg(feature = "weaponproceduralclip")]
    #[serde(default)]
    pub weaponproceduralclip: super::weaponproceduralclip::WeaponproceduralclipIndex,
    #[cfg(feature = "weaponproceduralrecoil")]
    #[serde(default)]
    pub weaponproceduralrecoil: super::weaponproceduralrecoil::WeaponproceduralrecoilIndex,
    #[cfg(feature = "zerogtraversalgraph")]
    #[serde(default)]
    pub zerogtraversalgraph: super::zerogtraversalgraph::ZerogtraversalgraphIndex,
}

impl RecordIndex {
    pub fn len(&self) -> usize {
        let mut n = 0;
        n += self.core.len();
        #[cfg(feature = "actor-actorledgegrabbing")]
        { n += self.actor_actorledgegrabbing.len(); }
        #[cfg(feature = "actor-actortargetedparams")]
        { n += self.actor_actortargetedparams.len(); }
        #[cfg(feature = "actor-actorviewlimits")]
        { n += self.actor_actorviewlimits.len(); }
        #[cfg(feature = "actor-actorzerogtraversalparams")]
        { n += self.actor_actorzerogtraversalparams.len(); }
        #[cfg(feature = "actor-atls_geo_jumppackconfig")]
        { n += self.actor_atls_geo_jumppackconfig.len(); }
        #[cfg(feature = "actor-externalforceresponse")]
        { n += self.actor_externalforceresponse.len(); }
        #[cfg(feature = "actor-gforce")]
        { n += self.actor_gforce.len(); }
        #[cfg(feature = "actor-headtrackinglimits")]
        { n += self.actor_headtrackinglimits.len(); }
        #[cfg(feature = "actor-inputdeflectiontime")]
        { n += self.actor_inputdeflectiontime.len(); }
        #[cfg(feature = "actor-locomotionpersonality")]
        { n += self.actor_locomotionpersonality.len(); }
        #[cfg(feature = "actor-lookahead")]
        { n += self.actor_lookahead.len(); }
        #[cfg(feature = "actor-playeranimatedinteractionconfig")]
        { n += self.actor_playeranimatedinteractionconfig.len(); }
        #[cfg(feature = "actor-playerdefaultactionsconfig")]
        { n += self.actor_playerdefaultactionsconfig.len(); }
        #[cfg(feature = "actor-quantumtravelcameraeffects")]
        { n += self.actor_quantumtravelcameraeffects.len(); }
        #[cfg(feature = "actor-skeletonconfigs")]
        { n += self.actor_skeletonconfigs.len(); }
        #[cfg(feature = "actor-stanceinfo")]
        { n += self.actor_stanceinfo.len(); }
        #[cfg(feature = "actor-targettrackingautozoom")]
        { n += self.actor_targettrackingautozoom.len(); }
        #[cfg(feature = "ads")]
        { n += self.ads.len(); }
        #[cfg(feature = "aianimationdata")]
        { n += self.aianimationdata.len(); }
        #[cfg(feature = "aiglobalsettings")]
        { n += self.aiglobalsettings.len(); }
        #[cfg(feature = "aimotive")]
        { n += self.aimotive.len(); }
        #[cfg(feature = "aiprofile")]
        { n += self.aiprofile.len(); }
        #[cfg(feature = "aiwavecollection")]
        { n += self.aiwavecollection.len(); }
        #[cfg(feature = "ammoparams")]
        { n += self.ammoparams.len(); }
        #[cfg(feature = "analytics")]
        { n += self.analytics.len(); }
        #[cfg(feature = "announcer")]
        { n += self.announcer.len(); }
        #[cfg(feature = "areaservices")]
        { n += self.areaservices.len(); }
        #[cfg(feature = "audio")]
        { n += self.audio.len(); }
        #[cfg(feature = "awardservice")]
        { n += self.awardservice.len(); }
        #[cfg(feature = "cameras")]
        { n += self.cameras.len(); }
        #[cfg(feature = "capacitorassignment")]
        { n += self.capacitorassignment.len(); }
        #[cfg(feature = "cargomanifest")]
        { n += self.cargomanifest.len(); }
        #[cfg(feature = "character")]
        { n += self.character.len(); }
        #[cfg(feature = "chatchannelfilters")]
        { n += self.chatchannelfilters.len(); }
        #[cfg(feature = "chatcommandfastaccess")]
        { n += self.chatcommandfastaccess.len(); }
        #[cfg(feature = "chatemoterecord")]
        { n += self.chatemoterecord.len(); }
        #[cfg(feature = "chatfilteroptions")]
        { n += self.chatfilteroptions.len(); }
        #[cfg(feature = "chatmanager")]
        { n += self.chatmanager.len(); }
        #[cfg(feature = "closecombat")]
        { n += self.closecombat.len(); }
        #[cfg(feature = "commodityconfiguration")]
        { n += self.commodityconfiguration.len(); }
        #[cfg(feature = "commoditytypedatabase")]
        { n += self.commoditytypedatabase.len(); }
        #[cfg(feature = "commsnotifications")]
        { n += self.commsnotifications.len(); }
        #[cfg(feature = "communicationatlconfig")]
        { n += self.communicationatlconfig.len(); }
        #[cfg(feature = "communicationconfig")]
        { n += self.communicationconfig.len(); }
        #[cfg(feature = "communicationname")]
        { n += self.communicationname.len(); }
        #[cfg(feature = "communicationsystem")]
        { n += self.communicationsystem.len(); }
        #[cfg(feature = "communicationvariableconfig")]
        { n += self.communicationvariableconfig.len(); }
        #[cfg(feature = "consumabletypesdatabase")]
        { n += self.consumabletypesdatabase.len(); }
        #[cfg(feature = "contextualcommunicationconfig")]
        { n += self.contextualcommunicationconfig.len(); }
        #[cfg(feature = "contracts")]
        { n += self.contracts.len(); }
        #[cfg(feature = "conversation")]
        { n += self.conversation.len(); }
        #[cfg(feature = "crafting")]
        { n += self.crafting.len(); }
        #[cfg(feature = "crewmanifest")]
        { n += self.crewmanifest.len(); }
        #[cfg(feature = "damage")]
        { n += self.damage.len(); }
        #[cfg(feature = "densityclasses")]
        { n += self.densityclasses.len(); }
        #[cfg(feature = "dialoguecontentbank")]
        { n += self.dialoguecontentbank.len(); }
        #[cfg(feature = "dialoguerealm")]
        { n += self.dialoguerealm.len(); }
        #[cfg(feature = "dynamiccameraeffects")]
        { n += self.dynamiccameraeffects.len(); }
        #[cfg(feature = "dynamiclightingrig")]
        { n += self.dynamiclightingrig.len(); }
        #[cfg(feature = "emotions")]
        { n += self.emotions.len(); }
        #[cfg(feature = "entities-haulingentityclass")]
        { n += self.entities_haulingentityclass.len(); }
        #[cfg(feature = "entities-scitem-actormovables")]
        { n += self.entities_scitem_actormovables.len(); }
        #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
        { n += self.entities_scitem_mastermodeexclusionglobalparams.len(); }
        #[cfg(feature = "entities-scitem-operatormoderecords")]
        { n += self.entities_scitem_operatormoderecords.len(); }
        #[cfg(feature = "entities-scitem-ships")]
        { n += self.entities_scitem_ships.len(); }
        #[cfg(feature = "entities-scitem-usables")]
        { n += self.entities_scitem_usables.len(); }
        #[cfg(feature = "entities-ui")]
        { n += self.entities_ui.len(); }
        #[cfg(feature = "entities-vfx")]
        { n += self.entities_vfx.len(); }
        #[cfg(feature = "entitlementpolicies")]
        { n += self.entitlementpolicies.len(); }
        #[cfg(feature = "environments")]
        { n += self.environments.len(); }
        #[cfg(feature = "evagraph")]
        { n += self.evagraph.len(); }
        #[cfg(feature = "explosiveordnance")]
        { n += self.explosiveordnance.len(); }
        #[cfg(feature = "factions")]
        { n += self.factions.len(); }
        #[cfg(feature = "fidgetconfig")]
        { n += self.fidgetconfig.len(); }
        #[cfg(feature = "foley")]
        { n += self.foley.len(); }
        #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
        { n += self.forcefeedback_forcefeedbackeffects.len(); }
        #[cfg(feature = "formation")]
        { n += self.formation.len(); }
        #[cfg(feature = "friendmanager")]
        { n += self.friendmanager.len(); }
        #[cfg(feature = "fuelparams")]
        { n += self.fuelparams.len(); }
        #[cfg(feature = "gamemode")]
        { n += self.gamemode.len(); }
        #[cfg(feature = "gamemodule")]
        { n += self.gamemodule.len(); }
        #[cfg(feature = "globalarmarkerparams")]
        { n += self.globalarmarkerparams.len(); }
        #[cfg(feature = "globalcargoloadingparams")]
        { n += self.globalcargoloadingparams.len(); }
        #[cfg(feature = "globalcommsnotificationparams")]
        { n += self.globalcommsnotificationparams.len(); }
        #[cfg(feature = "globalcuttableshapeparams")]
        { n += self.globalcuttableshapeparams.len(); }
        #[cfg(feature = "globalinteractionparams")]
        { n += self.globalinteractionparams.len(); }
        #[cfg(feature = "globalinventoryparams")]
        { n += self.globalinventoryparams.len(); }
        #[cfg(feature = "globalquantumdriveparams")]
        { n += self.globalquantumdriveparams.len(); }
        #[cfg(feature = "globalshopparams")]
        { n += self.globalshopparams.len(); }
        #[cfg(feature = "globaltutorialparams")]
        { n += self.globaltutorialparams.len(); }
        #[cfg(feature = "grips")]
        { n += self.grips.len(); }
        #[cfg(feature = "handholdgripdatabase")]
        { n += self.handholdgripdatabase.len(); }
        #[cfg(feature = "hardwaremouse")]
        { n += self.hardwaremouse.len(); }
        #[cfg(feature = "harvestable")]
        { n += self.harvestable.len(); }
        #[cfg(feature = "hazardawarenessparams")]
        { n += self.hazardawarenessparams.len(); }
        #[cfg(feature = "hints")]
        { n += self.hints.len(); }
        #[cfg(feature = "hudparams")]
        { n += self.hudparams.len(); }
        #[cfg(feature = "ifcs")]
        { n += self.ifcs.len(); }
        #[cfg(feature = "initialdamageoverrides")]
        { n += self.initialdamageoverrides.len(); }
        #[cfg(feature = "inputpromptconfig")]
        { n += self.inputpromptconfig.len(); }
        #[cfg(feature = "instancedinterior")]
        { n += self.instancedinterior.len(); }
        #[cfg(feature = "interactionconditions")]
        { n += self.interactionconditions.len(); }
        #[cfg(feature = "intoxication")]
        { n += self.intoxication.len(); }
        #[cfg(feature = "inventorycontainers")]
        { n += self.inventorycontainers.len(); }
        #[cfg(feature = "item")]
        { n += self.item.len(); }
        #[cfg(feature = "journalentry")]
        { n += self.journalentry.len(); }
        #[cfg(feature = "jumppoints")]
        { n += self.jumppoints.len(); }
        #[cfg(feature = "landingpadsize")]
        { n += self.landingpadsize.len(); }
        #[cfg(feature = "lawsystem")]
        { n += self.lawsystem.len(); }
        #[cfg(feature = "leangraph")]
        { n += self.leangraph.len(); }
        #[cfg(feature = "level")]
        { n += self.level.len(); }
        #[cfg(feature = "loadoutkits")]
        { n += self.loadoutkits.len(); }
        #[cfg(feature = "longtermpersistence")]
        { n += self.longtermpersistence.len(); }
        #[cfg(feature = "lootgeneration")]
        { n += self.lootgeneration.len(); }
        #[cfg(feature = "megamap")]
        { n += self.megamap.len(); }
        #[cfg(feature = "mining")]
        { n += self.mining.len(); }
        #[cfg(feature = "missionbroker")]
        { n += self.missionbroker.len(); }
        #[cfg(feature = "missiondata")]
        { n += self.missiondata.len(); }
        #[cfg(feature = "missionfailureconditions")]
        { n += self.missionfailureconditions.len(); }
        #[cfg(feature = "missiongiver")]
        { n += self.missiongiver.len(); }
        #[cfg(feature = "missionscenarios")]
        { n += self.missionscenarios.len(); }
        #[cfg(feature = "missiontype")]
        { n += self.missiontype.len(); }
        #[cfg(feature = "motionstatemachine")]
        { n += self.motionstatemachine.len(); }
        #[cfg(feature = "moveviewrestrictionpenalties")]
        { n += self.moveviewrestrictionpenalties.len(); }
        #[cfg(feature = "musiclogic")]
        { n += self.musiclogic.len(); }
        #[cfg(feature = "personalinnerthoughtrules")]
        { n += self.personalinnerthoughtrules.len(); }
        #[cfg(feature = "planetdaynighttemperatureparams")]
        { n += self.planetdaynighttemperatureparams.len(); }
        #[cfg(feature = "posturedatabase")]
        { n += self.posturedatabase.len(); }
        #[cfg(feature = "procbreathing")]
        { n += self.procbreathing.len(); }
        #[cfg(feature = "proceduralaimrigrecord")]
        { n += self.proceduralaimrigrecord.len(); }
        #[cfg(feature = "proceduralanimations")]
        { n += self.proceduralanimations.len(); }
        #[cfg(feature = "procedurallandingsetup")]
        { n += self.procedurallandingsetup.len(); }
        #[cfg(feature = "procedurallayout")]
        { n += self.procedurallayout.len(); }
        #[cfg(feature = "qteconfigs")]
        { n += self.qteconfigs.len(); }
        #[cfg(feature = "radarsystem")]
        { n += self.radarsystem.len(); }
        #[cfg(feature = "rastar")]
        { n += self.rastar.len(); }
        #[cfg(feature = "refinerynotificationconfiguration")]
        { n += self.refinerynotificationconfiguration.len(); }
        #[cfg(feature = "refiningprocess")]
        { n += self.refiningprocess.len(); }
        #[cfg(feature = "rendererpresets")]
        { n += self.rendererpresets.len(); }
        #[cfg(feature = "rentalnotificationparams")]
        { n += self.rentalnotificationparams.len(); }
        #[cfg(feature = "reputation")]
        { n += self.reputation.len(); }
        #[cfg(feature = "resourcetypedatabase")]
        { n += self.resourcetypedatabase.len(); }
        #[cfg(feature = "roomsystem")]
        { n += self.roomsystem.len(); }
        #[cfg(feature = "scarryableikinteractionlist")]
        { n += self.scarryableikinteractionlist.len(); }
        #[cfg(feature = "scitemcommscomponentsetup")]
        { n += self.scitemcommscomponentsetup.len(); }
        #[cfg(feature = "scitemdisplayscreenpreset")]
        { n += self.scitemdisplayscreenpreset.len(); }
        #[cfg(feature = "scitemmanufacturer")]
        { n += self.scitemmanufacturer.len(); }
        #[cfg(feature = "scuttableshapedefinition")]
        { n += self.scuttableshapedefinition.len(); }
        #[cfg(feature = "seatcdikconfigs")]
        { n += self.seatcdikconfigs.len(); }
        #[cfg(feature = "servicebeacon")]
        { n += self.servicebeacon.len(); }
        #[cfg(feature = "sgeometryviewdistanceratiocategories")]
        { n += self.sgeometryviewdistanceratiocategories.len(); }
        #[cfg(feature = "sglobalchargedrainbeamparams")]
        { n += self.sglobalchargedrainbeamparams.len(); }
        #[cfg(feature = "sglobalcrosshairparams")]
        { n += self.sglobalcrosshairparams.len(); }
        #[cfg(feature = "sglobalelectronparams")]
        { n += self.sglobalelectronparams.len(); }
        #[cfg(feature = "sglobalhealingbeamparams")]
        { n += self.sglobalhealingbeamparams.len(); }
        #[cfg(feature = "sglobalhitbehaviorparams")]
        { n += self.sglobalhitbehaviorparams.len(); }
        #[cfg(feature = "sglobalsalvagerepairbeamparams")]
        { n += self.sglobalsalvagerepairbeamparams.len(); }
        #[cfg(feature = "sglobaltractorbeamparams")]
        { n += self.sglobaltractorbeamparams.len(); }
        #[cfg(feature = "shipinsurancerecord")]
        { n += self.shipinsurancerecord.len(); }
        #[cfg(feature = "specialeventdatabase")]
        { n += self.specialeventdatabase.len(); }
        #[cfg(feature = "squantumdriveeffecttagstemplate")]
        { n += self.squantumdriveeffecttagstemplate.len(); }
        #[cfg(feature = "sreputationglobalcontextbbparams")]
        { n += self.sreputationglobalcontextbbparams.len(); }
        #[cfg(feature = "ssolarsystem")]
        { n += self.ssolarsystem.len(); }
        #[cfg(feature = "starmap")]
        { n += self.starmap.len(); }
        #[cfg(feature = "tacticalquery")]
        { n += self.tacticalquery.len(); }
        #[cfg(feature = "tagdatabase")]
        { n += self.tagdatabase.len(); }
        #[cfg(feature = "targetselector")]
        { n += self.targetselector.len(); }
        #[cfg(feature = "tintpalettes")]
        { n += self.tintpalettes.len(); }
        #[cfg(feature = "trackview")]
        { n += self.trackview.len(); }
        #[cfg(feature = "traversalcostconfig")]
        { n += self.traversalcostconfig.len(); }
        #[cfg(feature = "turret")]
        { n += self.turret.len(); }
        #[cfg(feature = "ui-animatedmarkers")]
        { n += self.ui_animatedmarkers.len(); }
        #[cfg(feature = "ui-armarkerconfiguration")]
        { n += self.ui_armarkerconfiguration.len(); }
        #[cfg(feature = "ui-buildingblocks")]
        { n += self.ui_buildingblocks.len(); }
        #[cfg(feature = "ui-digitalsignage")]
        { n += self.ui_digitalsignage.len(); }
        #[cfg(feature = "ui-directrtt")]
        { n += self.ui_directrtt.len(); }
        #[cfg(feature = "ui-dockingslotvisibility")]
        { n += self.ui_dockingslotvisibility.len(); }
        #[cfg(feature = "ui-elementsounds_deprecated")]
        { n += self.ui_elementsounds_deprecated.len(); }
        #[cfg(feature = "ui-flashobjectbindinggroups")]
        { n += self.ui_flashobjectbindinggroups.len(); }
        #[cfg(feature = "ui-flighthudmessages")]
        { n += self.ui_flighthudmessages.len(); }
        #[cfg(feature = "ui-frontend")]
        { n += self.ui_frontend.len(); }
        #[cfg(feature = "ui-graphs")]
        { n += self.ui_graphs.len(); }
        #[cfg(feature = "ui-holovehicleconfig")]
        { n += self.ui_holovehicleconfig.len(); }
        #[cfg(feature = "ui-hudcolors_shipcolorpalettes")]
        { n += self.ui_hudcolors_shipcolorpalettes.len(); }
        #[cfg(feature = "ui-innerthought")]
        { n += self.ui_innerthought.len(); }
        #[cfg(feature = "ui-itemkiosk")]
        { n += self.ui_itemkiosk.len(); }
        #[cfg(feature = "ui-itempreview_config")]
        { n += self.ui_itempreview_config.len(); }
        #[cfg(feature = "ui-itemtypedefinition")]
        { n += self.ui_itemtypedefinition.len(); }
        #[cfg(feature = "ui-localplayershoppingdata")]
        { n += self.ui_localplayershoppingdata.len(); }
        #[cfg(feature = "ui-marker_config")]
        { n += self.ui_marker_config.len(); }
        #[cfg(feature = "ui-markertrackingvolumeconfig")]
        { n += self.ui_markertrackingvolumeconfig.len(); }
        #[cfg(feature = "ui-missilelockreticleconfig")]
        { n += self.ui_missilelockreticleconfig.len(); }
        #[cfg(feature = "ui-mobiglas")]
        { n += self.ui_mobiglas.len(); }
        #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
        { n += self.ui_objectdatabankentrymarkerconfig.len(); }
        #[cfg(feature = "ui-playerchoice_imconfig_playerchoiceim")]
        { n += self.ui_playerchoice_imconfig_playerchoiceim.len(); }
        #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
        { n += self.ui_playerchoice_library_playerchoicelibrary.len(); }
        #[cfg(feature = "ui-playerchoice_pitconfig_playerchoicepersonalthought")]
        { n += self.ui_playerchoice_pitconfig_playerchoicepersonalthought.len(); }
        #[cfg(feature = "ui-playerchoice_signalconfig_interactorsignalconfig")]
        { n += self.ui_playerchoice_signalconfig_interactorsignalconfig.len(); }
        #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
        { n += self.ui_playerecggraph_config_playerecggraphconfig.len(); }
        #[cfg(feature = "ui-pointofinterestdata")]
        { n += self.ui_pointofinterestdata.len(); }
        #[cfg(feature = "ui-popups")]
        { n += self.ui_popups.len(); }
        #[cfg(feature = "ui-radar3dpresets")]
        { n += self.ui_radar3dpresets.len(); }
        #[cfg(feature = "ui-radardisplay_config_radar")]
        { n += self.ui_radardisplay_config_radar.len(); }
        #[cfg(feature = "ui-seatreticlearchetype")]
        { n += self.ui_seatreticlearchetype.len(); }
        #[cfg(feature = "ui-transformationinterpolatorrecords")]
        { n += self.ui_transformationinterpolatorrecords.len(); }
        #[cfg(feature = "ui-uiconfig_starcitizen")]
        { n += self.ui_uiconfig_starcitizen.len(); }
        #[cfg(feature = "ui-uielements")]
        { n += self.ui_uielements.len(); }
        #[cfg(feature = "ui-uimodes")]
        { n += self.ui_uimodes.len(); }
        #[cfg(feature = "ui-uistatedisplay")]
        { n += self.ui_uistatedisplay.len(); }
        #[cfg(feature = "ui-vehicleentrance")]
        { n += self.ui_vehicleentrance.len(); }
        #[cfg(feature = "ui-videocomms")]
        { n += self.ui_videocomms.len(); }
        #[cfg(feature = "ui-visor")]
        { n += self.ui_visor.len(); }
        #[cfg(feature = "ui-worlddisplay")]
        { n += self.ui_worlddisplay.len(); }
        #[cfg(feature = "unittest_unittestb")]
        { n += self.unittest_unittestb.len(); }
        #[cfg(feature = "vehicle")]
        { n += self.vehicle.len(); }
        #[cfg(feature = "vfx")]
        { n += self.vfx.len(); }
        #[cfg(feature = "vibrations")]
        { n += self.vibrations.len(); }
        #[cfg(feature = "videocommschannels")]
        { n += self.videocommschannels.len(); }
        #[cfg(feature = "voicebundle")]
        { n += self.voicebundle.len(); }
        #[cfg(feature = "voicechannelsettingsrecord")]
        { n += self.voicechannelsettingsrecord.len(); }
        #[cfg(feature = "voicesingle")]
        { n += self.voicesingle.len(); }
        #[cfg(feature = "weaponarmodifiers")]
        { n += self.weaponarmodifiers.len(); }
        #[cfg(feature = "weaponmisfiredef")]
        { n += self.weaponmisfiredef.len(); }
        #[cfg(feature = "weaponproceduralanimation")]
        { n += self.weaponproceduralanimation.len(); }
        #[cfg(feature = "weaponproceduralclip")]
        { n += self.weaponproceduralclip.len(); }
        #[cfg(feature = "weaponproceduralrecoil")]
        { n += self.weaponproceduralrecoil.len(); }
        #[cfg(feature = "zerogtraversalgraph")]
        { n += self.zerogtraversalgraph.len(); }
        n
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
