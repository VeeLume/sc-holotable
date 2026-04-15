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
    #[serde(default)]
    pub multi_feature: super::multi_feature::MultiFeaturePools,
    #[cfg(feature = "dormant")]
    #[serde(default)]
    pub dormant: super::dormant::DormantPools,
    #[cfg(feature = "actor-actorblockhelper")]
    #[serde(default)]
    pub actor_actorblockhelper: super::actor_actorblockhelper::ActorActorblockhelperPools,
    #[cfg(feature = "actor-actors")]
    #[serde(default)]
    pub actor_actors: super::actor_actors::ActorActorsPools,
    #[cfg(feature = "actor-externalforceresponse")]
    #[serde(default)]
    pub actor_externalforceresponse: super::actor_externalforceresponse::ActorExternalforceresponsePools,
    #[cfg(feature = "actor-gforce")]
    #[serde(default)]
    pub actor_gforce: super::actor_gforce::ActorGforcePools,
    #[cfg(feature = "actor-inputdeflectiontime")]
    #[serde(default)]
    pub actor_inputdeflectiontime: super::actor_inputdeflectiontime::ActorInputdeflectiontimePools,
    #[cfg(feature = "actor-playerdefaultactionsconfig")]
    #[serde(default)]
    pub actor_playerdefaultactionsconfig: super::actor_playerdefaultactionsconfig::ActorPlayerdefaultactionsconfigPools,
    #[cfg(feature = "actor-quantumtravelcameraeffects")]
    #[serde(default)]
    pub actor_quantumtravelcameraeffects: super::actor_quantumtravelcameraeffects::ActorQuantumtravelcameraeffectsPools,
    #[cfg(feature = "aiglobalsettings")]
    #[serde(default)]
    pub aiglobalsettings: super::aiglobalsettings::AiglobalsettingsPools,
    #[cfg(feature = "aiprofile")]
    #[serde(default)]
    pub aiprofile: super::aiprofile::AiprofilePools,
    #[cfg(feature = "aiwavecollection")]
    #[serde(default)]
    pub aiwavecollection: super::aiwavecollection::AiwavecollectionPools,
    #[cfg(feature = "analytics")]
    #[serde(default)]
    pub analytics: super::analytics::AnalyticsPools,
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
    #[cfg(feature = "cargomanifest")]
    #[serde(default)]
    pub cargomanifest: super::cargomanifest::CargomanifestPools,
    #[cfg(feature = "character")]
    #[serde(default)]
    pub character: super::character::CharacterPools,
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
    #[cfg(feature = "commodityconfiguration")]
    #[serde(default)]
    pub commodityconfiguration: super::commodityconfiguration::CommodityconfigurationPools,
    #[cfg(feature = "commoditytypedatabase")]
    #[serde(default)]
    pub commoditytypedatabase: super::commoditytypedatabase::CommoditytypedatabasePools,
    #[cfg(feature = "communicationatlconfig")]
    #[serde(default)]
    pub communicationatlconfig: super::communicationatlconfig::CommunicationatlconfigPools,
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
    #[cfg(feature = "creatures")]
    #[serde(default)]
    pub creatures: super::creatures::CreaturesPools,
    #[cfg(feature = "densityclasses")]
    #[serde(default)]
    pub densityclasses: super::densityclasses::DensityclassesPools,
    #[cfg(feature = "dialoguecontentbank")]
    #[serde(default)]
    pub dialoguecontentbank: super::dialoguecontentbank::DialoguecontentbankPools,
    #[cfg(feature = "dynamiccameraeffects")]
    #[serde(default)]
    pub dynamiccameraeffects: super::dynamiccameraeffects::DynamiccameraeffectsPools,
    #[cfg(feature = "emotions")]
    #[serde(default)]
    pub emotions: super::emotions::EmotionsPools,
    #[cfg(feature = "entities-aft")]
    #[serde(default)]
    pub entities_aft: super::entities_aft::EntitiesAftPools,
    #[cfg(feature = "entities-aicoversurface")]
    #[serde(default)]
    pub entities_aicoversurface: super::entities_aicoversurface::EntitiesAicoversurfacePools,
    #[cfg(feature = "entities-aigroupentity")]
    #[serde(default)]
    pub entities_aigroupentity: super::entities_aigroupentity::EntitiesAigroupentityPools,
    #[cfg(feature = "entities-ainavigationobstacle")]
    #[serde(default)]
    pub entities_ainavigationobstacle: super::entities_ainavigationobstacle::EntitiesAinavigationobstaclePools,
    #[cfg(feature = "entities-analyticshadow")]
    #[serde(default)]
    pub entities_analyticshadow: super::entities_analyticshadow::EntitiesAnalyticshadowPools,
    #[cfg(feature = "entities-animentity_character")]
    #[serde(default)]
    pub entities_animentity_character: super::entities_animentity_character::EntitiesAnimentity_characterPools,
    #[cfg(feature = "entities-area")]
    #[serde(default)]
    pub entities_area: super::entities_area::EntitiesAreaPools,
    #[cfg(feature = "entities-attachableobjectcontainerspawner")]
    #[serde(default)]
    pub entities_attachableobjectcontainerspawner: super::entities_attachableobjectcontainerspawner::EntitiesAttachableobjectcontainerspawnerPools,
    #[cfg(feature = "entities-audio")]
    #[serde(default)]
    pub entities_audio: super::entities_audio::EntitiesAudioPools,
    #[cfg(feature = "entities-basebuilding")]
    #[serde(default)]
    pub entities_basebuilding: super::entities_basebuilding::EntitiesBasebuildingPools,
    #[cfg(feature = "entities-breakablerock")]
    #[serde(default)]
    pub entities_breakablerock: super::entities_breakablerock::EntitiesBreakablerockPools,
    #[cfg(feature = "entities-capturearea")]
    #[serde(default)]
    pub entities_capturearea: super::entities_capturearea::EntitiesCaptureareaPools,
    #[cfg(feature = "entities-charactercustomizer")]
    #[serde(default)]
    pub entities_charactercustomizer: super::entities_charactercustomizer::EntitiesCharactercustomizerPools,
    #[cfg(feature = "entities-corpsemarker")]
    #[serde(default)]
    pub entities_corpsemarker: super::entities_corpsemarker::EntitiesCorpsemarkerPools,
    #[cfg(feature = "entities-cuttableshape")]
    #[serde(default)]
    pub entities_cuttableshape: super::entities_cuttableshape::EntitiesCuttableshapePools,
    #[cfg(feature = "entities-destruction")]
    #[serde(default)]
    pub entities_destruction: super::entities_destruction::EntitiesDestructionPools,
    #[cfg(feature = "entities-displayscreens")]
    #[serde(default)]
    pub entities_displayscreens: super::entities_displayscreens::EntitiesDisplayscreensPools,
    #[cfg(feature = "entities-ea")]
    #[serde(default)]
    pub entities_ea: super::entities_ea::EntitiesEaPools,
    #[cfg(feature = "entities-entityclassdefinition_colorgradient")]
    #[serde(default)]
    pub entities_entityclassdefinition_colorgradient: super::entities_entityclassdefinition_colorgradient::EntitiesEntityclassdefinition_colorgradientPools,
    #[cfg(feature = "entities-entityclassdefinition_pointofinterestprovider")]
    #[serde(default)]
    pub entities_entityclassdefinition_pointofinterestprovider: super::entities_entityclassdefinition_pointofinterestprovider::EntitiesEntityclassdefinition_pointofinterestproviderPools,
    #[cfg(feature = "entities-entityclassdefinition_racecheckpoint")]
    #[serde(default)]
    pub entities_entityclassdefinition_racecheckpoint: super::entities_entityclassdefinition_racecheckpoint::EntitiesEntityclassdefinition_racecheckpointPools,
    #[cfg(feature = "entities-entityclassdefinition_shopkiosk")]
    #[serde(default)]
    pub entities_entityclassdefinition_shopkiosk: super::entities_entityclassdefinition_shopkiosk::EntitiesEntityclassdefinition_shopkioskPools,
    #[cfg(feature = "entities-entityclassdefinition_spawnprotectionbarrier")]
    #[serde(default)]
    pub entities_entityclassdefinition_spawnprotectionbarrier: super::entities_entityclassdefinition_spawnprotectionbarrier::EntitiesEntityclassdefinition_spawnprotectionbarrierPools,
    #[cfg(feature = "entities-entityclassdefinition_spectatorpoint")]
    #[serde(default)]
    pub entities_entityclassdefinition_spectatorpoint: super::entities_entityclassdefinition_spectatorpoint::EntitiesEntityclassdefinition_spectatorpointPools,
    #[cfg(feature = "entities-entityclassdefinition_tilesocket")]
    #[serde(default)]
    pub entities_entityclassdefinition_tilesocket: super::entities_entityclassdefinition_tilesocket::EntitiesEntityclassdefinition_tilesocketPools,
    #[cfg(feature = "entities-entityspawncameracontroller")]
    #[serde(default)]
    pub entities_entityspawncameracontroller: super::entities_entityspawncameracontroller::EntitiesEntityspawncameracontrollerPools,
    #[cfg(feature = "entities-entityspawners")]
    #[serde(default)]
    pub entities_entityspawners: super::entities_entityspawners::EntitiesEntityspawnersPools,
    #[cfg(feature = "entities-environment")]
    #[serde(default)]
    pub entities_environment: super::entities_environment::EntitiesEnvironmentPools,
    #[cfg(feature = "entities-gamerulessingleton")]
    #[serde(default)]
    pub entities_gamerulessingleton: super::entities_gamerulessingleton::EntitiesGamerulessingletonPools,
    #[cfg(feature = "entities-gascloud")]
    #[serde(default)]
    pub entities_gascloud: super::entities_gascloud::EntitiesGascloudPools,
    #[cfg(feature = "entities-geometryinstancer")]
    #[serde(default)]
    pub entities_geometryinstancer: super::entities_geometryinstancer::EntitiesGeometryinstancerPools,
    #[cfg(feature = "entities-groundvehicles")]
    #[serde(default)]
    pub entities_groundvehicles: super::entities_groundvehicles::EntitiesGroundvehiclesPools,
    #[cfg(feature = "entities-holofield")]
    #[serde(default)]
    pub entities_holofield: super::entities_holofield::EntitiesHolofieldPools,
    #[cfg(feature = "entities-jumppoints")]
    #[serde(default)]
    pub entities_jumppoints: super::entities_jumppoints::EntitiesJumppointsPools,
    #[cfg(feature = "entities-ladder")]
    #[serde(default)]
    pub entities_ladder: super::entities_ladder::EntitiesLadderPools,
    #[cfg(feature = "entities-ledgeobject")]
    #[serde(default)]
    pub entities_ledgeobject: super::entities_ledgeobject::EntitiesLedgeobjectPools,
    #[cfg(feature = "entities-lights")]
    #[serde(default)]
    pub entities_lights: super::entities_lights::EntitiesLightsPools,
    #[cfg(feature = "entities-loadingplatformmanager")]
    #[serde(default)]
    pub entities_loadingplatformmanager: super::entities_loadingplatformmanager::EntitiesLoadingplatformmanagerPools,
    #[cfg(feature = "entities-locationmanager")]
    #[serde(default)]
    pub entities_locationmanager: super::entities_locationmanager::EntitiesLocationmanagerPools,
    #[cfg(feature = "entities-maglaunch")]
    #[serde(default)]
    pub entities_maglaunch: super::entities_maglaunch::EntitiesMaglaunchPools,
    #[cfg(feature = "entities-mastercontrollerentities")]
    #[serde(default)]
    pub entities_mastercontrollerentities: super::entities_mastercontrollerentities::EntitiesMastercontrollerentitiesPools,
    #[cfg(feature = "entities-missionobjectivemarker")]
    #[serde(default)]
    pub entities_missionobjectivemarker: super::entities_missionobjectivemarker::EntitiesMissionobjectivemarkerPools,
    #[cfg(feature = "entities-missionsystem")]
    #[serde(default)]
    pub entities_missionsystem: super::entities_missionsystem::EntitiesMissionsystemPools,
    #[cfg(feature = "entities-objectcontainers")]
    #[serde(default)]
    pub entities_objectcontainers: super::entities_objectcontainers::EntitiesObjectcontainersPools,
    #[cfg(feature = "entities-others")]
    #[serde(default)]
    pub entities_others: super::entities_others::EntitiesOthersPools,
    #[cfg(feature = "entities-partymembermarker")]
    #[serde(default)]
    pub entities_partymembermarker: super::entities_partymembermarker::EntitiesPartymembermarkerPools,
    #[cfg(feature = "entities-patrolgraph")]
    #[serde(default)]
    pub entities_patrolgraph: super::entities_patrolgraph::EntitiesPatrolgraphPools,
    #[cfg(feature = "entities-patrolgraphlink")]
    #[serde(default)]
    pub entities_patrolgraphlink: super::entities_patrolgraphlink::EntitiesPatrolgraphlinkPools,
    #[cfg(feature = "entities-patrolpoint")]
    #[serde(default)]
    pub entities_patrolpoint: super::entities_patrolpoint::EntitiesPatrolpointPools,
    #[cfg(feature = "entities-perceptionmodifierarea")]
    #[serde(default)]
    pub entities_perceptionmodifierarea: super::entities_perceptionmodifierarea::EntitiesPerceptionmodifierareaPools,
    #[cfg(feature = "entities-physicalizedjobboard")]
    #[serde(default)]
    pub entities_physicalizedjobboard: super::entities_physicalizedjobboard::EntitiesPhysicalizedjobboardPools,
    #[cfg(feature = "entities-physics")]
    #[serde(default)]
    pub entities_physics: super::entities_physics::EntitiesPhysicsPools,
    #[cfg(feature = "entities-planetkillvolume")]
    #[serde(default)]
    pub entities_planetkillvolume: super::entities_planetkillvolume::EntitiesPlanetkillvolumePools,
    #[cfg(feature = "entities-projectilespawnerentity")]
    #[serde(default)]
    pub entities_projectilespawnerentity: super::entities_projectilespawnerentity::EntitiesProjectilespawnerentityPools,
    #[cfg(feature = "entities-proximityassistmodifier")]
    #[serde(default)]
    pub entities_proximityassistmodifier: super::entities_proximityassistmodifier::EntitiesProximityassistmodifierPools,
    #[cfg(feature = "entities-quantumcolorshift")]
    #[serde(default)]
    pub entities_quantumcolorshift: super::entities_quantumcolorshift::EntitiesQuantumcolorshiftPools,
    #[cfg(feature = "entities-quantumtrail")]
    #[serde(default)]
    pub entities_quantumtrail: super::entities_quantumtrail::EntitiesQuantumtrailPools,
    #[cfg(feature = "entities-racering")]
    #[serde(default)]
    pub entities_racering: super::entities_racering::EntitiesRaceringPools,
    #[cfg(feature = "entities-rastar")]
    #[serde(default)]
    pub entities_rastar: super::entities_rastar::EntitiesRastarPools,
    #[cfg(feature = "entities-refinery")]
    #[serde(default)]
    pub entities_refinery: super::entities_refinery::EntitiesRefineryPools,
    #[cfg(feature = "entities-refuelatmospheres")]
    #[serde(default)]
    pub entities_refuelatmospheres: super::entities_refuelatmospheres::EntitiesRefuelatmospheresPools,
    #[cfg(feature = "entities-render")]
    #[serde(default)]
    pub entities_render: super::entities_render::EntitiesRenderPools,
    #[cfg(feature = "entities-restricted_areas")]
    #[serde(default)]
    pub entities_restricted_areas: super::entities_restricted_areas::EntitiesRestricted_areasPools,
    #[cfg(feature = "entities-roomsystem")]
    #[serde(default)]
    pub entities_roomsystem: super::entities_roomsystem::EntitiesRoomsystemPools,
    #[cfg(feature = "entities-scarmorymanager")]
    #[serde(default)]
    pub entities_scarmorymanager: super::entities_scarmorymanager::EntitiesScarmorymanagerPools,
    #[cfg(feature = "entities-scitem-actormovables")]
    #[serde(default)]
    pub entities_scitem_actormovables: super::entities_scitem_actormovables::EntitiesScitemActormovablesPools,
    #[cfg(feature = "entities-scitem-airlocks")]
    #[serde(default)]
    pub entities_scitem_airlocks: super::entities_scitem_airlocks::EntitiesScitemAirlocksPools,
    #[cfg(feature = "entities-scitem-carryables")]
    #[serde(default)]
    pub entities_scitem_carryables: super::entities_scitem_carryables::EntitiesScitemCarryablesPools,
    #[cfg(feature = "entities-scitem-characters")]
    #[serde(default)]
    pub entities_scitem_characters: super::entities_scitem_characters::EntitiesScitemCharactersPools,
    #[cfg(feature = "entities-scitem-commsreceiver")]
    #[serde(default)]
    pub entities_scitem_commsreceiver: super::entities_scitem_commsreceiver::EntitiesScitemCommsreceiverPools,
    #[cfg(feature = "entities-scitem-default_lensdisplay_pu")]
    #[serde(default)]
    pub entities_scitem_default_lensdisplay_pu: super::entities_scitem_default_lensdisplay_pu::EntitiesScitemDefault_lensdisplay_puPools,
    #[cfg(feature = "entities-scitem-deliveryitemportmanager")]
    #[serde(default)]
    pub entities_scitem_deliveryitemportmanager: super::entities_scitem_deliveryitemportmanager::EntitiesScitemDeliveryitemportmanagerPools,
    #[cfg(feature = "entities-scitem-doors")]
    #[serde(default)]
    pub entities_scitem_doors: super::entities_scitem_doors::EntitiesScitemDoorsPools,
    #[cfg(feature = "entities-scitem-entityclassdefinition_sequenceobjectitem")]
    #[serde(default)]
    pub entities_scitem_entityclassdefinition_sequenceobjectitem: super::entities_scitem_entityclassdefinition_sequenceobjectitem::EntitiesScitemEntityclassdefinition_sequenceobjectitemPools,
    #[cfg(feature = "entities-scitem-entityclassdefinition_test_gadget")]
    #[serde(default)]
    pub entities_scitem_entityclassdefinition_test_gadget: super::entities_scitem_entityclassdefinition_test_gadget::EntitiesScitemEntityclassdefinition_test_gadgetPools,
    #[cfg(feature = "entities-scitem-flair")]
    #[serde(default)]
    pub entities_scitem_flair: super::entities_scitem_flair::EntitiesScitemFlairPools,
    #[cfg(feature = "entities-scitem-gameplayinteractables")]
    #[serde(default)]
    pub entities_scitem_gameplayinteractables: super::entities_scitem_gameplayinteractables::EntitiesScitemGameplayinteractablesPools,
    #[cfg(feature = "entities-scitem-human")]
    #[serde(default)]
    pub entities_scitem_human: super::entities_scitem_human::EntitiesScitemHumanPools,
    #[cfg(feature = "entities-scitem-locations")]
    #[serde(default)]
    pub entities_scitem_locations: super::entities_scitem_locations::EntitiesScitemLocationsPools,
    #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
    #[serde(default)]
    pub entities_scitem_mastermodeexclusionglobalparams: super::entities_scitem_mastermodeexclusionglobalparams::EntitiesScitemMastermodeexclusionglobalparamsPools,
    #[cfg(feature = "entities-scitem-mission_entities")]
    #[serde(default)]
    pub entities_scitem_mission_entities: super::entities_scitem_mission_entities::EntitiesScitemMission_entitiesPools,
    #[cfg(feature = "entities-scitem-missionstorage")]
    #[serde(default)]
    pub entities_scitem_missionstorage: super::entities_scitem_missionstorage::EntitiesScitemMissionstoragePools,
    #[cfg(feature = "entities-scitem-remoteconnectionreceiver")]
    #[serde(default)]
    pub entities_scitem_remoteconnectionreceiver: super::entities_scitem_remoteconnectionreceiver::EntitiesScitemRemoteconnectionreceiverPools,
    #[cfg(feature = "entities-scitem-scitem_debris")]
    #[serde(default)]
    pub entities_scitem_scitem_debris: super::entities_scitem_scitem_debris::EntitiesScitemScitem_debrisPools,
    #[cfg(feature = "entities-scitem-ships")]
    #[serde(default)]
    pub entities_scitem_ships: super::entities_scitem_ships::EntitiesScitemShipsPools,
    #[cfg(feature = "entities-scitem-storage")]
    #[serde(default)]
    pub entities_scitem_storage: super::entities_scitem_storage::EntitiesScitemStoragePools,
    #[cfg(feature = "entities-scitem-suit")]
    #[serde(default)]
    pub entities_scitem_suit: super::entities_scitem_suit::EntitiesScitemSuitPools,
    #[cfg(feature = "entities-scitem-toggleableentities")]
    #[serde(default)]
    pub entities_scitem_toggleableentities: super::entities_scitem_toggleableentities::EntitiesScitemToggleableentitiesPools,
    #[cfg(feature = "entities-scitem-usables")]
    #[serde(default)]
    pub entities_scitem_usables: super::entities_scitem_usables::EntitiesScitemUsablesPools,
    #[cfg(feature = "entities-scitem-weapons")]
    #[serde(default)]
    pub entities_scitem_weapons: super::entities_scitem_weapons::EntitiesScitemWeaponsPools,
    #[cfg(feature = "entities-shadowregionentity")]
    #[serde(default)]
    pub entities_shadowregionentity: super::entities_shadowregionentity::EntitiesShadowregionentityPools,
    #[cfg(feature = "entities-slotspawnpoint")]
    #[serde(default)]
    pub entities_slotspawnpoint: super::entities_slotspawnpoint::EntitiesSlotspawnpointPools,
    #[cfg(feature = "entities-spawnhelper")]
    #[serde(default)]
    pub entities_spawnhelper: super::entities_spawnhelper::EntitiesSpawnhelperPools,
    #[cfg(feature = "entities-sunlight")]
    #[serde(default)]
    pub entities_sunlight: super::entities_sunlight::EntitiesSunlightPools,
    #[cfg(feature = "entities-test")]
    #[serde(default)]
    pub entities_test: super::entities_test::EntitiesTestPools,
    #[cfg(feature = "entities-tinymachine")]
    #[serde(default)]
    pub entities_tinymachine: super::entities_tinymachine::EntitiesTinymachinePools,
    #[cfg(feature = "entities-ui")]
    #[serde(default)]
    pub entities_ui: super::entities_ui::EntitiesUiPools,
    #[cfg(feature = "entities-unattendedvehiclemarker")]
    #[serde(default)]
    pub entities_unattendedvehiclemarker: super::entities_unattendedvehiclemarker::EntitiesUnattendedvehiclemarkerPools,
    #[cfg(feature = "entities-usablegroup")]
    #[serde(default)]
    pub entities_usablegroup: super::entities_usablegroup::EntitiesUsablegroupPools,
    #[cfg(feature = "entities-usablegroupcoordinator")]
    #[serde(default)]
    pub entities_usablegroupcoordinator: super::entities_usablegroupcoordinator::EntitiesUsablegroupcoordinatorPools,
    #[cfg(feature = "entities-vehicleobjectcontainer")]
    #[serde(default)]
    pub entities_vehicleobjectcontainer: super::entities_vehicleobjectcontainer::EntitiesVehicleobjectcontainerPools,
    #[cfg(feature = "entities-vfx")]
    #[serde(default)]
    pub entities_vfx: super::entities_vfx::EntitiesVfxPools,
    #[cfg(feature = "entities-watervolume")]
    #[serde(default)]
    pub entities_watervolume: super::entities_watervolume::EntitiesWatervolumePools,
    #[cfg(feature = "entities-worlddisplay")]
    #[serde(default)]
    pub entities_worlddisplay: super::entities_worlddisplay::EntitiesWorlddisplayPools,
    #[cfg(feature = "entitlementpolicies")]
    #[serde(default)]
    pub entitlementpolicies: super::entitlementpolicies::EntitlementpoliciesPools,
    #[cfg(feature = "entityclassdefinition")]
    #[serde(default)]
    pub entityclassdefinition: super::entityclassdefinition::EntityclassdefinitionPools,
    #[cfg(feature = "environments")]
    #[serde(default)]
    pub environments: super::environments::EnvironmentsPools,
    #[cfg(feature = "evagraph")]
    #[serde(default)]
    pub evagraph: super::evagraph::EvagraphPools,
    #[cfg(feature = "explosiveordnance")]
    #[serde(default)]
    pub explosiveordnance: super::explosiveordnance::ExplosiveordnancePools,
    #[cfg(feature = "fidgetconfig")]
    #[serde(default)]
    pub fidgetconfig: super::fidgetconfig::FidgetconfigPools,
    #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
    #[serde(default)]
    pub forcefeedback_forcefeedbackeffects: super::forcefeedback_forcefeedbackeffects::Forcefeedback_forcefeedbackeffectsPools,
    #[cfg(feature = "friendmanager")]
    #[serde(default)]
    pub friendmanager: super::friendmanager::FriendmanagerPools,
    #[cfg(feature = "gamemode")]
    #[serde(default)]
    pub gamemode: super::gamemode::GamemodePools,
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
    #[cfg(feature = "globalshopparams")]
    #[serde(default)]
    pub globalshopparams: super::globalshopparams::GlobalshopparamsPools,
    #[cfg(feature = "globaltutorialparams")]
    #[serde(default)]
    pub globaltutorialparams: super::globaltutorialparams::GlobaltutorialparamsPools,
    #[cfg(feature = "handholdgripdatabase")]
    #[serde(default)]
    pub handholdgripdatabase: super::handholdgripdatabase::HandholdgripdatabasePools,
    #[cfg(feature = "hardwaremouse")]
    #[serde(default)]
    pub hardwaremouse: super::hardwaremouse::HardwaremousePools,
    #[cfg(feature = "harvestable")]
    #[serde(default)]
    pub harvestable: super::harvestable::HarvestablePools,
    #[cfg(feature = "hudparams")]
    #[serde(default)]
    pub hudparams: super::hudparams::HudparamsPools,
    #[cfg(feature = "ifcs")]
    #[serde(default)]
    pub ifcs: super::ifcs::IfcsPools,
    #[cfg(feature = "instancedinterior")]
    #[serde(default)]
    pub instancedinterior: super::instancedinterior::InstancedinteriorPools,
    #[cfg(feature = "interactionconditions")]
    #[serde(default)]
    pub interactionconditions: super::interactionconditions::InteractionconditionsPools,
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
    #[cfg(feature = "lawsystem")]
    #[serde(default)]
    pub lawsystem: super::lawsystem::LawsystemPools,
    #[cfg(feature = "leangraph")]
    #[serde(default)]
    pub leangraph: super::leangraph::LeangraphPools,
    #[cfg(feature = "longtermpersistence")]
    #[serde(default)]
    pub longtermpersistence: super::longtermpersistence::LongtermpersistencePools,
    #[cfg(feature = "lootgeneration")]
    #[serde(default)]
    pub lootgeneration: super::lootgeneration::LootgenerationPools,
    #[cfg(feature = "megamap")]
    #[serde(default)]
    pub megamap: super::megamap::MegamapPools,
    #[cfg(feature = "missionbroker")]
    #[serde(default)]
    pub missionbroker: super::missionbroker::MissionbrokerPools,
    #[cfg(feature = "missiondata")]
    #[serde(default)]
    pub missiondata: super::missiondata::MissiondataPools,
    #[cfg(feature = "missionfailureconditions")]
    #[serde(default)]
    pub missionfailureconditions: super::missionfailureconditions::MissionfailureconditionsPools,
    #[cfg(feature = "motionstatemachine")]
    #[serde(default)]
    pub motionstatemachine: super::motionstatemachine::MotionstatemachinePools,
    #[cfg(feature = "musiclogic")]
    #[serde(default)]
    pub musiclogic: super::musiclogic::MusiclogicPools,
    #[cfg(feature = "personalinnerthoughtrules")]
    #[serde(default)]
    pub personalinnerthoughtrules: super::personalinnerthoughtrules::PersonalinnerthoughtrulesPools,
    #[cfg(feature = "planetdaynighttemperatureparams")]
    #[serde(default)]
    pub planetdaynighttemperatureparams: super::planetdaynighttemperatureparams::PlanetdaynighttemperatureparamsPools,
    #[cfg(feature = "procbreathing")]
    #[serde(default)]
    pub procbreathing: super::procbreathing::ProcbreathingPools,
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
    #[cfg(feature = "scuttableshapedefinition")]
    #[serde(default)]
    pub scuttableshapedefinition: super::scuttableshapedefinition::ScuttableshapedefinitionPools,
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
    #[cfg(feature = "sreputationglobalcontextbbparams")]
    #[serde(default)]
    pub sreputationglobalcontextbbparams: super::sreputationglobalcontextbbparams::SreputationglobalcontextbbparamsPools,
    #[cfg(feature = "starmap")]
    #[serde(default)]
    pub starmap: super::starmap::StarmapPools,
    #[cfg(feature = "tacticalquery")]
    #[serde(default)]
    pub tacticalquery: super::tacticalquery::TacticalqueryPools,
    #[cfg(feature = "tagdatabase")]
    #[serde(default)]
    pub tagdatabase: super::tagdatabase::TagdatabasePools,
    #[cfg(feature = "trackview")]
    #[serde(default)]
    pub trackview: super::trackview::TrackviewPools,
    #[cfg(feature = "transitsystem")]
    #[serde(default)]
    pub transitsystem: super::transitsystem::TransitsystemPools,
    #[cfg(feature = "transponder")]
    #[serde(default)]
    pub transponder: super::transponder::TransponderPools,
    #[cfg(feature = "transportsystem")]
    #[serde(default)]
    pub transportsystem: super::transportsystem::TransportsystemPools,
    #[cfg(feature = "turret")]
    #[serde(default)]
    pub turret: super::turret::TurretPools,
    #[cfg(feature = "ui-animatedmarkers")]
    #[serde(default)]
    pub ui_animatedmarkers: super::ui_animatedmarkers::UiAnimatedmarkersPools,
    #[cfg(feature = "ui-areamap")]
    #[serde(default)]
    pub ui_areamap: super::ui_areamap::UiAreamapPools,
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
    #[cfg(feature = "ui-flashobjectbindinggroups")]
    #[serde(default)]
    pub ui_flashobjectbindinggroups: super::ui_flashobjectbindinggroups::UiFlashobjectbindinggroupsPools,
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
    #[cfg(feature = "ui-itemtypedefinition")]
    #[serde(default)]
    pub ui_itemtypedefinition: super::ui_itemtypedefinition::UiItemtypedefinitionPools,
    #[cfg(feature = "ui-markertrackingvolumeconfig")]
    #[serde(default)]
    pub ui_markertrackingvolumeconfig: super::ui_markertrackingvolumeconfig::UiMarkertrackingvolumeconfigPools,
    #[cfg(feature = "ui-missilelockreticleconfig")]
    #[serde(default)]
    pub ui_missilelockreticleconfig: super::ui_missilelockreticleconfig::UiMissilelockreticleconfigPools,
    #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
    #[serde(default)]
    pub ui_objectdatabankentrymarkerconfig: super::ui_objectdatabankentrymarkerconfig::UiObjectdatabankentrymarkerconfigPools,
    #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
    #[serde(default)]
    pub ui_playerchoice_library_playerchoicelibrary: super::ui_playerchoice_library_playerchoicelibrary::UiPlayerchoice_library_playerchoicelibraryPools,
    #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
    #[serde(default)]
    pub ui_playerecggraph_config_playerecggraphconfig: super::ui_playerecggraph_config_playerecggraphconfig::UiPlayerecggraph_config_playerecggraphconfigPools,
    #[cfg(feature = "ui-popups")]
    #[serde(default)]
    pub ui_popups: super::ui_popups::UiPopupsPools,
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
    #[cfg(feature = "ui-uimodes")]
    #[serde(default)]
    pub ui_uimodes: super::ui_uimodes::UiUimodesPools,
    #[cfg(feature = "ui-uistatedisplay")]
    #[serde(default)]
    pub ui_uistatedisplay: super::ui_uistatedisplay::UiUistatedisplayPools,
    #[cfg(feature = "ui-videocomms")]
    #[serde(default)]
    pub ui_videocomms: super::ui_videocomms::UiVideocommsPools,
    #[cfg(feature = "unittest_unittestb")]
    #[serde(default)]
    pub unittest_unittestb: super::unittest_unittestb::Unittest_unittestbPools,
    #[cfg(feature = "vehicle")]
    #[serde(default)]
    pub vehicle: super::vehicle::VehiclePools,
    #[cfg(feature = "vfx")]
    #[serde(default)]
    pub vfx: super::vfx::VfxPools,
    #[cfg(feature = "zerogtraversalgraph")]
    #[serde(default)]
    pub zerogtraversalgraph: super::zerogtraversalgraph::ZerogtraversalgraphPools,
}
