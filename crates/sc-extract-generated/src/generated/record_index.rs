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

#[derive(Default)]
pub struct RecordIndex {
    pub multi_feature: super::multi_feature::MultiFeatureIndex,
    #[cfg(feature = "actor-externalforceresponse")]
    pub actor_externalforceresponse: super::actor_externalforceresponse::ActorExternalforceresponseIndex,
    #[cfg(feature = "actor-gforce")]
    pub actor_gforce: super::actor_gforce::ActorGforceIndex,
    #[cfg(feature = "actor-inputdeflectiontime")]
    pub actor_inputdeflectiontime: super::actor_inputdeflectiontime::ActorInputdeflectiontimeIndex,
    #[cfg(feature = "actor-playerdefaultactionsconfig")]
    pub actor_playerdefaultactionsconfig: super::actor_playerdefaultactionsconfig::ActorPlayerdefaultactionsconfigIndex,
    #[cfg(feature = "actor-quantumtravelcameraeffects")]
    pub actor_quantumtravelcameraeffects: super::actor_quantumtravelcameraeffects::ActorQuantumtravelcameraeffectsIndex,
    #[cfg(feature = "aiglobalsettings")]
    pub aiglobalsettings: super::aiglobalsettings::AiglobalsettingsIndex,
    #[cfg(feature = "aiprofile")]
    pub aiprofile: super::aiprofile::AiprofileIndex,
    #[cfg(feature = "aiwavecollection")]
    pub aiwavecollection: super::aiwavecollection::AiwavecollectionIndex,
    #[cfg(feature = "analytics")]
    pub analytics: super::analytics::AnalyticsIndex,
    #[cfg(feature = "areaservices")]
    pub areaservices: super::areaservices::AreaservicesIndex,
    #[cfg(feature = "audio")]
    pub audio: super::audio::AudioIndex,
    #[cfg(feature = "awardservice")]
    pub awardservice: super::awardservice::AwardserviceIndex,
    #[cfg(feature = "cameras")]
    pub cameras: super::cameras::CamerasIndex,
    #[cfg(feature = "character")]
    pub character: super::character::CharacterIndex,
    #[cfg(feature = "chatcommandfastaccess")]
    pub chatcommandfastaccess: super::chatcommandfastaccess::ChatcommandfastaccessIndex,
    #[cfg(feature = "chatemoterecord")]
    pub chatemoterecord: super::chatemoterecord::ChatemoterecordIndex,
    #[cfg(feature = "chatfilteroptions")]
    pub chatfilteroptions: super::chatfilteroptions::ChatfilteroptionsIndex,
    #[cfg(feature = "chatmanager")]
    pub chatmanager: super::chatmanager::ChatmanagerIndex,
    #[cfg(feature = "commodityconfiguration")]
    pub commodityconfiguration: super::commodityconfiguration::CommodityconfigurationIndex,
    #[cfg(feature = "commoditytypedatabase")]
    pub commoditytypedatabase: super::commoditytypedatabase::CommoditytypedatabaseIndex,
    #[cfg(feature = "communicationatlconfig")]
    pub communicationatlconfig: super::communicationatlconfig::CommunicationatlconfigIndex,
    #[cfg(feature = "communicationsystem")]
    pub communicationsystem: super::communicationsystem::CommunicationsystemIndex,
    #[cfg(feature = "communicationvariableconfig")]
    pub communicationvariableconfig: super::communicationvariableconfig::CommunicationvariableconfigIndex,
    #[cfg(feature = "consumabletypesdatabase")]
    pub consumabletypesdatabase: super::consumabletypesdatabase::ConsumabletypesdatabaseIndex,
    #[cfg(feature = "contextualcommunicationconfig")]
    pub contextualcommunicationconfig: super::contextualcommunicationconfig::ContextualcommunicationconfigIndex,
    #[cfg(feature = "contracts")]
    pub contracts: super::contracts::ContractsIndex,
    #[cfg(feature = "conversation")]
    pub conversation: super::conversation::ConversationIndex,
    #[cfg(feature = "crafting")]
    pub crafting: super::crafting::CraftingIndex,
    #[cfg(feature = "densityclasses")]
    pub densityclasses: super::densityclasses::DensityclassesIndex,
    #[cfg(feature = "dialoguecontentbank")]
    pub dialoguecontentbank: super::dialoguecontentbank::DialoguecontentbankIndex,
    #[cfg(feature = "dynamiccameraeffects")]
    pub dynamiccameraeffects: super::dynamiccameraeffects::DynamiccameraeffectsIndex,
    #[cfg(feature = "emotions")]
    pub emotions: super::emotions::EmotionsIndex,
    #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
    pub entities_scitem_mastermodeexclusionglobalparams: super::entities_scitem_mastermodeexclusionglobalparams::EntitiesScitemMastermodeexclusionglobalparamsIndex,
    #[cfg(feature = "entities-scitem-ships")]
    pub entities_scitem_ships: super::entities_scitem_ships::EntitiesScitemShipsIndex,
    #[cfg(feature = "entities-scitem-usables")]
    pub entities_scitem_usables: super::entities_scitem_usables::EntitiesScitemUsablesIndex,
    #[cfg(feature = "entities-ui")]
    pub entities_ui: super::entities_ui::EntitiesUiIndex,
    #[cfg(feature = "entities-vfx")]
    pub entities_vfx: super::entities_vfx::EntitiesVfxIndex,
    #[cfg(feature = "entitlementpolicies")]
    pub entitlementpolicies: super::entitlementpolicies::EntitlementpoliciesIndex,
    #[cfg(feature = "environments")]
    pub environments: super::environments::EnvironmentsIndex,
    #[cfg(feature = "evagraph")]
    pub evagraph: super::evagraph::EvagraphIndex,
    #[cfg(feature = "explosiveordnance")]
    pub explosiveordnance: super::explosiveordnance::ExplosiveordnanceIndex,
    #[cfg(feature = "fidgetconfig")]
    pub fidgetconfig: super::fidgetconfig::FidgetconfigIndex,
    #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
    pub forcefeedback_forcefeedbackeffects: super::forcefeedback_forcefeedbackeffects::Forcefeedback_forcefeedbackeffectsIndex,
    #[cfg(feature = "friendmanager")]
    pub friendmanager: super::friendmanager::FriendmanagerIndex,
    #[cfg(feature = "gamemode")]
    pub gamemode: super::gamemode::GamemodeIndex,
    #[cfg(feature = "globalarmarkerparams")]
    pub globalarmarkerparams: super::globalarmarkerparams::GlobalarmarkerparamsIndex,
    #[cfg(feature = "globalcargoloadingparams")]
    pub globalcargoloadingparams: super::globalcargoloadingparams::GlobalcargoloadingparamsIndex,
    #[cfg(feature = "globalcommsnotificationparams")]
    pub globalcommsnotificationparams: super::globalcommsnotificationparams::GlobalcommsnotificationparamsIndex,
    #[cfg(feature = "globalcuttableshapeparams")]
    pub globalcuttableshapeparams: super::globalcuttableshapeparams::GlobalcuttableshapeparamsIndex,
    #[cfg(feature = "globalinteractionparams")]
    pub globalinteractionparams: super::globalinteractionparams::GlobalinteractionparamsIndex,
    #[cfg(feature = "globalshopparams")]
    pub globalshopparams: super::globalshopparams::GlobalshopparamsIndex,
    #[cfg(feature = "globaltutorialparams")]
    pub globaltutorialparams: super::globaltutorialparams::GlobaltutorialparamsIndex,
    #[cfg(feature = "handholdgripdatabase")]
    pub handholdgripdatabase: super::handholdgripdatabase::HandholdgripdatabaseIndex,
    #[cfg(feature = "hardwaremouse")]
    pub hardwaremouse: super::hardwaremouse::HardwaremouseIndex,
    #[cfg(feature = "harvestable")]
    pub harvestable: super::harvestable::HarvestableIndex,
    #[cfg(feature = "hudparams")]
    pub hudparams: super::hudparams::HudparamsIndex,
    #[cfg(feature = "ifcs")]
    pub ifcs: super::ifcs::IfcsIndex,
    #[cfg(feature = "instancedinterior")]
    pub instancedinterior: super::instancedinterior::InstancedinteriorIndex,
    #[cfg(feature = "interactionconditions")]
    pub interactionconditions: super::interactionconditions::InteractionconditionsIndex,
    #[cfg(feature = "inventorycontainers")]
    pub inventorycontainers: super::inventorycontainers::InventorycontainersIndex,
    #[cfg(feature = "item")]
    pub item: super::item::ItemIndex,
    #[cfg(feature = "journalentry")]
    pub journalentry: super::journalentry::JournalentryIndex,
    #[cfg(feature = "jumppoints")]
    pub jumppoints: super::jumppoints::JumppointsIndex,
    #[cfg(feature = "lawsystem")]
    pub lawsystem: super::lawsystem::LawsystemIndex,
    #[cfg(feature = "leangraph")]
    pub leangraph: super::leangraph::LeangraphIndex,
    #[cfg(feature = "longtermpersistence")]
    pub longtermpersistence: super::longtermpersistence::LongtermpersistenceIndex,
    #[cfg(feature = "lootgeneration")]
    pub lootgeneration: super::lootgeneration::LootgenerationIndex,
    #[cfg(feature = "megamap")]
    pub megamap: super::megamap::MegamapIndex,
    #[cfg(feature = "missiondata")]
    pub missiondata: super::missiondata::MissiondataIndex,
    #[cfg(feature = "missionfailureconditions")]
    pub missionfailureconditions: super::missionfailureconditions::MissionfailureconditionsIndex,
    #[cfg(feature = "motionstatemachine")]
    pub motionstatemachine: super::motionstatemachine::MotionstatemachineIndex,
    #[cfg(feature = "musiclogic")]
    pub musiclogic: super::musiclogic::MusiclogicIndex,
    #[cfg(feature = "planetdaynighttemperatureparams")]
    pub planetdaynighttemperatureparams: super::planetdaynighttemperatureparams::PlanetdaynighttemperatureparamsIndex,
    #[cfg(feature = "procbreathing")]
    pub procbreathing: super::procbreathing::ProcbreathingIndex,
    #[cfg(feature = "procedurallayout")]
    pub procedurallayout: super::procedurallayout::ProcedurallayoutIndex,
    #[cfg(feature = "qteconfigs")]
    pub qteconfigs: super::qteconfigs::QteconfigsIndex,
    #[cfg(feature = "radarsystem")]
    pub radarsystem: super::radarsystem::RadarsystemIndex,
    #[cfg(feature = "rastar")]
    pub rastar: super::rastar::RastarIndex,
    #[cfg(feature = "refinerynotificationconfiguration")]
    pub refinerynotificationconfiguration: super::refinerynotificationconfiguration::RefinerynotificationconfigurationIndex,
    #[cfg(feature = "refiningprocess")]
    pub refiningprocess: super::refiningprocess::RefiningprocessIndex,
    #[cfg(feature = "rentalnotificationparams")]
    pub rentalnotificationparams: super::rentalnotificationparams::RentalnotificationparamsIndex,
    #[cfg(feature = "reputation")]
    pub reputation: super::reputation::ReputationIndex,
    #[cfg(feature = "resourcetypedatabase")]
    pub resourcetypedatabase: super::resourcetypedatabase::ResourcetypedatabaseIndex,
    #[cfg(feature = "roomsystem")]
    pub roomsystem: super::roomsystem::RoomsystemIndex,
    #[cfg(feature = "scuttableshapedefinition")]
    pub scuttableshapedefinition: super::scuttableshapedefinition::ScuttableshapedefinitionIndex,
    #[cfg(feature = "servicebeacon")]
    pub servicebeacon: super::servicebeacon::ServicebeaconIndex,
    #[cfg(feature = "sgeometryviewdistanceratiocategories")]
    pub sgeometryviewdistanceratiocategories: super::sgeometryviewdistanceratiocategories::SgeometryviewdistanceratiocategoriesIndex,
    #[cfg(feature = "sglobalchargedrainbeamparams")]
    pub sglobalchargedrainbeamparams: super::sglobalchargedrainbeamparams::SglobalchargedrainbeamparamsIndex,
    #[cfg(feature = "sglobalcrosshairparams")]
    pub sglobalcrosshairparams: super::sglobalcrosshairparams::SglobalcrosshairparamsIndex,
    #[cfg(feature = "sglobalelectronparams")]
    pub sglobalelectronparams: super::sglobalelectronparams::SglobalelectronparamsIndex,
    #[cfg(feature = "sglobalhealingbeamparams")]
    pub sglobalhealingbeamparams: super::sglobalhealingbeamparams::SglobalhealingbeamparamsIndex,
    #[cfg(feature = "sglobalhitbehaviorparams")]
    pub sglobalhitbehaviorparams: super::sglobalhitbehaviorparams::SglobalhitbehaviorparamsIndex,
    #[cfg(feature = "sglobalsalvagerepairbeamparams")]
    pub sglobalsalvagerepairbeamparams: super::sglobalsalvagerepairbeamparams::SglobalsalvagerepairbeamparamsIndex,
    #[cfg(feature = "sglobaltractorbeamparams")]
    pub sglobaltractorbeamparams: super::sglobaltractorbeamparams::SglobaltractorbeamparamsIndex,
    #[cfg(feature = "shipinsurancerecord")]
    pub shipinsurancerecord: super::shipinsurancerecord::ShipinsurancerecordIndex,
    #[cfg(feature = "specialeventdatabase")]
    pub specialeventdatabase: super::specialeventdatabase::SpecialeventdatabaseIndex,
    #[cfg(feature = "sreputationglobalcontextbbparams")]
    pub sreputationglobalcontextbbparams: super::sreputationglobalcontextbbparams::SreputationglobalcontextbbparamsIndex,
    #[cfg(feature = "starmap")]
    pub starmap: super::starmap::StarmapIndex,
    #[cfg(feature = "tacticalquery")]
    pub tacticalquery: super::tacticalquery::TacticalqueryIndex,
    #[cfg(feature = "tagdatabase")]
    pub tagdatabase: super::tagdatabase::TagdatabaseIndex,
    #[cfg(feature = "trackview")]
    pub trackview: super::trackview::TrackviewIndex,
    #[cfg(feature = "turret")]
    pub turret: super::turret::TurretIndex,
    #[cfg(feature = "ui-animatedmarkers")]
    pub ui_animatedmarkers: super::ui_animatedmarkers::UiAnimatedmarkersIndex,
    #[cfg(feature = "ui-armarkerconfiguration")]
    pub ui_armarkerconfiguration: super::ui_armarkerconfiguration::UiArmarkerconfigurationIndex,
    #[cfg(feature = "ui-buildingblocks")]
    pub ui_buildingblocks: super::ui_buildingblocks::UiBuildingblocksIndex,
    #[cfg(feature = "ui-digitalsignage")]
    pub ui_digitalsignage: super::ui_digitalsignage::UiDigitalsignageIndex,
    #[cfg(feature = "ui-directrtt")]
    pub ui_directrtt: super::ui_directrtt::UiDirectrttIndex,
    #[cfg(feature = "ui-dockingslotvisibility")]
    pub ui_dockingslotvisibility: super::ui_dockingslotvisibility::UiDockingslotvisibilityIndex,
    #[cfg(feature = "ui-flashobjectbindinggroups")]
    pub ui_flashobjectbindinggroups: super::ui_flashobjectbindinggroups::UiFlashobjectbindinggroupsIndex,
    #[cfg(feature = "ui-frontend")]
    pub ui_frontend: super::ui_frontend::UiFrontendIndex,
    #[cfg(feature = "ui-holovehicleconfig")]
    pub ui_holovehicleconfig: super::ui_holovehicleconfig::UiHolovehicleconfigIndex,
    #[cfg(feature = "ui-hudcolors_shipcolorpalettes")]
    pub ui_hudcolors_shipcolorpalettes: super::ui_hudcolors_shipcolorpalettes::UiHudcolors_shipcolorpalettesIndex,
    #[cfg(feature = "ui-innerthought")]
    pub ui_innerthought: super::ui_innerthought::UiInnerthoughtIndex,
    #[cfg(feature = "ui-itemtypedefinition")]
    pub ui_itemtypedefinition: super::ui_itemtypedefinition::UiItemtypedefinitionIndex,
    #[cfg(feature = "ui-markertrackingvolumeconfig")]
    pub ui_markertrackingvolumeconfig: super::ui_markertrackingvolumeconfig::UiMarkertrackingvolumeconfigIndex,
    #[cfg(feature = "ui-missilelockreticleconfig")]
    pub ui_missilelockreticleconfig: super::ui_missilelockreticleconfig::UiMissilelockreticleconfigIndex,
    #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
    pub ui_objectdatabankentrymarkerconfig: super::ui_objectdatabankentrymarkerconfig::UiObjectdatabankentrymarkerconfigIndex,
    #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
    pub ui_playerchoice_library_playerchoicelibrary: super::ui_playerchoice_library_playerchoicelibrary::UiPlayerchoice_library_playerchoicelibraryIndex,
    #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
    pub ui_playerecggraph_config_playerecggraphconfig: super::ui_playerecggraph_config_playerecggraphconfig::UiPlayerecggraph_config_playerecggraphconfigIndex,
    #[cfg(feature = "ui-popups")]
    pub ui_popups: super::ui_popups::UiPopupsIndex,
    #[cfg(feature = "ui-radardisplay_config_radar")]
    pub ui_radardisplay_config_radar: super::ui_radardisplay_config_radar::UiRadardisplay_config_radarIndex,
    #[cfg(feature = "ui-seatreticlearchetype")]
    pub ui_seatreticlearchetype: super::ui_seatreticlearchetype::UiSeatreticlearchetypeIndex,
    #[cfg(feature = "ui-transformationinterpolatorrecords")]
    pub ui_transformationinterpolatorrecords: super::ui_transformationinterpolatorrecords::UiTransformationinterpolatorrecordsIndex,
    #[cfg(feature = "ui-uiconfig_starcitizen")]
    pub ui_uiconfig_starcitizen: super::ui_uiconfig_starcitizen::UiUiconfig_starcitizenIndex,
    #[cfg(feature = "ui-uimodes")]
    pub ui_uimodes: super::ui_uimodes::UiUimodesIndex,
    #[cfg(feature = "ui-uistatedisplay")]
    pub ui_uistatedisplay: super::ui_uistatedisplay::UiUistatedisplayIndex,
    #[cfg(feature = "ui-videocomms")]
    pub ui_videocomms: super::ui_videocomms::UiVideocommsIndex,
    #[cfg(feature = "unittest_unittestb")]
    pub unittest_unittestb: super::unittest_unittestb::Unittest_unittestbIndex,
    #[cfg(feature = "vehicle")]
    pub vehicle: super::vehicle::VehicleIndex,
    #[cfg(feature = "vfx")]
    pub vfx: super::vfx::VfxIndex,
    #[cfg(feature = "zerogtraversalgraph")]
    pub zerogtraversalgraph: super::zerogtraversalgraph::ZerogtraversalgraphIndex,
}

impl RecordIndex {
    #[allow(unused_mut)]
    pub fn len(&self) -> usize {
        let mut n = 0;
        n += self.multi_feature.len();
        #[cfg(feature = "actor-externalforceresponse")]
        { n += self.actor_externalforceresponse.len(); }
        #[cfg(feature = "actor-gforce")]
        { n += self.actor_gforce.len(); }
        #[cfg(feature = "actor-inputdeflectiontime")]
        { n += self.actor_inputdeflectiontime.len(); }
        #[cfg(feature = "actor-playerdefaultactionsconfig")]
        { n += self.actor_playerdefaultactionsconfig.len(); }
        #[cfg(feature = "actor-quantumtravelcameraeffects")]
        { n += self.actor_quantumtravelcameraeffects.len(); }
        #[cfg(feature = "aiglobalsettings")]
        { n += self.aiglobalsettings.len(); }
        #[cfg(feature = "aiprofile")]
        { n += self.aiprofile.len(); }
        #[cfg(feature = "aiwavecollection")]
        { n += self.aiwavecollection.len(); }
        #[cfg(feature = "analytics")]
        { n += self.analytics.len(); }
        #[cfg(feature = "areaservices")]
        { n += self.areaservices.len(); }
        #[cfg(feature = "audio")]
        { n += self.audio.len(); }
        #[cfg(feature = "awardservice")]
        { n += self.awardservice.len(); }
        #[cfg(feature = "cameras")]
        { n += self.cameras.len(); }
        #[cfg(feature = "character")]
        { n += self.character.len(); }
        #[cfg(feature = "chatcommandfastaccess")]
        { n += self.chatcommandfastaccess.len(); }
        #[cfg(feature = "chatemoterecord")]
        { n += self.chatemoterecord.len(); }
        #[cfg(feature = "chatfilteroptions")]
        { n += self.chatfilteroptions.len(); }
        #[cfg(feature = "chatmanager")]
        { n += self.chatmanager.len(); }
        #[cfg(feature = "commodityconfiguration")]
        { n += self.commodityconfiguration.len(); }
        #[cfg(feature = "commoditytypedatabase")]
        { n += self.commoditytypedatabase.len(); }
        #[cfg(feature = "communicationatlconfig")]
        { n += self.communicationatlconfig.len(); }
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
        #[cfg(feature = "densityclasses")]
        { n += self.densityclasses.len(); }
        #[cfg(feature = "dialoguecontentbank")]
        { n += self.dialoguecontentbank.len(); }
        #[cfg(feature = "dynamiccameraeffects")]
        { n += self.dynamiccameraeffects.len(); }
        #[cfg(feature = "emotions")]
        { n += self.emotions.len(); }
        #[cfg(feature = "entities-scitem-mastermodeexclusionglobalparams")]
        { n += self.entities_scitem_mastermodeexclusionglobalparams.len(); }
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
        #[cfg(feature = "fidgetconfig")]
        { n += self.fidgetconfig.len(); }
        #[cfg(feature = "forcefeedback_forcefeedbackeffects")]
        { n += self.forcefeedback_forcefeedbackeffects.len(); }
        #[cfg(feature = "friendmanager")]
        { n += self.friendmanager.len(); }
        #[cfg(feature = "gamemode")]
        { n += self.gamemode.len(); }
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
        #[cfg(feature = "globalshopparams")]
        { n += self.globalshopparams.len(); }
        #[cfg(feature = "globaltutorialparams")]
        { n += self.globaltutorialparams.len(); }
        #[cfg(feature = "handholdgripdatabase")]
        { n += self.handholdgripdatabase.len(); }
        #[cfg(feature = "hardwaremouse")]
        { n += self.hardwaremouse.len(); }
        #[cfg(feature = "harvestable")]
        { n += self.harvestable.len(); }
        #[cfg(feature = "hudparams")]
        { n += self.hudparams.len(); }
        #[cfg(feature = "ifcs")]
        { n += self.ifcs.len(); }
        #[cfg(feature = "instancedinterior")]
        { n += self.instancedinterior.len(); }
        #[cfg(feature = "interactionconditions")]
        { n += self.interactionconditions.len(); }
        #[cfg(feature = "inventorycontainers")]
        { n += self.inventorycontainers.len(); }
        #[cfg(feature = "item")]
        { n += self.item.len(); }
        #[cfg(feature = "journalentry")]
        { n += self.journalentry.len(); }
        #[cfg(feature = "jumppoints")]
        { n += self.jumppoints.len(); }
        #[cfg(feature = "lawsystem")]
        { n += self.lawsystem.len(); }
        #[cfg(feature = "leangraph")]
        { n += self.leangraph.len(); }
        #[cfg(feature = "longtermpersistence")]
        { n += self.longtermpersistence.len(); }
        #[cfg(feature = "lootgeneration")]
        { n += self.lootgeneration.len(); }
        #[cfg(feature = "megamap")]
        { n += self.megamap.len(); }
        #[cfg(feature = "missiondata")]
        { n += self.missiondata.len(); }
        #[cfg(feature = "missionfailureconditions")]
        { n += self.missionfailureconditions.len(); }
        #[cfg(feature = "motionstatemachine")]
        { n += self.motionstatemachine.len(); }
        #[cfg(feature = "musiclogic")]
        { n += self.musiclogic.len(); }
        #[cfg(feature = "planetdaynighttemperatureparams")]
        { n += self.planetdaynighttemperatureparams.len(); }
        #[cfg(feature = "procbreathing")]
        { n += self.procbreathing.len(); }
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
        #[cfg(feature = "rentalnotificationparams")]
        { n += self.rentalnotificationparams.len(); }
        #[cfg(feature = "reputation")]
        { n += self.reputation.len(); }
        #[cfg(feature = "resourcetypedatabase")]
        { n += self.resourcetypedatabase.len(); }
        #[cfg(feature = "roomsystem")]
        { n += self.roomsystem.len(); }
        #[cfg(feature = "scuttableshapedefinition")]
        { n += self.scuttableshapedefinition.len(); }
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
        #[cfg(feature = "sreputationglobalcontextbbparams")]
        { n += self.sreputationglobalcontextbbparams.len(); }
        #[cfg(feature = "starmap")]
        { n += self.starmap.len(); }
        #[cfg(feature = "tacticalquery")]
        { n += self.tacticalquery.len(); }
        #[cfg(feature = "tagdatabase")]
        { n += self.tagdatabase.len(); }
        #[cfg(feature = "trackview")]
        { n += self.trackview.len(); }
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
        #[cfg(feature = "ui-flashobjectbindinggroups")]
        { n += self.ui_flashobjectbindinggroups.len(); }
        #[cfg(feature = "ui-frontend")]
        { n += self.ui_frontend.len(); }
        #[cfg(feature = "ui-holovehicleconfig")]
        { n += self.ui_holovehicleconfig.len(); }
        #[cfg(feature = "ui-hudcolors_shipcolorpalettes")]
        { n += self.ui_hudcolors_shipcolorpalettes.len(); }
        #[cfg(feature = "ui-innerthought")]
        { n += self.ui_innerthought.len(); }
        #[cfg(feature = "ui-itemtypedefinition")]
        { n += self.ui_itemtypedefinition.len(); }
        #[cfg(feature = "ui-markertrackingvolumeconfig")]
        { n += self.ui_markertrackingvolumeconfig.len(); }
        #[cfg(feature = "ui-missilelockreticleconfig")]
        { n += self.ui_missilelockreticleconfig.len(); }
        #[cfg(feature = "ui-objectdatabankentrymarkerconfig")]
        { n += self.ui_objectdatabankentrymarkerconfig.len(); }
        #[cfg(feature = "ui-playerchoice_library_playerchoicelibrary")]
        { n += self.ui_playerchoice_library_playerchoicelibrary.len(); }
        #[cfg(feature = "ui-playerecggraph_config_playerecggraphconfig")]
        { n += self.ui_playerecggraph_config_playerecggraphconfig.len(); }
        #[cfg(feature = "ui-popups")]
        { n += self.ui_popups.len(); }
        #[cfg(feature = "ui-radardisplay_config_radar")]
        { n += self.ui_radardisplay_config_radar.len(); }
        #[cfg(feature = "ui-seatreticlearchetype")]
        { n += self.ui_seatreticlearchetype.len(); }
        #[cfg(feature = "ui-transformationinterpolatorrecords")]
        { n += self.ui_transformationinterpolatorrecords.len(); }
        #[cfg(feature = "ui-uiconfig_starcitizen")]
        { n += self.ui_uiconfig_starcitizen.len(); }
        #[cfg(feature = "ui-uimodes")]
        { n += self.ui_uimodes.len(); }
        #[cfg(feature = "ui-uistatedisplay")]
        { n += self.ui_uistatedisplay.len(); }
        #[cfg(feature = "ui-videocomms")]
        { n += self.ui_videocomms.len(); }
        #[cfg(feature = "unittest_unittestb")]
        { n += self.unittest_unittestb.len(); }
        #[cfg(feature = "vehicle")]
        { n += self.vehicle.len(); }
        #[cfg(feature = "vfx")]
        { n += self.vfx.len(); }
        #[cfg(feature = "zerogtraversalgraph")]
        { n += self.zerogtraversalgraph.len(); }
        n
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
