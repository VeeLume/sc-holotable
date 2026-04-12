// GENERATED FILE — DO NOT EDIT
//
// Produced by `tools/sc-generator`.
// Regenerate with:
//
//     cargo run -p sc-generator -- --p4k <path-to-Data.p4k>
//
// Any hand edits will be lost on the next run.

#![allow(non_camel_case_types, dead_code)]

use serde::{Deserialize, Serialize};

/// DCB enum: `ARDataType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ARDataType {
    /// DCB value: `Player`
    Player,
    /// DCB value: `ShopItem`
    ShopItem,
    /// DCB value: `PointOfInterest`
    PointOfInterest,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ARLabelMovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ARLabelMovementType {
    /// DCB value: `Fixed`
    Fixed,
    /// DCB value: `FacingPlayer`
    FacingPlayer,
    /// DCB value: `Rotating`
    Rotating,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AbilityAttemptResult`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityAttemptResult {
    /// DCB value: `Activated`
    Activated,
    /// DCB value: `Deactivated`
    Deactivated,
    /// DCB value: `Failed`
    Failed,
    /// DCB value: `Fatigued`
    Fatigued,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AbilityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityType {
    /// DCB value: `BeingRestrained`
    BeingRestrained,
    /// DCB value: `BeingTakenDown`
    BeingTakenDown,
    /// DCB value: `BeingUnrestrained`
    BeingUnrestrained,
    /// DCB value: `ChangeMode`
    ChangeMode,
    /// DCB value: `Climb`
    Climb,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Duck`
    Duck,
    /// DCB value: `Fire`
    Fire,
    /// DCB value: `UseConsumable`
    UseConsumable,
    /// DCB value: `EatAndDrink`
    EatAndDrink,
    /// DCB value: `EatAndDrink2H`
    EatAndDrink2H,
    /// DCB value: `HoldBreath`
    HoldBreath,
    /// DCB value: `Jump`
    Jump,
    /// DCB value: `Land`
    Land,
    /// DCB value: `LadderSprint`
    LadderSprint,
    /// DCB value: `LadderDodge`
    LadderDodge,
    /// DCB value: `Lean`
    Lean,
    /// DCB value: `Melee`
    Melee,
    /// DCB value: `MedicalMelee`
    MedicalMelee,
    /// DCB value: `Mirror`
    Mirror,
    /// DCB value: `MobiGlas`
    MobiGlas,
    /// DCB value: `Movement`
    Movement,
    /// DCB value: `Rotation`
    Rotation,
    /// DCB value: `PersonalInnerThought`
    PersonalInnerThought,
    /// DCB value: `Place`
    Place,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `ProneMove`
    ProneMove,
    /// DCB value: `ProneRoll`
    ProneRoll,
    /// DCB value: `Reload`
    Reload,
    /// DCB value: `AmmoRepool`
    AmmoRepool,
    /// DCB value: `Walk`
    Walk,
    /// DCB value: `Run`
    Run,
    /// DCB value: `Sprint`
    Sprint,
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Stow`
    Stow,
    /// DCB value: `Restrain`
    Restrain,
    /// DCB value: `PreTake`
    PreTake,
    /// DCB value: `Take`
    Take,
    /// DCB value: `TakeDown`
    TakeDown,
    /// DCB value: `ThrowOverhand`
    ThrowOverhand,
    /// DCB value: `ThrowUnderhand`
    ThrowUnderhand,
    /// DCB value: `UnarmedCombat`
    UnarmedCombat,
    /// DCB value: `Unstow`
    Unstow,
    /// DCB value: `Unrestrain`
    Unrestrain,
    /// DCB value: `Zoom`
    Zoom,
    /// DCB value: `HandSignal`
    HandSignal,
    /// DCB value: `Carry`
    Carry,
    /// DCB value: `Carry2H`
    Carry2H,
    /// DCB value: `Equip`
    Equip,
    /// DCB value: `SocialEmote`
    SocialEmote,
    /// DCB value: `Fall`
    Fall,
    /// DCB value: `Inspect`
    Inspect,
    /// DCB value: `InteractionMode`
    InteractionMode,
    /// DCB value: `BlockingInteractionMode`
    BlockingInteractionMode,
    /// DCB value: `TryOn`
    TryOn,
    /// DCB value: `SeatTransition`
    SeatTransition,
    /// DCB value: `DiscardItem`
    DiscardItem,
    /// DCB value: `TurnStep`
    TurnStep,
    /// DCB value: `Lookback`
    Lookback,
    /// DCB value: `ItemRaise`
    ItemRaise,
    /// DCB value: `HeadTrack`
    HeadTrack,
    /// DCB value: `FreeLook`
    FreeLook,
    /// DCB value: `MeleeBlock`
    MeleeBlock,
    /// DCB value: `LootingInteraction`
    LootingInteraction,
    /// DCB value: `UsableBeingDraggedOut`
    UsableBeingDraggedOut,
    /// DCB value: `UsableBeingDroppedIn`
    UsableBeingDroppedIn,
    /// DCB value: `UsableDragUserOut`
    UsableDragUserOut,
    /// DCB value: `UsableDropUserIn`
    UsableDropUserIn,
    /// DCB value: `UsableInteraction`
    UsableInteraction,
    /// DCB value: `MeleeHeavy`
    MeleeHeavy,
    /// DCB value: `StickyFilter`
    StickyFilter,
    /// DCB value: `Dodge`
    Dodge,
    /// DCB value: `ProtectFace`
    ProtectFace,
    /// DCB value: `SuitEquip`
    SuitEquip,
    /// DCB value: `EVA`
    EVA,
    /// DCB value: `BodyCarrying`
    BodyCarrying,
    /// DCB value: `BodyDragging`
    BodyDragging,
    /// DCB value: `TrackviewControlled`
    TrackviewControlled,
    /// DCB value: `Fidgets`
    Fidgets,
    /// DCB value: `DamageReactions`
    DamageReactions,
    /// DCB value: `EffortMoveSet`
    EffortMoveSet,
    /// DCB value: `WalkToPlace`
    WalkToPlace,
    /// DCB value: `Looting`
    Looting,
    /// DCB value: `OpenCloseContainer`
    OpenCloseContainer,
    /// DCB value: `UseConsumable2H`
    UseConsumable2H,
    /// DCB value: `VisorWipe`
    VisorWipe,
    /// DCB value: `CanUseMovable`
    CanUseMovable,
    /// DCB value: `ThrowReady`
    ThrowReady,
    /// DCB value: `PlaceReady`
    PlaceReady,
    /// DCB value: `PrimeItem`
    PrimeItem,
    /// DCB value: `Hack`
    Hack,
    /// DCB value: `UsingMovable`
    UsingMovable,
    /// DCB value: `ZeroGTraversal`
    ZeroGTraversal,
    /// DCB value: `ZeroGTraversalMovement`
    ZeroGTraversalMovement,
    /// DCB value: `ZeroGTraversalHandAttach`
    ZeroGTraversalHandAttach,
    /// DCB value: `WeaponMount`
    WeaponMount,
    /// DCB value: `Scanning`
    Scanning,
    /// DCB value: `TurnOverApply`
    TurnOverApply,
    /// DCB value: `SyncedRevival`
    SyncedRevival,
    /// DCB value: `Vault`
    Vault,
    /// DCB value: `VaultHigh`
    VaultHigh,
    /// DCB value: `Mantle`
    Mantle,
    /// DCB value: `MantleHigh`
    MantleHigh,
    /// DCB value: `DrunkStumble`
    DrunkStumble,
    /// DCB value: `SelfTarget`
    SelfTarget,
    /// DCB value: `ForceReactions`
    ForceReactions,
    /// DCB value: `SoftLockWeapons`
    SoftLockWeapons,
    /// DCB value: `AnyPlayerAnimatedInteraction`
    AnyPlayerAnimatedInteraction,
    /// DCB value: `LeftHandPlayerAnimatedInteraction`
    LeftHandPlayerAnimatedInteraction,
    /// DCB value: `RightHandPlayerAnimatedInteraction`
    RightHandPlayerAnimatedInteraction,
    /// DCB value: `FixOverheat`
    FixOverheat,
    /// DCB value: `OpenInventory`
    OpenInventory,
    /// DCB value: `Misfire`
    Misfire,
    /// DCB value: `SyncedMeleeAttack`
    SyncedMeleeAttack,
    /// DCB value: `SyncedMeleeDefend`
    SyncedMeleeDefend,
    /// DCB value: `Swim`
    Swim,
    /// DCB value: `WeaponLowering`
    WeaponLowering,
    /// DCB value: `Slide`
    Slide,
    /// DCB value: `SlideDrop`
    SlideDrop,
    /// DCB value: `Malfunctioning`
    Malfunctioning,
    /// DCB value: `StaticFiringModeDeploy`
    StaticFiringModeDeploy,
    /// DCB value: `StaticFiringModeIdle`
    StaticFiringModeIdle,
    /// DCB value: `StaticFiringModeRetract`
    StaticFiringModeRetract,
    /// DCB value: `AttachedWeaponDeploy`
    AttachedWeaponDeploy,
    /// DCB value: `AttachedWeaponIdle`
    AttachedWeaponIdle,
    /// DCB value: `AttachedWeaponRetract`
    AttachedWeaponRetract,
    /// DCB value: `EquipWearable`
    EquipWearable,
    /// DCB value: `VehicleWeaponFire`
    VehicleWeaponFire,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AccountBadge`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountBadge {
    /// DCB value: `RSI_BADGE_QA`
    RSI_BADGE_QA,
    /// DCB value: `RSI_BADGE_STAFF`
    RSI_BADGE_STAFF,
    /// DCB value: `RSI_BADGE_GM`
    RSI_BADGE_GM,
    /// DCB value: `RSI_BADGE_SUBSCRIBER`
    RSI_BADGE_SUBSCRIBER,
    /// DCB value: `RSI_BADGE_CONCIERGE`
    RSI_BADGE_CONCIERGE,
    /// DCB value: `RSI_BADGE_MMHC`
    RSI_BADGE_MMHC,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActivationMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationMethod {
    /// DCB value: `ActivateOnAttach`
    ActivateOnAttach,
    /// DCB value: `ActivateOnDemand`
    ActivateOnDemand,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActivationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActivationMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Press`
    Press,
    /// DCB value: `Tap`
    Tap,
    /// DCB value: `DoubleTap`
    DoubleTap,
    /// DCB value: `Hold`
    Hold,
    /// DCB value: `Release`
    Release,
    /// DCB value: `Movement`
    Movement,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActiveRange`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActiveRange {
    /// DCB value: `InteriorRange`
    InteriorRange,
    /// DCB value: `ExteriorRange`
    ExteriorRange,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorBodyDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorBodyDirection {
    /// DCB value: `None`
    None,
    /// DCB value: `Chest`
    Chest,
    /// DCB value: `Back`
    Back,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Left`
    Left,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorBone`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorBone {
    /// DCB value: `Hips`
    Hips,
    /// DCB value: `Spine`
    Spine,
    /// DCB value: `Spine2`
    Spine2,
    /// DCB value: `Spine3`
    Spine3,
    /// DCB value: `Neck`
    Neck,
    /// DCB value: `Head`
    Head,
    /// DCB value: `EyeRight`
    EyeRight,
    /// DCB value: `EyeLeft`
    EyeLeft,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `Weapon2`
    Weapon2,
    /// DCB value: `FootRight`
    FootRight,
    /// DCB value: `FootLeft`
    FootLeft,
    /// DCB value: `ArmRight`
    ArmRight,
    /// DCB value: `ArmLeft`
    ArmLeft,
    /// DCB value: `ForearmRight`
    ForearmRight,
    /// DCB value: `ForearmLeft`
    ForearmLeft,
    /// DCB value: `CalfRight`
    CalfRight,
    /// DCB value: `CalfLeft`
    CalfLeft,
    /// DCB value: `Camera`
    Camera,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorFilter`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorFilter {
    /// DCB value: `OnlyPlayer`
    OnlyPlayer,
    /// DCB value: `OnlyAI`
    OnlyAI,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStatCooldownType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStatCooldownType {
    /// DCB value: `Time`
    Time,
    /// DCB value: `Points`
    Points,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStatType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStatType {
    /// DCB value: `Hunger`
    Hunger,
    /// DCB value: `Thirst`
    Thirst,
    /// DCB value: `BloodDrugLevel`
    BloodDrugLevel,
    /// DCB value: `OverdoseLevel`
    OverdoseLevel,
    /// DCB value: `BodyTemperature`
    BodyTemperature,
    /// DCB value: `SuitTemperature`
    SuitTemperature,
    /// DCB value: `Stun`
    Stun,
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `Pressure`
    Pressure,
    /// DCB value: `GasSaturationO2`
    GasSaturationO2,
    /// DCB value: `DownedDamage`
    DownedDamage,
    /// DCB value: `HealthPool`
    HealthPool,
    /// DCB value: `HealthHead`
    HealthHead,
    /// DCB value: `HealthTorso`
    HealthTorso,
    /// DCB value: `HealthLeftArm`
    HealthLeftArm,
    /// DCB value: `HealthRightArm`
    HealthRightArm,
    /// DCB value: `HealthLeftLeg`
    HealthLeftLeg,
    /// DCB value: `HealthRightLeg`
    HealthRightLeg,
    /// DCB value: `WearHead`
    WearHead,
    /// DCB value: `WearTorso`
    WearTorso,
    /// DCB value: `WearLeftArm`
    WearLeftArm,
    /// DCB value: `WearRightArm`
    WearRightArm,
    /// DCB value: `WearLeftLeg`
    WearLeftLeg,
    /// DCB value: `WearRightLeg`
    WearRightLeg,
    /// DCB value: `BodyRadiation`
    BodyRadiation,
    /// DCB value: `SuitRadiation`
    SuitRadiation,
    /// DCB value: `GasSaturationCO2`
    GasSaturationCO2,
    /// DCB value: `GasSaturationCO`
    GasSaturationCO,
    /// DCB value: `Hygiene`
    Hygiene,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByAimStanceState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByAimStanceState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `Relaxed`
    Relaxed,
    /// DCB value: `Ready`
    Ready,
    /// DCB value: `Lowered`
    Lowered,
    /// DCB value: `ADS`
    ADS,
    /// DCB value: `Inspect`
    Inspect,
    /// DCB value: `MeleeBlock`
    MeleeBlock,
    /// DCB value: `ThrowReady`
    ThrowReady,
    /// DCB value: `SelfTarget`
    SelfTarget,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByCharacterType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByCharacterType {
    /// DCB value: `Any`
    Any,
    /// DCB value: `PlayersOnly`
    PlayersOnly,
    /// DCB value: `NonPlayersOnly`
    NonPlayersOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByHeldItemType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByHeldItemType {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Pistol`
    Pistol,
    /// DCB value: `Stocked`
    Stocked,
    /// DCB value: `Shouldered`
    Shouldered,
    /// DCB value: `MeleeMode`
    MeleeMode,
    /// DCB value: `mobiGlas`
    mobiGlas,
    /// DCB value: `CombatThrowable`
    CombatThrowable,
    /// DCB value: `Food`
    Food,
    /// DCB value: `Drink`
    Drink,
    /// DCB value: `MediPenHeal`
    MediPenHeal,
    /// DCB value: `Other`
    Other,
    /// DCB value: `Movable`
    Movable,
    /// DCB value: `TractorBeam`
    TractorBeam,
    /// DCB value: `MiningTool`
    MiningTool,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByLeanState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByLeanState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByLocomotionSet`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByLocomotionSet {
    /// DCB value: `Any`
    Any,
    /// DCB value: `BodyCarry`
    BodyCarry,
    /// DCB value: `Drunk`
    Drunk,
    /// DCB value: `Standard`
    Standard,
    /// DCB value: `Effort`
    Effort,
    /// DCB value: `Hurt`
    Hurt,
    /// DCB value: `Stumble`
    Stumble,
    /// DCB value: `BodyDragging`
    BodyDragging,
    /// DCB value: `Movable`
    Movable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByMotionSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByMotionSpeed {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Movement`
    Movement,
    /// DCB value: `Walk`
    Walk,
    /// DCB value: `Run`
    Run,
    /// DCB value: `WalkOrRun`
    WalkOrRun,
    /// DCB value: `Sprint`
    Sprint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByPoseState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByPoseState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `NoWeapon`
    NoWeapon,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `Carry`
    Carry,
    /// DCB value: `EnterCarry`
    EnterCarry,
    /// DCB value: `ExitCarry`
    ExitCarry,
    /// DCB value: `UnarmedCombat`
    UnarmedCombat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterBySkeleton`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterBySkeleton {
    /// DCB value: `Any`
    Any,
    /// DCB value: `HumanMale`
    HumanMale,
    /// DCB value: `HumanFemale`
    HumanFemale,
    /// DCB value: `Vanduul`
    Vanduul,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByStanceState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByStanceState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `ProneFront`
    ProneFront,
    /// DCB value: `ProneBack`
    ProneBack,
    /// DCB value: `ProneV2`
    ProneV2,
    /// DCB value: `CoverLow`
    CoverLow,
    /// DCB value: `CoverHigh`
    CoverHigh,
    /// DCB value: `Seated`
    Seated,
    /// DCB value: `StandSquashed`
    StandSquashed,
    /// DCB value: `Swim`
    Swim,
    /// DCB value: `ZeroGSurfaceTraversal`
    ZeroGSurfaceTraversal,
    /// DCB value: `Underground`
    Underground,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateFilterByState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateFilterByState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `BodyCarried`
    BodyCarried,
    /// DCB value: `BodyDragged`
    BodyDragged,
    /// DCB value: `BodyDragging`
    BodyDragging,
    /// DCB value: `EVA`
    EVA,
    /// DCB value: `Jump`
    Jump,
    /// DCB value: `Fall`
    Fall,
    /// DCB value: `Land`
    Land,
    /// DCB value: `KnockDown`
    KnockDown,
    /// DCB value: `Fly`
    Fly,
    /// DCB value: `Ground`
    Ground,
    /// DCB value: `Ladder`
    Ladder,
    /// DCB value: `Interacting`
    Interacting,
    /// DCB value: `Ledge`
    Ledge,
    /// DCB value: `Restrain`
    Restrain,
    /// DCB value: `Unrestrain`
    Unrestrain,
    /// DCB value: `TakeDown`
    TakeDown,
    /// DCB value: `Usable`
    Usable,
    /// DCB value: `WeaponMount`
    WeaponMount,
    /// DCB value: `Scanning`
    Scanning,
    /// DCB value: `Linked`
    Linked,
    /// DCB value: `ZeroGTraversal`
    ZeroGTraversal,
    /// DCB value: `ZeroGLaunch`
    ZeroGLaunch,
    /// DCB value: `ZeroGHandholdTraversal`
    ZeroGHandholdTraversal,
    /// DCB value: `Swim`
    Swim,
    /// DCB value: `Reload`
    Reload,
    /// DCB value: `HolsterUnholster`
    HolsterUnholster,
    /// DCB value: `Dead`
    Dead,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStateSelection_Stance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStateSelection_Stance {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `ProneBack`
    ProneBack,
    /// DCB value: `CoverLow`
    CoverLow,
    /// DCB value: `CoverHigh`
    CoverHigh,
    /// DCB value: `Seated`
    Seated,
    /// DCB value: `StandSquashed`
    StandSquashed,
    /// DCB value: `Swim`
    Swim,
    /// DCB value: `ZeroGSurfaceTraversal`
    ZeroGSurfaceTraversal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStatusType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStatusType {
    /// DCB value: `Concussion`
    Concussion,
    /// DCB value: `Hypothermia`
    Hypothermia,
    /// DCB value: `Hyperthermia`
    Hyperthermia,
    /// DCB value: `Depressurization`
    Depressurization,
    /// DCB value: `PoorAtmosphereQuality`
    PoorAtmosphereQuality,
    /// DCB value: `LowHealth`
    LowHealth,
    /// DCB value: `Hurt`
    Hurt,
    /// DCB value: `Damaged`
    Damaged,
    /// DCB value: `Ruined`
    Ruined,
    /// DCB value: `Starved`
    Starved,
    /// DCB value: `Dehydrated`
    Dehydrated,
    /// DCB value: `Dying`
    Dying,
    /// DCB value: `MildlyIntoxicated`
    MildlyIntoxicated,
    /// DCB value: `Intoxicated`
    Intoxicated,
    /// DCB value: `Downed`
    Downed,
    /// DCB value: `Overdosed`
    Overdosed,
    /// DCB value: `Injury`
    Injury,
    /// DCB value: `MajorInjury`
    MajorInjury,
    /// DCB value: `DeadlyInjury`
    DeadlyInjury,
    /// DCB value: `RadiationSicknessMild`
    RadiationSicknessMild,
    /// DCB value: `RadiationSicknessModerate`
    RadiationSicknessModerate,
    /// DCB value: `RadiationSicknessSevere`
    RadiationSicknessSevere,
    /// DCB value: `DistortionInterference`
    DistortionInterference,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ActorStatusWidget`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorStatusWidget {
    /// DCB value: `HealthPool`
    HealthPool,
    /// DCB value: `Oxygen`
    Oxygen,
    /// DCB value: `BodyTemperature`
    BodyTemperature,
    /// DCB value: `ExternalTemperature`
    ExternalTemperature,
    /// DCB value: `InjuryDoll`
    InjuryDoll,
    /// DCB value: `HeartMonitor`
    HeartMonitor,
    /// DCB value: `Hunger`
    Hunger,
    /// DCB value: `Thirst`
    Thirst,
    /// DCB value: `BloodDrugLevel`
    BloodDrugLevel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AgentStance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentStance {
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `CoverLow`
    CoverLow,
    /// DCB value: `CoverHigh`
    CoverHigh,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AmmoCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmmoCategory {
    /// DCB value: `None`
    None,
    /// DCB value: `_5mm`
    _5mm,
    /// DCB value: `_7mm`
    _7mm,
    /// DCB value: `_10mm`
    _10mm,
    /// DCB value: `_50cal`
    _50cal,
    /// DCB value: `_50cal_pistol`
    _50cal_pistol,
    /// DCB value: `_12g`
    _12g,
    /// DCB value: `Electron`
    Electron,
    /// DCB value: `Coil`
    Coil,
    /// DCB value: `Plasma`
    Plasma,
    /// DCB value: `Laser`
    Laser,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AmmoSpawnType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmmoSpawnType {
    /// DCB value: `AllClients`
    AllClients,
    /// DCB value: `ServerReplicated`
    ServerReplicated,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimFootSyncMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimFootSyncMethod {
    /// DCB value: `Pairs`
    Pairs,
    /// DCB value: `Cyclic`
    Cyclic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimatedMarker_State`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimatedMarker_State {
    /// DCB value: `Locking`
    Locking,
    /// DCB value: `LockLost`
    LockLost,
    /// DCB value: `Locked`
    Locked,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimationGraph_TimeModifier`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationGraph_TimeModifier {
    /// DCB value: `None`
    None,
    /// DCB value: `Total`
    Total,
    /// DCB value: `Stagger`
    Stagger,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimationGraph_TrackType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationGraph_TrackType {
    /// DCB value: `RotateX`
    RotateX,
    /// DCB value: `RotateY`
    RotateY,
    /// DCB value: `RotateZ`
    RotateZ,
    /// DCB value: `Scale`
    Scale,
    /// DCB value: `Number`
    Number,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimationScopeContextTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationScopeContextTypes {
    /// DCB value: `Self`
    Self_,
    /// DCB value: `ObjectContainer`
    ObjectContainer,
    /// DCB value: `ItemPort`
    ItemPort,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AnimationVariableInterpolationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationVariableInterpolationType {
    /// DCB value: `Linear`
    Linear,
    /// DCB value: `Acceleration`
    Acceleration,
    /// DCB value: `Deceleration`
    Deceleration,
    /// DCB value: `Snap`
    Snap,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AsteroidStatePropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AsteroidStatePropertyType {
    /// DCB value: `DebrisDensity`
    DebrisDensity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AtmosphereStatePropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AtmosphereStatePropertyType {
    /// DCB value: `Pressure`
    Pressure,
    /// DCB value: `Temperature`
    Temperature,
    /// DCB value: `Humidity`
    Humidity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AtmosphereType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AtmosphereType {
    /// DCB value: `Static`
    Static,
    /// DCB value: `Dynamic`
    Dynamic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AttachmentZoneType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttachmentZoneType {
    /// DCB value: `Hosted`
    Hosted,
    /// DCB value: `Host`
    Host,
    /// DCB value: `HostParent`
    HostParent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `AttackType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttackType {
    /// DCB value: `LightRight`
    LightRight,
    /// DCB value: `LightLeft`
    LightLeft,
    /// DCB value: `HeavyRight`
    HeavyRight,
    /// DCB value: `HeavyLeft`
    HeavyLeft,
    /// DCB value: `SyringeStab`
    SyringeStab,
    /// DCB value: `AI_Light`
    AI_Light,
    /// DCB value: `AI_Heavy`
    AI_Heavy,
    /// DCB value: `AI_SwipeLeft`
    AI_SwipeLeft,
    /// DCB value: `AI_SwipeRight`
    AI_SwipeRight,
    /// DCB value: `AI_Stab`
    AI_Stab,
    /// DCB value: `AI_Leap`
    AI_Leap,
    /// DCB value: `AI_HammerDown`
    AI_HammerDown,
    /// DCB value: `AI_PushBack`
    AI_PushBack,
    /// DCB value: `AI_Shoot`
    AI_Shoot,
    /// DCB value: `AI_TestAttack`
    AI_TestAttack,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ActivationButtonAction`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ActivationButtonAction {
    /// DCB value: `Up`
    Up,
    /// DCB value: `Down`
    Down,
    /// DCB value: `DoubleClick`
    DoubleClick,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_AnimationDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_AnimationDirection {
    /// DCB value: `Forward`
    Forward,
    /// DCB value: `Reverse`
    Reverse,
    /// DCB value: `Alternate`
    Alternate,
    /// DCB value: `AlternateReverse`
    AlternateReverse,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_AnimationFillMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_AnimationFillMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Forward`
    Forward,
    /// DCB value: `Backward`
    Backward,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_AudioEvent`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_AudioEvent {
    /// DCB value: `Change`
    Change,
    /// DCB value: `ChangeToTrue`
    ChangeToTrue,
    /// DCB value: `ChangeToFalse`
    ChangeToFalse,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_AutoScalingMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_AutoScalingMethod {
    /// DCB value: `None`
    None,
    /// DCB value: `Fill`
    Fill,
    /// DCB value: `Contain`
    Contain,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_Axis`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_Axis {
    /// DCB value: `None`
    None,
    /// DCB value: `X`
    X,
    /// DCB value: `Y`
    Y,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_AxisDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_AxisDirection {
    /// DCB value: `Horizontal`
    Horizontal,
    /// DCB value: `Vertical`
    Vertical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BackgroundType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BackgroundType {
    /// DCB value: `None`
    None,
    /// DCB value: `Texture`
    Texture,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsArithmeticType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsArithmeticType {
    /// DCB value: `Add`
    Add,
    /// DCB value: `Sub`
    Sub,
    /// DCB value: `Mul`
    Mul,
    /// DCB value: `Div`
    Div,
    /// DCB value: `Min`
    Min,
    /// DCB value: `Max`
    Max,
    /// DCB value: `Mod`
    Mod,
    /// DCB value: `Pow`
    Pow,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsBuiltInVariableTypeInteger`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsBuiltInVariableTypeInteger {
    /// DCB value: `ServerTime`
    ServerTime,
    /// DCB value: `ListIndex`
    ListIndex,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsBuiltInVariableTypeNumber`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsBuiltInVariableTypeNumber {
    /// DCB value: `TimerSeconds`
    TimerSeconds,
    /// DCB value: `CameraDistance`
    CameraDistance,
    /// DCB value: `AspectRatio`
    AspectRatio,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsCaseModifierType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsCaseModifierType {
    /// DCB value: `None`
    None,
    /// DCB value: `Upper`
    Upper,
    /// DCB value: `Lower`
    Lower,
    /// DCB value: `Pascal`
    Pascal,
    /// DCB value: `Camel`
    Camel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsCurrencyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsCurrencyType {
    /// DCB value: `CURRENCY_UEC`
    CURRENCY_UEC,
    /// DCB value: `CURRENCY_REC`
    CURRENCY_REC,
    /// DCB value: `CURRENCY_AUEC`
    CURRENCY_AUEC,
    /// DCB value: `CURRENCY_MER`
    CURRENCY_MER,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsFunctionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsFunctionType {
    /// DCB value: `Abs`
    Abs,
    /// DCB value: `Acos`
    Acos,
    /// DCB value: `Asin`
    Asin,
    /// DCB value: `Atan`
    Atan,
    /// DCB value: `Ceil`
    Ceil,
    /// DCB value: `Cos`
    Cos,
    /// DCB value: `Cubed`
    Cubed,
    /// DCB value: `Exp`
    Exp,
    /// DCB value: `Floor`
    Floor,
    /// DCB value: `Log`
    Log,
    /// DCB value: `ModInt`
    ModInt,
    /// DCB value: `ModFrac`
    ModFrac,
    /// DCB value: `Round`
    Round,
    /// DCB value: `Sin`
    Sin,
    /// DCB value: `Sqrt`
    Sqrt,
    /// DCB value: `Squared`
    Squared,
    /// DCB value: `Tan`
    Tan,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsIntegerConstants`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsIntegerConstants {
    /// DCB value: `InvalidIndex`
    InvalidIndex,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BindingsIntegerFunctionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BindingsIntegerFunctionType {
    /// DCB value: `Abs`
    Abs,
    /// DCB value: `Squared`
    Squared,
    /// DCB value: `Cubed`
    Cubed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BooleanField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BooleanField {
    /// DCB value: `AffectsAutosize`
    AffectsAutosize,
    /// DCB value: `AffectsLayout`
    AffectsLayout,
    /// DCB value: `AutoFontSize`
    AutoFontSize,
    /// DCB value: `AutoScroll`
    AutoScroll,
    /// DCB value: `CenterDragProxyOnCursor`
    CenterDragProxyOnCursor,
    /// DCB value: `Chamfer`
    Chamfer,
    /// DCB value: `CustomPivot`
    CustomPivot,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `DoStroke`
    DoStroke,
    /// DCB value: `DoFill`
    DoFill,
    /// DCB value: `Editable`
    Editable,
    /// DCB value: `EditBoxSingleLine`
    EditBoxSingleLine,
    /// DCB value: `EditBoxAllowSpaces`
    EditBoxAllowSpaces,
    /// DCB value: `EditBoxBindingsAuthoritative`
    EditBoxBindingsAuthoritative,
    /// DCB value: `EditBoxConfirmDeselect`
    EditBoxConfirmDeselect,
    /// DCB value: `EditBoxHiddenText`
    EditBoxHiddenText,
    /// DCB value: `EnableColorOverlay`
    EnableColorOverlay,
    /// DCB value: `EnableNineSliceRect`
    EnableNineSliceRect,
    /// DCB value: `EnableDrawInGeneralPass`
    EnableDrawInGeneralPass,
    /// DCB value: `EnableBackground`
    EnableBackground,
    /// DCB value: `EnableTopLeftBorderChamfer`
    EnableTopLeftBorderChamfer,
    /// DCB value: `EnableTopRightBorderChamfer`
    EnableTopRightBorderChamfer,
    /// DCB value: `EnableBottomRightBorderChamfer`
    EnableBottomRightBorderChamfer,
    /// DCB value: `EnableBottomLeftBorderChamfer`
    EnableBottomLeftBorderChamfer,
    /// DCB value: `EnableMaxWidth`
    EnableMaxWidth,
    /// DCB value: `EnableMaxHeight`
    EnableMaxHeight,
    /// DCB value: `EnableMinWidth`
    EnableMinWidth,
    /// DCB value: `EnableMinHeight`
    EnableMinHeight,
    /// DCB value: `EnableSegmentedBarFill`
    EnableSegmentedBarFill,
    /// DCB value: `EnableSegmentedFill`
    EnableSegmentedFill,
    /// DCB value: `EnableSilhouette`
    EnableSilhouette,
    /// DCB value: `EntityAnimationNormalizeTime`
    EntityAnimationNormalizeTime,
    /// DCB value: `EntityAnimationWrapTime`
    EntityAnimationWrapTime,
    /// DCB value: `EntityHolographic`
    EntityHolographic,
    /// DCB value: `FadeXAxis`
    FadeXAxis,
    /// DCB value: `FadeYAxis`
    FadeYAxis,
    /// DCB value: `FadeZAxis`
    FadeZAxis,
    /// DCB value: `GhostPrimRearOffset`
    GhostPrimRearOffset,
    /// DCB value: `ImageFlipHorizontal`
    ImageFlipHorizontal,
    /// DCB value: `ImageFlipVertical`
    ImageFlipVertical,
    /// DCB value: `IncludeInGroundingEffects`
    IncludeInGroundingEffects,
    /// DCB value: `InheritDisabledState`
    InheritDisabledState,
    /// DCB value: `InheritDownState`
    InheritDownState,
    /// DCB value: `InheritHoverState`
    InheritHoverState,
    /// DCB value: `InheritRadialRotation`
    InheritRadialRotation,
    /// DCB value: `InheritRadialShapeWarp`
    InheritRadialShapeWarp,
    /// DCB value: `InheritSelectedState`
    InheritSelectedState,
    /// DCB value: `Instantiated`
    Instantiated,
    /// DCB value: `Interactable`
    Interactable,
    /// DCB value: `IsActive`
    IsActive,
    /// DCB value: `Flip`
    Flip,
    /// DCB value: `FillStroke`
    FillStroke,
    /// DCB value: `MaintainGapLength`
    MaintainGapLength,
    /// DCB value: `MouseTestRadialShape`
    MouseTestRadialShape,
    /// DCB value: `MoviePlaying`
    MoviePlaying,
    /// DCB value: `ParallelGaps`
    ParallelGaps,
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `PivotProgressAngleWidget`
    PivotProgressAngleWidget,
    /// DCB value: `PivotStartAngleWidget`
    PivotStartAngleWidget,
    /// DCB value: `PrimitiveFlipHorizontal`
    PrimitiveFlipHorizontal,
    /// DCB value: `PrimitiveFlipVertical`
    PrimitiveFlipVertical,
    /// DCB value: `PrimitiveHasPerspective`
    PrimitiveHasPerspective,
    /// DCB value: `PrimitiveIsFacingCamera`
    PrimitiveIsFacingCamera,
    /// DCB value: `PrimitiveIsGrouped`
    PrimitiveIsGrouped,
    /// DCB value: `PropagateInteractionStates`
    PropagateInteractionStates,
    /// DCB value: `RenderAsHTML`
    RenderAsHTML,
    /// DCB value: `RenderShape`
    RenderShape,
    /// DCB value: `RotateProgressAngleWidget`
    RotateProgressAngleWidget,
    /// DCB value: `RotateStartAngleWidget`
    RotateStartAngleWidget,
    /// DCB value: `Selected`
    Selected,
    /// DCB value: `SvgFlipHorizontal`
    SvgFlipHorizontal,
    /// DCB value: `SvgFlipVertical`
    SvgFlipVertical,
    /// DCB value: `TextBold`
    TextBold,
    /// DCB value: `TextItalic`
    TextItalic,
    /// DCB value: `TextKerning`
    TextKerning,
    /// DCB value: `TextUnderline`
    TextUnderline,
    /// DCB value: `WordWrap`
    WordWrap,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_BooleanWriteMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_BooleanWriteMode {
    /// DCB value: `Invert`
    Invert,
    /// DCB value: `Set`
    Set,
    /// DCB value: `Clear`
    Clear,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_CanvasCoordinateMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_CanvasCoordinateMethod {
    /// DCB value: `useRaw`
    useRaw,
    /// DCB value: `aspectOverridesWidth`
    aspectOverridesWidth,
    /// DCB value: `aspectOverridesHeight`
    aspectOverridesHeight,
    /// DCB value: `auto`
    auto,
    /// DCB value: `longestSide`
    longestSide,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_CanvasWidgetSizingMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_CanvasWidgetSizingMethod {
    /// DCB value: `Size`
    Size,
    /// DCB value: `Scale`
    Scale,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ColorField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ColorField {
    /// DCB value: `BackgroundColor`
    BackgroundColor,
    /// DCB value: `BackgroundColorTopLeft`
    BackgroundColorTopLeft,
    /// DCB value: `BackgroundColorTopRight`
    BackgroundColorTopRight,
    /// DCB value: `BackgroundColorBottomLeft`
    BackgroundColorBottomLeft,
    /// DCB value: `BackgroundColorBottomRight`
    BackgroundColorBottomRight,
    /// DCB value: `BorderColorLeft`
    BorderColorLeft,
    /// DCB value: `BorderColorRight`
    BorderColorRight,
    /// DCB value: `BorderColorTop`
    BorderColorTop,
    /// DCB value: `BorderColorBottom`
    BorderColorBottom,
    /// DCB value: `FillColor`
    FillColor,
    /// DCB value: `GlowColor`
    GlowColor,
    /// DCB value: `LightColor`
    LightColor,
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `SegmentColor`
    SegmentColor,
    /// DCB value: `SilhouetteColor`
    SilhouetteColor,
    /// DCB value: `StripStartColor`
    StripStartColor,
    /// DCB value: `StripEndColor`
    StripEndColor,
    /// DCB value: `StrokeColor`
    StrokeColor,
    /// DCB value: `TintColor`
    TintColor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ColorStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ColorStyle {
    /// DCB value: `Base`
    Base,
    /// DCB value: `Positive`
    Positive,
    /// DCB value: `Moderate`
    Moderate,
    /// DCB value: `Critical`
    Critical,
    /// DCB value: `Accent1`
    Accent1,
    /// DCB value: `Accent2`
    Accent2,
    /// DCB value: `Bright`
    Bright,
    /// DCB value: `Selected`
    Selected,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `Background`
    Background,
    /// DCB value: `ContactNeutral`
    ContactNeutral,
    /// DCB value: `ContactParty`
    ContactParty,
    /// DCB value: `ContactPositiveRep`
    ContactPositiveRep,
    /// DCB value: `ContactNegativeRep`
    ContactNegativeRep,
    /// DCB value: `ContactAgressive`
    ContactAgressive,
    /// DCB value: `ContactUnknown`
    ContactUnknown,
    /// DCB value: `MissionObjectives`
    MissionObjectives,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ComparisonOperatorType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ComparisonOperatorType {
    /// DCB value: `Equal`
    Equal,
    /// DCB value: `NotEqual`
    NotEqual,
    /// DCB value: `Greater`
    Greater,
    /// DCB value: `GreaterOrEqual`
    GreaterOrEqual,
    /// DCB value: `Less`
    Less,
    /// DCB value: `LessOrEqual`
    LessOrEqual,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ComponentParameter`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ComponentParameter {
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ContentBoxPosition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ContentBoxPosition {
    /// DCB value: `Top`
    Top,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Bottom`
    Bottom,
    /// DCB value: `Left`
    Left,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_CullingLevel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_CullingLevel {
    /// DCB value: `None`
    None,
    /// DCB value: `Render`
    Render,
    /// DCB value: `Node`
    Node,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_CurvatureAxis`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_CurvatureAxis {
    /// DCB value: `X`
    X,
    /// DCB value: `Y`
    Y,
    /// DCB value: `Z`
    Z,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_DraggablePolicyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_DraggablePolicyType {
    /// DCB value: `None`
    None,
    /// DCB value: `DropTargetItem`
    DropTargetItem,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_DropTargetPolicyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_DropTargetPolicyType {
    /// DCB value: `None`
    None,
    /// DCB value: `DropTarget`
    DropTarget,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_DropdownAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_DropdownAlignment {
    /// DCB value: `Up`
    Up,
    /// DCB value: `Down`
    Down,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_DustParticleMovementRestriction`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_DustParticleMovementRestriction {
    /// DCB value: `None`
    None,
    /// DCB value: `ZoomExclusive`
    ZoomExclusive,
    /// DCB value: `TranslationExclusive`
    TranslationExclusive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FillStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FillStyle {
    /// DCB value: `Filled`
    Filled,
    /// DCB value: `Ghost`
    Ghost,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FlexAxisJustification`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FlexAxisJustification {
    /// DCB value: `Start`
    Start,
    /// DCB value: `End`
    End,
    /// DCB value: `Center`
    Center,
    /// DCB value: `SpaceBetween`
    SpaceBetween,
    /// DCB value: `SpaceAround`
    SpaceAround,
    /// DCB value: `SpaceEvenly`
    SpaceEvenly,
    /// DCB value: `Stretch`
    Stretch,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FlexDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FlexDirection {
    /// DCB value: `Row`
    Row,
    /// DCB value: `RowReverse`
    RowReverse,
    /// DCB value: `Column`
    Column,
    /// DCB value: `ColumnReverse`
    ColumnReverse,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FlexItemAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FlexItemAlignment {
    /// DCB value: `Start`
    Start,
    /// DCB value: `End`
    End,
    /// DCB value: `Center`
    Center,
    /// DCB value: `Stretch`
    Stretch,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FlexWrap`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FlexWrap {
    /// DCB value: `Wrap`
    Wrap,
    /// DCB value: `WrapReverse`
    WrapReverse,
    /// DCB value: `NoWrap`
    NoWrap,
    /// DCB value: `NoWrapInfinite`
    NoWrapInfinite,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_FlipDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_FlipDirection {
    /// DCB value: `None`
    None,
    /// DCB value: `Vertical`
    Vertical,
    /// DCB value: `Horizontal`
    Horizontal,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_GrabBounds`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_GrabBounds {
    /// DCB value: `Sphere`
    Sphere,
    /// DCB value: `Box`
    Box,
    /// DCB value: `Camera`
    Camera,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_GrabRotationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_GrabRotationMode {
    /// DCB value: `Orbital`
    Orbital,
    /// DCB value: `Planar`
    Planar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_GridPackDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_GridPackDirection {
    /// DCB value: `Horizontal`
    Horizontal,
    /// DCB value: `Vertical`
    Vertical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_IconWidgetPreset`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_IconWidgetPreset {
    /// DCB value: `_None`
    _None,
    /// DCB value: `ArrowHollowUp`
    ArrowHollowUp,
    /// DCB value: `ArrowHollowRight`
    ArrowHollowRight,
    /// DCB value: `ArrowHollowDown`
    ArrowHollowDown,
    /// DCB value: `ArrowHollowLeft`
    ArrowHollowLeft,
    /// DCB value: `ArrowHollowCurvedLeft`
    ArrowHollowCurvedLeft,
    /// DCB value: `ArrowHollowCurvedRight`
    ArrowHollowCurvedRight,
    /// DCB value: `ArrowHollowCurvedDoubleLeft`
    ArrowHollowCurvedDoubleLeft,
    /// DCB value: `ArrowHollowCurvedDoubleRight`
    ArrowHollowCurvedDoubleRight,
    /// DCB value: `ArrowHollowCurvedDownLeft`
    ArrowHollowCurvedDownLeft,
    /// DCB value: `ArrowHollowCurvedDownRight`
    ArrowHollowCurvedDownRight,
    /// DCB value: `ArrowUp`
    ArrowUp,
    /// DCB value: `ArrowRight`
    ArrowRight,
    /// DCB value: `ArrowDown`
    ArrowDown,
    /// DCB value: `ArrowLeft`
    ArrowLeft,
    /// DCB value: `ArrowUpLeft`
    ArrowUpLeft,
    /// DCB value: `ArrowUpRight`
    ArrowUpRight,
    /// DCB value: `ArrowDownRight`
    ArrowDownRight,
    /// DCB value: `ArrowDownLeft`
    ArrowDownLeft,
    /// DCB value: `ArrowHookLeft`
    ArrowHookLeft,
    /// DCB value: `ArrowHookRight`
    ArrowHookRight,
    /// DCB value: `ArrowDiamond`
    ArrowDiamond,
    /// DCB value: `ArrowSquare`
    ArrowSquare,
    /// DCB value: `ArrowExpandDownUp`
    ArrowExpandDownUp,
    /// DCB value: `ArrowExpandUpDown`
    ArrowExpandUpDown,
    /// DCB value: `ArrowCurvedLeft`
    ArrowCurvedLeft,
    /// DCB value: `ArrowCurvedRight`
    ArrowCurvedRight,
    /// DCB value: `ArrowCurvedDoubleLeft`
    ArrowCurvedDoubleLeft,
    /// DCB value: `ArrowCurvedDoubleRight`
    ArrowCurvedDoubleRight,
    /// DCB value: `ArrowCurvedDownLeft`
    ArrowCurvedDownLeft,
    /// DCB value: `ArrowCurvedDownRight`
    ArrowCurvedDownRight,
    /// DCB value: `ArrowFullCircleCCW`
    ArrowFullCircleCCW,
    /// DCB value: `ArrowFullCircleCW`
    ArrowFullCircleCW,
    /// DCB value: `ArrowHalfCircleCCW`
    ArrowHalfCircleCCW,
    /// DCB value: `ArrowHalfCircleCW`
    ArrowHalfCircleCW,
    /// DCB value: `ArrowHalfCircleExclamationCCW`
    ArrowHalfCircleExclamationCCW,
    /// DCB value: `ArrowHalfCircleExclamationCW`
    ArrowHalfCircleExclamationCW,
    /// DCB value: `ArrowCaratUp`
    ArrowCaratUp,
    /// DCB value: `ArrowCaratRight`
    ArrowCaratRight,
    /// DCB value: `ArrowCaratDown`
    ArrowCaratDown,
    /// DCB value: `ArrowCaratLeft`
    ArrowCaratLeft,
    /// DCB value: `ArrowCaratDoubleUp`
    ArrowCaratDoubleUp,
    /// DCB value: `ArrowCaratDoubleRight`
    ArrowCaratDoubleRight,
    /// DCB value: `ArrowCaratDoubleDown`
    ArrowCaratDoubleDown,
    /// DCB value: `ArrowCaratDoubleLeft`
    ArrowCaratDoubleLeft,
    /// DCB value: `ArrowEncasedUp`
    ArrowEncasedUp,
    /// DCB value: `ArrowEncasedRight`
    ArrowEncasedRight,
    /// DCB value: `ArrowEncasedDown`
    ArrowEncasedDown,
    /// DCB value: `ArrowEncasedLeft`
    ArrowEncasedLeft,
    /// DCB value: `ArrowEncasedUpLeft`
    ArrowEncasedUpLeft,
    /// DCB value: `ArrowEncasedUpRight`
    ArrowEncasedUpRight,
    /// DCB value: `ArrowEncasedDownRight`
    ArrowEncasedDownRight,
    /// DCB value: `ArrowEncasedDownLeft`
    ArrowEncasedDownLeft,
    /// DCB value: `ArrowEncasedHookLeft`
    ArrowEncasedHookLeft,
    /// DCB value: `ArrowEncasedHookRight`
    ArrowEncasedHookRight,
    /// DCB value: `ArrowEncasedCaratUp`
    ArrowEncasedCaratUp,
    /// DCB value: `ArrowEncasedCaratRight`
    ArrowEncasedCaratRight,
    /// DCB value: `ArrowEncasedCaratDown`
    ArrowEncasedCaratDown,
    /// DCB value: `ArrowEncasedCaratLeft`
    ArrowEncasedCaratLeft,
    /// DCB value: `ArrowEncasedCaratDoubleUp`
    ArrowEncasedCaratDoubleUp,
    /// DCB value: `ArrowEncasedCaratDoubleRight`
    ArrowEncasedCaratDoubleRight,
    /// DCB value: `ArrowEncasedCaratDoubleDown`
    ArrowEncasedCaratDoubleDown,
    /// DCB value: `ArrowEncasedCaratDoubleLeft`
    ArrowEncasedCaratDoubleLeft,
    /// DCB value: `GeneralCheckmark`
    GeneralCheckmark,
    /// DCB value: `GeneralCircleFilled`
    GeneralCircleFilled,
    /// DCB value: `GeneralX`
    GeneralX,
    /// DCB value: `GeneralInfo`
    GeneralInfo,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ImageScalingMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ImageScalingMethod {
    /// DCB value: `Fill`
    Fill,
    /// DCB value: `Contain`
    Contain,
    /// DCB value: `Cover`
    Cover,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_IntegerField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_IntegerField {
    /// DCB value: `AutoScrollBehavior`
    AutoScrollBehavior,
    /// DCB value: `BorderTopLeftRadiusBehavior`
    BorderTopLeftRadiusBehavior,
    /// DCB value: `BorderTopRightRadiusBehavior`
    BorderTopRightRadiusBehavior,
    /// DCB value: `BorderBottomRightRadiusBehavior`
    BorderBottomRightRadiusBehavior,
    /// DCB value: `BorderBottomLeftRadiusBehavior`
    BorderBottomLeftRadiusBehavior,
    /// DCB value: `CanvasWidgetSizingMethod`
    CanvasWidgetSizingMethod,
    /// DCB value: `CaretIndex`
    CaretIndex,
    /// DCB value: `CaseModifier`
    CaseModifier,
    /// DCB value: `CurvatureAxis`
    CurvatureAxis,
    /// DCB value: `DepthBehavior`
    DepthBehavior,
    /// DCB value: `DraggablePolicy`
    DraggablePolicy,
    /// DCB value: `DropTargetPolicy`
    DropTargetPolicy,
    /// DCB value: `DustParticleCount`
    DustParticleCount,
    /// DCB value: `DustParticleMovementRestriction`
    DustParticleMovementRestriction,
    /// DCB value: `EditBoxCharLimit`
    EditBoxCharLimit,
    /// DCB value: `EntityImageSource`
    EntityImageSource,
    /// DCB value: `FlexDirection`
    FlexDirection,
    /// DCB value: `FlexWrap`
    FlexWrap,
    /// DCB value: `FlexAxisJustification`
    FlexAxisJustification,
    /// DCB value: `FlexCrossAxisJustification`
    FlexCrossAxisJustification,
    /// DCB value: `FlexItemAlignment`
    FlexItemAlignment,
    /// DCB value: `FocusIndex`
    FocusIndex,
    /// DCB value: `GhostPrimCount`
    GhostPrimCount,
    /// DCB value: `GridPackDirection`
    GridPackDirection,
    /// DCB value: `HeightBehavior`
    HeightBehavior,
    /// DCB value: `HorizontalAlignment`
    HorizontalAlignment,
    /// DCB value: `ImageScalingBehavior`
    ImageScalingBehavior,
    /// DCB value: `LayoutDisplayOrder`
    LayoutDisplayOrder,
    /// DCB value: `LayoutPolicy`
    LayoutPolicy,
    /// DCB value: `MaxWidthBehavior`
    MaxWidthBehavior,
    /// DCB value: `MaxHeightBehavior`
    MaxHeightBehavior,
    /// DCB value: `MinWidthBehavior`
    MinWidthBehavior,
    /// DCB value: `MinHeightBehavior`
    MinHeightBehavior,
    /// DCB value: `MovieStartTimeMs`
    MovieStartTimeMs,
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `RadialPolyResolution`
    RadialPolyResolution,
    /// DCB value: `RuntimeImageSource`
    RuntimeImageSource,
    /// DCB value: `ScrollBehavior`
    ScrollBehavior,
    /// DCB value: `ScrollDirection`
    ScrollDirection,
    /// DCB value: `ScrollEasingType`
    ScrollEasingType,
    /// DCB value: `ScrollPolicy`
    ScrollPolicy,
    /// DCB value: `SegmentProgressBehavior`
    SegmentProgressBehavior,
    /// DCB value: `SegmentEasing`
    SegmentEasing,
    /// DCB value: `Segments`
    Segments,
    /// DCB value: `SegmentSizeBehavior`
    SegmentSizeBehavior,
    /// DCB value: `SegmentSpacingBehavior`
    SegmentSpacingBehavior,
    /// DCB value: `Sides`
    Sides,
    /// DCB value: `SliderMode`
    SliderMode,
    /// DCB value: `SquashAxis`
    SquashAxis,
    /// DCB value: `StackDirection`
    StackDirection,
    /// DCB value: `StrokeCapStyle`
    StrokeCapStyle,
    /// DCB value: `StrokeJointStyle`
    StrokeJointStyle,
    /// DCB value: `StrokeAlignment`
    StrokeAlignment,
    /// DCB value: `SvgScalingBehavior`
    SvgScalingBehavior,
    /// DCB value: `TextureGroup`
    TextureGroup,
    /// DCB value: `TickBoxMode`
    TickBoxMode,
    /// DCB value: `VerticalAlignment`
    VerticalAlignment,
    /// DCB value: `WidthBehavior`
    WidthBehavior,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_JumpTerm`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_JumpTerm {
    /// DCB value: `Start`
    Start,
    /// DCB value: `End`
    End,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_LayoutPolicyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_LayoutPolicyType {
    /// DCB value: `None`
    None,
    /// DCB value: `Flex`
    Flex,
    /// DCB value: `GridPack`
    GridPack,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ListNavigationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ListNavigationType {
    /// DCB value: `None`
    None,
    /// DCB value: `PrimaryTab`
    PrimaryTab,
    /// DCB value: `SecondaryTab`
    SecondaryTab,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_LocalizedField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_LocalizedField {
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `EditBoxHelperString`
    EditBoxHelperString,
    /// DCB value: `Text`
    Text,
    /// DCB value: `TooltipText`
    TooltipText,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_NavigationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_NavigationType {
    /// DCB value: `Back`
    Back,
    /// DCB value: `Close`
    Close,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_NumberField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_NumberField {
    /// DCB value: `Alpha`
    Alpha,
    /// DCB value: `AnchorX`
    AnchorX,
    /// DCB value: `AnchorY`
    AnchorY,
    /// DCB value: `AnchorZ`
    AnchorZ,
    /// DCB value: `AutoScrollSpeed`
    AutoScrollSpeed,
    /// DCB value: `AutoScrollStartPause`
    AutoScrollStartPause,
    /// DCB value: `AutoScrollEndPause`
    AutoScrollEndPause,
    /// DCB value: `AutoScrollFadeSpeed`
    AutoScrollFadeSpeed,
    /// DCB value: `BorderTopWidth`
    BorderTopWidth,
    /// DCB value: `BorderRightWidth`
    BorderRightWidth,
    /// DCB value: `BorderBottomWidth`
    BorderBottomWidth,
    /// DCB value: `BorderLeftWidth`
    BorderLeftWidth,
    /// DCB value: `BorderTopLeftRadius`
    BorderTopLeftRadius,
    /// DCB value: `BorderTopRightRadius`
    BorderTopRightRadius,
    /// DCB value: `BorderBottomRightRadius`
    BorderBottomRightRadius,
    /// DCB value: `BorderBottomLeftRadius`
    BorderBottomLeftRadius,
    /// DCB value: `CharsPerSecond`
    CharsPerSecond,
    /// DCB value: `ConicSlantFactor`
    ConicSlantFactor,
    /// DCB value: `CornerRatio`
    CornerRatio,
    /// DCB value: `DissolveOpacity`
    DissolveOpacity,
    /// DCB value: `DustParticleDiameter`
    DustParticleDiameter,
    /// DCB value: `DustParticleOffsetX`
    DustParticleOffsetX,
    /// DCB value: `DustParticleOffsetY`
    DustParticleOffsetY,
    /// DCB value: `DustParticleOffsetZ`
    DustParticleOffsetZ,
    /// DCB value: `DustZoomFactor`
    DustZoomFactor,
    /// DCB value: `EntityContentOrientationX`
    EntityContentOrientationX,
    /// DCB value: `EntityContentOrientationY`
    EntityContentOrientationY,
    /// DCB value: `EntityContentOrientationZ`
    EntityContentOrientationZ,
    /// DCB value: `EntityContentScaleLimitMin`
    EntityContentScaleLimitMin,
    /// DCB value: `EntityContentScaleLimitMax`
    EntityContentScaleLimitMax,
    /// DCB value: `EntityDirt`
    EntityDirt,
    /// DCB value: `EntityWear`
    EntityWear,
    /// DCB value: `InterferenceAmount`
    InterferenceAmount,
    /// DCB value: `FillStrokeWidth`
    FillStrokeWidth,
    /// DCB value: `FlexColumnSpacing`
    FlexColumnSpacing,
    /// DCB value: `FlexRowSpacing`
    FlexRowSpacing,
    /// DCB value: `FlexGrowProportion`
    FlexGrowProportion,
    /// DCB value: `FlexShrinkProportion`
    FlexShrinkProportion,
    /// DCB value: `FontSize`
    FontSize,
    /// DCB value: `GapAngle`
    GapAngle,
    /// DCB value: `GeomEntityAnimationTime`
    GeomEntityAnimationTime,
    /// DCB value: `GhostPrimAlphaClamp`
    GhostPrimAlphaClamp,
    /// DCB value: `GhostPrimAlphaFactor`
    GhostPrimAlphaFactor,
    /// DCB value: `GhostPrimScaleFactor`
    GhostPrimScaleFactor,
    /// DCB value: `GlowAmount`
    GlowAmount,
    /// DCB value: `GridPackGutter`
    GridPackGutter,
    /// DCB value: `HitDetectionOffset`
    HitDetectionOffset,
    /// DCB value: `ImageColorPickerStepSizeX`
    ImageColorPickerStepSizeX,
    /// DCB value: `ImageColorPickerStepSizeY`
    ImageColorPickerStepSizeY,
    /// DCB value: `LayoutTransitionDelay`
    LayoutTransitionDelay,
    /// DCB value: `LayoutTransitionDuration`
    LayoutTransitionDuration,
    /// DCB value: `LetterSpacing`
    LetterSpacing,
    /// DCB value: `LightBulbRadius`
    LightBulbRadius,
    /// DCB value: `LightIntensity`
    LightIntensity,
    /// DCB value: `LightFrustumAngle`
    LightFrustumAngle,
    /// DCB value: `LightRadius`
    LightRadius,
    /// DCB value: `LinearSliderMinValue`
    LinearSliderMinValue,
    /// DCB value: `LinearSliderMaxValue`
    LinearSliderMaxValue,
    /// DCB value: `LineSpacing`
    LineSpacing,
    /// DCB value: `LocalCoordinateSpace`
    LocalCoordinateSpace,
    /// DCB value: `MarginTop`
    MarginTop,
    /// DCB value: `MarginRight`
    MarginRight,
    /// DCB value: `MarginBottom`
    MarginBottom,
    /// DCB value: `MarginLeft`
    MarginLeft,
    /// DCB value: `MarginFront`
    MarginFront,
    /// DCB value: `MarginBack`
    MarginBack,
    /// DCB value: `MaxMeterClamp`
    MaxMeterClamp,
    /// DCB value: `MaxProgressClamp`
    MaxProgressClamp,
    /// DCB value: `MaxSizeX`
    MaxSizeX,
    /// DCB value: `MaxSizeY`
    MaxSizeY,
    /// DCB value: `MinMeterClamp`
    MinMeterClamp,
    /// DCB value: `MinProgressClamp`
    MinProgressClamp,
    /// DCB value: `MinSizeX`
    MinSizeX,
    /// DCB value: `MinSizeY`
    MinSizeY,
    /// DCB value: `NineSliceTop`
    NineSliceTop,
    /// DCB value: `NineSliceRight`
    NineSliceRight,
    /// DCB value: `NineSliceBottom`
    NineSliceBottom,
    /// DCB value: `NineSliceLeft`
    NineSliceLeft,
    /// DCB value: `NineSliceScale`
    NineSliceScale,
    /// DCB value: `OrientationX`
    OrientationX,
    /// DCB value: `OrientationXOffset`
    OrientationXOffset,
    /// DCB value: `OrientationY`
    OrientationY,
    /// DCB value: `OrientationYOffset`
    OrientationYOffset,
    /// DCB value: `OrientationZ`
    OrientationZ,
    /// DCB value: `OrientationZOffset`
    OrientationZOffset,
    /// DCB value: `OverflowWidthFadeThreshold`
    OverflowWidthFadeThreshold,
    /// DCB value: `OverflowHeightFadeThreshold`
    OverflowHeightFadeThreshold,
    /// DCB value: `OverflowDepthFadeThreshold`
    OverflowDepthFadeThreshold,
    /// DCB value: `PaddingTop`
    PaddingTop,
    /// DCB value: `PaddingRight`
    PaddingRight,
    /// DCB value: `PaddingBottom`
    PaddingBottom,
    /// DCB value: `PaddingLeft`
    PaddingLeft,
    /// DCB value: `PaddingFront`
    PaddingFront,
    /// DCB value: `PaddingBack`
    PaddingBack,
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `PivotX`
    PivotX,
    /// DCB value: `PivotY`
    PivotY,
    /// DCB value: `PivotZ`
    PivotZ,
    /// DCB value: `PosX`
    PosX,
    /// DCB value: `PosXOffset`
    PosXOffset,
    /// DCB value: `PosY`
    PosY,
    /// DCB value: `PosYOffset`
    PosYOffset,
    /// DCB value: `PosZ`
    PosZ,
    /// DCB value: `PosZOffset`
    PosZOffset,
    /// DCB value: `PrimitiveConstantScale`
    PrimitiveConstantScale,
    /// DCB value: `PrimitiveInterferenceEffect`
    PrimitiveInterferenceEffect,
    /// DCB value: `Progress`
    Progress,
    /// DCB value: `RadialTransformMultiplier`
    RadialTransformMultiplier,
    /// DCB value: `ScaleX`
    ScaleX,
    /// DCB value: `ScaleY`
    ScaleY,
    /// DCB value: `ScaleZ`
    ScaleZ,
    /// DCB value: `ScrollPixelIncrement`
    ScrollPixelIncrement,
    /// DCB value: `ScrollEasingTime`
    ScrollEasingTime,
    /// DCB value: `SegmentAngle`
    SegmentAngle,
    /// DCB value: `SegmentSize`
    SegmentSize,
    /// DCB value: `SegmentSpacingSize`
    SegmentSpacingSize,
    /// DCB value: `SegmentXOffset`
    SegmentXOffset,
    /// DCB value: `SilhouetteWidth`
    SilhouetteWidth,
    /// DCB value: `SizeX`
    SizeX,
    /// DCB value: `SizeY`
    SizeY,
    /// DCB value: `SizeZ`
    SizeZ,
    /// DCB value: `SliderStepSize`
    SliderStepSize,
    /// DCB value: `StackSpacing`
    StackSpacing,
    /// DCB value: `StartAngle`
    StartAngle,
    /// DCB value: `StartDelayTime`
    StartDelayTime,
    /// DCB value: `UStart`
    UStart,
    /// DCB value: `VStart`
    VStart,
    /// DCB value: `USize`
    USize,
    /// DCB value: `VSize`
    VSize,
    /// DCB value: `StepAngle`
    StepAngle,
    /// DCB value: `StepMidPoint`
    StepMidPoint,
    /// DCB value: `StripWidth`
    StripWidth,
    /// DCB value: `StrokeExtent`
    StrokeExtent,
    /// DCB value: `StrokeWidth`
    StrokeWidth,
    /// DCB value: `SvgContainPositionX`
    SvgContainPositionX,
    /// DCB value: `SvgContainPositionY`
    SvgContainPositionY,
    /// DCB value: `SvgPlayhead`
    SvgPlayhead,
    /// DCB value: `TintStrength`
    TintStrength,
    /// DCB value: `WindowCameraFOV`
    WindowCameraFOV,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_OverflowBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_OverflowBehavior {
    /// DCB value: `Visible`
    Visible,
    /// DCB value: `Fade`
    Fade,
    /// DCB value: `Clip`
    Clip,
    /// DCB value: `ClipFade`
    ClipFade,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ProgressMeterState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ProgressMeterState {
    /// DCB value: `Static`
    Static,
    /// DCB value: `Active`
    Active,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_RendererType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_RendererType {
    /// DCB value: `None`
    None,
    /// DCB value: `Entity`
    Entity,
    /// DCB value: `Flash`
    Flash,
    /// DCB value: `Paint`
    Paint,
    /// DCB value: `Primitive`
    Primitive,
    /// DCB value: `ForceFlash`
    ForceFlash,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_RotationField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_RotationField {
    /// DCB value: `Orientation`
    Orientation,
    /// DCB value: `OrientationOffset`
    OrientationOffset,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SIPrefix`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SIPrefix {
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `Yocto`
    Yocto,
    /// DCB value: `Zepto`
    Zepto,
    /// DCB value: `Atto`
    Atto,
    /// DCB value: `Femto`
    Femto,
    /// DCB value: `Pico`
    Pico,
    /// DCB value: `Nano`
    Nano,
    /// DCB value: `Micro`
    Micro,
    /// DCB value: `Milli`
    Milli,
    /// DCB value: `Unit`
    Unit,
    /// DCB value: `Kilo`
    Kilo,
    /// DCB value: `Mega`
    Mega,
    /// DCB value: `Giga`
    Giga,
    /// DCB value: `AstronomicalUnit`
    AstronomicalUnit,
    /// DCB value: `LightYear`
    LightYear,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SIUnit`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SIUnit {
    /// DCB value: `None`
    None,
    /// DCB value: `Acceleration`
    Acceleration,
    /// DCB value: `Area`
    Area,
    /// DCB value: `Distance`
    Distance,
    /// DCB value: `Force`
    Force,
    /// DCB value: `Percent`
    Percent,
    /// DCB value: `Power`
    Power,
    /// DCB value: `Speed`
    Speed,
    /// DCB value: `Temperature`
    Temperature,
    /// DCB value: `Volume`
    Volume,
    /// DCB value: `Weight`
    Weight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SVGScalingBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SVGScalingBehavior {
    /// DCB value: `Contain`
    Contain,
    /// DCB value: `Center`
    Center,
    /// DCB value: `StretchX`
    StretchX,
    /// DCB value: `StretchY`
    StretchY,
    /// DCB value: `StretchXY`
    StretchXY,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ScrollBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ScrollBehavior {
    /// DCB value: `Pixel`
    Pixel,
    /// DCB value: `Item`
    Item,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_ScrollPolicyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_ScrollPolicyType {
    /// DCB value: `None`
    None,
    /// DCB value: `UnidirectionalScroller`
    UnidirectionalScroller,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SegmentProgressBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SegmentProgressBehavior {
    /// DCB value: `Clip`
    Clip,
    /// DCB value: `Alpha`
    Alpha,
    /// DCB value: `Visibility`
    Visibility,
    /// DCB value: `ThicknessIn`
    ThicknessIn,
    /// DCB value: `ThicknessInAlpha`
    ThicknessInAlpha,
    /// DCB value: `ThicknessOut`
    ThicknessOut,
    /// DCB value: `ThicknessOutAlpha`
    ThicknessOutAlpha,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SeparatorStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SeparatorStyle {
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Secondary`
    Secondary,
    /// DCB value: `Tertiary`
    Tertiary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SizeBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SizeBehavior {
    /// DCB value: `Fixed`
    Fixed,
    /// DCB value: `Percent`
    Percent,
    /// DCB value: `PercentOfX`
    PercentOfX,
    /// DCB value: `PercentOfY`
    PercentOfY,
    /// DCB value: `PercentOfZ`
    PercentOfZ,
    /// DCB value: `Auto`
    Auto,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_SliderMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_SliderMode {
    /// DCB value: `HorizontalBar`
    HorizontalBar,
    /// DCB value: `VerticalBar`
    VerticalBar,
    /// DCB value: `HorizontalMarker`
    HorizontalMarker,
    /// DCB value: `VerticalMarker`
    VerticalMarker,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StackDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StackDirection {
    /// DCB value: `Horizontal`
    Horizontal,
    /// DCB value: `Vertical`
    Vertical,
    /// DCB value: `Depth`
    Depth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StringField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StringField {
    /// DCB value: `ActorEntityDNAString`
    ActorEntityDNAString,
    /// DCB value: `ActorEntityIdleName`
    ActorEntityIdleName,
    /// DCB value: `ActorEntityFacialName`
    ActorEntityFacialName,
    /// DCB value: `CanvasReferenceRecord`
    CanvasReferenceRecord,
    /// DCB value: `EditBoxConfirmTrigger`
    EditBoxConfirmTrigger,
    /// DCB value: `EditBoxVariablePath`
    EditBoxVariablePath,
    /// DCB value: `EntityGeometryTag`
    EntityGeometryTag,
    /// DCB value: `EntityLoadoutName0`
    EntityLoadoutName0,
    /// DCB value: `EntityLoadoutName1`
    EntityLoadoutName1,
    /// DCB value: `EntityLoadoutName2`
    EntityLoadoutName2,
    /// DCB value: `EntityLoadoutName3`
    EntityLoadoutName3,
    /// DCB value: `EntityMaterialPath`
    EntityMaterialPath,
    /// DCB value: `EntityTintPalettePath`
    EntityTintPalettePath,
    /// DCB value: `FontStyleRecord`
    FontStyleRecord,
    /// DCB value: `GeneralEntityClassName`
    GeneralEntityClassName,
    /// DCB value: `GeomEntityAnimationPath`
    GeomEntityAnimationPath,
    /// DCB value: `GeomEntityGeometryPath`
    GeomEntityGeometryPath,
    /// DCB value: `ImageColorPickerAnchorXURL0`
    ImageColorPickerAnchorXURL0,
    /// DCB value: `ImageColorPickerAnchorYURL0`
    ImageColorPickerAnchorYURL0,
    /// DCB value: `ImageColorPickerOutputColorURL0`
    ImageColorPickerOutputColorURL0,
    /// DCB value: `ImagePath`
    ImagePath,
    /// DCB value: `InteractionsOnLeftClickTriggerURL0`
    InteractionsOnLeftClickTriggerURL0,
    /// DCB value: `InteractionsOnLeftClickTriggerURL1`
    InteractionsOnLeftClickTriggerURL1,
    /// DCB value: `LightImagePath`
    LightImagePath,
    /// DCB value: `MovieListContainerURL`
    MovieListContainerURL,
    /// DCB value: `MovieListVariableURL`
    MovieListVariableURL,
    /// DCB value: `MoviePath`
    MoviePath,
    /// DCB value: `ParamInput0`
    ParamInput0,
    /// DCB value: `ParamInput1`
    ParamInput1,
    /// DCB value: `ParamInput2`
    ParamInput2,
    /// DCB value: `ParamInput3`
    ParamInput3,
    /// DCB value: `ParamInput4`
    ParamInput4,
    /// DCB value: `ParamInput5`
    ParamInput5,
    /// DCB value: `ParamInput6`
    ParamInput6,
    /// DCB value: `ParamInput7`
    ParamInput7,
    /// DCB value: `ParamInput8`
    ParamInput8,
    /// DCB value: `ParticleEffectName`
    ParticleEffectName,
    /// DCB value: `PrimaryStateTag`
    PrimaryStateTag,
    /// DCB value: `PrimitiveMaterialPath`
    PrimitiveMaterialPath,
    /// DCB value: `QuarternaryStateTag`
    QuarternaryStateTag,
    /// DCB value: `QuinaryStateTag`
    QuinaryStateTag,
    /// DCB value: `SecondaryStateTag`
    SecondaryStateTag,
    /// DCB value: `StripMaterialPath`
    StripMaterialPath,
    /// DCB value: `StyleReferenceRecord`
    StyleReferenceRecord,
    /// DCB value: `SenaryStateTag`
    SenaryStateTag,
    /// DCB value: `SvgPath`
    SvgPath,
    /// DCB value: `TertiaryStateTag`
    TertiaryStateTag,
    /// DCB value: `TextMaterial`
    TextMaterial,
    /// DCB value: `URLOptional`
    URLOptional,
    /// DCB value: `URLPostfix`
    URLPostfix,
    /// DCB value: `VariableInput0`
    VariableInput0,
    /// DCB value: `VariableInput1`
    VariableInput1,
    /// DCB value: `VariableInput2`
    VariableInput2,
    /// DCB value: `VariableInput3`
    VariableInput3,
    /// DCB value: `VariableInput4`
    VariableInput4,
    /// DCB value: `VehicleEntityClassName`
    VehicleEntityClassName,
    /// DCB value: `VehicleEntityLoadoutName`
    VehicleEntityLoadoutName,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StrokeAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StrokeAlignment {
    /// DCB value: `Center`
    Center,
    /// DCB value: `Inside`
    Inside,
    /// DCB value: `Outside`
    Outside,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StrokeCapStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StrokeCapStyle {
    /// DCB value: `Round`
    Round,
    /// DCB value: `None`
    None,
    /// DCB value: `Square`
    Square,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StrokeJointStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StrokeJointStyle {
    /// DCB value: `Round`
    Round,
    /// DCB value: `Bevel`
    Bevel,
    /// DCB value: `Miter`
    Miter,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_StyleCountType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_StyleCountType {
    /// DCB value: `Every`
    Every,
    /// DCB value: `First`
    First,
    /// DCB value: `Last`
    Last,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_TextAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_TextAlignment {
    /// DCB value: `Left`
    Left,
    /// DCB value: `Center`
    Center,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Justify`
    Justify,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_TextFieldWidgetStylePreset`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_TextFieldWidgetStylePreset {
    /// DCB value: `Title1`
    Title1,
    /// DCB value: `Title2`
    Title2,
    /// DCB value: `Title3`
    Title3,
    /// DCB value: `Title4`
    Title4,
    /// DCB value: `Title5`
    Title5,
    /// DCB value: `Heading1`
    Heading1,
    /// DCB value: `Heading2`
    Heading2,
    /// DCB value: `Heading3`
    Heading3,
    /// DCB value: `Heading4`
    Heading4,
    /// DCB value: `Heading5`
    Heading5,
    /// DCB value: `Heading6`
    Heading6,
    /// DCB value: `Body`
    Body,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_TextureOrientation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_TextureOrientation {
    /// DCB value: `Landscape`
    Landscape,
    /// DCB value: `Portrait`
    Portrait,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_TransformField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_TransformField {
    /// DCB value: `PositionOrientation`
    PositionOrientation,
    /// DCB value: `PositionOrientationOffset`
    PositionOrientationOffset,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_UnidirectionalAutoScrollBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_UnidirectionalAutoScrollBehavior {
    /// DCB value: `LeftToRight_Always`
    LeftToRight_Always,
    /// DCB value: `LeftToRight_IfLong`
    LeftToRight_IfLong,
    /// DCB value: `RightToLeft_Always`
    RightToLeft_Always,
    /// DCB value: `RightToLeft_IfLong`
    RightToLeft_IfLong,
    /// DCB value: `Bounce`
    Bounce,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_UnidirectionalScrollDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_UnidirectionalScrollDirection {
    /// DCB value: `DeriveFromLayout`
    DeriveFromLayout,
    /// DCB value: `Horizontal`
    Horizontal,
    /// DCB value: `Vertical`
    Vertical,
    /// DCB value: `Depth`
    Depth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_VectorField`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_VectorField {
    /// DCB value: `Position`
    Position,
    /// DCB value: `PositionOffset`
    PositionOffset,
    /// DCB value: `Orientation`
    Orientation,
    /// DCB value: `OrientationOffset`
    OrientationOffset,
    /// DCB value: `Scale`
    Scale,
    /// DCB value: `Size`
    Size,
    /// DCB value: `Pivot`
    Pivot,
    /// DCB value: `Anchor`
    Anchor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_VerticalTextAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_VerticalTextAlignment {
    /// DCB value: `Top`
    Top,
    /// DCB value: `Center`
    Center,
    /// DCB value: `Bottom`
    Bottom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BB_WidgetType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BB_WidgetType {
    /// DCB value: `ActorEntity`
    ActorEntity,
    /// DCB value: `Base`
    Base,
    /// DCB value: `Canvas`
    Canvas,
    /// DCB value: `Card`
    Card,
    /// DCB value: `CardTexture`
    CardTexture,
    /// DCB value: `Circle`
    Circle,
    /// DCB value: `Clone`
    Clone,
    /// DCB value: `CustomShape`
    CustomShape,
    /// DCB value: `EditBox`
    EditBox,
    /// DCB value: `EnvironmentProbe`
    EnvironmentProbe,
    /// DCB value: `GeneralEntity`
    GeneralEntity,
    /// DCB value: `GeomEntity`
    GeomEntity,
    /// DCB value: `HoloVolume`
    HoloVolume,
    /// DCB value: `Image`
    Image,
    /// DCB value: `Light`
    Light,
    /// DCB value: `Line`
    Line,
    /// DCB value: `List`
    List,
    /// DCB value: `LineList`
    LineList,
    /// DCB value: `Movie`
    Movie,
    /// DCB value: `ParticleEffect`
    ParticleEffect,
    /// DCB value: `Polygon`
    Polygon,
    /// DCB value: `Polymorphic`
    Polymorphic,
    /// DCB value: `RuntimeImage`
    RuntimeImage,
    /// DCB value: `RuntimeVolume`
    RuntimeVolume,
    /// DCB value: `Slider`
    Slider,
    /// DCB value: `Slice`
    Slice,
    /// DCB value: `Strip`
    Strip,
    /// DCB value: `Text`
    Text,
    /// DCB value: `TickBox`
    TickBox,
    /// DCB value: `VehicleEntity`
    VehicleEntity,
    /// DCB value: `Window`
    Window,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `BettingFormat`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BettingFormat {
    /// DCB value: `TopPlayer`
    TopPlayer,
    /// DCB value: `TopTeam`
    TopTeam,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CIGAudioContextNamingStrategy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CIGAudioContextNamingStrategy {
    /// DCB value: `DataCoreFilePathAndEntityClassName`
    DataCoreFilePathAndEntityClassName,
    /// DCB value: `EntityClassNameAndEntityName`
    EntityClassNameAndEntityName,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CameraViewTransitionMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CameraViewTransitionMode {
    /// DCB value: `ExpSlerp`
    ExpSlerp,
    /// DCB value: `Nlerp`
    Nlerp,
    /// DCB value: `NlerpCubic`
    NlerpCubic,
    /// DCB value: `Slerp`
    Slerp,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CanvasInstantiationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CanvasInstantiationMode {
    /// DCB value: `RootDeferred`
    RootDeferred,
    /// DCB value: `CanvasDeferred`
    CanvasDeferred,
    /// DCB value: `Block`
    Block,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CargoFaceStackingSupport`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CargoFaceStackingSupport {
    /// DCB value: `StackAll`
    StackAll,
    /// DCB value: `StackSelf`
    StackSelf,
    /// DCB value: `StackNone`
    StackNone,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ChannelColor`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelColor {
    /// DCB value: `Blue`
    Blue,
    /// DCB value: `White`
    White,
    /// DCB value: `Silver`
    Silver,
    /// DCB value: `Grey`
    Grey,
    /// DCB value: `Charcoal`
    Charcoal,
    /// DCB value: `Red`
    Red,
    /// DCB value: `Orange`
    Orange,
    /// DCB value: `Yellow`
    Yellow,
    /// DCB value: `Chartreuse`
    Chartreuse,
    /// DCB value: `Green`
    Green,
    /// DCB value: `Mint`
    Mint,
    /// DCB value: `Cyan`
    Cyan,
    /// DCB value: `Azure`
    Azure,
    /// DCB value: `Indigo`
    Indigo,
    /// DCB value: `Purple`
    Purple,
    /// DCB value: `Pink`
    Pink,
    /// DCB value: `Hotpink`
    Hotpink,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ClassMigrationValidationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassMigrationValidationType {
    /// DCB value: `None`
    None,
    /// DCB value: `DataAssert`
    DataAssert,
    /// DCB value: `DataCritical`
    DataCritical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CombatStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatStyle {
    /// DCB value: `NotSet`
    NotSet,
    /// DCB value: `Civilian`
    Civilian,
    /// DCB value: `Merc`
    Merc,
    /// DCB value: `SpecOps`
    SpecOps,
    /// DCB value: `Slaver`
    Slaver,
    /// DCB value: `OMC`
    OMC,
    /// DCB value: `Galson`
    Galson,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ComparisonType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComparisonType {
    /// DCB value: `HigherOrEqualTo`
    HigherOrEqualTo,
    /// DCB value: `LowerOrEqualTo`
    LowerOrEqualTo,
    /// DCB value: `Equals`
    Equals,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ConditionResult`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionResult {
    /// DCB value: `Pass`
    Pass,
    /// DCB value: `Fail`
    Fail,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ContractBoolParamType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractBoolParamType {
    /// DCB value: `Illegal`
    Illegal,
    /// DCB value: `ShowLifeTimeInMobiGlas`
    ShowLifeTimeInMobiGlas,
    /// DCB value: `FailIfSentToPrison`
    FailIfSentToPrison,
    /// DCB value: `FailIfBecameCriminal`
    FailIfBecameCriminal,
    /// DCB value: `FailIfLeavePrison`
    FailIfLeavePrison,
    /// DCB value: `NotifyOnAvailable`
    NotifyOnAvailable,
    /// DCB value: `OnceOnly`
    OnceOnly,
    /// DCB value: `CanReacceptAfterAbandoning`
    CanReacceptAfterAbandoning,
    /// DCB value: `CanReacceptAfterFailing`
    CanReacceptAfterFailing,
    /// DCB value: `HasPersonalCooldown`
    HasPersonalCooldown,
    /// DCB value: `CanBeShared`
    CanBeShared,
    /// DCB value: `HasCompleteButton`
    HasCompleteButton,
    /// DCB value: `HideInMobiGlas`
    HideInMobiGlas,
    /// DCB value: `AcceptMissionSpawnedCargo`
    AcceptMissionSpawnedCargo,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ContractEndCommsReason`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractEndCommsReason {
    /// DCB value: `Complete`
    Complete,
    /// DCB value: `Fail`
    Fail,
    /// DCB value: `Abandon`
    Abandon,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ContractIntParamType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractIntParamType {
    /// DCB value: `MaxPlayersPerInstance`
    MaxPlayersPerInstance,
    /// DCB value: `AbandonedCooldownTime`
    AbandonedCooldownTime,
    /// DCB value: `AbandonedCooldownTimeVariation`
    AbandonedCooldownTimeVariation,
    /// DCB value: `PersonalCooldownTime`
    PersonalCooldownTime,
    /// DCB value: `PersonalCooldownTimeVariation`
    PersonalCooldownTimeVariation,
    /// DCB value: `TimeToComplete`
    TimeToComplete,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ContractStringParamType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractStringParamType {
    /// DCB value: `Title`
    Title,
    /// DCB value: `TitleHUD`
    TitleHUD,
    /// DCB value: `Description`
    Description,
    /// DCB value: `Contractor`
    Contractor,
    /// DCB value: `CommsChannelName`
    CommsChannelName,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorAttachmentType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorAttachmentType {
    /// DCB value: `Held`
    Held,
    /// DCB value: `Attached`
    Attached,
    /// DCB value: `Wearing`
    Wearing,
    /// DCB value: `PersonalInventory`
    PersonalInventory,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorCommsState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorCommsState {
    /// DCB value: `BeingCalled`
    BeingCalled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorEnvironment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorEnvironment {
    /// DCB value: `Greenzone`
    Greenzone,
    /// DCB value: `UsingKiosk`
    UsingKiosk,
    /// DCB value: `NearInteractableObject`
    NearInteractableObject,
    /// DCB value: `NearMineableRock`
    NearMineableRock,
    /// DCB value: `NearDraggableBody`
    NearDraggableBody,
    /// DCB value: `InRestrainRange`
    InRestrainRange,
    /// DCB value: `InTakeDownRange`
    InTakeDownRange,
    /// DCB value: `InMeleeRange`
    InMeleeRange,
    /// DCB value: `InMantleRange`
    InMantleRange,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorInteractionState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorInteractionState {
    /// DCB value: `InInteractModeNothingUnderCursor`
    InInteractModeNothingUnderCursor,
    /// DCB value: `InInteractModeInteractableUnderCursor`
    InInteractModeInteractableUnderCursor,
    /// DCB value: `InInteractModeScreenUnderCursor`
    InInteractModeScreenUnderCursor,
    /// DCB value: `QuickSelectWheelOpen`
    QuickSelectWheelOpen,
    /// DCB value: `PITMenuOpen`
    PITMenuOpen,
    /// DCB value: `PITMenuOpenMultiplePages`
    PITMenuOpenMultiplePages,
    /// DCB value: `InventoryOpen`
    InventoryOpen,
    /// DCB value: `InventoryOpenItemUnderCursor`
    InventoryOpenItemUnderCursor,
    /// DCB value: `InventoryOpenContextMenuOpen`
    InventoryOpenContextMenuOpen,
    /// DCB value: `LootingOpen`
    LootingOpen,
    /// DCB value: `UsableLinkedInteractive`
    UsableLinkedInteractive,
    /// DCB value: `PISWheelOpen`
    PISWheelOpen,
    /// DCB value: `PISWheelOpenMultiplePages`
    PISWheelOpenMultiplePages,
    /// DCB value: `SwapWheelOpen`
    SwapWheelOpen,
    /// DCB value: `SwapWheelOpenMultiplePages`
    SwapWheelOpenMultiplePages,
    /// DCB value: `PISWheelOpenCustomisablePrimarySecondary`
    PISWheelOpenCustomisablePrimarySecondary,
    /// DCB value: `PISWheelOpenContextMenuOpen`
    PISWheelOpenContextMenuOpen,
    /// DCB value: `QuickSelectWheelOpenSegmentHasContextMenu`
    QuickSelectWheelOpenSegmentHasContextMenu,
    /// DCB value: `QuickSelectWheelOpenHoverOverNavigationSegment`
    QuickSelectWheelOpenHoverOverNavigationSegment,
    /// DCB value: `QuickSelectWheelOpenMultiplePages`
    QuickSelectWheelOpenMultiplePages,
    /// DCB value: `InteractionPromptWithSingleOptionAvailable`
    InteractionPromptWithSingleOptionAvailable,
    /// DCB value: `InteractionPromptWithMultipleOptionsAvailable`
    InteractionPromptWithMultipleOptionsAvailable,
    /// DCB value: `InteractionPromptHasAvailableSecondaryAction`
    InteractionPromptHasAvailableSecondaryAction,
    /// DCB value: `InteractionPromptOverridesHintDescription`
    InteractionPromptOverridesHintDescription,
    /// DCB value: `OffscreenInteractionAvailable`
    OffscreenInteractionAvailable,
    /// DCB value: `HeldItemHasAvailablePrimaryAction`
    HeldItemHasAvailablePrimaryAction,
    /// DCB value: `HeldItemHasAvailableSecondaryAction`
    HeldItemHasAvailableSecondaryAction,
    /// DCB value: `InputAwaitingRebind`
    InputAwaitingRebind,
    /// DCB value: `UsingNonFocusedModeKiosk`
    UsingNonFocusedModeKiosk,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorLadderState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorLadderState {
    /// DCB value: `WithinMidPointRange`
    WithinMidPointRange,
    /// DCB value: `WithinLaunchAngle`
    WithinLaunchAngle,
    /// DCB value: `IsLadderSlideable`
    IsLadderSlideable,
    /// DCB value: `CanDodgeRight`
    CanDodgeRight,
    /// DCB value: `CanDodgeLeft`
    CanDodgeLeft,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorMissionOfferReceived`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorMissionOfferReceived {
    /// DCB value: `OfferReceived`
    OfferReceived,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorSightZeroMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorSightZeroMode {
    /// DCB value: `Manual`
    Manual,
    /// DCB value: `Auto`
    Auto,
    /// DCB value: `AutoIsZeroed`
    AutoIsZeroed,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorSuitState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorSuitState {
    /// DCB value: `VisorDirty`
    VisorDirty,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorToolState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorToolState {
    /// DCB value: `TractorBeamInUseOnObject`
    TractorBeamInUseOnObject,
    /// DCB value: `TractorBeamInDetachMode`
    TractorBeamInDetachMode,
    /// DCB value: `TractorBeamInCargoMode`
    TractorBeamInCargoMode,
    /// DCB value: `TractorBeamValidPlacementTarget`
    TractorBeamValidPlacementTarget,
    /// DCB value: `TractorBeamMovingToValidTarget`
    TractorBeamMovingToValidTarget,
    /// DCB value: `TractorBeamChargingThrow`
    TractorBeamChargingThrow,
    /// DCB value: `MediGunInUseOnTarget`
    MediGunInUseOnTarget,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorWeaponState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorWeaponState {
    /// DCB value: `Reloadable`
    Reloadable,
    /// DCB value: `AmmoFull`
    AmmoFull,
    /// DCB value: `AmmoEmpty`
    AmmoEmpty,
    /// DCB value: `ModifierAttachment`
    ModifierAttachment,
    /// DCB value: `AltFire`
    AltFire,
    /// DCB value: `HasModifierPortsAvailable`
    HasModifierPortsAvailable,
    /// DCB value: `HasAmmoAvailableInItemPortOrInventory`
    HasAmmoAvailableInItemPortOrInventory,
    /// DCB value: `ActivatableUnderbarrelAttachment`
    ActivatableUnderbarrelAttachment,
    /// DCB value: `Customizing`
    Customizing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionActorZeroGEVAState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionActorZeroGEVAState {
    /// DCB value: `CanAttachToSurface`
    CanAttachToSurface,
    /// DCB value: `CanAttachToGrip`
    CanAttachToGrip,
    /// DCB value: `IsAttachedToGrip`
    IsAttachedToGrip,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionMobiglasMapState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionMobiglasMapState {
    /// DCB value: `InteriorMapActive`
    InteriorMapActive,
    /// DCB value: `InteriorMapAvailable`
    InteriorMapAvailable,
    /// DCB value: `InteriorMapRouteAvailable`
    InteriorMapRouteAvailable,
    /// DCB value: `InteriorMapRouteSet`
    InteriorMapRouteSet,
    /// DCB value: `InteriorMapCrossSectionViewActive`
    InteriorMapCrossSectionViewActive,
    /// DCB value: `InteriorMapHasMultipleZones`
    InteriorMapHasMultipleZones,
    /// DCB value: `InteriorMapHasMultipleSections`
    InteriorMapHasMultipleSections,
    /// DCB value: `StarMapActive`
    StarMapActive,
    /// DCB value: `StarMapInJumpTunnel`
    StarMapInJumpTunnel,
    /// DCB value: `StarMapRouteAvailable`
    StarMapRouteAvailable,
    /// DCB value: `StarMapRouteSet`
    StarMapRouteSet,
    /// DCB value: `GalacticMapActive`
    GalacticMapActive,
    /// DCB value: `GeneralMapStepBackAvailable`
    GeneralMapStepBackAvailable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionOptInEventState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionOptInEventState {
    /// DCB value: `OptInEventAvailable`
    OptInEventAvailable,
    /// DCB value: `OptInEventActive`
    OptInEventActive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionRemoteTurret`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionRemoteTurret {
    /// DCB value: `HasRemoteTurretAccess`
    HasRemoteTurretAccess,
    /// DCB value: `MultipleRemoteTurrets`
    MultipleRemoteTurrets,
    /// DCB value: `RemoteTurret1Available`
    RemoteTurret1Available,
    /// DCB value: `RemoteTurret2Available`
    RemoteTurret2Available,
    /// DCB value: `RemoteTurret3Available`
    RemoteTurret3Available,
    /// DCB value: `UsingRemoteTurret`
    UsingRemoteTurret,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionTransporterMotionSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionTransporterMotionSpeed {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Movement`
    Movement,
    /// DCB value: `Sprint`
    Sprint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionTryOnState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionTryOnState {
    /// DCB value: `Active`
    Active,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleAutoLandState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleAutoLandState {
    /// DCB value: `Ready`
    Ready,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleAutoSelectedContact`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleAutoSelectedContact {
    /// DCB value: `None`
    None,
    /// DCB value: `All`
    All,
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `Attacker`
    Attacker,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleComponents`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleComponents {
    /// DCB value: `Missiles`
    Missiles,
    /// DCB value: `Guns`
    Guns,
    /// DCB value: `Radar`
    Radar,
    /// DCB value: `QuantumInterdictionSnare`
    QuantumInterdictionSnare,
    /// DCB value: `MiningLaser`
    MiningLaser,
    /// DCB value: `QuantumDrive`
    QuantumDrive,
    /// DCB value: `Bombs`
    Bombs,
    /// DCB value: `Gimbals`
    Gimbals,
    /// DCB value: `CurrentOrdnanceTypeEmpty`
    CurrentOrdnanceTypeEmpty,
    /// DCB value: `JumpThrusterPack`
    JumpThrusterPack,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleCounterMeasureState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleCounterMeasureState {
    /// DCB value: `HasChaff`
    HasChaff,
    /// DCB value: `HasFlare`
    HasFlare,
    /// DCB value: `IncomingCounterWithChaff`
    IncomingCounterWithChaff,
    /// DCB value: `IncomingCounterWithFlare`
    IncomingCounterWithFlare,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleDestroyedState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleDestroyedState {
    /// DCB value: `Alive`
    Alive,
    /// DCB value: `SoftDeath`
    SoftDeath,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleDocking`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleDocking {
    /// DCB value: `NoDockingTube`
    NoDockingTube,
    /// DCB value: `Active`
    Active,
    /// DCB value: `CanAutoDock`
    CanAutoDock,
    /// DCB value: `AutoDocking`
    AutoDocking,
    /// DCB value: `NeedUndockRequest`
    NeedUndockRequest,
    /// DCB value: `NearDockingStation`
    NearDockingStation,
    /// DCB value: `NearDockingShip`
    NearDockingShip,
    /// DCB value: `DockingTargetValid`
    DockingTargetValid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleEjectorSeat`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleEjectorSeat {
    /// DCB value: `EjectorSeat`
    EjectorSeat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleEnvironment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleEnvironment {
    /// DCB value: `IncomingMissiles`
    IncomingMissiles,
    /// DCB value: `ContactOnRadar`
    ContactOnRadar,
    /// DCB value: `HostileOnRadar`
    HostileOnRadar,
    /// DCB value: `BlobOnRadar`
    BlobOnRadar,
    /// DCB value: `UnknownOnRadar`
    UnknownOnRadar,
    /// DCB value: `NearMineableRock`
    NearMineableRock,
    /// DCB value: `AttackerOnRadar`
    AttackerOnRadar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleHealthState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleHealthState {
    /// DCB value: `Below10Percent`
    Below10Percent,
    /// DCB value: `Below20Percent`
    Below20Percent,
    /// DCB value: `Below30Percent`
    Below30Percent,
    /// DCB value: `Below40Percent`
    Below40Percent,
    /// DCB value: `Below50Percent`
    Below50Percent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleJumpDriveState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleJumpDriveState {
    /// DCB value: `HasJumpDrive`
    HasJumpDrive,
    /// DCB value: `HasLinkedJumpPoint`
    HasLinkedJumpPoint,
    /// DCB value: `TunedToJumpPoint`
    TunedToJumpPoint,
    /// DCB value: `JumpPointClosed`
    JumpPointClosed,
    /// DCB value: `ShipInATCQueue`
    ShipInATCQueue,
    /// DCB value: `InJumpTunnel`
    InJumpTunnel,
    /// DCB value: `TakingDistortionDamage`
    TakingDistortionDamage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleLandingArea`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleLandingArea {
    /// DCB value: `NoLandingArea`
    NoLandingArea,
    /// DCB value: `InsideNonReservedLandingArea`
    InsideNonReservedLandingArea,
    /// DCB value: `Reserved`
    Reserved,
    /// DCB value: `NeedTakeOffPermission`
    NeedTakeOffPermission,
    /// DCB value: `WaitingForTakeOff`
    WaitingForTakeOff,
    /// DCB value: `NearLandingArea`
    NearLandingArea,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleLandingGearState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleLandingGearState {
    /// DCB value: `Down`
    Down,
    /// DCB value: `Up`
    Up,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleLockedTarget`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleLockedTarget {
    /// DCB value: `None`
    None,
    /// DCB value: `All`
    All,
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `Attacker`
    Attacker,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleMainThrustersState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleMainThrustersState {
    /// DCB value: `Off`
    Off,
    /// DCB value: `On`
    On,
    /// DCB value: `Destroyed`
    Destroyed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleMasterMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleMasterMode {
    /// DCB value: `SCM`
    SCM,
    /// DCB value: `NAV`
    NAV,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleMiningLaserMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleMiningLaserMode {
    /// DCB value: `Fracture`
    Fracture,
    /// DCB value: `Extractor`
    Extractor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehiclePinnedTarget`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehiclePinnedTarget {
    /// DCB value: `None`
    None,
    /// DCB value: `All`
    All,
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `Attacker`
    Attacker,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleQuantumTravelState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleQuantumTravelState {
    /// DCB value: `QT_RouteSet`
    QT_RouteSet,
    /// DCB value: `QT_ReadyToEngage`
    QT_ReadyToEngage,
    /// DCB value: `QT_CanTravel`
    QT_CanTravel,
    /// DCB value: `QT_PartyLeaderRouteSet`
    QT_PartyLeaderRouteSet,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleSalvage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleSalvage {
    /// DCB value: `SupportsScraping`
    SupportsScraping,
    /// DCB value: `SupportsStructural`
    SupportsStructural,
    /// DCB value: `SupportsTractor`
    SupportsTractor,
    /// DCB value: `SupportsFocusHeads`
    SupportsFocusHeads,
    /// DCB value: `SupportsFocusStructural`
    SupportsFocusStructural,
    /// DCB value: `SupportsHeadsBeamSpacing`
    SupportsHeadsBeamSpacing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleScanWaveAvailability`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleScanWaveAvailability {
    /// DCB value: `Available`
    Available,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleSeatTypeState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleSeatTypeState {
    /// DCB value: `SeatToExterior`
    SeatToExterior,
    /// DCB value: `SeatToInterior`
    SeatToInterior,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleState {
    /// DCB value: `Docked`
    Docked,
    /// DCB value: `Landed`
    Landed,
    /// DCB value: `Active`
    Active,
    /// DCB value: `Quantum`
    Quantum,
    /// DCB value: `Static`
    Static,
    /// DCB value: `VolatileCargoExplosionSoon`
    VolatileCargoExplosionSoon,
    /// DCB value: `BombTargetActive`
    BombTargetActive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleSystems`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleSystems {
    /// DCB value: `Power_On`
    Power_On,
    /// DCB value: `DecoupledMode_On`
    DecoupledMode_On,
    /// DCB value: `CruiseControl_On`
    CruiseControl_On,
    /// DCB value: `Missile_Locked`
    Missile_Locked,
    /// DCB value: `Afterburner_Active`
    Afterburner_Active,
    /// DCB value: `SpeedLimiter_On`
    SpeedLimiter_On,
    /// DCB value: `GForceSafety_On`
    GForceSafety_On,
    /// DCB value: `Rotation_Locked`
    Rotation_Locked,
    /// DCB value: `AtMaxSpeed`
    AtMaxSpeed,
    /// DCB value: `AtMaxCruiseSpeed`
    AtMaxCruiseSpeed,
    /// DCB value: `PlayerAcceleration_Active`
    PlayerAcceleration_Active,
    /// DCB value: `Shields_On`
    Shields_On,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleWeaponState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleWeaponState {
    /// DCB value: `WeaponGroup1Set`
    WeaponGroup1Set,
    /// DCB value: `WeaponGroup2Set`
    WeaponGroup2Set,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ControlHintConditionVehicleWeaponSystems`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlHintConditionVehicleWeaponSystems {
    /// DCB value: `PrecisionTargetingEnabled`
    PrecisionTargetingEnabled,
    /// DCB value: `StaggeredFiringEnabled`
    StaggeredFiringEnabled,
    /// DCB value: `LagPipsEnabled`
    LagPipsEnabled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CounterMeasureType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CounterMeasureType {
    /// DCB value: `Flare`
    Flare,
    /// DCB value: `Chaff`
    Chaff,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CoverBodyDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoverBodyDirection {
    /// DCB value: `None`
    None,
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CtxGraph_ContextActionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CtxGraph_ContextActionType {
    /// DCB value: `Load`
    Load,
    /// DCB value: `Unload`
    Unload,
    /// DCB value: `Enter`
    Enter,
    /// DCB value: `Leave`
    Leave,
    /// DCB value: `Unfocus`
    Unfocus,
    /// DCB value: `Focus`
    Focus,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CurrencyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyType {
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `UEC`
    UEC,
    /// DCB value: `REC`
    REC,
    /// DCB value: `ALPHA`
    ALPHA,
    /// DCB value: `MER`
    MER,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Cursor`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cursor {
    /// DCB value: `Auto`
    Auto,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `DollyCamera`
    DollyCamera,
    /// DCB value: `Grabbable`
    Grabbable,
    /// DCB value: `GrabDisabled`
    GrabDisabled,
    /// DCB value: `Grabbed`
    Grabbed,
    /// DCB value: `InteractMode`
    InteractMode,
    /// DCB value: `InteractableZoom`
    InteractableZoom,
    /// DCB value: `Press`
    Press,
    /// DCB value: `SlideHorizontal`
    SlideHorizontal,
    /// DCB value: `SlideVertical`
    SlideVertical,
    /// DCB value: `Standard`
    Standard,
    /// DCB value: `TextEntry`
    TextEntry,
    /// DCB value: `Conversation`
    Conversation,
    /// DCB value: `Throw`
    Throw,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CurveEndPoint`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveEndPoint {
    /// DCB value: `MinOptimal`
    MinOptimal,
    /// DCB value: `MaxOptimal`
    MaxOptimal,
    /// DCB value: `MaxPower`
    MaxPower,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `CurveStartPoint`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurveStartPoint {
    /// DCB value: `MinPower`
    MinPower,
    /// DCB value: `MinOptimal`
    MinOptimal,
    /// DCB value: `MaxOptimal`
    MaxOptimal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DamageToKillType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageToKillType {
    /// DCB value: `none`
    none,
    /// DCB value: `melee`
    melee,
    /// DCB value: `unarmedMelee`
    unarmedMelee,
    /// DCB value: `explosion`
    explosion,
    /// DCB value: `bullet`
    bullet,
    /// DCB value: `suffocate`
    suffocate,
    /// DCB value: `drown`
    drown,
    /// DCB value: `energyWeapon`
    energyWeapon,
    /// DCB value: `collision`
    collision,
    /// DCB value: `fallDamage`
    fallDamage,
    /// DCB value: `crash`
    crash,
    /// DCB value: `suicide`
    suicide,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DamageTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageTypes {
    /// DCB value: `Physical`
    Physical,
    /// DCB value: `Energy`
    Energy,
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `Thermal`
    Thermal,
    /// DCB value: `Biochemical`
    Biochemical,
    /// DCB value: `Stun`
    Stun,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DateScheduleRepeat`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DateScheduleRepeat {
    /// DCB value: `OneOff`
    OneOff,
    /// DCB value: `RepeatDaily`
    RepeatDaily,
    /// DCB value: `RepeatMonthly`
    RepeatMonthly,
    /// DCB value: `RepeatYearly`
    RepeatYearly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DaylightParticleGroupActivation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DaylightParticleGroupActivation {
    /// DCB value: `AlwaysActive`
    AlwaysActive,
    /// DCB value: `ActiveDuringNight`
    ActiveDuringNight,
    /// DCB value: `ActiveDuringDay`
    ActiveDuringDay,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DeathReason`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeathReason {
    /// DCB value: `none`
    none,
    /// DCB value: `instantDeathFromHitDamage`
    instantDeathFromHitDamage,
    /// DCB value: `deathFromTemperature`
    deathFromTemperature,
    /// DCB value: `deathFromHungerOrThirst`
    deathFromHungerOrThirst,
    /// DCB value: `deathFromOverdose`
    deathFromOverdose,
    /// DCB value: `downedFromHitDamage`
    downedFromHitDamage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DeliveryObjectiveType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeliveryObjectiveType {
    /// DCB value: `None`
    None,
    /// DCB value: `Freight_Resource`
    Freight_Resource,
    /// DCB value: `Freight_Entity`
    Freight_Entity,
    /// DCB value: `ItemPort_Entity`
    ItemPort_Entity,
    /// DCB value: `Locker_Entity`
    Locker_Entity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DeviceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceType {
    /// DCB value: `Keyboard`
    Keyboard,
    /// DCB value: `Mouse`
    Mouse,
    /// DCB value: `Gamepad`
    Gamepad,
    /// DCB value: `Joystick`
    Joystick,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DirectForceTypeFilter`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectForceTypeFilter {
    /// DCB value: `Any`
    Any,
    /// DCB value: `ActorBump`
    ActorBump,
    /// DCB value: `Melee`
    Melee,
    /// DCB value: `Physics`
    Physics,
    /// DCB value: `Projectile`
    Projectile,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DirectRenderStage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectRenderStage {
    /// DCB value: `None`
    None,
    /// DCB value: `BeforeToneMapping`
    BeforeToneMapping,
    /// DCB value: `AfterToneMapping`
    AfterToneMapping,
    /// DCB value: `AfterPostProcessing`
    AfterPostProcessing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DisplayCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayCategory {
    /// DCB value: `Critical`
    Critical,
    /// DCB value: `Warning`
    Warning,
    /// DCB value: `MinorWarning`
    MinorWarning,
    /// DCB value: `Normal`
    Normal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `DisturbanceStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisturbanceStyle {
    /// DCB value: `CenteredNoise`
    CenteredNoise,
    /// DCB value: `Figure8`
    Figure8,
    /// DCB value: `Gaussian`
    Gaussian,
    /// DCB value: `Waterfall`
    Waterfall,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAEntityInclusionMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAEntityInclusionMode {
    /// DCB value: `DoNotInclude`
    DoNotInclude,
    /// DCB value: `ReadyToInclude`
    ReadyToInclude,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAGameCompletionAwardType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAGameCompletionAwardType {
    /// DCB value: `WinOrLoss`
    WinOrLoss,
    /// DCB value: `ScoreboardPlacement`
    ScoreboardPlacement,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAGunGameArmorLevels`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAGunGameArmorLevels {
    /// DCB value: `Light`
    Light,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Heavy`
    Heavy,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIAwarenessLevel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIAwarenessLevel {
    /// DCB value: `None`
    None,
    /// DCB value: `Curious`
    Curious,
    /// DCB value: `Threatened`
    Threatened,
    /// DCB value: `Combat`
    Combat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIDisturbanceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIDisturbanceType {
    /// DCB value: `VisualHostile`
    VisualHostile,
    /// DCB value: `VisualDisguisedHostile`
    VisualDisguisedHostile,
    /// DCB value: `VisualBody`
    VisualBody,
    /// DCB value: `VisualDeath`
    VisualDeath,
    /// DCB value: `AudioCombat`
    AudioCombat,
    /// DCB value: `AudioHostile`
    AudioHostile,
    /// DCB value: `AudioDisruption`
    AudioDisruption,
    /// DCB value: `AudioDistraction`
    AudioDistraction,
    /// DCB value: `AreaDisruption`
    AreaDisruption,
    /// DCB value: `Damage`
    Damage,
    /// DCB value: `GroupInvestigation`
    GroupInvestigation,
    /// DCB value: `RadarScan`
    RadarScan,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIMagazineRules`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIMagazineRules {
    /// DCB value: `FollowGamerules`
    FollowGamerules,
    /// DCB value: `FiniteMagazines`
    FiniteMagazines,
    /// DCB value: `InfiniteMagazines`
    InfiniteMagazines,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAINavigationGeneration`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAINavigationGeneration {
    /// DCB value: `Included`
    Included,
    /// DCB value: `Excluded`
    Excluded,
    /// DCB value: `IgnoreWalkability`
    IgnoreWalkability,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIPerceptionAudioType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIPerceptionAudioType {
    /// DCB value: `BulletHit`
    BulletHit,
    /// DCB value: `BulletWhiz`
    BulletWhiz,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `Movement`
    Movement,
    /// DCB value: `Explosion`
    Explosion,
    /// DCB value: `ThrownObject`
    ThrownObject,
    /// DCB value: `Grenade`
    Grenade,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Ragdoll`
    Ragdoll,
    /// DCB value: `Takedown`
    Takedown,
    /// DCB value: `Disruption`
    Disruption,
    /// DCB value: `Custom`
    Custom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIPerceptionBehaviour`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIPerceptionBehaviour {
    /// DCB value: `None`
    None,
    /// DCB value: `TurnHead`
    TurnHead,
    /// DCB value: `TurnBody`
    TurnBody,
    /// DCB value: `Investigate`
    Investigate,
    /// DCB value: `Combat`
    Combat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIPerceptionContext`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIPerceptionContext {
    /// DCB value: `Default`
    Default,
    /// DCB value: `OnSeat`
    OnSeat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAIWeaponShootingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAIWeaponShootingMode {
    /// DCB value: `Single`
    Single,
    /// DCB value: `Burst`
    Burst,
    /// DCB value: `Rapid`
    Rapid,
    /// DCB value: `Charge`
    Charge,
    /// DCB value: `Melee`
    Melee,
    /// DCB value: `TractorBeam`
    TractorBeam,
    /// DCB value: `Repair`
    Repair,
    /// DCB value: `Beam`
    Beam,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EALoadoutSnapshotType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EALoadoutSnapshotType {
    /// DCB value: `EA_ELIMINATION`
    EA_ELIMINATION,
    /// DCB value: `EA_MARINE`
    EA_MARINE,
    /// DCB value: `EA_SLAVER`
    EA_SLAVER,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAMvpType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAMvpType {
    /// DCB value: `None`
    None,
    /// DCB value: `Score`
    Score,
    /// DCB value: `Kills`
    Kills,
    /// DCB value: `Assists`
    Assists,
    /// DCB value: `LeastDeaths`
    LeastDeaths,
    /// DCB value: `FastestLap`
    FastestLap,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAOptionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAOptionType {
    /// DCB value: `GameMode`
    GameMode,
    /// DCB value: `Map`
    Map,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAPickupAudioTrigger`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAPickupAudioTrigger {
    /// DCB value: `PickupAquiredByPlayer`
    PickupAquiredByPlayer,
    /// DCB value: `PickupAquiredByNLPC`
    PickupAquiredByNLPC,
    /// DCB value: `PickupAquireFailedByPlayer`
    PickupAquireFailedByPlayer,
    /// DCB value: `PickupAquireFailedByNLPC`
    PickupAquireFailedByNLPC,
    /// DCB value: `PickupSpawned`
    PickupSpawned,
    /// DCB value: `PickupExpired`
    PickupExpired,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAPickupType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAPickupType {
    /// DCB value: `Ballistics`
    Ballistics,
    /// DCB value: `Missiles`
    Missiles,
    /// DCB value: `Fuel`
    Fuel,
    /// DCB value: `Repair`
    Repair,
    /// DCB value: `KillConfirmed`
    KillConfirmed,
    /// DCB value: `SpecialEvent`
    SpecialEvent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EASOPSpawnState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EASOPSpawnState {
    /// DCB value: `None`
    None,
    /// DCB value: `On`
    On,
    /// DCB value: `Off`
    Off,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EASpawnScreenMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EASpawnScreenMode {
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `InitialSpawn`
    InitialSpawn,
    /// DCB value: `AlwaysActive`
    AlwaysActive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EATransportOnTransitionFinished`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EATransportOnTransitionFinished {
    /// DCB value: `DoNothing`
    DoNothing,
    /// DCB value: `Explode`
    Explode,
    /// DCB value: `Land`
    Land,
    /// DCB value: `LoopNatural`
    LoopNatural,
    /// DCB value: `LoopTeleportToSpawner`
    LoopTeleportToSpawner,
    /// DCB value: `NextTransition`
    NextTransition,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EATransportTransitionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EATransportTransitionType {
    /// DCB value: `Unknown`
    Unknown,
    /// DCB value: `FlySpline`
    FlySpline,
    /// DCB value: `QTravel`
    QTravel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAccumulatorType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAccumulatorType {
    /// DCB value: `Wear`
    Wear,
    /// DCB value: `Dirt`
    Dirt,
    /// DCB value: `Wetness`
    Wetness,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorActionEntityCarryableState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorActionEntityCarryableState {
    /// DCB value: `Undefined`
    Undefined,
    /// DCB value: `Carryable_Settled`
    Carryable_Settled,
    /// DCB value: `Carryable_Carried`
    Carryable_Carried,
    /// DCB value: `Carryable_CarriedAndEquipped`
    Carryable_CarriedAndEquipped,
    /// DCB value: `Carryable_EquippedWorn`
    Carryable_EquippedWorn,
    /// DCB value: `Carryable_CarriedAndInspected`
    Carryable_CarriedAndInspected,
    /// DCB value: `Carryable_Dropped`
    Carryable_Dropped,
    /// DCB value: `Carryable_Stowed`
    Carryable_Stowed,
    /// DCB value: `Carryable_Offered`
    Carryable_Offered,
    /// DCB value: `Carryable_Stored`
    Carryable_Stored,
    /// DCB value: `Carryable_HangingOnOutfitHanger`
    Carryable_HangingOnOutfitHanger,
    /// DCB value: `Special_Gripped`
    Special_Gripped,
    /// DCB value: `Special_Mounted`
    Special_Mounted,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorActionHandlerEventType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorActionHandlerEventType {
    /// DCB value: `Sleep`
    Sleep,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorGForceCameraEffectCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorGForceCameraEffectCategory {
    /// DCB value: `IfcsDefault`
    IfcsDefault,
    /// DCB value: `IfcsBoost`
    IfcsBoost,
    /// DCB value: `JumpDrive`
    JumpDrive,
    /// DCB value: `Legacy`
    Legacy,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorHostedArticulatedEntityProfile`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorHostedArticulatedEntityProfile {
    /// DCB value: `None`
    None,
    /// DCB value: `Default`
    Default,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `ActiveKinematic`
    ActiveKinematic,
    /// DCB value: `ActiveDynamic`
    ActiveDynamic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorLookAheadHeadRollType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorLookAheadHeadRollType {
    /// DCB value: `VehicleHorizonAlignment`
    VehicleHorizonAlignment,
    /// DCB value: `VehicleVelocityRoll`
    VehicleVelocityRoll,
    /// DCB value: `MgvHorizonAlignment`
    MgvHorizonAlignment,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorLookAheadTargetPointType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorLookAheadTargetPointType {
    /// DCB value: `VehicleForward`
    VehicleForward,
    /// DCB value: `VehicleForwardHorizon`
    VehicleForwardHorizon,
    /// DCB value: `VehicleVelocityTranslation`
    VehicleVelocityTranslation,
    /// DCB value: `VehicleVelocityPitchYaw`
    VehicleVelocityPitchYaw,
    /// DCB value: `LockedTarget`
    LockedTarget,
    /// DCB value: `JumpPointSpline`
    JumpPointSpline,
    /// DCB value: `VJoy`
    VJoy,
    /// DCB value: `TurretForward`
    TurretForward,
    /// DCB value: `TurretLockedTarget`
    TurretLockedTarget,
    /// DCB value: `TurretVJoy`
    TurretVJoy,
    /// DCB value: `TurretPitchYaw`
    TurretPitchYaw,
    /// DCB value: `TurretPointerTarget`
    TurretPointerTarget,
    /// DCB value: `LockedTargetPadlock`
    LockedTargetPadlock,
    /// DCB value: `TurretLockedTargetPadlock`
    TurretLockedTargetPadlock,
    /// DCB value: `MgvForward`
    MgvForward,
    /// DCB value: `MgvPitchYaw`
    MgvPitchYaw,
    /// DCB value: `MgvVJoy`
    MgvVJoy,
    /// DCB value: `MgvVLockedTarget`
    MgvVLockedTarget,
    /// DCB value: `MgvLockedTargetPadlock`
    MgvLockedTargetPadlock,
    /// DCB value: `QuantumBoostTarget`
    QuantumBoostTarget,
    /// DCB value: `AdsPadlock`
    AdsPadlock,
    /// DCB value: `AdsCrosshair`
    AdsCrosshair,
    /// DCB value: `CustomPoint`
    CustomPoint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorPhysicalizationProfile`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorPhysicalizationProfile {
    /// DCB value: `None`
    None,
    /// DCB value: `Default`
    Default,
    /// DCB value: `Default_PoseMatch`
    Default_PoseMatch,
    /// DCB value: `FloppyRagdoll`
    FloppyRagdoll,
    /// DCB value: `DrivenRagdoll`
    DrivenRagdoll,
    /// DCB value: `NewRagdoll`
    NewRagdoll,
    /// DCB value: `PassiveRagdoll`
    PassiveRagdoll,
    /// DCB value: `Linked`
    Linked,
    /// DCB value: `Flying`
    Flying,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorStanceUpAlignMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorStanceUpAlignMode {
    /// DCB value: `Auto`
    Auto,
    /// DCB value: `OnlyAlignToSurfaceNormal`
    OnlyAlignToSurfaceNormal,
    /// DCB value: `OnlyAlignToGravity`
    OnlyAlignToGravity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorStateFilterByAimingRestriction`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorStateFilterByAimingRestriction {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `Kiosk`
    Kiosk,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorStateFilterByBoolState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorStateFilterByBoolState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `True`
    True,
    /// DCB value: `False`
    False,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorStateFilterByPlayerCamera`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorStateFilterByPlayerCamera {
    /// DCB value: `Any`
    Any,
    /// DCB value: `FirstPerson`
    FirstPerson,
    /// DCB value: `ThirdPerson`
    ThirdPerson,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EActorType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EActorType {
    /// DCB value: `Regular`
    Regular,
    /// DCB value: `PlayerCorpse`
    PlayerCorpse,
    /// DCB value: `DebugGhost`
    DebugGhost,
    /// DCB value: `Transport`
    Transport,
    /// DCB value: `DeadBody`
    DeadBody,
    /// DCB value: `PlayerShadow`
    PlayerShadow,
    /// DCB value: `Creature`
    Creature,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAimFireDetectionMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAimFireDetectionMode {
    /// DCB value: `Temperature`
    Temperature,
    /// DCB value: `Radius`
    Radius,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAimableAimType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAimableAimType {
    /// DCB value: `PipAiming`
    PipAiming,
    /// DCB value: `TargetPainting`
    TargetPainting,
    /// DCB value: `TargetAuto`
    TargetAuto,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAimableGimbalState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAimableGimbalState {
    /// DCB value: `Fixed`
    Fixed,
    /// DCB value: `Unlocked`
    Unlocked,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAimableUser`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAimableUser {
    /// DCB value: `Pilot`
    Pilot,
    /// DCB value: `MannedTurret`
    MannedTurret,
    /// DCB value: `RemoteTurret`
    RemoteTurret,
    /// DCB value: `Seat`
    Seat,
    /// DCB value: `GroundVehicleDriver`
    GroundVehicleDriver,
    /// DCB value: `BoatDriver`
    BoatDriver,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAmmoContainerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAmmoContainerType {
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Medical`
    Medical,
    /// DCB value: `Salvage`
    Salvage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAnnouncementPriority`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAnnouncementPriority {
    /// DCB value: `None`
    None,
    /// DCB value: `SkipQueue`
    SkipQueue,
    /// DCB value: `PlayNow`
    PlayNow,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAnnouncerGameTokenType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAnnouncerGameTokenType {
    /// DCB value: `None`
    None,
    /// DCB value: `GameMode`
    GameMode,
    /// DCB value: `Team`
    Team,
    /// DCB value: `Level`
    Level,
    /// DCB value: `Phase`
    Phase,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EApplicationFormBodyTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EApplicationFormBodyTypes {
    /// DCB value: `Male`
    Male,
    /// DCB value: `Female`
    Female,
    /// DCB value: `PreferToSelfDescribe`
    PreferToSelfDescribe,
    /// DCB value: `PreferNotToSay`
    PreferNotToSay,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioBreathEvents`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioBreathEvents {
    /// DCB value: `BreathIn`
    BreathIn,
    /// DCB value: `BreathOut`
    BreathOut,
    /// DCB value: `BreathingStarted`
    BreathingStarted,
    /// DCB value: `BreathingStopped`
    BreathingStopped,
    /// DCB value: `BreathHold`
    BreathHold,
    /// DCB value: `BreathRelease`
    BreathRelease,
    /// DCB value: `BreathCustom`
    BreathCustom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioBreathParams`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioBreathParams {
    /// DCB value: `BreathDuration`
    BreathDuration,
    /// DCB value: `BreathVolume`
    BreathVolume,
    /// DCB value: `BreathAirSpeed`
    BreathAirSpeed,
    /// DCB value: `BloodOxygen`
    BloodOxygen,
    /// DCB value: `BlackOut`
    BlackOut,
    /// DCB value: `Health`
    Health,
    /// DCB value: `Stamina`
    Stamina,
    /// DCB value: `BreathOxygenLevel`
    BreathOxygenLevel,
    /// DCB value: `Exertion`
    Exertion,
    /// DCB value: `Recovery`
    Recovery,
    /// DCB value: `BodyTemperature`
    BodyTemperature,
    /// DCB value: `SuitTemperature`
    SuitTemperature,
    /// DCB value: `ApparentTemperature`
    ApparentTemperature,
    /// DCB value: `BreathHeldRatio`
    BreathHeldRatio,
    /// DCB value: `LungFullness`
    LungFullness,
    /// DCB value: `BreathInOut`
    BreathInOut,
    /// DCB value: `GForce`
    GForce,
    /// DCB value: `GForceStress`
    GForceStress,
    /// DCB value: `GForcePassout`
    GForcePassout,
    /// DCB value: `StyleActiveTime`
    StyleActiveTime,
    /// DCB value: `CustomParameter`
    CustomParameter,
    /// DCB value: `TorsoWear`
    TorsoWear,
    /// DCB value: `ArmsLock`
    ArmsLock,
    /// DCB value: `IsInEVA`
    IsInEVA,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioControllerEntityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioControllerEntityType {
    /// DCB value: `None`
    None,
    /// DCB value: `Actor`
    Actor,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Communication`
    Communication,
    /// DCB value: `TransitCarriage`
    TransitCarriage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioEnvironmentFeedbackMovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioEnvironmentFeedbackMovementType {
    /// DCB value: `EnvironmentMovement_X`
    EnvironmentMovement_X,
    /// DCB value: `EnvironmentMovement_Y`
    EnvironmentMovement_Y,
    /// DCB value: `EnvironmentMovement_Z`
    EnvironmentMovement_Z,
    /// DCB value: `EnvironmentMovement_Magnitude`
    EnvironmentMovement_Magnitude,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioGameContextType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioGameContextType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `ActorDefault`
    ActorDefault,
    /// DCB value: `ActorEVA`
    ActorEVA,
    /// DCB value: `VehicleDriver`
    VehicleDriver,
    /// DCB value: `VehicleCrew`
    VehicleCrew,
    /// DCB value: `RemoteTurret`
    RemoteTurret,
    /// DCB value: `Dead`
    Dead,
    /// DCB value: `Cutscene`
    Cutscene,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioIFCSOutputData`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioIFCSOutputData {
    /// DCB value: `IFCS_Update_Mode`
    IFCS_Update_Mode,
    /// DCB value: `Ship_Aerodynamics_DragMagnitude`
    Ship_Aerodynamics_DragMagnitude,
    /// DCB value: `Ship_Aerodynamics_LiftMagnitude`
    Ship_Aerodynamics_LiftMagnitude,
    /// DCB value: `Ship_AfterburnerCommanded`
    Ship_AfterburnerCommanded,
    /// DCB value: `Ship_AfterburnerEnabled`
    Ship_AfterburnerEnabled,
    /// DCB value: `Ship_AfterburnerEnabledScaled`
    Ship_AfterburnerEnabledScaled,
    /// DCB value: `Ship_AfterburnerRampUpRatio`
    Ship_AfterburnerRampUpRatio,
    /// DCB value: `Ship_AfterburnerCapacityRatio`
    Ship_AfterburnerCapacityRatio,
    /// DCB value: `Ship_GSafe_Enabled`
    Ship_GSafe_Enabled,
    /// DCB value: `Ship_Coupled`
    Ship_Coupled,
    /// DCB value: `Ship_FuelRatio`
    Ship_FuelRatio,
    /// DCB value: `Ship_HoverMode`
    Ship_HoverMode,
    /// DCB value: `Ship_HoverModeAnimationPosition`
    Ship_HoverModeAnimationPosition,
    /// DCB value: `Ship_HoverDriftForwardBackward`
    Ship_HoverDriftForwardBackward,
    /// DCB value: `Ship_HoverDriftLeftRight`
    Ship_HoverDriftLeftRight,
    /// DCB value: `Ship_HoverDriftMagnitude`
    Ship_HoverDriftMagnitude,
    /// DCB value: `Ship_Planet_Altitude`
    Ship_Planet_Altitude,
    /// DCB value: `Ship_Planet_AtmosphericDensity`
    Ship_Planet_AtmosphericDensity,
    /// DCB value: `Ship_Planet_HeightAboveGround`
    Ship_Planet_HeightAboveGround,
    /// DCB value: `Ship_Atmospheric_Mach`
    Ship_Atmospheric_Mach,
    /// DCB value: `Ship_Atmospheric_StagnationTemperature`
    Ship_Atmospheric_StagnationTemperature,
    /// DCB value: `Ship_Atmospheric_StagnationTemperatureFlareStarted`
    Ship_Atmospheric_StagnationTemperatureFlareStarted,
    /// DCB value: `Ship_Atmospheric_StagnationTemperatureNormalized`
    Ship_Atmospheric_StagnationTemperatureNormalized,
    /// DCB value: `Ship_Atmospheric_Wind_Speed_Rotational_Non_Normalised`
    Ship_Atmospheric_Wind_Speed_Rotational_Non_Normalised,
    /// DCB value: `Ship_Atmospheric_Wind_Speed_Translational_Non_Normalised`
    Ship_Atmospheric_Wind_Speed_Translational_Non_Normalised,
    /// DCB value: `Ship_Translation_Speed`
    Ship_Translation_Speed,
    /// DCB value: `Ship_Translation_Speed_NonNormalized`
    Ship_Translation_Speed_NonNormalized,
    /// DCB value: `Ship_Translation_Acceleration_MaxAll`
    Ship_Translation_Acceleration_MaxAll,
    /// DCB value: `Ship_Translation_Acceleration_Forward`
    Ship_Translation_Acceleration_Forward,
    /// DCB value: `Ship_Translation_Acceleration_Backward`
    Ship_Translation_Acceleration_Backward,
    /// DCB value: `Ship_Translation_Acceleration_ForwardBackward`
    Ship_Translation_Acceleration_ForwardBackward,
    /// DCB value: `Ship_Translation_Acceleration_UpDown`
    Ship_Translation_Acceleration_UpDown,
    /// DCB value: `Ship_Translation_Acceleration_UpDownSigned`
    Ship_Translation_Acceleration_UpDownSigned,
    /// DCB value: `Ship_Translation_Acceleration_LeftRight`
    Ship_Translation_Acceleration_LeftRight,
    /// DCB value: `Ship_Translation_Acceleration_LeftRightSigned`
    Ship_Translation_Acceleration_LeftRightSigned,
    /// DCB value: `Ship_Translation_Acceleration_MaxLeftRightUpDown`
    Ship_Translation_Acceleration_MaxLeftRightUpDown,
    /// DCB value: `Ship_Translation_GSafe_Strength`
    Ship_Translation_GSafe_Strength,
    /// DCB value: `Ship_Translation_InCruiseRange`
    Ship_Translation_InCruiseRange,
    /// DCB value: `Ship_Rotation_Speed`
    Ship_Rotation_Speed,
    /// DCB value: `Ship_Rotation_Speed_NonNormalized`
    Ship_Rotation_Speed_NonNormalized,
    /// DCB value: `Ship_Rotation_Speed_Pitch`
    Ship_Rotation_Speed_Pitch,
    /// DCB value: `Ship_Rotation_Speed_Pitch_Signed`
    Ship_Rotation_Speed_Pitch_Signed,
    /// DCB value: `Ship_Rotation_Speed_Yaw`
    Ship_Rotation_Speed_Yaw,
    /// DCB value: `Ship_Rotation_Speed_Yaw_Signed`
    Ship_Rotation_Speed_Yaw_Signed,
    /// DCB value: `Ship_Rotation_Speed_Roll`
    Ship_Rotation_Speed_Roll,
    /// DCB value: `Ship_Rotation_Speed_Roll_Signed`
    Ship_Rotation_Speed_Roll_Signed,
    /// DCB value: `Ship_Rotation_Speed_MaxAll`
    Ship_Rotation_Speed_MaxAll,
    /// DCB value: `Ship_Rotation_Speed_MaxPitchYaw`
    Ship_Rotation_Speed_MaxPitchYaw,
    /// DCB value: `Ship_Rotation_InputDivergence`
    Ship_Rotation_InputDivergence,
    /// DCB value: `Ship_Rotation_InputDivergence_Pitch`
    Ship_Rotation_InputDivergence_Pitch,
    /// DCB value: `Ship_Rotation_InputDivergence_Yaw`
    Ship_Rotation_InputDivergence_Yaw,
    /// DCB value: `Ship_Rotation_InputDivergence_Roll`
    Ship_Rotation_InputDivergence_Roll,
    /// DCB value: `Ship_Rotation_InputDivergence_MaxAll`
    Ship_Rotation_InputDivergence_MaxAll,
    /// DCB value: `Ship_Rotation_InputDivergence_MaxPitchYaw`
    Ship_Rotation_InputDivergence_MaxPitchYaw,
    /// DCB value: `Gravlev_Compression_One`
    Gravlev_Compression_One,
    /// DCB value: `Gravlev_Compression_Two`
    Gravlev_Compression_Two,
    /// DCB value: `Gravlev_Compression_Three`
    Gravlev_Compression_Three,
    /// DCB value: `Gravlev_Compression_Four`
    Gravlev_Compression_Four,
    /// DCB value: `Gravlev_Compression_All`
    Gravlev_Compression_All,
    /// DCB value: `Gravlev_Compression_Average`
    Gravlev_Compression_Average,
    /// DCB value: `Gravlev_Compression_Max`
    Gravlev_Compression_Max,
    /// DCB value: `Gravlev_Compression_Normalized_One`
    Gravlev_Compression_Normalized_One,
    /// DCB value: `Gravlev_Compression_Normalized_Two`
    Gravlev_Compression_Normalized_Two,
    /// DCB value: `Gravlev_Compression_Normalized_Three`
    Gravlev_Compression_Normalized_Three,
    /// DCB value: `Gravlev_Compression_Normalized_Four`
    Gravlev_Compression_Normalized_Four,
    /// DCB value: `Gravlev_Compression_Normalized_All`
    Gravlev_Compression_Normalized_All,
    /// DCB value: `Gravlev_Compression_Normalized_Average`
    Gravlev_Compression_Normalized_Average,
    /// DCB value: `Gravlev_Compression_Normalized_Max`
    Gravlev_Compression_Normalized_Max,
    /// DCB value: `Gravlev_Enabled`
    Gravlev_Enabled,
    /// DCB value: `Ship_Linear_VelocityLengthNormalizedLS`
    Ship_Linear_VelocityLengthNormalizedLS,
    /// DCB value: `Ship_Linear_VelocityForwardBackwardNormalizedLS`
    Ship_Linear_VelocityForwardBackwardNormalizedLS,
    /// DCB value: `Ship_Linear_VelocityRightLeftNormalizedLS`
    Ship_Linear_VelocityRightLeftNormalizedLS,
    /// DCB value: `Ship_Linear_VelocityUpDownNormalizedLS`
    Ship_Linear_VelocityUpDownNormalizedLS,
    /// DCB value: `Ship_Linear_VelocityGoalLengthNormalizedLS`
    Ship_Linear_VelocityGoalLengthNormalizedLS,
    /// DCB value: `Ship_Angular_AccelerationLengthLS`
    Ship_Angular_AccelerationLengthLS,
    /// DCB value: `Ship_Linear_RealAccelerationLS`
    Ship_Linear_RealAccelerationLS,
    /// DCB value: `Ship_MaxSpeedNav`
    Ship_MaxSpeedNav,
    /// DCB value: `Ship_AtmosphericDensity`
    Ship_AtmosphericDensity,
    /// DCB value: `Ship_IsInControlSurfaceMode`
    Ship_IsInControlSurfaceMode,
    /// DCB value: `Ship_IsLanded`
    Ship_IsLanded,
    /// DCB value: `Ship_IsGearDown`
    Ship_IsGearDown,
    /// DCB value: `Ship_IsMainThrustersOn`
    Ship_IsMainThrustersOn,
    /// DCB value: `Ship_RawInputRotation`
    Ship_RawInputRotation,
    /// DCB value: `Ship_NavModeEnabled`
    Ship_NavModeEnabled,
    /// DCB value: `Ship_ThrusterDisconnectActive`
    Ship_ThrusterDisconnectActive,
    /// DCB value: `Ship_ThrusterControlSurfaceMix`
    Ship_ThrusterControlSurfaceMix,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioTriggerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioTriggerType {
    /// DCB value: `None`
    None,
    /// DCB value: `WeaponFire`
    WeaponFire,
    /// DCB value: `EntityEffect`
    EntityEffect,
    /// DCB value: `Footsteps`
    Footsteps,
    /// DCB value: `Foley`
    Foley,
    /// DCB value: `ProceduralClip`
    ProceduralClip,
    /// DCB value: `Thruster`
    Thruster,
    /// DCB value: `Dialogue`
    Dialogue,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAudioValueOutputCameraInputs`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAudioValueOutputCameraInputs {
    /// DCB value: `EAVOCI_None`
    EAVOCI_None,
    /// DCB value: `EAVOCI_Snapshot`
    EAVOCI_Snapshot,
    /// DCB value: `EAVOCI_Average`
    EAVOCI_Average,
    /// DCB value: `EAVOCI_AverageRooted`
    EAVOCI_AverageRooted,
    /// DCB value: `EAVOCI_Max`
    EAVOCI_Max,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAutoFillType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAutoFillType {
    /// DCB value: `None`
    None,
    /// DCB value: `Balance`
    Balance,
    /// DCB value: `TopToBottom`
    TopToBottom,
    /// DCB value: `BottomToTop`
    BottomToTop,
    /// DCB value: `SmartBalancing`
    SmartBalancing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAuxiliaryProxy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAuxiliaryProxy {
    /// DCB value: `Head_Proxies`
    Head_Proxies,
    /// DCB value: `Neck_Proxies`
    Neck_Proxies,
    /// DCB value: `Spine_1_Proxies`
    Spine_1_Proxies,
    /// DCB value: `Spine_2_Proxies`
    Spine_2_Proxies,
    /// DCB value: `Spine_3_Proxies`
    Spine_3_Proxies,
    /// DCB value: `Hips_Proxies`
    Hips_Proxies,
    /// DCB value: `L_Shoulder_Proxie`
    L_Shoulder_Proxie,
    /// DCB value: `L_Arm_Proxies`
    L_Arm_Proxies,
    /// DCB value: `L_ForeArm_Proxies`
    L_ForeArm_Proxies,
    /// DCB value: `L_Hand_Proxies`
    L_Hand_Proxies,
    /// DCB value: `L_Thigh_Proxies`
    L_Thigh_Proxies,
    /// DCB value: `L_Knee_Proxies`
    L_Knee_Proxies,
    /// DCB value: `L_Foot_Proxies`
    L_Foot_Proxies,
    /// DCB value: `L_Toe_Proxies`
    L_Toe_Proxies,
    /// DCB value: `R_Shoulder_Proxie`
    R_Shoulder_Proxie,
    /// DCB value: `R_Arm_Proxies`
    R_Arm_Proxies,
    /// DCB value: `R_ForeArm_Proxies`
    R_ForeArm_Proxies,
    /// DCB value: `R_Hand_Proxies`
    R_Hand_Proxies,
    /// DCB value: `R_Thigh_Proxies`
    R_Thigh_Proxies,
    /// DCB value: `R_Knee_Proxies`
    R_Knee_Proxies,
    /// DCB value: `R_Foot_Proxies`
    R_Foot_Proxies,
    /// DCB value: `R_Toe_Proxies`
    R_Toe_Proxies,
    /// DCB value: `Helper_01_Proxies`
    Helper_01_Proxies,
    /// DCB value: `Helper_02_Proxies`
    Helper_02_Proxies,
    /// DCB value: `Helper_03_Proxies`
    Helper_03_Proxies,
    /// DCB value: `Helper_04_Proxies`
    Helper_04_Proxies,
    /// DCB value: `Helper_05_Proxies`
    Helper_05_Proxies,
    /// DCB value: `Helper_06_Proxies`
    Helper_06_Proxies,
    /// DCB value: `Helper_07_Proxies`
    Helper_07_Proxies,
    /// DCB value: `Helper_08_Proxies`
    Helper_08_Proxies,
    /// DCB value: `Helper_09_Proxies`
    Helper_09_Proxies,
    /// DCB value: `Helper_10_Proxies`
    Helper_10_Proxies,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EAwardId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EAwardId {
    /// DCB value: `None`
    None,
    /// DCB value: `VanduulSwarm_Win_Legacy`
    VanduulSwarm_Win_Legacy,
    /// DCB value: `VanduulSwarm_Win`
    VanduulSwarm_Win,
    /// DCB value: `PirateSwarm_Win`
    PirateSwarm_Win,
    /// DCB value: `PirateSwarm_Speedrun`
    PirateSwarm_Speedrun,
    /// DCB value: `PirateSwarm_SpecialCondition`
    PirateSwarm_SpecialCondition,
    /// DCB value: `Tank_Win`
    Tank_Win,
    /// DCB value: `GunGame_Win`
    GunGame_Win,
    /// DCB value: `ExperimentalMode_Played`
    ExperimentalMode_Played,
    /// DCB value: `Killed_Developer`
    Killed_Developer,
    /// DCB value: `PvE_Deathless`
    PvE_Deathless,
    /// DCB value: `Xmas_Win`
    Xmas_Win,
    /// DCB value: `LunarNewYear_Win`
    LunarNewYear_Win,
    /// DCB value: `Valentines_Win`
    Valentines_Win,
    /// DCB value: `Halloween_Win`
    Halloween_Win,
    /// DCB value: `Halloween_Win_Tier2`
    Halloween_Win_Tier2,
    /// DCB value: `Halloween_Win_Tier3`
    Halloween_Win_Tier3,
    /// DCB value: `SC_GameMasterEventItem`
    SC_GameMasterEventItem,
    /// DCB value: `SC_LootableEventItem`
    SC_LootableEventItem,
    /// DCB value: `StPatricks_Win`
    StPatricks_Win,
    /// DCB value: `WelcomeToPyro_Chapter_One`
    WelcomeToPyro_Chapter_One,
    /// DCB value: `WelcomeToPyro_Chapter_Two`
    WelcomeToPyro_Chapter_Two,
    /// DCB value: `WelcomeToPyro_Chapter_Two_VIG`
    WelcomeToPyro_Chapter_Two_VIG,
    /// DCB value: `WelcomeToPyro_Chapter_Two_CFP`
    WelcomeToPyro_Chapter_Two_CFP,
    /// DCB value: `WelcomeToPyro_Chapter_Two_HH`
    WelcomeToPyro_Chapter_Two_HH,
    /// DCB value: `WelcomeToPyro_Firesale`
    WelcomeToPyro_Firesale,
    /// DCB value: `WelcomeToPyro_MiningRush`
    WelcomeToPyro_MiningRush,
    /// DCB value: `WelcomeToPyro_SalvageRush`
    WelcomeToPyro_SalvageRush,
    /// DCB value: `WelcomeToPyro_FiresaleCombo`
    WelcomeToPyro_FiresaleCombo,
    /// DCB value: `Training_UEE`
    Training_UEE,
    /// DCB value: `HuntingThePolaris`
    HuntingThePolaris,
    /// DCB value: `SC_BDay11_Gold`
    SC_BDay11_Gold,
    /// DCB value: `SC_BDay11_Platinum`
    SC_BDay11_Platinum,
    /// DCB value: `RT_GG_HRST_T1`
    RT_GG_HRST_T1,
    /// DCB value: `RT_GG_CRUS_T1`
    RT_GG_CRUS_T1,
    /// DCB value: `RT_GG_ARCC_T1`
    RT_GG_ARCC_T1,
    /// DCB value: `RT_GG_MITE_T1`
    RT_GG_MITE_T1,
    /// DCB value: `RT_GG_HRST_T2`
    RT_GG_HRST_T2,
    /// DCB value: `RT_GG_CRUS_T2`
    RT_GG_CRUS_T2,
    /// DCB value: `RT_GG_ARCC_T2`
    RT_GG_ARCC_T2,
    /// DCB value: `RT_GG_MITE_T2`
    RT_GG_MITE_T2,
    /// DCB value: `RT_GG_HRST_T3`
    RT_GG_HRST_T3,
    /// DCB value: `RT_GG_CRUS_T3`
    RT_GG_CRUS_T3,
    /// DCB value: `RT_GG_ARCC_T3`
    RT_GG_ARCC_T3,
    /// DCB value: `RT_GG_MITE_T3`
    RT_GG_MITE_T3,
    /// DCB value: `RT_GG_HRST_T4`
    RT_GG_HRST_T4,
    /// DCB value: `RT_GG_CRUS_T4`
    RT_GG_CRUS_T4,
    /// DCB value: `RT_GG_ARCC_T4`
    RT_GG_ARCC_T4,
    /// DCB value: `RT_GG_MITE_T4`
    RT_GG_MITE_T4,
    /// DCB value: `RT_GG_HRST_T5`
    RT_GG_HRST_T5,
    /// DCB value: `RT_GG_CRUS_T5`
    RT_GG_CRUS_T5,
    /// DCB value: `RT_GG_ARCC_T5`
    RT_GG_ARCC_T5,
    /// DCB value: `RT_GG_MITE_T5`
    RT_GG_MITE_T5,
    /// DCB value: `RT_CVH_CFP_1`
    RT_CVH_CFP_1,
    /// DCB value: `RT_CVH_CFP_2`
    RT_CVH_CFP_2,
    /// DCB value: `RT_CVH_CFP_3`
    RT_CVH_CFP_3,
    /// DCB value: `RT_CVH_HH_1_Awarded`
    RT_CVH_HH_1_Awarded,
    /// DCB value: `RT_CVH_HH_2_Awarded`
    RT_CVH_HH_2_Awarded,
    /// DCB value: `RT_CVH_HH_3_Awarded`
    RT_CVH_HH_3_Awarded,
    /// DCB value: `RT_CVH_HH_1`
    RT_CVH_HH_1,
    /// DCB value: `RT_CVH_HH_2`
    RT_CVH_HH_2,
    /// DCB value: `RT_CVH_HH_3`
    RT_CVH_HH_3,
    /// DCB value: `RT_CVH_CFP_1_Awarded`
    RT_CVH_CFP_1_Awarded,
    /// DCB value: `RT_CVH_CFP_2_Awarded`
    RT_CVH_CFP_2_Awarded,
    /// DCB value: `RT_CVH_CFP_3_Awarded`
    RT_CVH_CFP_3_Awarded,
    /// DCB value: `R_PU_CA_OP_1`
    R_PU_CA_OP_1,
    /// DCB value: `R_PU_CA_OP_2`
    R_PU_CA_OP_2,
    /// DCB value: `R_PU_CA_OP_3`
    R_PU_CA_OP_3,
    /// DCB value: `R_PU_CA_OP_4`
    R_PU_CA_OP_4,
    /// DCB value: `R_PU_CA_TRANSPORT_1`
    R_PU_CA_TRANSPORT_1,
    /// DCB value: `R_PU_CA_TRANSPORT_2`
    R_PU_CA_TRANSPORT_2,
    /// DCB value: `R_PU_CA_TRANSPORT_3`
    R_PU_CA_TRANSPORT_3,
    /// DCB value: `R_PU_CA_TRANSPORT_4`
    R_PU_CA_TRANSPORT_4,
    /// DCB value: `R_PU_CA_DEFENSE_1`
    R_PU_CA_DEFENSE_1,
    /// DCB value: `R_PU_CA_DEFENSE_2`
    R_PU_CA_DEFENSE_2,
    /// DCB value: `R_PU_CA_DEFENSE_3`
    R_PU_CA_DEFENSE_3,
    /// DCB value: `R_PU_CA_DEFENSE_4`
    R_PU_CA_DEFENSE_4,
    /// DCB value: `R_PU_CA_COLLECTION_1`
    R_PU_CA_COLLECTION_1,
    /// DCB value: `R_PU_CA_COLLECTION_2`
    R_PU_CA_COLLECTION_2,
    /// DCB value: `R_PU_CA_COLLECTION_3`
    R_PU_CA_COLLECTION_3,
    /// DCB value: `R_PU_CA_COLLECTION_4`
    R_PU_CA_COLLECTION_4,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EBuildModeSubMenu`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EBuildModeSubMenu {
    /// DCB value: `LandClaim`
    LandClaim,
    /// DCB value: `NewStructure`
    NewStructure,
    /// DCB value: `ExistingStructure`
    ExistingStructure,
    /// DCB value: `ResourceManagement`
    ResourceManagement,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EBuildingBlocksFlattenBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EBuildingBlocksFlattenBehavior {
    /// DCB value: `None`
    None,
    /// DCB value: `FlattenAs3D`
    FlattenAs3D,
    /// DCB value: `FlattenAs2D`
    FlattenAs2D,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECIGTestA`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECIGTestA {
    /// DCB value: `Item_A`
    Item_A,
    /// DCB value: `Item_B`
    Item_B,
    /// DCB value: `Item_C`
    Item_C,
    /// DCB value: `Item_D`
    Item_D,
    /// DCB value: `Item_E`
    Item_E,
    /// DCB value: `Item_F`
    Item_F,
    /// DCB value: `Item_G`
    Item_G,
    /// DCB value: `Item_H`
    Item_H,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECameraTransitionRelativeTo`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECameraTransitionRelativeTo {
    /// DCB value: `Start`
    Start,
    /// DCB value: `End`
    End,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECameraViewTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECameraViewTypes {
    /// DCB value: `Simple3P`
    Simple3P,
    /// DCB value: `Static`
    Static,
    /// DCB value: `StaticFixedSpectator`
    StaticFixedSpectator,
    /// DCB value: `ThirdPersonBase`
    ThirdPersonBase,
    /// DCB value: `Orbit`
    Orbit,
    /// DCB value: `OrbitEntity`
    OrbitEntity,
    /// DCB value: `OrbitEntityCinematic`
    OrbitEntityCinematic,
    /// DCB value: `OrbitPassenger`
    OrbitPassenger,
    /// DCB value: `OrbitPlayer`
    OrbitPlayer,
    /// DCB value: `OrbitPoint`
    OrbitPoint,
    /// DCB value: `OrbitVehicle`
    OrbitVehicle,
    /// DCB value: `OrbitSCItemSeat`
    OrbitSCItemSeat,
    /// DCB value: `OrbitPassengerSCItemSeat`
    OrbitPassengerSCItemSeat,
    /// DCB value: `OrbitSpectatorFollow`
    OrbitSpectatorFollow,
    /// DCB value: `OrbitSpectatorFollowVehicle`
    OrbitSpectatorFollowVehicle,
    /// DCB value: `ChaseVehicle`
    ChaseVehicle,
    /// DCB value: `ChaseSCItemSeat`
    ChaseSCItemSeat,
    /// DCB value: `SeatCockpit`
    SeatCockpit,
    /// DCB value: `LookBehind`
    LookBehind,
    /// DCB value: `LookVehicleItem`
    LookVehicleItem,
    /// DCB value: `FreeCam`
    FreeCam,
    /// DCB value: `TimeCam`
    TimeCam,
    /// DCB value: `FPSDeathCam`
    FPSDeathCam,
    /// DCB value: `CinematicTwoShipsFrame`
    CinematicTwoShipsFrame,
    /// DCB value: `FirstPersonSpectator`
    FirstPersonSpectator,
    /// DCB value: `RemoteTurret`
    RemoteTurret,
    /// DCB value: `FirstPersonBase`
    FirstPersonBase,
    /// DCB value: `KillerDeathCam`
    KillerDeathCam,
    /// DCB value: `PlayerInventory`
    PlayerInventory,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECarryableDefaultInteractions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECarryableDefaultInteractions {
    /// DCB value: `Carry`
    Carry,
    /// DCB value: `Place`
    Place,
    /// DCB value: `Drop`
    Drop,
    /// DCB value: `EquipToItemport`
    EquipToItemport,
    /// DCB value: `Store`
    Store,
    /// DCB value: `HoldReady`
    HoldReady,
    /// DCB value: `EquipWearable`
    EquipWearable,
    /// DCB value: `Inspect`
    Inspect,
    /// DCB value: `Flip`
    Flip,
    /// DCB value: `SwapAttachments`
    SwapAttachments,
    /// DCB value: `AttachToHeldItem`
    AttachToHeldItem,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECarryableSequenceActions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECarryableSequenceActions {
    /// DCB value: `Undefined`
    Undefined,
    /// DCB value: `BespokeTake`
    BespokeTake,
    /// DCB value: `BespokePlace`
    BespokePlace,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECarryableState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECarryableState {
    /// DCB value: `eCS_Dropped`
    eCS_Dropped,
    /// DCB value: `eCS_Carried`
    eCS_Carried,
    /// DCB value: `eCS_Settled`
    eCS_Settled,
    /// DCB value: `eCS_CarriedAndEquipped`
    eCS_CarriedAndEquipped,
    /// DCB value: `eCS_Stowed`
    eCS_Stowed,
    /// DCB value: `eCS_Offered`
    eCS_Offered,
    /// DCB value: `eCS_CarriedAndInspected`
    eCS_CarriedAndInspected,
    /// DCB value: `eCS_EquippedWorn`
    eCS_EquippedWorn,
    /// DCB value: `eCS_Stored`
    eCS_Stored,
    /// DCB value: `eCS_HangingOnOutfitHanger`
    eCS_HangingOnOutfitHanger,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterCustomizerDNARegion`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterCustomizerDNARegion {
    /// DCB value: `LeftBrow`
    LeftBrow,
    /// DCB value: `RightBrow`
    RightBrow,
    /// DCB value: `LeftEye`
    LeftEye,
    /// DCB value: `RightEye`
    RightEye,
    /// DCB value: `Nose`
    Nose,
    /// DCB value: `LeftEar`
    LeftEar,
    /// DCB value: `RightEar`
    RightEar,
    /// DCB value: `LeftCheek`
    LeftCheek,
    /// DCB value: `RightCheek`
    RightCheek,
    /// DCB value: `Mouth`
    Mouth,
    /// DCB value: `Jawline`
    Jawline,
    /// DCB value: `Crown`
    Crown,
    /// DCB value: `Neck`
    Neck,
    /// DCB value: `Invalid`
    Invalid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterCustomizerFeature`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterCustomizerFeature {
    /// DCB value: `Skin`
    Skin,
    /// DCB value: `Complexion`
    Complexion,
    /// DCB value: `Hair`
    Hair,
    /// DCB value: `FacialHair`
    FacialHair,
    /// DCB value: `Eyebrows`
    Eyebrows,
    /// DCB value: `HairDye`
    HairDye,
    /// DCB value: `Eyes`
    Eyes,
    /// DCB value: `Makeup`
    Makeup,
    /// DCB value: `Tattoos`
    Tattoos,
    /// DCB value: `BodySkin`
    BodySkin,
    /// DCB value: `Invalid`
    Invalid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterCustomizerItemSelectMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterCustomizerItemSelectMode {
    /// DCB value: `HeadSelection`
    HeadSelection,
    /// DCB value: `BindingsValue`
    BindingsValue,
    /// DCB value: `Toggle`
    Toggle,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterCustomizerTextureSelectSlot`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterCustomizerTextureSelectSlot {
    /// DCB value: `MakeupSlot1`
    MakeupSlot1,
    /// DCB value: `MakeupSlot2`
    MakeupSlot2,
    /// DCB value: `MakeupSlot3`
    MakeupSlot3,
    /// DCB value: `TattooSlot1`
    TattooSlot1,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterCustomizerTextureSlot`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterCustomizerTextureSlot {
    /// DCB value: `SLOT1`
    SLOT1,
    /// DCB value: `SLOT2`
    SLOT2,
    /// DCB value: `SLOT3`
    SLOT3,
    /// DCB value: `SLOT4`
    SLOT4,
    /// DCB value: `SLOT5`
    SLOT5,
    /// DCB value: `SLOT6`
    SLOT6,
    /// DCB value: `SLOT7`
    SLOT7,
    /// DCB value: `SLOT8`
    SLOT8,
    /// DCB value: `SLOT9`
    SLOT9,
    /// DCB value: `SLOT10`
    SLOT10,
    /// DCB value: `SLOT11`
    SLOT11,
    /// DCB value: `SLOT12`
    SLOT12,
    /// DCB value: `SLOT13`
    SLOT13,
    /// DCB value: `SLOT14`
    SLOT14,
    /// DCB value: `SLOT15`
    SLOT15,
    /// DCB value: `SLOT16`
    SLOT16,
    /// DCB value: `SLOT17`
    SLOT17,
    /// DCB value: `SLOT18`
    SLOT18,
    /// DCB value: `SLOT19`
    SLOT19,
    /// DCB value: `SLOT_INVALID`
    SLOT_INVALID,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterGenerationArchetypeBuild`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterGenerationArchetypeBuild {
    /// DCB value: `Skinny`
    Skinny,
    /// DCB value: `Average`
    Average,
    /// DCB value: `Heavy`
    Heavy,
    /// DCB value: `Muscular`
    Muscular,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECharacterGenerationArchetypeEthnicity`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECharacterGenerationArchetypeEthnicity {
    /// DCB value: `African`
    African,
    /// DCB value: `Asian`
    Asian,
    /// DCB value: `Caucasian`
    Caucasian,
    /// DCB value: `Hispanic`
    Hispanic,
    /// DCB value: `Other`
    Other,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EChargeDrainMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EChargeDrainMode {
    /// DCB value: `Charge`
    Charge,
    /// DCB value: `Drain`
    Drain,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EChatEmoteType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EChatEmoteType {
    /// DCB value: `Social`
    Social,
    /// DCB value: `CombatSignal`
    CombatSignal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECommsNotificationTiming`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECommsNotificationTiming {
    /// DCB value: `Simultaneous`
    Simultaneous,
    /// DCB value: `HUDNotificationFirst`
    HUDNotificationFirst,
    /// DCB value: `CommsNotificationFirst`
    CommsNotificationFirst,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECommsRTTLocation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECommsRTTLocation {
    /// DCB value: `None`
    None,
    /// DCB value: `Visor`
    Visor,
    /// DCB value: `Mobiglas`
    Mobiglas,
    /// DCB value: `VehicleMFD`
    VehicleMFD,
    /// DCB value: `Tannoy`
    Tannoy,
    /// DCB value: `Hologram`
    Hologram,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EComparisonMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EComparisonMode {
    /// DCB value: `Equal`
    Equal,
    /// DCB value: `Greater`
    Greater,
    /// DCB value: `Less`
    Less,
    /// DCB value: `GreaterOrEqual`
    GreaterOrEqual,
    /// DCB value: `LessOrEqual`
    LessOrEqual,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EConsumableResourceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EConsumableResourceType {
    /// DCB value: `None`
    None,
    /// DCB value: `Fuel`
    Fuel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EContextMenuOptionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EContextMenuOptionType {
    /// DCB value: `AddFriend`
    AddFriend,
    /// DCB value: `Report`
    Report,
    /// DCB value: `Mute`
    Mute,
    /// DCB value: `Lobby_Invite`
    Lobby_Invite,
    /// DCB value: `Lobby_Leave`
    Lobby_Leave,
    /// DCB value: `Lobby_Kick`
    Lobby_Kick,
    /// DCB value: `Lobby_TransferLeader`
    Lobby_TransferLeader,
    /// DCB value: `Lobby_Promote`
    Lobby_Promote,
    /// DCB value: `Vote_Kick`
    Vote_Kick,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EControlledSubstanceClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EControlledSubstanceClass {
    /// DCB value: `ClassA`
    ClassA,
    /// DCB value: `ClassB`
    ClassB,
    /// DCB value: `ClassC`
    ClassC,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EConversationHubLinkType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EConversationHubLinkType {
    /// DCB value: `First`
    First,
    /// DCB value: `Random`
    Random,
    /// DCB value: `RandomCanRepeat`
    RandomCanRepeat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECraftingCostResultCompositionOption`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECraftingCostResultCompositionOption {
    /// DCB value: `Include`
    Include,
    /// DCB value: `Exclude`
    Exclude,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECraftingMachineDoorState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECraftingMachineDoorState {
    /// DCB value: `Opened`
    Opened,
    /// DCB value: `Closing`
    Closing,
    /// DCB value: `Closed`
    Closed,
    /// DCB value: `Locked`
    Locked,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECraftingProcessType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECraftingProcessType {
    /// DCB value: `Create`
    Create,
    /// DCB value: `Refine`
    Refine,
    /// DCB value: `Repair`
    Repair,
    /// DCB value: `Upgrade`
    Upgrade,
    /// DCB value: `Dismantle`
    Dismantle,
    /// DCB value: `Research`
    Research,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ECustomSettingType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ECustomSettingType {
    /// DCB value: `TimeLimit`
    TimeLimit,
    /// DCB value: `ScoreLimit`
    ScoreLimit,
    /// DCB value: `EnableMatchCycling`
    EnableMatchCycling,
    /// DCB value: `RandomizeMapOnMatchCycle`
    RandomizeMapOnMatchCycle,
    /// DCB value: `EnableTeamSwitching`
    EnableTeamSwitching,
    /// DCB value: `EnableTeamBalancing`
    EnableTeamBalancing,
    /// DCB value: `DisableAllPickups`
    DisableAllPickups,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDFMVictoryScoringType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDFMVictoryScoringType {
    /// DCB value: `Kills`
    Kills,
    /// DCB value: `Score`
    Score,
    /// DCB value: `Deaths`
    Deaths,
    /// DCB value: `GunGameLevel`
    GunGameLevel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDNAEditType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDNAEditType {
    /// DCB value: `None`
    None,
    /// DCB value: `Blending`
    Blending,
    /// DCB value: `Sculpting`
    Sculpting,
    /// DCB value: `Preset`
    Preset,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDecayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDecayType {
    /// DCB value: `Sink`
    Sink,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDefaultActionsEntityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDefaultActionsEntityType {
    /// DCB value: `MountedGun`
    MountedGun,
    /// DCB value: `ActorMovable`
    ActorMovable,
    /// DCB value: `DraggableBody`
    DraggableBody,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDefaultColliderBehaviour`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDefaultColliderBehaviour {
    /// DCB value: `Pushable`
    Pushable,
    /// DCB value: `NonPushale`
    NonPushale,
    /// DCB value: `Kinematic`
    Kinematic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDefaultEntitlement`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDefaultEntitlement {
    /// DCB value: `Offline`
    Offline,
    /// DCB value: `AllModes`
    AllModes,
    /// DCB value: `StarMarine`
    StarMarine,
    /// DCB value: `ArenaCommander`
    ArenaCommander,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDeformerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDeformerType {
    /// DCB value: `Standard`
    Standard,
    /// DCB value: `OriginalSkin`
    OriginalSkin,
    /// DCB value: `Protos`
    Protos,
    /// DCB value: `ProtosMisc`
    ProtosMisc,
    /// DCB value: `WD_Elastic`
    WD_Elastic,
    /// DCB value: `WD_BShapeExclusion`
    WD_BShapeExclusion,
    /// DCB value: `WD_NUScaling`
    WD_NUScaling,
    /// DCB value: `WD_ElasticNUScaling`
    WD_ElasticNUScaling,
    /// DCB value: `WD_ElasticDQSkinning`
    WD_ElasticDQSkinning,
    /// DCB value: `Cloth`
    Cloth,
    /// DCB value: `LinearSkinning`
    LinearSkinning,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDelayUnit`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDelayUnit {
    /// DCB value: `Seconds`
    Seconds,
    /// DCB value: `RPM`
    RPM,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDelinkMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDelinkMode {
    /// DCB value: `NoDelink`
    NoDelink,
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Quick`
    Quick,
    /// DCB value: `Forced`
    Forced,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDeltaSignaturePriority`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDeltaSignaturePriority {
    /// DCB value: `None`
    None,
    /// DCB value: `Auxiliary`
    Auxiliary,
    /// DCB value: `Primary`
    Primary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDifficultyModifierType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDifficultyModifierType {
    /// DCB value: `PlayerIncomingDamage`
    PlayerIncomingDamage,
    /// DCB value: `AIIncomingDamage`
    AIIncomingDamage,
    /// DCB value: `InjuryChance`
    InjuryChance,
    /// DCB value: `DownedDamageRate`
    DownedDamageRate,
    /// DCB value: `BuddyDownedDamageRate`
    BuddyDownedDamageRate,
    /// DCB value: `MedpenRestoration`
    MedpenRestoration,
    /// DCB value: `OxypenRestoration`
    OxypenRestoration,
    /// DCB value: `WingmanHelpTrigger`
    WingmanHelpTrigger,
    /// DCB value: `MissileUsageChance`
    MissileUsageChance,
    /// DCB value: `BurstFireTime`
    BurstFireTime,
    /// DCB value: `AIOnFootAccuracy`
    AIOnFootAccuracy,
    /// DCB value: `AIShipAccuracy`
    AIShipAccuracy,
    /// DCB value: `AIOnFootPerception`
    AIOnFootPerception,
    /// DCB value: `AIGrenadeTokenCooldown`
    AIGrenadeTokenCooldown,
    /// DCB value: `AIMercyTimer`
    AIMercyTimer,
    /// DCB value: `VehicleDamage_AI_HitBy_AI`
    VehicleDamage_AI_HitBy_AI,
    /// DCB value: `VehicleDamage_AI_HitBy_Player`
    VehicleDamage_AI_HitBy_Player,
    /// DCB value: `VehicleDamage_Player_HitBy_AI`
    VehicleDamage_Player_HitBy_AI,
    /// DCB value: `VehicleDamage_Player_HitBy_Player`
    VehicleDamage_Player_HitBy_Player,
    /// DCB value: `VehicleDamage_Uncontrolled_HitBy_AI`
    VehicleDamage_Uncontrolled_HitBy_AI,
    /// DCB value: `VehicleDamage_Uncontrolled_HitBy_Player`
    VehicleDamage_Uncontrolled_HitBy_Player,
    /// DCB value: `RequiredEatAndDrink`
    RequiredEatAndDrink,
    /// DCB value: `SlowerMagRelevellingEnabled`
    SlowerMagRelevellingEnabled,
    /// DCB value: `SlowerMagRelevellingSpeed`
    SlowerMagRelevellingSpeed,
    /// DCB value: `ReducedCheckpoints`
    ReducedCheckpoints,
    /// DCB value: `ScanningRestrictions`
    ScanningRestrictions,
    /// DCB value: `NoControlHints`
    NoControlHints,
    /// DCB value: `NoHardTutorials`
    NoHardTutorials,
    /// DCB value: `NoSoftTutorials`
    NoSoftTutorials,
    /// DCB value: `NoQTETutorials`
    NoQTETutorials,
    /// DCB value: `NoPlayerInnerThoughtHints`
    NoPlayerInnerThoughtHints,
    /// DCB value: `NoCrosshair`
    NoCrosshair,
    /// DCB value: `NoHitMarker`
    NoHitMarker,
    /// DCB value: `NoPlayerHitIndicator`
    NoPlayerHitIndicator,
    /// DCB value: `NoGrenadeMarker`
    NoGrenadeMarker,
    /// DCB value: `ManualShipWeaponRackStorage`
    ManualShipWeaponRackStorage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDifficultyRange_GameKnowledge`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDifficultyRange_GameKnowledge {
    /// DCB value: `Noob_gaming_or_tutorial_1`
    Noob_gaming_or_tutorial_1,
    /// DCB value: `FPS_mechanics_walk_shoot_mobiGlass_2`
    FPS_mechanics_walk_shoot_mobiGlass_2,
    /// DCB value: `Flight_mechanics_fly_dock_quantum_3`
    Flight_mechanics_fly_dock_quantum_3,
    /// DCB value: `Standard_understanding_FPS_flight_professions_4`
    Standard_understanding_FPS_flight_professions_4,
    /// DCB value: `Expert_understanding_FPS_flight_professions_5`
    Expert_understanding_FPS_flight_professions_5,
    /// DCB value: `Pro_understanding_of_optimal_tactics_6`
    Pro_understanding_of_optimal_tactics_6,
    /// DCB value: `Basically_a_Dev_7`
    Basically_a_Dev_7,
    /// DCB value: `No_content_like_this_yet_8`
    No_content_like_this_yet_8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDifficultyRange_MechanicalSkill`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDifficultyRange_MechanicalSkill {
    /// DCB value: `Hands_free_gaming_1`
    Hands_free_gaming_1,
    /// DCB value: `Zero_risk_of_action_2`
    Zero_risk_of_action_2,
    /// DCB value: `Easy_PvE_only_action_3`
    Easy_PvE_only_action_3,
    /// DCB value: `Normal_PvE_only_action_4`
    Normal_PvE_only_action_4,
    /// DCB value: `Hard_PvE_or_Easy_PvP_action_5`
    Hard_PvE_or_Easy_PvP_action_5,
    /// DCB value: `Multiplayer_PvE_or_Expert_PvP_action_6`
    Multiplayer_PvE_or_Expert_PvP_action_6,
    /// DCB value: `PvE_PvP_large_group_action_eg_warzone_7`
    PvE_PvP_large_group_action_eg_warzone_7,
    /// DCB value: `No_content_like_this_yet_8`
    No_content_like_this_yet_8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDifficultyRange_MentalLoad`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDifficultyRange_MentalLoad {
    /// DCB value: `AFK_gaming_1`
    AFK_gaming_1,
    /// DCB value: `Requires_minimal_thought_2`
    Requires_minimal_thought_2,
    /// DCB value: `Routine_light_work_3`
    Routine_light_work_3,
    /// DCB value: `Moments_of_concentration_required_4`
    Moments_of_concentration_required_4,
    /// DCB value: `Like_spinning_10_plates_at_once_5`
    Like_spinning_10_plates_at_once_5,
    /// DCB value: `Extremely_hard_to_manage_alone_6`
    Extremely_hard_to_manage_alone_6,
    /// DCB value: `Insane_complexity_NOT_soloable_7`
    Insane_complexity_NOT_soloable_7,
    /// DCB value: `No_content_like_this_yet_8`
    No_content_like_this_yet_8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDifficultyRange_RiskOfLoss`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDifficultyRange_RiskOfLoss {
    /// DCB value: `Safe_and_sound_zzzz_1`
    Safe_and_sound_zzzz_1,
    /// DCB value: `Barely_even_breaking_a_sweat_2`
    Barely_even_breaking_a_sweat_2,
    /// DCB value: `Minimal_danger_FPS_NOT_ship_action_3`
    Minimal_danger_FPS_NOT_ship_action_3,
    /// DCB value: `Ship_could_get_damaged_Could_lose_cargo_4`
    Ship_could_get_damaged_Could_lose_cargo_4,
    /// DCB value: `Player_might_die_Ship_could_explode_5`
    Player_might_die_Ship_could_explode_5,
    /// DCB value: `Player_likely_to_die_Ship_too_6`
    Player_likely_to_die_Ship_too_6,
    /// DCB value: `Without_help_Player_and_Ship_die_7`
    Without_help_Player_and_Ship_die_7,
    /// DCB value: `No_content_like_this_yet_8`
    No_content_like_this_yet_8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDockingTubeAnimationStage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDockingTubeAnimationStage {
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `PreDocked`
    PreDocked,
    /// DCB value: `Docked`
    Docked,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDoorCollisionReactionDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDoorCollisionReactionDirection {
    /// DCB value: `All`
    All,
    /// DCB value: `Up`
    Up,
    /// DCB value: `Down`
    Down,
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Front`
    Front,
    /// DCB value: `Back`
    Back,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDoorDestructionBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDoorDestructionBehavior {
    /// DCB value: `UnlockDoor`
    UnlockDoor,
    /// DCB value: `BreakDoor`
    BreakDoor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDoorPortalLookupMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDoorPortalLookupMode {
    /// DCB value: `Automatic`
    Automatic,
    /// DCB value: `AABB_Center`
    AABB_Center,
    /// DCB value: `Pivot`
    Pivot,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDoorPoweredState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDoorPoweredState {
    /// DCB value: `Powered`
    Powered,
    /// DCB value: `Unpowered`
    Unpowered,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EDynamicRigLightType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EDynamicRigLightType {
    /// DCB value: `Key`
    Key,
    /// DCB value: `Fill`
    Fill,
    /// DCB value: `Rim`
    Rim,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEAActionItemMessageType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEAActionItemMessageType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `WaitingForPlayers`
    WaitingForPlayers,
    /// DCB value: `BalancingTeams`
    BalancingTeams,
    /// DCB value: `MatchStarting`
    MatchStarting,
    /// DCB value: `MatchEnding`
    MatchEnding,
    /// DCB value: `Contested`
    Contested,
    /// DCB value: `OutOfPosition`
    OutOfPosition,
    /// DCB value: `PressKeyToSkip`
    PressKeyToSkip,
    /// DCB value: `Retry`
    Retry,
    /// DCB value: `Respawning`
    Respawning,
    /// DCB value: `RespawnPrompt`
    RespawnPrompt,
    /// DCB value: `RespawnTimer`
    RespawnTimer,
    /// DCB value: `Award`
    Award,
    /// DCB value: `ReadyWaitingOnKeyPress`
    ReadyWaitingOnKeyPress,
    /// DCB value: `ReadyWaitingOnKeyPressForceReady`
    ReadyWaitingOnKeyPressForceReady,
    /// DCB value: `ReadyWaitingForOtherPlayers`
    ReadyWaitingForOtherPlayers,
    /// DCB value: `WaitingForPlayersWithDebugSkip`
    WaitingForPlayersWithDebugSkip,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEACapturableEntityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEACapturableEntityType {
    /// DCB value: `Terminal`
    Terminal,
    /// DCB value: `Prop`
    Prop,
    /// DCB value: `LockedSpawn`
    LockedSpawn,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEACriticalMessageType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEACriticalMessageType {
    /// DCB value: `Local_FirstBlood`
    Local_FirstBlood,
    /// DCB value: `Local_Ace`
    Local_Ace,
    /// DCB value: `Local_KillingSpree`
    Local_KillingSpree,
    /// DCB value: `Local_OnFinalKill`
    Local_OnFinalKill,
    /// DCB value: `Player_FirstBlood`
    Player_FirstBlood,
    /// DCB value: `Player_Ace`
    Player_Ace,
    /// DCB value: `Player_KillingSpree`
    Player_KillingSpree,
    /// DCB value: `Player_OnFinalKill`
    Player_OnFinalKill,
    /// DCB value: `EnemyTeam_OnFinalKill`
    EnemyTeam_OnFinalKill,
    /// DCB value: `FriendlyTeam_OnFinalKill`
    FriendlyTeam_OnFinalKill,
    /// DCB value: `Missile_Replenished`
    Missile_Replenished,
    /// DCB value: `Ammo_Replenished`
    Ammo_Replenished,
    /// DCB value: `Vehicle_Repaired`
    Vehicle_Repaired,
    /// DCB value: `Vehicle_Refueled`
    Vehicle_Refueled,
    /// DCB value: `Missile_Replenish_Failed`
    Missile_Replenish_Failed,
    /// DCB value: `Ammo_Replenish_Failed`
    Ammo_Replenish_Failed,
    /// DCB value: `Vehicle_Repair_Failed`
    Vehicle_Repair_Failed,
    /// DCB value: `Vehicle_Refuel_Failed`
    Vehicle_Refuel_Failed,
    /// DCB value: `CTRL_Captured`
    CTRL_Captured,
    /// DCB value: `CTRL_Lost`
    CTRL_Lost,
    /// DCB value: `CTRL_Neutralized`
    CTRL_Neutralized,
    /// DCB value: `CTRL_Phase_Notification`
    CTRL_Phase_Notification,
    /// DCB value: `Respawn_Replenished`
    Respawn_Replenished,
    /// DCB value: `INVALID`
    INVALID,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEAMessageTriggerFrequency`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEAMessageTriggerFrequency {
    /// DCB value: `Always`
    Always,
    /// DCB value: `OncePerLife`
    OncePerLife,
    /// DCB value: `OncePerRound`
    OncePerRound,
    /// DCB value: `ActiveWhenInside`
    ActiveWhenInside,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEAObjectiveState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEAObjectiveState {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Capturing`
    Capturing,
    /// DCB value: `MultipleCapturing`
    MultipleCapturing,
    /// DCB value: `Contesting`
    Contesting,
    /// DCB value: `ReturningToOwner`
    ReturningToOwner,
    /// DCB value: `Cooldown`
    Cooldown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEAPlayableAreaOnExit`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEAPlayableAreaOnExit {
    /// DCB value: `None`
    None,
    /// DCB value: `Autopilot`
    Autopilot,
    /// DCB value: `DamagePerSecond`
    DamagePerSecond,
    /// DCB value: `Kill`
    Kill,
    /// DCB value: `Disable`
    Disable,
    /// DCB value: `Redout`
    Redout,
    /// DCB value: `SimulationGlitch`
    SimulationGlitch,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEASpawnMulticrewType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEASpawnMulticrewType {
    /// DCB value: `Public`
    Public,
    /// DCB value: `SquadOnly`
    SquadOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEASpecialEventPass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEASpecialEventPass {
    /// DCB value: `EA_SpecialEvent_LunarNewYear`
    EA_SpecialEvent_LunarNewYear,
    /// DCB value: `EA_SpecialEvent_ValentinesDay`
    EA_SpecialEvent_ValentinesDay,
    /// DCB value: `EA_SpecialEvent_StPatricks`
    EA_SpecialEvent_StPatricks,
    /// DCB value: `EA_SpecialEvent_AprilFools`
    EA_SpecialEvent_AprilFools,
    /// DCB value: `EA_SpecialEvent_Invictus`
    EA_SpecialEvent_Invictus,
    /// DCB value: `EA_SpecialEvent_AlienWeek`
    EA_SpecialEvent_AlienWeek,
    /// DCB value: `EA_SpecialEvent_FoundationFestival`
    EA_SpecialEvent_FoundationFestival,
    /// DCB value: `EA_SpecialEvent_PirateWeek`
    EA_SpecialEvent_PirateWeek,
    /// DCB value: `EA_SpecialEvent_CitizenCon`
    EA_SpecialEvent_CitizenCon,
    /// DCB value: `EA_SpecialEvent_Halloween`
    EA_SpecialEvent_Halloween,
    /// DCB value: `EA_SpecialEvent_IAE`
    EA_SpecialEvent_IAE,
    /// DCB value: `EA_SpecialEvent_Xmas`
    EA_SpecialEvent_Xmas,
    /// DCB value: `EA_SpecialEvent_FightOrFlight`
    EA_SpecialEvent_FightOrFlight,
    /// DCB value: `EA_SpecialEvent_MurrayCup`
    EA_SpecialEvent_MurrayCup,
    /// DCB value: `EA_SpecialEvent_PlayerBirthday`
    EA_SpecialEvent_PlayerBirthday,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEndCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEndCondition {
    /// DCB value: `None`
    None,
    /// DCB value: `AllUsed`
    AllUsed,
    /// DCB value: `NumberOfUsablesToUse`
    NumberOfUsablesToUse,
    /// DCB value: `AllUsed_WithSync`
    AllUsed_WithSync,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEnemyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEnemyType {
    /// DCB value: `ET_Invalid`
    ET_Invalid,
    /// DCB value: `ET_Boss`
    ET_Boss,
    /// DCB value: `ET_Butterfly`
    ET_Butterfly,
    /// DCB value: `ET_Bee`
    ET_Bee,
    /// DCB value: `ET_Tonbo`
    ET_Tonbo,
    /// DCB value: `ET_Momiji`
    ET_Momiji,
    /// DCB value: `ET_Sasori`
    ET_Sasori,
    /// DCB value: `ET_Midori`
    ET_Midori,
    /// DCB value: `ET_Galboss`
    ET_Galboss,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEntityComponentCommsChannelJoinType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEntityComponentCommsChannelJoinType {
    /// DCB value: `AutoJoin`
    AutoJoin,
    /// DCB value: `Invite`
    Invite,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEntityMarkerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEntityMarkerType {
    /// DCB value: `None`
    None,
    /// DCB value: `LandingZone`
    LandingZone,
    /// DCB value: `MedPen`
    MedPen,
    /// DCB value: `AmmoCrate`
    AmmoCrate,
    /// DCB value: `GrenadeCrate`
    GrenadeCrate,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEntryFlagCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEntryFlagCondition {
    /// DCB value: `Tracked`
    Tracked,
    /// DCB value: `Locked`
    Locked,
    /// DCB value: `Pinned`
    Pinned,
    /// DCB value: `DetectedByActiveRadar`
    DetectedByActiveRadar,
    /// DCB value: `DetectedByDeltaSignature`
    DetectedByDeltaSignature,
    /// DCB value: `ExtendedContact`
    ExtendedContact,
    /// DCB value: `Tagged`
    Tagged,
    /// DCB value: `BoxoutActive`
    BoxoutActive,
    /// DCB value: `IsObjective`
    IsObjective,
    /// DCB value: `IsPartyMember`
    IsPartyMember,
    /// DCB value: `ShowInFPS`
    ShowInFPS,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EEventTriggerMask`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EEventTriggerMask {
    /// DCB value: `AIAndPlayer`
    AIAndPlayer,
    /// DCB value: `OnlyAI`
    OnlyAI,
    /// DCB value: `OnlyPlayer`
    OnlyPlayer,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EExcludeSpawnGender`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EExcludeSpawnGender {
    /// DCB value: `Male`
    Male,
    /// DCB value: `Female`
    Female,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFaceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFaceType {
    /// DCB value: `Bubble`
    Bubble,
    /// DCB value: `FrontBack`
    FrontBack,
    /// DCB value: `Quadrant`
    Quadrant,
    /// DCB value: `Box`
    Box,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFiringRangePenaltyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFiringRangePenaltyType {
    /// DCB value: `EnemyReachedPlayer`
    EnemyReachedPlayer,
    /// DCB value: `Friendlyfire`
    Friendlyfire,
    /// DCB value: `HeadShot`
    HeadShot,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFirstSelectMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFirstSelectMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Once`
    Once,
    /// DCB value: `Always`
    Always,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFitnessImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFitnessImprovementType {
    /// DCB value: `Stamina`
    Stamina,
    /// DCB value: `BlackoutRedoutThreshold`
    BlackoutRedoutThreshold,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EForceReactionLeanHumanSpineBoneName`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EForceReactionLeanHumanSpineBoneName {
    /// DCB value: `Hips`
    Hips,
    /// DCB value: `Spine`
    Spine,
    /// DCB value: `Spine1`
    Spine1,
    /// DCB value: `Spine2`
    Spine2,
    /// DCB value: `Spine3`
    Spine3,
    /// DCB value: `Neck`
    Neck,
    /// DCB value: `Neck1`
    Neck1,
    /// DCB value: `Head`
    Head,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EForceReactionLeanVanduulSpineBoneName`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EForceReactionLeanVanduulSpineBoneName {
    /// DCB value: `Hips`
    Hips,
    /// DCB value: `Spine`
    Spine,
    /// DCB value: `Spine1`
    Spine1,
    /// DCB value: `Spine2`
    Spine2,
    /// DCB value: `Spine3`
    Spine3,
    /// DCB value: `Neck`
    Neck,
    /// DCB value: `Neck1`
    Neck1,
    /// DCB value: `Head`
    Head,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EForceReactionLeanXianSpineBoneName`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EForceReactionLeanXianSpineBoneName {
    /// DCB value: `Hips`
    Hips,
    /// DCB value: `Spine1`
    Spine1,
    /// DCB value: `Spine2`
    Spine2,
    /// DCB value: `Spine3`
    Spine3,
    /// DCB value: `Spine4`
    Spine4,
    /// DCB value: `Neck1`
    Neck1,
    /// DCB value: `Neck2`
    Neck2,
    /// DCB value: `Neck3`
    Neck3,
    /// DCB value: `Neck4`
    Neck4,
    /// DCB value: `Head`
    Head,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFortitudeImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFortitudeImprovementType {
    /// DCB value: `HungerDecay`
    HungerDecay,
    /// DCB value: `ThirstDecay`
    ThirstDecay,
    /// DCB value: `DrugEffects`
    DrugEffects,
    /// DCB value: `Health`
    Health,
    /// DCB value: `InjuryChance`
    InjuryChance,
    /// DCB value: `StunDuration`
    StunDuration,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EFrontendGameModeButton`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EFrontendGameModeButton {
    /// DCB value: `PU`
    PU,
    /// DCB value: `EA`
    EA,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameCollisionClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameCollisionClass {
    /// DCB value: `Actor_Capsule`
    Actor_Capsule,
    /// DCB value: `Actor_Body`
    Actor_Body,
    /// DCB value: `Actor_PlayerControlled`
    Actor_PlayerControlled,
    /// DCB value: `Actor_AIControlled`
    Actor_AIControlled,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Vehicle_Interior`
    Vehicle_Interior,
    /// DCB value: `Ragdoll`
    Ragdoll,
    /// DCB value: `Projectile`
    Projectile,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `Holo_Volume`
    Holo_Volume,
    /// DCB value: `Spewgun_Projectile`
    Spewgun_Projectile,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameDifficulty`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameDifficulty {
    /// DCB value: `None`
    None,
    /// DCB value: `VeryEasy`
    VeryEasy,
    /// DCB value: `Easy`
    Easy,
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Hard`
    Hard,
    /// DCB value: `VeryHard`
    VeryHard,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameModeFilters`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameModeFilters {
    /// DCB value: `Released`
    Released,
    /// DCB value: `Flight`
    Flight,
    /// DCB value: `GroundVehicles`
    GroundVehicles,
    /// DCB value: `FPS`
    FPS,
    /// DCB value: `COOP`
    COOP,
    /// DCB value: `PvP`
    PvP,
    /// DCB value: `PvE`
    PvE,
    /// DCB value: `Solo`
    Solo,
    /// DCB value: `Leaderboards`
    Leaderboards,
    /// DCB value: `Teams`
    Teams,
    /// DCB value: `Racing`
    Racing,
    /// DCB value: `Experimental`
    Experimental,
    /// DCB value: `MultiCrew`
    MultiCrew,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameModeId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameModeId {
    /// DCB value: `S42_Default`
    S42_Default,
    /// DCB value: `SC_Frontend`
    SC_Frontend,
    /// DCB value: `SC_Default`
    SC_Default,
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `EA_BattleRoyale`
    EA_BattleRoyale,
    /// DCB value: `EA_FreeFlight`
    EA_FreeFlight,
    /// DCB value: `EA_PirateSwarm`
    EA_PirateSwarm,
    /// DCB value: `EA_SquadronBattle`
    EA_SquadronBattle,
    /// DCB value: `EA_VanduulSwarm`
    EA_VanduulSwarm,
    /// DCB value: `EA_ClassicRace`
    EA_ClassicRace,
    /// DCB value: `EA_Elimination`
    EA_Elimination,
    /// DCB value: `EA_TeamElimination`
    EA_TeamElimination,
    /// DCB value: `EA_Control`
    EA_Control,
    /// DCB value: `EA_TheatersOfWar`
    EA_TheatersOfWar,
    /// DCB value: `EA_IterativeTesting`
    EA_IterativeTesting,
    /// DCB value: `EA_Duel`
    EA_Duel,
    /// DCB value: `EA_FPSGunGame`
    EA_FPSGunGame,
    /// DCB value: `EA_Horde`
    EA_Horde,
    /// DCB value: `EA_VanduulInvasion`
    EA_VanduulInvasion,
    /// DCB value: `EA_ExperimentalMode_1`
    EA_ExperimentalMode_1,
    /// DCB value: `EA_ExperimentalMode_2`
    EA_ExperimentalMode_2,
    /// DCB value: `EA_ExperimentalMode_3`
    EA_ExperimentalMode_3,
    /// DCB value: `EA_ExperimentalMode_4`
    EA_ExperimentalMode_4,
    /// DCB value: `EA_ExperimentalMode_5`
    EA_ExperimentalMode_5,
    /// DCB value: `EA_ExperimentalMode_6`
    EA_ExperimentalMode_6,
    /// DCB value: `EA_ExperimentalMode_7`
    EA_ExperimentalMode_7,
    /// DCB value: `EA_ExperimentalMode_8`
    EA_ExperimentalMode_8,
    /// DCB value: `EA_ExperimentalMode_9`
    EA_ExperimentalMode_9,
    /// DCB value: `EA_ExperimentalMode_10`
    EA_ExperimentalMode_10,
    /// DCB value: `EA_ExperimentalMode_11`
    EA_ExperimentalMode_11,
    /// DCB value: `EA_ExperimentalMode_12`
    EA_ExperimentalMode_12,
    /// DCB value: `EA_GravRace`
    EA_GravRace,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameModePlayedId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameModePlayedId {
    /// DCB value: `None`
    None,
    /// DCB value: `EA_Played_GunGame`
    EA_Played_GunGame,
    /// DCB value: `EA_Played_TankRoyale`
    EA_Played_TankRoyale,
    /// DCB value: `EA_Played_TeamTank`
    EA_Played_TeamTank,
    /// DCB value: `EA_Played_SingleWeapon`
    EA_Played_SingleWeapon,
    /// DCB value: `EA_Played_MirrorMatch`
    EA_Played_MirrorMatch,
    /// DCB value: `EA_Played_MM_Vanduul`
    EA_Played_MM_Vanduul,
    /// DCB value: `EA_Played_MM_Dogfight`
    EA_Played_MM_Dogfight,
    /// DCB value: `EA_Played_MM_ClassicRace`
    EA_Played_MM_ClassicRace,
    /// DCB value: `EA_Played_KillConfirmedFPS`
    EA_Played_KillConfirmedFPS,
    /// DCB value: `EA_Played_KillConfirmedVehicle`
    EA_Played_KillConfirmedVehicle,
    /// DCB value: `EA_Played_Wingman`
    EA_Played_Wingman,
    /// DCB value: `EA_Played_TeamElimination`
    EA_Played_TeamElimination,
    /// DCB value: `EA_Played_GravRace`
    EA_Played_GravRace,
    /// DCB value: `EA_Played_GravRoyale`
    EA_Played_GravRoyale,
    /// DCB value: `EA_Played_RN_SquadronBattle`
    EA_Played_RN_SquadronBattle,
    /// DCB value: `EA_Played_RN_Duel`
    EA_Played_RN_Duel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGameRulesEventType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGameRulesEventType {
    /// DCB value: `PlayerKill`
    PlayerKill,
    /// DCB value: `PlayerKillAssist`
    PlayerKillAssist,
    /// DCB value: `PlayerKillAssistTeam1`
    PlayerKillAssistTeam1,
    /// DCB value: `PlayerKillAssistTeam2`
    PlayerKillAssistTeam2,
    /// DCB value: `PlayerBledOut`
    PlayerBledOut,
    /// DCB value: `PlayerTeamKill`
    PlayerTeamKill,
    /// DCB value: `Accident`
    Accident,
    /// DCB value: `Suicide`
    Suicide,
    /// DCB value: `Tagged_PlayerKillAssist`
    Tagged_PlayerKillAssist,
    /// DCB value: `PlayerBleeding`
    PlayerBleeding,
    /// DCB value: `DistortionDamage`
    DistortionDamage,
    /// DCB value: `DistortionDisabledShip`
    DistortionDisabledShip,
    /// DCB value: `DamageShip`
    DamageShip,
    /// DCB value: `DamageTeamShip`
    DamageTeamShip,
    /// DCB value: `DestroyedShip`
    DestroyedShip,
    /// DCB value: `DestroyedTeamShip`
    DestroyedTeamShip,
    /// DCB value: `DestroyedShipEjected`
    DestroyedShipEjected,
    /// DCB value: `PlayerKillEjected`
    PlayerKillEjected,
    /// DCB value: `CompletedLap`
    CompletedLap,
    /// DCB value: `Award_UnaidedKill`
    Award_UnaidedKill,
    /// DCB value: `Award_Untouchable`
    Award_Untouchable,
    /// DCB value: `Award_Ace`
    Award_Ace,
    /// DCB value: `Award_AceBonus`
    Award_AceBonus,
    /// DCB value: `Award_AceKill`
    Award_AceKill,
    /// DCB value: `Award_AceKillBonus`
    Award_AceKillBonus,
    /// DCB value: `Award_KillingSpree`
    Award_KillingSpree,
    /// DCB value: `Award_KillingSpreeBonus`
    Award_KillingSpreeBonus,
    /// DCB value: `Award_KillingSpreeKill`
    Award_KillingSpreeKill,
    /// DCB value: `Award_KillingSpreeKillBonus`
    Award_KillingSpreeKillBonus,
    /// DCB value: `Award_NemesisKill`
    Award_NemesisKill,
    /// DCB value: `Award_RedemptionKill`
    Award_RedemptionKill,
    /// DCB value: `Award_ResurgentKill`
    Award_ResurgentKill,
    /// DCB value: `Award_ResurgentKillBonus`
    Award_ResurgentKillBonus,
    /// DCB value: `Award_RevengeKill`
    Award_RevengeKill,
    /// DCB value: `Award_FirstBlood`
    Award_FirstBlood,
    /// DCB value: `Award_KillAssist`
    Award_KillAssist,
    /// DCB value: `Award_Savior`
    Award_Savior,
    /// DCB value: `Award_SquadronRevengeKill`
    Award_SquadronRevengeKill,
    /// DCB value: `Award_UnderdogKill`
    Award_UnderdogKill,
    /// DCB value: `Award_CrashRoberts`
    Award_CrashRoberts,
    /// DCB value: `Award_CheapShot`
    Award_CheapShot,
    /// DCB value: `Award_ForcedEject`
    Award_ForcedEject,
    /// DCB value: `Award_ForcedError`
    Award_ForcedError,
    /// DCB value: `Award_Goodnight`
    Award_Goodnight,
    /// DCB value: `Award_LightsOut`
    Award_LightsOut,
    /// DCB value: `Award_MartyrKill`
    Award_MartyrKill,
    /// DCB value: `Award_ControlTerminalCaptured`
    Award_ControlTerminalCaptured,
    /// DCB value: `Award_ControlTerminalDefended`
    Award_ControlTerminalDefended,
    /// DCB value: `Award_ControlTerminalCaptureAssist`
    Award_ControlTerminalCaptureAssist,
    /// DCB value: `Award_ControlTerminalHackerKilled`
    Award_ControlTerminalHackerKilled,
    /// DCB value: `Award_ControlTerminalDomination`
    Award_ControlTerminalDomination,
    /// DCB value: `Award_CaptureAreaCaptureBegin`
    Award_CaptureAreaCaptureBegin,
    /// DCB value: `Award_CaptureAreaCaptureComplete`
    Award_CaptureAreaCaptureComplete,
    /// DCB value: `Award_CaptureAreaContesting`
    Award_CaptureAreaContesting,
    /// DCB value: `Award_CaptureAreaCapturing`
    Award_CaptureAreaCapturing,
    /// DCB value: `Award_CaptureCloseCallKill`
    Award_CaptureCloseCallKill,
    /// DCB value: `Award_CaptureAreaNeutralized`
    Award_CaptureAreaNeutralized,
    /// DCB value: `Award_CaptureReversing`
    Award_CaptureReversing,
    /// DCB value: `Award_DefenderKill`
    Award_DefenderKill,
    /// DCB value: `Award_AttackerKill`
    Award_AttackerKill,
    /// DCB value: `Award_Hemorrhage`
    Award_Hemorrhage,
    /// DCB value: `Award_Headshot`
    Award_Headshot,
    /// DCB value: `Award_MeleeKill`
    Award_MeleeKill,
    /// DCB value: `Award_TakeDown`
    Award_TakeDown,
    /// DCB value: `DamageItem`
    DamageItem,
    /// DCB value: `DamageTeamItem`
    DamageTeamItem,
    /// DCB value: `Award_TerminalTick`
    Award_TerminalTick,
    /// DCB value: `Award_PhaseWon`
    Award_PhaseWon,
    /// DCB value: `Award_MatchVictory`
    Award_MatchVictory,
    /// DCB value: `Award_MatchDefeat`
    Award_MatchDefeat,
    /// DCB value: `Award_SecondPlace`
    Award_SecondPlace,
    /// DCB value: `Award_ThirdPlace`
    Award_ThirdPlace,
    /// DCB value: `Award_DestroyPhaseObjective`
    Award_DestroyPhaseObjective,
    /// DCB value: `Award_DamagePhaseObjective`
    Award_DamagePhaseObjective,
    /// DCB value: `Award_DamageSentToObjective`
    Award_DamageSentToObjective,
    /// DCB value: `Award_KillCollected`
    Award_KillCollected,
    /// DCB value: `Award_KillDenied`
    Award_KillDenied,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGasCloudFadeVolumeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGasCloudFadeVolumeType {
    /// DCB value: `Sphere`
    Sphere,
    /// DCB value: `Cube`
    Cube,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGasCloudLightType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGasCloudLightType {
    /// DCB value: `Omni`
    Omni,
    /// DCB value: `Projector`
    Projector,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGasCloudOverrideVolumeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGasCloudOverrideVolumeType {
    /// DCB value: `Sphere`
    Sphere,
    /// DCB value: `Cube`
    Cube,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGeometrySlots`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGeometrySlots {
    /// DCB value: `Main`
    Main,
    /// DCB value: `LegacyHelper`
    LegacyHelper,
    /// DCB value: `DebrisPieces`
    DebrisPieces,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGeometryVisAreaMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGeometryVisAreaMode {
    /// DCB value: `AABB_Center`
    AABB_Center,
    /// DCB value: `Pivot`
    Pivot,
    /// DCB value: `Ignore_VisAreas`
    Ignore_VisAreas,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGimbalMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGimbalMode {
    /// DCB value: `SeatFixed`
    SeatFixed,
    /// DCB value: `SeatFixedAds`
    SeatFixedAds,
    /// DCB value: `SeatFixedAssisted`
    SeatFixedAssisted,
    /// DCB value: `SeatManual`
    SeatManual,
    /// DCB value: `SeatManualAds`
    SeatManualAds,
    /// DCB value: `SeatManualAssisted`
    SeatManualAssisted,
    /// DCB value: `SeatAuto`
    SeatAuto,
    /// DCB value: `MannedTurretFixed`
    MannedTurretFixed,
    /// DCB value: `MannedTurretFixedAds`
    MannedTurretFixedAds,
    /// DCB value: `MannedTurretFixedAssisted`
    MannedTurretFixedAssisted,
    /// DCB value: `MannedTurretManual`
    MannedTurretManual,
    /// DCB value: `MannedTurretManualAds`
    MannedTurretManualAds,
    /// DCB value: `MannedTurretManualAssisted`
    MannedTurretManualAssisted,
    /// DCB value: `MannedTurretAuto`
    MannedTurretAuto,
    /// DCB value: `RemoteTurretFixed`
    RemoteTurretFixed,
    /// DCB value: `RemoteTurretFixedAds`
    RemoteTurretFixedAds,
    /// DCB value: `RemoteTurretFixedAssisted`
    RemoteTurretFixedAssisted,
    /// DCB value: `RemoteTurretManual`
    RemoteTurretManual,
    /// DCB value: `RemoteTurretManualAds`
    RemoteTurretManualAds,
    /// DCB value: `RemoteTurretManualAssisted`
    RemoteTurretManualAssisted,
    /// DCB value: `RemoteTurretAuto`
    RemoteTurretAuto,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGimbalOrder`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGimbalOrder {
    /// DCB value: `YawPitch`
    YawPitch,
    /// DCB value: `PitchYaw`
    PitchYaw,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGravlevDataOutput`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGravlevDataOutput {
    /// DCB value: `SpringCompressionOne`
    SpringCompressionOne,
    /// DCB value: `SpringCompressionTwo`
    SpringCompressionTwo,
    /// DCB value: `SpringCompressionThree`
    SpringCompressionThree,
    /// DCB value: `SpringCompressionFour`
    SpringCompressionFour,
    /// DCB value: `SpringCompressionNormalizedOne`
    SpringCompressionNormalizedOne,
    /// DCB value: `SpringCompressionNormalizedTwo`
    SpringCompressionNormalizedTwo,
    /// DCB value: `SpringCompressionNormalizedThree`
    SpringCompressionNormalizedThree,
    /// DCB value: `SpringCompressionNormalizedFour`
    SpringCompressionNormalizedFour,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EGripUser`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EGripUser {
    /// DCB value: `AI`
    AI,
    /// DCB value: `Player`
    Player,
    /// DCB value: `All`
    All,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHUDNotificationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHUDNotificationType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Group`
    Group,
    /// DCB value: `Party`
    Party,
    /// DCB value: `Beacon`
    Beacon,
    /// DCB value: `Tutorial`
    Tutorial,
    /// DCB value: `ProgressBar`
    ProgressBar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingCodeCommandParamType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingCodeCommandParamType {
    /// DCB value: `Int`
    Int,
    /// DCB value: `Char`
    Char,
    /// DCB value: `Float`
    Float,
    /// DCB value: `Coordinate`
    Coordinate,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingCodeCommandType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingCodeCommandType {
    /// DCB value: `None`
    None,
    /// DCB value: `Move`
    Move,
    /// DCB value: `Stop`
    Stop,
    /// DCB value: `Swap`
    Swap,
    /// DCB value: `Ping`
    Ping,
    /// DCB value: `Wrap`
    Wrap,
    /// DCB value: `Inject`
    Inject,
    /// DCB value: `Slowdown`
    Slowdown,
    /// DCB value: `CancelAbility`
    CancelAbility,
    /// DCB value: `Spawn`
    Spawn,
    /// DCB value: `RotatePreview`
    RotatePreview,
    /// DCB value: `Help`
    Help,
    /// DCB value: `Commands`
    Commands,
    /// DCB value: `AutoCorrect`
    AutoCorrect,
    /// DCB value: `Exit`
    Exit,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingCodeEventType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingCodeEventType {
    /// DCB value: `None`
    None,
    /// DCB value: `UnrecognizedCommand`
    UnrecognizedCommand,
    /// DCB value: `HackingInitiated`
    HackingInitiated,
    /// DCB value: `HackingStarted`
    HackingStarted,
    /// DCB value: `DefenderDetected`
    DefenderDetected,
    /// DCB value: `DefenderAlerted`
    DefenderAlerted,
    /// DCB value: `DefenderAlertOver`
    DefenderAlertOver,
    /// DCB value: `DefenderSpottedIntruder`
    DefenderSpottedIntruder,
    /// DCB value: `DefenderLostIntruder`
    DefenderLostIntruder,
    /// DCB value: `DefenderStartedSwap`
    DefenderStartedSwap,
    /// DCB value: `DefenderCompletedSwap`
    DefenderCompletedSwap,
    /// DCB value: `LinkPointActivated`
    LinkPointActivated,
    /// DCB value: `LinkPointDeactivated`
    LinkPointDeactivated,
    /// DCB value: `LinkPointLinked`
    LinkPointLinked,
    /// DCB value: `LinkPointUnlinked`
    LinkPointUnlinked,
    /// DCB value: `IntruderCaptured`
    IntruderCaptured,
    /// DCB value: `HackTimedOut`
    HackTimedOut,
    /// DCB value: `HackAborted`
    HackAborted,
    /// DCB value: `HackSuccessful`
    HackSuccessful,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingErrorType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingErrorType {
    /// DCB value: `Ok`
    Ok,
    /// DCB value: `AbilityIsChargingUp`
    AbilityIsChargingUp,
    /// DCB value: `AbilityIsActive`
    AbilityIsActive,
    /// DCB value: `AbilityIsCoolingDown`
    AbilityIsCoolingDown,
    /// DCB value: `AbilityInjectNoOverlappedPoint`
    AbilityInjectNoOverlappedPoint,
    /// DCB value: `AbilityInjectPointAlreadyActive`
    AbilityInjectPointAlreadyActive,
    /// DCB value: `AbilityInjectPointAlreadyInactive`
    AbilityInjectPointAlreadyInactive,
    /// DCB value: `AbilitySwapSpareNodeUnavailable`
    AbilitySwapSpareNodeUnavailable,
    /// DCB value: `AbilitySwapTargetNodeCantBeSwapped`
    AbilitySwapTargetNodeCantBeSwapped,
    /// DCB value: `AbilitySwapTargetNodeIsUnrevealed`
    AbilitySwapTargetNodeIsUnrevealed,
    /// DCB value: `AbilityWrapTeleportWhileMoving`
    AbilityWrapTeleportWhileMoving,
    /// DCB value: `AbilityWrapAgentAlreadyTeleporting`
    AbilityWrapAgentAlreadyTeleporting,
    /// DCB value: `AbilityWrapDestinationNodeIsUnrevealed`
    AbilityWrapDestinationNodeIsUnrevealed,
    /// DCB value: `AbilityWrapNoTeleportationDirectionAvailable`
    AbilityWrapNoTeleportationDirectionAvailable,
    /// DCB value: `AbilityWrapInvalidDestination`
    AbilityWrapInvalidDestination,
    /// DCB value: `AbilityWrapAmbiguousDestination`
    AbilityWrapAmbiguousDestination,
    /// DCB value: `CodeInvalidCommandToken`
    CodeInvalidCommandToken,
    /// DCB value: `CodeTooManyArgs`
    CodeTooManyArgs,
    /// DCB value: `CodeTooLittleArgs`
    CodeTooLittleArgs,
    /// DCB value: `CodeInvalidArgType`
    CodeInvalidArgType,
    /// DCB value: `CodeInvalidArgFormat`
    CodeInvalidArgFormat,
    /// DCB value: `MovementCantMoveToUnrevealedNode`
    MovementCantMoveToUnrevealedNode,
    /// DCB value: `SpawnInappropriateGameState`
    SpawnInappropriateGameState,
    /// DCB value: `SpawnInvalidNodeSelected`
    SpawnInvalidNodeSelected,
    /// DCB value: `UnknownError`
    UnknownError,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingFlagOverrideType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingFlagOverrideType {
    /// DCB value: `UsePreset`
    UsePreset,
    /// DCB value: `False`
    False,
    /// DCB value: `True`
    True,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHackingParamsAbilityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHackingParamsAbilityType {
    /// DCB value: `Invalid`
    Invalid,
    /// DCB value: `NodeSwap`
    NodeSwap,
    /// DCB value: `Ping`
    Ping,
    /// DCB value: `Inject`
    Inject,
    /// DCB value: `Slowdown`
    Slowdown,
    /// DCB value: `WrapAround`
    WrapAround,
    /// DCB value: `Any`
    Any,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHandMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHandMode {
    /// DCB value: `LeftHand`
    LeftHand,
    /// DCB value: `RightHand`
    RightHand,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHandholdAttachOrientationSpace`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHandholdAttachOrientationSpace {
    /// DCB value: `ActorSpace`
    ActorSpace,
    /// DCB value: `HandholdSpace`
    HandholdSpace,
    /// DCB value: `EntitySpace`
    EntitySpace,
    /// DCB value: `ZoneSpace`
    ZoneSpace,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHazardTagListBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHazardTagListBehavior {
    /// DCB value: `OneTagIsRequired`
    OneTagIsRequired,
    /// DCB value: `OneTagWillExempt`
    OneTagWillExempt,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHeadWearHair`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHeadWearHair {
    /// DCB value: `any`
    any,
    /// DCB value: `snoopy_cap`
    snoopy_cap,
    /// DCB value: `hatHair`
    hatHair,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHealingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHealingMode {
    /// DCB value: `Target`
    Target,
    /// DCB value: `Self`
    Self_,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHealingValueType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHealingValueType {
    /// DCB value: `Health`
    Health,
    /// DCB value: `Buff`
    Buff,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHelmetState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHelmetState {
    /// DCB value: `NONE`
    NONE,
    /// DCB value: `Open`
    Open,
    /// DCB value: `Opening`
    Opening,
    /// DCB value: `Closed`
    Closed,
    /// DCB value: `Closing`
    Closing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHelmetStateMachine`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHelmetStateMachine {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Visor`
    Visor,
    /// DCB value: `Strap`
    Strap,
    /// DCB value: `EVA`
    EVA,
    /// DCB value: `Targeting`
    Targeting,
    /// DCB value: `Lights`
    Lights,
    /// DCB value: `Opacity`
    Opacity,
    /// DCB value: `Equip`
    Equip,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHitType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHitType {
    /// DCB value: `Invalid`
    Invalid,
    /// DCB value: `Melee`
    Melee,
    /// DCB value: `Collision`
    Collision,
    /// DCB value: `Crash`
    Crash,
    /// DCB value: `Frag`
    Frag,
    /// DCB value: `Explosion`
    Explosion,
    /// DCB value: `TakeDown`
    TakeDown,
    /// DCB value: `Punish`
    Punish,
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Fire`
    Fire,
    /// DCB value: `Bullet`
    Bullet,
    /// DCB value: `VehicleDestruction`
    VehicleDestruction,
    /// DCB value: `EventDamage`
    EventDamage,
    /// DCB value: `BleedOut`
    BleedOut,
    /// DCB value: `ElectricArc`
    ElectricArc,
    /// DCB value: `Repair`
    Repair,
    /// DCB value: `Suffocate`
    Suffocate,
    /// DCB value: `Suicide`
    Suicide,
    /// DCB value: `SelfDestruct`
    SelfDestruct,
    /// DCB value: `BoundaryViolation`
    BoundaryViolation,
    /// DCB value: `Drown`
    Drown,
    /// DCB value: `DamageOverTime`
    DamageOverTime,
    /// DCB value: `Hazard`
    Hazard,
    /// DCB value: `Extraction`
    Extraction,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHitmarkerPositionMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHitmarkerPositionMethod {
    /// DCB value: `ScreenCenter`
    ScreenCenter,
    /// DCB value: `CrosshairPosition`
    CrosshairPosition,
    /// DCB value: `CrosshairPositionSnapshot`
    CrosshairPositionSnapshot,
    /// DCB value: `HitpositionWorld`
    HitpositionWorld,
    /// DCB value: `HitpositionScreen`
    HitpositionScreen,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHoloFieldShape`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHoloFieldShape {
    /// DCB value: `HoloField_Mesh`
    HoloField_Mesh,
    /// DCB value: `HoloField_Sphere`
    HoloField_Sphere,
    /// DCB value: `HoloField_HardCodedSphere`
    HoloField_HardCodedSphere,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHolographicVolumeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHolographicVolumeType {
    /// DCB value: `Sphere`
    Sphere,
    /// DCB value: `Cube`
    Cube,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EHoverPowerStage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EHoverPowerStage {
    /// DCB value: `PoweredOff`
    PoweredOff,
    /// DCB value: `PoweringOff`
    PoweringOff,
    /// DCB value: `PoweringOn`
    PoweringOn,
    /// DCB value: `PoweredOn`
    PoweredOn,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSFormationModeMessage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSFormationModeMessage {
    /// DCB value: `LinkAvailable`
    LinkAvailable,
    /// DCB value: `MoveToHost`
    MoveToHost,
    /// DCB value: `ConsentToJoin`
    ConsentToJoin,
    /// DCB value: `Joined`
    Joined,
    /// DCB value: `CannotJoin`
    CannotJoin,
    /// DCB value: `Aborted`
    Aborted,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSFormationState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSFormationState {
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `Blocked`
    Blocked,
    /// DCB value: `Available_MoveToHost`
    Available_MoveToHost,
    /// DCB value: `Available_WaitForConsent`
    Available_WaitForConsent,
    /// DCB value: `Active_Transition`
    Active_Transition,
    /// DCB value: `Active_Settled`
    Active_Settled,
    /// DCB value: `Leaving`
    Leaving,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSModifiableNumbers`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSModifiableNumbers {
    /// DCB value: `TestNumber`
    TestNumber,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSModifiableNumbersLegacy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSModifiableNumbersLegacy {
    /// DCB value: `SCMSpeed`
    SCMSpeed,
    /// DCB value: `BoostSpeedForward`
    BoostSpeedForward,
    /// DCB value: `BoostSpeedBackward`
    BoostSpeedBackward,
    /// DCB value: `MaxSpeed`
    MaxSpeed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSModifiableVectors`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSModifiableVectors {
    /// DCB value: `TestVector`
    TestVector,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSModifiableVectorsLegacy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSModifiableVectorsLegacy {
    /// DCB value: `MaxAngularVelocity`
    MaxAngularVelocity,
    /// DCB value: `PositiveLinearScale`
    PositiveLinearScale,
    /// DCB value: `NegativeLinearScale`
    NegativeLinearScale,
    /// DCB value: `PositiveAngularScale`
    PositiveAngularScale,
    /// DCB value: `NegativeAngularScale`
    NegativeAngularScale,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIFCSModifierType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIFCSModifierType {
    /// DCB value: `Additive`
    Additive,
    /// DCB value: `Scalar`
    Scalar,
    /// DCB value: `Override`
    Override,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIfcsEspType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIfcsEspType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `DefaultAds`
    DefaultAds,
    /// DCB value: `Strong`
    Strong,
    /// DCB value: `StrongAds`
    StrongAds,
    /// DCB value: `Disabled`
    Disabled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EIfcsPhysicsDampingType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EIfcsPhysicsDampingType {
    /// DCB value: `Destroyed`
    Destroyed,
    /// DCB value: `InQuantum`
    InQuantum,
    /// DCB value: `ThrustAvailable`
    ThrustAvailable,
    /// DCB value: `ThrustUnavailable`
    ThrustUnavailable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EImpoundingTrigger`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EImpoundingTrigger {
    /// DCB value: `IllegalParking`
    IllegalParking,
    /// DCB value: `PadRamming`
    PadRamming,
    /// DCB value: `TrespassImpound`
    TrespassImpound,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInfractionTrigger`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInfractionTrigger {
    /// DCB value: `KillActor`
    KillActor,
    /// DCB value: `ForcedIntoDowned`
    ForcedIntoDowned,
    /// DCB value: `DownedDirectDamage`
    DownedDirectDamage,
    /// DCB value: `Manslaughter`
    Manslaughter,
    /// DCB value: `DestroyVehicle`
    DestroyVehicle,
    /// DCB value: `DestroyEntity`
    DestroyEntity,
    /// DCB value: `IllegalParking`
    IllegalParking,
    /// DCB value: `Interdiction`
    Interdiction,
    /// DCB value: `FireWeapon`
    FireWeapon,
    /// DCB value: `RamVehicle`
    RamVehicle,
    /// DCB value: `RamActor`
    RamActor,
    /// DCB value: `PadRamming`
    PadRamming,
    /// DCB value: `AssaultActor`
    AssaultActor,
    /// DCB value: `AssaultLawEnforcement`
    AssaultLawEnforcement,
    /// DCB value: `DamageEntity`
    DamageEntity,
    /// DCB value: `GreenZonePropertyDamage`
    GreenZonePropertyDamage,
    /// DCB value: `KnockoutActor`
    KnockoutActor,
    /// DCB value: `MeleeActor`
    MeleeActor,
    /// DCB value: `Arrest`
    Arrest,
    /// DCB value: `PrisonEscape`
    PrisonEscape,
    /// DCB value: `PrisonSuicide`
    PrisonSuicide,
    /// DCB value: `KillPrisoner`
    KillPrisoner,
    /// DCB value: `Trespassing`
    Trespassing,
    /// DCB value: `State_Trespassing`
    State_Trespassing,
    /// DCB value: `State_Intruding`
    State_Intruding,
    /// DCB value: `State_WeaponDrawn`
    State_WeaponDrawn,
    /// DCB value: `State_HoldingIllegalItem`
    State_HoldingIllegalItem,
    /// DCB value: `State_IllegalVehicleTowing`
    State_IllegalVehicleTowing,
    /// DCB value: `LowBDL`
    LowBDL,
    /// DCB value: `HighBDL`
    HighBDL,
    /// DCB value: `HighBDLUnconscious`
    HighBDLUnconscious,
    /// DCB value: `RestrictedAreaTrespass`
    RestrictedAreaTrespass,
    /// DCB value: `RemoveItemFromCargoGrid`
    RemoveItemFromCargoGrid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInputPromptBoundTo`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInputPromptBoundTo {
    /// DCB value: `ActorEyes`
    ActorEyes,
    /// DCB value: `EntityRoot`
    EntityRoot,
    /// DCB value: `TopBoundingBoxZSurface`
    TopBoundingBoxZSurface,
    /// DCB value: `NearestBoundingBoxSurface`
    NearestBoundingBoxSurface,
    /// DCB value: `NearestBoundingBoxIgnoreFurthestSurfaces`
    NearestBoundingBoxIgnoreFurthestSurfaces,
    /// DCB value: `InteractionPointFixedOffset`
    InteractionPointFixedOffset,
    /// DCB value: `Tmp_AngleConstraintForwardDirection`
    Tmp_AngleConstraintForwardDirection,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInputPromptMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInputPromptMode {
    /// DCB value: `SingleClick`
    SingleClick,
    /// DCB value: `MultiClick`
    MultiClick,
    /// DCB value: `Hold`
    Hold,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInputPromptPriority`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInputPromptPriority {
    /// DCB value: `LowPriority`
    LowPriority,
    /// DCB value: `MediumPriority`
    MediumPriority,
    /// DCB value: `HighPriority`
    HighPriority,
    /// DCB value: `HighestPriority`
    HighestPriority,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInstanceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInstanceType {
    /// DCB value: `Entrance`
    Entrance,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInteractionConditionTargetEntity`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInteractionConditionTargetEntity {
    /// DCB value: `Interactor`
    Interactor,
    /// DCB value: `Interactable`
    Interactable,
    /// DCB value: `InteractableItemOwner`
    InteractableItemOwner,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EInteractionTriggerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EInteractionTriggerType {
    /// DCB value: `Self`
    Self_,
    /// DCB value: `Parent`
    Parent,
    /// DCB value: `Root`
    Root,
    /// DCB value: `Children`
    Children,
    /// DCB value: `FullHierarchy`
    FullHierarchy,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemActionEventType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemActionEventType {
    /// DCB value: `PowerOn`
    PowerOn,
    /// DCB value: `PowerOff`
    PowerOff,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemClass {
    /// DCB value: `Civilian`
    Civilian,
    /// DCB value: `Competition`
    Competition,
    /// DCB value: `Industrial`
    Industrial,
    /// DCB value: `Military`
    Military,
    /// DCB value: `Stealth`
    Stealth,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemFunctionalityCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemFunctionalityCondition {
    /// DCB value: `Damage`
    Damage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemPortAttachImplType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemPortAttachImplType {
    /// DCB value: `Bone`
    Bone,
    /// DCB value: `Skin`
    Skin,
    /// DCB value: `Face`
    Face,
    /// DCB value: `Entity`
    Entity,
    /// DCB value: `StatObj`
    StatObj,
    /// DCB value: `Noattach`
    Noattach,
    /// DCB value: `Logical`
    Logical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemPortAttachRotationLimitAxis`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemPortAttachRotationLimitAxis {
    /// DCB value: `None`
    None,
    /// DCB value: `XAxis`
    XAxis,
    /// DCB value: `YAxis`
    YAxis,
    /// DCB value: `ZAxis`
    ZAxis,
    /// DCB value: `XYZAxis`
    XYZAxis,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemPortConnectionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemPortConnectionType {
    /// DCB value: `NoConnection`
    NoConnection,
    /// DCB value: `DefaultConnection`
    DefaultConnection,
    /// DCB value: `ExteriorConnection`
    ExteriorConnection,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemPortPhysicsGridBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemPortPhysicsGridBehavior {
    /// DCB value: `Interior`
    Interior,
    /// DCB value: `Exterior`
    Exterior,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemResourceNegativeStates`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemResourceNegativeStates {
    /// DCB value: `eNone`
    eNone,
    /// DCB value: `eDamaged`
    eDamaged,
    /// DCB value: `eOverheating`
    eOverheating,
    /// DCB value: `eNoPower`
    eNoPower,
    /// DCB value: `eDistortion`
    eDistortion,
    /// DCB value: `eOverheatedShutDown`
    eOverheatedShutDown,
    /// DCB value: `eDestroyed`
    eDestroyed,
    /// DCB value: `eIgnited`
    eIgnited,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemShopAdjustmentMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemShopAdjustmentMode {
    /// DCB value: `NoAdjustment`
    NoAdjustment,
    /// DCB value: `Override`
    Override,
    /// DCB value: `Offset`
    Offset,
    /// DCB value: `Scale`
    Scale,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemShopReference`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemShopReference {
    /// DCB value: `Item`
    Item,
    /// DCB value: `Rack`
    Rack,
    /// DCB value: `InteractionPoint`
    InteractionPoint,
    /// DCB value: `PlayerCamera`
    PlayerCamera,
    /// DCB value: `PlayerActor`
    PlayerActor,
    /// DCB value: `Pedestal`
    Pedestal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemStatType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemStatType {
    /// DCB value: `Flight_IFCS_BoostSpeedBackward`
    Flight_IFCS_BoostSpeedBackward,
    /// DCB value: `Flight_IFCS_BoostSpeedForward`
    Flight_IFCS_BoostSpeedForward,
    /// DCB value: `Flight_IFCS_MaxSpeed`
    Flight_IFCS_MaxSpeed,
    /// DCB value: `Flight_IFCS_ScmSpeed`
    Flight_IFCS_ScmSpeed,
    /// DCB value: `Missile_ArmTime`
    Missile_ArmTime,
    /// DCB value: `Missile_LockRangeMax`
    Missile_LockRangeMax,
    /// DCB value: `Missile_LockRangeMin`
    Missile_LockRangeMin,
    /// DCB value: `Missile_LockTime`
    Missile_LockTime,
    /// DCB value: `Missile_TopSpeed`
    Missile_TopSpeed,
    /// DCB value: `Missile_TotalDamage`
    Missile_TotalDamage,
    /// DCB value: `QDrive_Acceleration_Boost`
    QDrive_Acceleration_Boost,
    /// DCB value: `QDrive_Acceleration_Linear`
    QDrive_Acceleration_Linear,
    /// DCB value: `QDrive_Counteraction_Boost`
    QDrive_Counteraction_Boost,
    /// DCB value: `QDrive_Counteraction_Linear`
    QDrive_Counteraction_Linear,
    /// DCB value: `QDrive_MaxSpeed_Boost`
    QDrive_MaxSpeed_Boost,
    /// DCB value: `QDrive_MaxSpeed_Linear`
    QDrive_MaxSpeed_Linear,
    /// DCB value: `Shield_ChargeToFull`
    Shield_ChargeToFull,
    /// DCB value: `Shield_DamagedRegenDelay`
    Shield_DamagedRegenDelay,
    /// DCB value: `Shield_Health`
    Shield_Health,
    /// DCB value: `Shield_Regen`
    Shield_Regen,
    /// DCB value: `Vehicle_Mass`
    Vehicle_Mass,
    /// DCB value: `Vehicle_SizeCount_Countermeasures`
    Vehicle_SizeCount_Countermeasures,
    /// DCB value: `Vehicle_SizeCount_Ordnance`
    Vehicle_SizeCount_Ordnance,
    /// DCB value: `Vehicle_SizeCount_Thrusters`
    Vehicle_SizeCount_Thrusters,
    /// DCB value: `Vehicle_SizeCount_Weapon`
    Vehicle_SizeCount_Weapon,
    /// DCB value: `Weapon_AmmoCapacity`
    Weapon_AmmoCapacity,
    /// DCB value: `Weapon_FireRate`
    Weapon_FireRate,
    /// DCB value: `Weapon_Velocity`
    Weapon_Velocity,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemSubType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemSubType {
    /// DCB value: `ADSComputer`
    ADSComputer,
    /// DCB value: `AirlockPart`
    AirlockPart,
    /// DCB value: `Ammo_1000mm`
    Ammo_1000mm,
    /// DCB value: `Ammo_20mm`
    Ammo_20mm,
    /// DCB value: `Ammo_24mm`
    Ammo_24mm,
    /// DCB value: `Ammo_25mm`
    Ammo_25mm,
    /// DCB value: `Ammo_28mm`
    Ammo_28mm,
    /// DCB value: `Ammo_30mm`
    Ammo_30mm,
    /// DCB value: `Ammo_35mm`
    Ammo_35mm,
    /// DCB value: `Ammo_40mm`
    Ammo_40mm,
    /// DCB value: `Ammo_50mm`
    Ammo_50mm,
    /// DCB value: `Ammo_60mm`
    Ammo_60mm,
    /// DCB value: `Ammo_Rail_60mm`
    Ammo_Rail_60mm,
    /// DCB value: `Ammo_Rail_80mm`
    Ammo_Rail_80mm,
    /// DCB value: `AmmoBox_Ballistic_120rd_106mm_exp`
    AmmoBox_Ballistic_120rd_106mm_exp,
    /// DCB value: `Armor`
    Armor,
    /// DCB value: `Autopilot`
    Autopilot,
    /// DCB value: `Awesome`
    Awesome,
    /// DCB value: `BallTurret`
    BallTurret,
    /// DCB value: `Bar`
    Bar,
    /// DCB value: `Barrel`
    Barrel,
    /// DCB value: `BombRack`
    BombRack,
    /// DCB value: `Bottle`
    Bottle,
    /// DCB value: `BottomAttachment`
    BottomAttachment,
    /// DCB value: `BottomTurret`
    BottomTurret,
    /// DCB value: `Box`
    Box,
    /// DCB value: `Can`
    Can,
    /// DCB value: `CanardTurret`
    CanardTurret,
    /// DCB value: `Cargo`
    Cargo,
    /// DCB value: `Cockpit_Audio`
    Cockpit_Audio,
    /// DCB value: `Constellation`
    Constellation,
    /// DCB value: `Consumable`
    Consumable,
    /// DCB value: `CountermeasureLauncher`
    CountermeasureLauncher,
    /// DCB value: `CPU`
    CPU,
    /// DCB value: `Credit`
    Credit,
    /// DCB value: `Default`
    Default,
    /// DCB value: `delta`
    delta,
    /// DCB value: `DoorPart`
    DoorPart,
    /// DCB value: `External`
    External,
    /// DCB value: `EyeWare`
    EyeWare,
    /// DCB value: `Female`
    Female,
    /// DCB value: `Female_Kid`
    Female_Kid,
    /// DCB value: `FiringMechanism`
    FiringMechanism,
    /// DCB value: `FixedThruster`
    FixedThruster,
    /// DCB value: `Flair_Hanging`
    Flair_Hanging,
    /// DCB value: `Flair_Wall_Picture`
    Flair_Wall_Picture,
    /// DCB value: `Flair_Wall_Interaction`
    Flair_Wall_Interaction,
    /// DCB value: `Flair_Static`
    Flair_Static,
    /// DCB value: `Flair_Surface_Clutter`
    Flair_Surface_Clutter,
    /// DCB value: `Flair_Surface_DisplayCase`
    Flair_Surface_DisplayCase,
    /// DCB value: `Flair_Surface_HoloViewer`
    Flair_Surface_HoloViewer,
    /// DCB value: `Flair_Floor_WallAligned`
    Flair_Floor_WallAligned,
    /// DCB value: `Flashlight`
    Flashlight,
    /// DCB value: `FlexThruster`
    FlexThruster,
    /// DCB value: `Fluid`
    Fluid,
    /// DCB value: `Fuel`
    Fuel,
    /// DCB value: `Fuse`
    Fuse,
    /// DCB value: `Gadget`
    Gadget,
    /// DCB value: `ghostHornet`
    ghostHornet,
    /// DCB value: `Glass`
    Glass,
    /// DCB value: `Grapple`
    Grapple,
    /// DCB value: `Grenade`
    Grenade,
    /// DCB value: `GroundVehicleMissile`
    GroundVehicleMissile,
    /// DCB value: `GroundVehicleMissileRack`
    GroundVehicleMissileRack,
    /// DCB value: `Gun`
    Gun,
    /// DCB value: `GunTurret`
    GunTurret,
    /// DCB value: `Hacking`
    Hacking,
    /// DCB value: `Handheld`
    Handheld,
    /// DCB value: `Harvestable`
    Harvestable,
    /// DCB value: `Hat`
    Hat,
    /// DCB value: `Helmet`
    Helmet,
    /// DCB value: `Heavy`
    Heavy,
    /// DCB value: `Idris`
    Idris,
    /// DCB value: `Idris_Turret`
    Idris_Turret,
    /// DCB value: `Interior_Audio`
    Interior_Audio,
    /// DCB value: `Inventory_Container`
    Inventory_Container,
    /// DCB value: `IronSight`
    IronSight,
    /// DCB value: `JetPack`
    JetPack,
    /// DCB value: `JointThruster`
    JointThruster,
    /// DCB value: `JumpDrive`
    JumpDrive,
    /// DCB value: `Junk`
    Junk,
    /// DCB value: `Knife`
    Knife,
    /// DCB value: `Kopion`
    Kopion,
    /// DCB value: `LandingSystem`
    LandingSystem,
    /// DCB value: `Large`
    Large,
    /// DCB value: `legs`
    legs,
    /// DCB value: `Light`
    Light,
    /// DCB value: `LightArmor`
    LightArmor,
    /// DCB value: `ln`
    ln,
    /// DCB value: `LongRangeRadar`
    LongRangeRadar,
    /// DCB value: `Magazine`
    Magazine,
    /// DCB value: `Male`
    Male,
    /// DCB value: `Male_Kid`
    Male_Kid,
    /// DCB value: `MannedTurret`
    MannedTurret,
    /// DCB value: `Marok`
    Marok,
    /// DCB value: `Medical`
    Medical,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `MedPack`
    MedPack,
    /// DCB value: `MeleeMedium`
    MeleeMedium,
    /// DCB value: `MidRangeRadar`
    MidRangeRadar,
    /// DCB value: `Mineable`
    Mineable,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `MissileRack`
    MissileRack,
    /// DCB value: `MissileTurret`
    MissileTurret,
    /// DCB value: `Mission`
    Mission,
    /// DCB value: `Motherboard`
    Motherboard,
    /// DCB value: `NoseMounted`
    NoseMounted,
    /// DCB value: `Oxygen`
    Oxygen,
    /// DCB value: `OxygenCap`
    OxygenCap,
    /// DCB value: `PDCTurret`
    PDCTurret,
    /// DCB value: `Personal`
    Personal,
    /// DCB value: `Pilot`
    Pilot,
    /// DCB value: `Plant`
    Plant,
    /// DCB value: `Power`
    Power,
    /// DCB value: `PowerArray`
    PowerArray,
    /// DCB value: `Power_Idris`
    Power_Idris,
    /// DCB value: `Prop`
    Prop,
    /// DCB value: `QDrive`
    QDrive,
    /// DCB value: `QuantumFuel`
    QuantumFuel,
    /// DCB value: `QuasiGrazer`
    QuasiGrazer,
    /// DCB value: `Radar`
    Radar,
    /// DCB value: `Retaliator`
    Retaliator,
    /// DCB value: `Retro`
    Retro,
    /// DCB value: `Rocket`
    Rocket,
    /// DCB value: `Sachet`
    Sachet,
    /// DCB value: `SalvageModifier_TractorBeam`
    SalvageModifier_TractorBeam,
    /// DCB value: `Scanner`
    Scanner,
    /// DCB value: `ShortRangeRadar`
    ShortRangeRadar,
    /// DCB value: `SignatureReductor`
    SignatureReductor,
    /// DCB value: `SkinTest`
    SkinTest,
    /// DCB value: `Small`
    Small,
    /// DCB value: `SpaceMineRack`
    SpaceMineRack,
    /// DCB value: `Stormwal`
    Stormwal,
    /// DCB value: `superHornet`
    superHornet,
    /// DCB value: `SystemAccess`
    SystemAccess,
    /// DCB value: `TargetingComputer`
    TargetingComputer,
    /// DCB value: `ThrusterPack`
    ThrusterPack,
    /// DCB value: `Tin`
    Tin,
    /// DCB value: `TopTurret`
    TopTurret,
    /// DCB value: `Torpedo`
    Torpedo,
    /// DCB value: `Trophy`
    Trophy,
    /// DCB value: `Unmanned`
    Unmanned,
    /// DCB value: `Utility`
    Utility,
    /// DCB value: `Vanduul`
    Vanduul,
    /// DCB value: `Xian`
    Xian,
    /// DCB value: `Valakkar`
    Valakkar,
    /// DCB value: `VectorThruster`
    VectorThruster,
    /// DCB value: `Vehicle_Boat`
    Vehicle_Boat,
    /// DCB value: `Vehicle_GroundVehicle`
    Vehicle_GroundVehicle,
    /// DCB value: `Vehicle_PowerSuit`
    Vehicle_PowerSuit,
    /// DCB value: `Vehicle_Spaceship`
    Vehicle_Spaceship,
    /// DCB value: `Ventilation`
    Ventilation,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `WeaponControl`
    WeaponControl,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EItemType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EItemType {
    /// DCB value: `AIModule`
    AIModule,
    /// DCB value: `AirTrafficController`
    AirTrafficController,
    /// DCB value: `AmmoBox`
    AmmoBox,
    /// DCB value: `AmmoCrate`
    AmmoCrate,
    /// DCB value: `Armor`
    Armor,
    /// DCB value: `Audio`
    Audio,
    /// DCB value: `Avionics`
    Avionics,
    /// DCB value: `AttachedPart`
    AttachedPart,
    /// DCB value: `Battery`
    Battery,
    /// DCB value: `BoatController`
    BoatController,
    /// DCB value: `BodyArmor`
    BodyArmor,
    /// DCB value: `Bomb`
    Bomb,
    /// DCB value: `BombLauncher`
    BombLauncher,
    /// DCB value: `Bottle`
    Bottle,
    /// DCB value: `Button`
    Button,
    /// DCB value: `CapacitorAssignmentController`
    CapacitorAssignmentController,
    /// DCB value: `Cargo`
    Cargo,
    /// DCB value: `CargoGrid`
    CargoGrid,
    /// DCB value: `Char_Accessory_Eyes`
    Char_Accessory_Eyes,
    /// DCB value: `Char_Accessory_Head`
    Char_Accessory_Head,
    /// DCB value: `Char_Armor_Arms`
    Char_Armor_Arms,
    /// DCB value: `Char_Armor_Feet`
    Char_Armor_Feet,
    /// DCB value: `Char_Armor_Helmet`
    Char_Armor_Helmet,
    /// DCB value: `Char_Armor_Legs`
    Char_Armor_Legs,
    /// DCB value: `Char_Armor_Torso`
    Char_Armor_Torso,
    /// DCB value: `Char_Armor_Undersuit`
    Char_Armor_Undersuit,
    /// DCB value: `Char_Armor_Backpack`
    Char_Armor_Backpack,
    /// DCB value: `Char_Body`
    Char_Body,
    /// DCB value: `Char_Clothing_Feet`
    Char_Clothing_Feet,
    /// DCB value: `Char_Clothing_Hands`
    Char_Clothing_Hands,
    /// DCB value: `Char_Clothing_Hat`
    Char_Clothing_Hat,
    /// DCB value: `Char_Clothing_Mask`
    Char_Clothing_Mask,
    /// DCB value: `Char_Clothing_Jumpsuit`
    Char_Clothing_Jumpsuit,
    /// DCB value: `Char_Clothing_Legs`
    Char_Clothing_Legs,
    /// DCB value: `Char_Clothing_Torso_0`
    Char_Clothing_Torso_0,
    /// DCB value: `Char_Clothing_Torso_1`
    Char_Clothing_Torso_1,
    /// DCB value: `Char_Clothing_Torso_2`
    Char_Clothing_Torso_2,
    /// DCB value: `Char_Clothing_Undershirt`
    Char_Clothing_Undershirt,
    /// DCB value: `Char_Clothing_Backpack`
    Char_Clothing_Backpack,
    /// DCB value: `Char_Flair`
    Char_Flair,
    /// DCB value: `Char_Hair_Color`
    Char_Hair_Color,
    /// DCB value: `Char_Hair_Prop`
    Char_Hair_Prop,
    /// DCB value: `Char_Head`
    Char_Head,
    /// DCB value: `Char_Head_Beard`
    Char_Head_Beard,
    /// DCB value: `Char_Head_Eyebrow`
    Char_Head_Eyebrow,
    /// DCB value: `Char_Head_Eyelash`
    Char_Head_Eyelash,
    /// DCB value: `Char_Head_Eyes`
    Char_Head_Eyes,
    /// DCB value: `Char_Head_Hair`
    Char_Head_Hair,
    /// DCB value: `Char_Head_Stubble`
    Char_Head_Stubble,
    /// DCB value: `Char_Head_Piercings`
    Char_Head_Piercings,
    /// DCB value: `Char_Lens`
    Char_Lens,
    /// DCB value: `Char_Skin_Color`
    Char_Skin_Color,
    /// DCB value: `Cloth`
    Cloth,
    /// DCB value: `CommsController`
    CommsController,
    /// DCB value: `Container`
    Container,
    /// DCB value: `ControlPanel`
    ControlPanel,
    /// DCB value: `Cooler`
    Cooler,
    /// DCB value: `CoolerController`
    CoolerController,
    /// DCB value: `Crafter`
    Crafter,
    /// DCB value: `Creature_Body`
    Creature_Body,
    /// DCB value: `Creature_Hair`
    Creature_Hair,
    /// DCB value: `Creature_Harvest`
    Creature_Harvest,
    /// DCB value: `Currency`
    Currency,
    /// DCB value: `Debris`
    Debris,
    /// DCB value: `Decal`
    Decal,
    /// DCB value: `Display`
    Display,
    /// DCB value: `DockingAnimator`
    DockingAnimator,
    /// DCB value: `DockingCollar`
    DockingCollar,
    /// DCB value: `DockingController`
    DockingController,
    /// DCB value: `Door`
    Door,
    /// DCB value: `DoorController`
    DoorController,
    /// DCB value: `Drink`
    Drink,
    /// DCB value: `Elevator`
    Elevator,
    /// DCB value: `EMP`
    EMP,
    /// DCB value: `EnergyController`
    EnergyController,
    /// DCB value: `ExternalFuelTank`
    ExternalFuelTank,
    /// DCB value: `Flair_Cockpit`
    Flair_Cockpit,
    /// DCB value: `Flair_Floor`
    Flair_Floor,
    /// DCB value: `Flair_Surface`
    Flair_Surface,
    /// DCB value: `Flair_Wall`
    Flair_Wall,
    /// DCB value: `FlightController`
    FlightController,
    /// DCB value: `Food`
    Food,
    /// DCB value: `FPS_AttachmentBarrel`
    FPS_AttachmentBarrel,
    /// DCB value: `FPS_AttachmentBottom`
    FPS_AttachmentBottom,
    /// DCB value: `FPS_AttachmentOptics`
    FPS_AttachmentOptics,
    /// DCB value: `FPS_Consumable`
    FPS_Consumable,
    /// DCB value: `FPS_Cooler`
    FPS_Cooler,
    /// DCB value: `FPS_Deployable`
    FPS_Deployable,
    /// DCB value: `FPS_Magazine`
    FPS_Magazine,
    /// DCB value: `FPS_PowerGen`
    FPS_PowerGen,
    /// DCB value: `FPS_Radar`
    FPS_Radar,
    /// DCB value: `FPS_Scanner`
    FPS_Scanner,
    /// DCB value: `FPS_Throwable`
    FPS_Throwable,
    /// DCB value: `FPS_WeaponMelee`
    FPS_WeaponMelee,
    /// DCB value: `FPS_WeaponShouldered`
    FPS_WeaponShouldered,
    /// DCB value: `FPS_WeaponSidearm`
    FPS_WeaponSidearm,
    /// DCB value: `FPS_WeaponStocked`
    FPS_WeaponStocked,
    /// DCB value: `FPS_WeaponUtility`
    FPS_WeaponUtility,
    /// DCB value: `FuelController`
    FuelController,
    /// DCB value: `FuelIntake`
    FuelIntake,
    /// DCB value: `FuelNozzle`
    FuelNozzle,
    /// DCB value: `FuelTank`
    FuelTank,
    /// DCB value: `Gadget`
    Gadget,
    /// DCB value: `GravityGenerator`
    GravityGenerator,
    /// DCB value: `Grenade`
    Grenade,
    /// DCB value: `GroundVehicleMissileLauncher`
    GroundVehicleMissileLauncher,
    /// DCB value: `HangarExpansion`
    HangarExpansion,
    /// DCB value: `HangarStock`
    HangarStock,
    /// DCB value: `Interior`
    Interior,
    /// DCB value: `InventoryContainer`
    InventoryContainer,
    /// DCB value: `JumpDrive`
    JumpDrive,
    /// DCB value: `LandingSystem`
    LandingSystem,
    /// DCB value: `LandingGear`
    LandingGear,
    /// DCB value: `LifeSupportGenerator`
    LifeSupportGenerator,
    /// DCB value: `LifeSupportTank`
    LifeSupportTank,
    /// DCB value: `LifeSupportVent`
    LifeSupportVent,
    /// DCB value: `Light`
    Light,
    /// DCB value: `LightController`
    LightController,
    /// DCB value: `Lightgroup`
    Lightgroup,
    /// DCB value: `LiveSupport`
    LiveSupport,
    /// DCB value: `MaelstromPart`
    MaelstromPart,
    /// DCB value: `Magazine`
    Magazine,
    /// DCB value: `MainEngine`
    MainEngine,
    /// DCB value: `MainThruster`
    MainThruster,
    /// DCB value: `ManneuverThruster`
    ManneuverThruster,
    /// DCB value: `MiningController`
    MiningController,
    /// DCB value: `MiningModifier`
    MiningModifier,
    /// DCB value: `Misc`
    Misc,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `MissileController`
    MissileController,
    /// DCB value: `MissileLauncher`
    MissileLauncher,
    /// DCB value: `MobiGlas`
    MobiGlas,
    /// DCB value: `Module`
    Module,
    /// DCB value: `MultiLight`
    MultiLight,
    /// DCB value: `NOITEM_Player`
    NOITEM_Player,
    /// DCB value: `NOITEM_Vehicle`
    NOITEM_Vehicle,
    /// DCB value: `Paints`
    Paints,
    /// DCB value: `PersistentHab`
    PersistentHab,
    /// DCB value: `PersonalInnerThought`
    PersonalInnerThought,
    /// DCB value: `Ping`
    Ping,
    /// DCB value: `Player`
    Player,
    /// DCB value: `PowerPlant`
    PowerPlant,
    /// DCB value: `QuantumDrive`
    QuantumDrive,
    /// DCB value: `QuantumInterdictionGenerator`
    QuantumInterdictionGenerator,
    /// DCB value: `QuantumFuelTank`
    QuantumFuelTank,
    /// DCB value: `Radar`
    Radar,
    /// DCB value: `Relay`
    Relay,
    /// DCB value: `RemoteConnection`
    RemoteConnection,
    /// DCB value: `RemovableChip`
    RemovableChip,
    /// DCB value: `RemovableBlade`
    RemovableBlade,
    /// DCB value: `Room`
    Room,
    /// DCB value: `SalvageController`
    SalvageController,
    /// DCB value: `SalvageFieldEmitter`
    SalvageFieldEmitter,
    /// DCB value: `SalvageFieldSupporter`
    SalvageFieldSupporter,
    /// DCB value: `SalvageFillerStation`
    SalvageFillerStation,
    /// DCB value: `SalvageHead`
    SalvageHead,
    /// DCB value: `SalvageInternalStorage`
    SalvageInternalStorage,
    /// DCB value: `SalvageModifier`
    SalvageModifier,
    /// DCB value: `Scanner`
    Scanner,
    /// DCB value: `Seat`
    Seat,
    /// DCB value: `SeatAccess`
    SeatAccess,
    /// DCB value: `SeatDashboard`
    SeatDashboard,
    /// DCB value: `SelfDestruct`
    SelfDestruct,
    /// DCB value: `Sensor`
    Sensor,
    /// DCB value: `Shield`
    Shield,
    /// DCB value: `ShieldController`
    ShieldController,
    /// DCB value: `Ship`
    Ship,
    /// DCB value: `ShopDisplay`
    ShopDisplay,
    /// DCB value: `SpaceMine`
    SpaceMine,
    /// DCB value: `SpaceMineLauncher`
    SpaceMineLauncher,
    /// DCB value: `StatusScreen`
    StatusScreen,
    /// DCB value: `Suit`
    Suit,
    /// DCB value: `TargetSelector`
    TargetSelector,
    /// DCB value: `Thumbnail`
    Thumbnail,
    /// DCB value: `ToolArm`
    ToolArm,
    /// DCB value: `TowingBeam`
    TowingBeam,
    /// DCB value: `TractorBeam`
    TractorBeam,
    /// DCB value: `Transponder`
    Transponder,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `TurretBase`
    TurretBase,
    /// DCB value: `Usable`
    Usable,
    /// DCB value: `UtilityTurret`
    UtilityTurret,
    /// DCB value: `Visor`
    Visor,
    /// DCB value: `WeaponAttachment`
    WeaponAttachment,
    /// DCB value: `WeaponController`
    WeaponController,
    /// DCB value: `WeaponDefensive`
    WeaponDefensive,
    /// DCB value: `WeaponGun`
    WeaponGun,
    /// DCB value: `WeaponMining`
    WeaponMining,
    /// DCB value: `WeaponPersonal`
    WeaponPersonal,
    /// DCB value: `WeaponRegenPool`
    WeaponRegenPool,
    /// DCB value: `WheeledController`
    WheeledController,
    /// DCB value: `WeaponMount`
    WeaponMount,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EKnockbackBodyPart`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EKnockbackBodyPart {
    /// DCB value: `Indirect`
    Indirect,
    /// DCB value: `Head`
    Head,
    /// DCB value: `Torso`
    Torso,
    /// DCB value: `LeftArm`
    LeftArm,
    /// DCB value: `RightArm`
    RightArm,
    /// DCB value: `LeftLeg`
    LeftLeg,
    /// DCB value: `RightLeg`
    RightLeg,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EKnockbackHitType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EKnockbackHitType {
    /// DCB value: `Physics`
    Physics,
    /// DCB value: `Projectile`
    Projectile,
    /// DCB value: `Melee`
    Melee,
    /// DCB value: `Explosion`
    Explosion,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELandClaimType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELandClaimType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Prospecting`
    Prospecting,
    /// DCB value: `Beacon`
    Beacon,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELandingAreaCanBeUsedBy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELandingAreaCanBeUsedBy {
    /// DCB value: `None`
    None,
    /// DCB value: `AI`
    AI,
    /// DCB value: `Player`
    Player,
    /// DCB value: `All`
    All,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELawLicenseType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELawLicenseType {
    /// DCB value: `IllegalGoodsLicense_ClassA`
    IllegalGoodsLicense_ClassA,
    /// DCB value: `IllegalGoodsLicense_ClassB`
    IllegalGoodsLicense_ClassB,
    /// DCB value: `IllegalGoodsLicense_ClassC`
    IllegalGoodsLicense_ClassC,
    /// DCB value: `IllegalGoodsLicense_Prohibited`
    IllegalGoodsLicense_Prohibited,
    /// DCB value: `ArrestLicense`
    ArrestLicense,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELensDisplayMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELensDisplayMode {
    /// DCB value: `FPS`
    FPS,
    /// DCB value: `Kiosk`
    Kiosk,
    /// DCB value: `MobiGlas`
    MobiGlas,
    /// DCB value: `Multiplayer`
    Multiplayer,
    /// DCB value: `OverrideFPS`
    OverrideFPS,
    /// DCB value: `PIT`
    PIT,
    /// DCB value: `TrackView`
    TrackView,
    /// DCB value: `Tutorial`
    Tutorial,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `PauseMenu`
    PauseMenu,
    /// DCB value: `ADS`
    ADS,
    /// DCB value: `FiringRange`
    FiringRange,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELicenseType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELicenseType {
    /// DCB value: `Gold`
    Gold,
    /// DCB value: `Platinum`
    Platinum,
    /// DCB value: `GameMasterEventItem`
    GameMasterEventItem,
    /// DCB value: `LootableEventItem`
    LootableEventItem,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightAffectsGI`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightAffectsGI {
    /// DCB value: `On`
    On,
    /// DCB value: `Off`
    Off,
    /// DCB value: `InheritAffectsObjects`
    InheritAffectsObjects,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightEnabledWithGI`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightEnabledWithGI {
    /// DCB value: `Always`
    Always,
    /// DCB value: `WithGI`
    WithGI,
    /// DCB value: `WithoutGI`
    WithoutGI,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightImportance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightImportance {
    /// DCB value: `Decorative`
    Decorative,
    /// DCB value: `Standard`
    Standard,
    /// DCB value: `Secondary`
    Secondary,
    /// DCB value: `Key`
    Key,
    /// DCB value: `Cinematic`
    Cinematic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightState {
    /// DCB value: `Off`
    Off,
    /// DCB value: `Default`
    Default,
    /// DCB value: `Auxiliary`
    Auxiliary,
    /// DCB value: `Emergency`
    Emergency,
    /// DCB value: `Cinematic`
    Cinematic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightStateOverride`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightStateOverride {
    /// DCB value: `Off`
    Off,
    /// DCB value: `Default`
    Default,
    /// DCB value: `Auxiliary`
    Auxiliary,
    /// DCB value: `Emergency`
    Emergency,
    /// DCB value: `Cinematic`
    Cinematic,
    /// DCB value: `DisableOverride`
    DisableOverride,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELightType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELightType {
    /// DCB value: `Omni`
    Omni,
    /// DCB value: `SoftOmni`
    SoftOmni,
    /// DCB value: `Projector`
    Projector,
    /// DCB value: `Planar`
    Planar,
    /// DCB value: `Ambient`
    Ambient,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELinkMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELinkMode {
    /// DCB value: `NoLink`
    NoLink,
    /// DCB value: `FeetLink`
    FeetLink,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELoadingScreenType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELoadingScreenType {
    /// DCB value: `StarCitizen`
    StarCitizen,
    /// DCB value: `ElectronicAccess`
    ElectronicAccess,
    /// DCB value: `Squadron42`
    Squadron42,
    /// DCB value: `Frontend`
    Frontend,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELoadoutGroup`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELoadoutGroup {
    /// DCB value: `Body`
    Body,
    /// DCB value: `SkinTone`
    SkinTone,
    /// DCB value: `Head`
    Head,
    /// DCB value: `Brows`
    Brows,
    /// DCB value: `Eyes`
    Eyes,
    /// DCB value: `Hair`
    Hair,
    /// DCB value: `HairColour`
    HairColour,
    /// DCB value: `FacialHair`
    FacialHair,
    /// DCB value: `FacialHairColour`
    FacialHairColour,
    /// DCB value: `FaceAttachments1`
    FaceAttachments1,
    /// DCB value: `FaceAttachments2`
    FaceAttachments2,
    /// DCB value: `Piercings1`
    Piercings1,
    /// DCB value: `Piercings2`
    Piercings2,
    /// DCB value: `Tattoos`
    Tattoos,
    /// DCB value: `TattooFace`
    TattooFace,
    /// DCB value: `TattooBody`
    TattooBody,
    /// DCB value: `Undersuit`
    Undersuit,
    /// DCB value: `Helmet`
    Helmet,
    /// DCB value: `Torso`
    Torso,
    /// DCB value: `Torso2`
    Torso2,
    /// DCB value: `Torso3`
    Torso3,
    /// DCB value: `TorsoAttachments`
    TorsoAttachments,
    /// DCB value: `Outfit`
    Outfit,
    /// DCB value: `Outfit2`
    Outfit2,
    /// DCB value: `Outfit3`
    Outfit3,
    /// DCB value: `Outfit4`
    Outfit4,
    /// DCB value: `Arm`
    Arm,
    /// DCB value: `Leg`
    Leg,
    /// DCB value: `Feet`
    Feet,
    /// DCB value: `PrimaryWeapon`
    PrimaryWeapon,
    /// DCB value: `PrimaryOptics`
    PrimaryOptics,
    /// DCB value: `UnderBarrelAttachment`
    UnderBarrelAttachment,
    /// DCB value: `BarrelAttachment`
    BarrelAttachment,
    /// DCB value: `PrimaryAttachments`
    PrimaryAttachments,
    /// DCB value: `SecondaryWeapon`
    SecondaryWeapon,
    /// DCB value: `SecondaryOptics`
    SecondaryOptics,
    /// DCB value: `SecondaryAttachments`
    SecondaryAttachments,
    /// DCB value: `MeleeWeapon`
    MeleeWeapon,
    /// DCB value: `Grenades`
    Grenades,
    /// DCB value: `MedicalSupplies`
    MedicalSupplies,
    /// DCB value: `Item`
    Item,
    /// DCB value: `Mobiglas`
    Mobiglas,
    /// DCB value: `Utility1`
    Utility1,
    /// DCB value: `Utility2`
    Utility2,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELocationTypeLevel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELocationTypeLevel {
    /// DCB value: `SolarSystem`
    SolarSystem,
    /// DCB value: `Region`
    Region,
    /// DCB value: `LocalSystem`
    LocalSystem,
    /// DCB value: `PlanetOrMoon`
    PlanetOrMoon,
    /// DCB value: `POIOrCluster`
    POIOrCluster,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELootFullnessMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELootFullnessMode {
    /// DCB value: `stopAfterExceed`
    stopAfterExceed,
    /// DCB value: `preventExceed`
    preventExceed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELootPruningLevel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELootPruningLevel {
    /// DCB value: `none`
    none,
    /// DCB value: `containerSize`
    containerSize,
    /// DCB value: `fullnessTarget`
    fullnessTarget,
    /// DCB value: `dynamic`
    dynamic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ELootingDefaultInteractions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ELootingDefaultInteractions {
    /// DCB value: `OpenInventoryUI`
    OpenInventoryUI,
    /// DCB value: `OpenLootingUI`
    OpenLootingUI,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMagLaunchState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMagLaunchState {
    /// DCB value: `Off`
    Off,
    /// DCB value: `Lift`
    Lift,
    /// DCB value: `Launch`
    Launch,
    /// DCB value: `LaunchComplete`
    LaunchComplete,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMagRecoveryState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMagRecoveryState {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Deploying`
    Deploying,
    /// DCB value: `WaitingForTrap`
    WaitingForTrap,
    /// DCB value: `MovingToHover`
    MovingToHover,
    /// DCB value: `MovingDown`
    MovingDown,
    /// DCB value: `Landed`
    Landed,
    /// DCB value: `LandingCompletedEngineOff`
    LandingCompletedEngineOff,
    /// DCB value: `Retracting`
    Retracting,
    /// DCB value: `Done`
    Done,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMapId`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMapId {
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `EA_AstorsClearing`
    EA_AstorsClearing,
    /// DCB value: `EA_BloodshotRidge`
    EA_BloodshotRidge,
    /// DCB value: `EA_BrokenMoon`
    EA_BrokenMoon,
    /// DCB value: `EA_CaplanCircuit`
    EA_CaplanCircuit,
    /// DCB value: `EA_ClioIslands`
    EA_ClioIslands,
    /// DCB value: `EA_Crossroads`
    EA_Crossroads,
    /// DCB value: `EA_CryAstro_DC3412`
    EA_CryAstro_DC3412,
    /// DCB value: `EA_Daymar`
    EA_Daymar,
    /// DCB value: `EA_DaymarDunes`
    EA_DaymarDunes,
    /// DCB value: `EA_Demien`
    EA_Demien,
    /// DCB value: `EA_DemienComms`
    EA_DemienComms,
    /// DCB value: `EA_DogfightTest`
    EA_DogfightTest,
    /// DCB value: `EA_DunlowDerby`
    EA_DunlowDerby,
    /// DCB value: `EA_DyingStar`
    EA_DyingStar,
    /// DCB value: `EA_Echo11`
    EA_Echo11,
    /// DCB value: `EA_EuterpeIcebreaker`
    EA_EuterpeIcebreaker,
    /// DCB value: `EA_EZHab`
    EA_EZHab,
    /// DCB value: `EA_Gundo`
    EA_Gundo,
    /// DCB value: `EA_HurstonGroundArena`
    EA_HurstonGroundArena,
    /// DCB value: `EA_JerichoStation`
    EA_JerichoStation,
    /// DCB value: `EA_Kareah`
    EA_Kareah,
    /// DCB value: `EA_LorvilleOutskirts`
    EA_LorvilleOutskirts,
    /// DCB value: `EA_MagdaGroundArena`
    EA_MagdaGroundArena,
    /// DCB value: `EA_MakersPoint`
    EA_MakersPoint,
    /// DCB value: `EA_MicroTechRiver`
    EA_MicroTechRiver,
    /// DCB value: `EA_MinersLament`
    EA_MinersLament,
    /// DCB value: `EA_Ministry`
    EA_Ministry,
    /// DCB value: `EA_NHS_Arena`
    EA_NHS_Arena,
    /// DCB value: `EA_NHS_DeffordLink`
    EA_NHS_DeffordLink,
    /// DCB value: `EA_NHS_HalloranCircuit`
    EA_NHS_HalloranCircuit,
    /// DCB value: `EA_NHS_OldVanderval`
    EA_NHS_OldVanderval,
    /// DCB value: `EA_NHS_Rikkord`
    EA_NHS_Rikkord,
    /// DCB value: `EA_NHS_Wetlands`
    EA_NHS_Wetlands,
    /// DCB value: `EA_Pyro2`
    EA_Pyro2,
    /// DCB value: `EA_PyroJump`
    EA_PyroJump,
    /// DCB value: `EA_ScenarioTwo`
    EA_ScenarioTwo,
    /// DCB value: `EA_TheGoodDr`
    EA_TheGoodDr,
    /// DCB value: `EA_TheSkyScraper`
    EA_TheSkyScraper,
    /// DCB value: `EA_TheSnakePit`
    EA_TheSnakePit,
    /// DCB value: `EA_TheSnakePit_Reverse`
    EA_TheSnakePit_Reverse,
    /// DCB value: `EA_RayariStation`
    EA_RayariStation,
    /// DCB value: `EA_YadarValley`
    EA_YadarValley,
    /// DCB value: `EA_YelaGroundArena`
    EA_YelaGroundArena,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMarkerProviders`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMarkerProviders {
    /// DCB value: `Grenades`
    Grenades,
    /// DCB value: `Player`
    Player,
    /// DCB value: `UnattendedVehicle`
    UnattendedVehicle,
    /// DCB value: `VehicleEntrance`
    VehicleEntrance,
    /// DCB value: `ObjectDataBank`
    ObjectDataBank,
    /// DCB value: `NavigationWaypoints`
    NavigationWaypoints,
    /// DCB value: `LocationPins`
    LocationPins,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMasterMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMasterMode {
    /// DCB value: `Invalid`
    Invalid,
    /// DCB value: `Navigation`
    Navigation,
    /// DCB value: `SCM`
    SCM,
    /// DCB value: `Stealth`
    Stealth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMatchCycleType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMatchCycleType {
    /// DCB value: `None`
    None,
    /// DCB value: `NewServer`
    NewServer,
    /// DCB value: `RestartLevel`
    RestartLevel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMatchNetworkType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMatchNetworkType {
    /// DCB value: `Offline`
    Offline,
    /// DCB value: `Online`
    Online,
    /// DCB value: `Custom`
    Custom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMeshChunks`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMeshChunks {
    /// DCB value: `vneck_zone`
    vneck_zone,
    /// DCB value: `torso01_zone`
    torso01_zone,
    /// DCB value: `torso02_zone`
    torso02_zone,
    /// DCB value: `torso03_zone`
    torso03_zone,
    /// DCB value: `torso04_zone`
    torso04_zone,
    /// DCB value: `head_zone`
    head_zone,
    /// DCB value: `hips_zone`
    hips_zone,
    /// DCB value: `underwear_top_zone`
    underwear_top_zone,
    /// DCB value: `underwear_zone`
    underwear_zone,
    /// DCB value: `pants_acc_zone`
    pants_acc_zone,
    /// DCB value: `shirt_acc_zone`
    shirt_acc_zone,
    /// DCB value: `l_shoulder_zone`
    l_shoulder_zone,
    /// DCB value: `l_arm01_zone`
    l_arm01_zone,
    /// DCB value: `l_arm02_zone`
    l_arm02_zone,
    /// DCB value: `l_arm03_zone`
    l_arm03_zone,
    /// DCB value: `l_arm04_zone`
    l_arm04_zone,
    /// DCB value: `l_arm05_zone`
    l_arm05_zone,
    /// DCB value: `l_arm05_body_zone`
    l_arm05_body_zone,
    /// DCB value: `l_arm05_torso0_zone`
    l_arm05_torso0_zone,
    /// DCB value: `l_hand_zone`
    l_hand_zone,
    /// DCB value: `l_leg01_zone`
    l_leg01_zone,
    /// DCB value: `l_leg02_zone`
    l_leg02_zone,
    /// DCB value: `l_leg03_zone`
    l_leg03_zone,
    /// DCB value: `l_leg04_zone`
    l_leg04_zone,
    /// DCB value: `l_foot_zone`
    l_foot_zone,
    /// DCB value: `l_eye_zone`
    l_eye_zone,
    /// DCB value: `r_shoulder_zone`
    r_shoulder_zone,
    /// DCB value: `r_arm01_zone`
    r_arm01_zone,
    /// DCB value: `r_arm02_zone`
    r_arm02_zone,
    /// DCB value: `r_arm03_zone`
    r_arm03_zone,
    /// DCB value: `r_arm04_zone`
    r_arm04_zone,
    /// DCB value: `r_arm05_zone`
    r_arm05_zone,
    /// DCB value: `r_arm05_body_zone`
    r_arm05_body_zone,
    /// DCB value: `r_arm05_torso0_zone`
    r_arm05_torso0_zone,
    /// DCB value: `r_hand_zone`
    r_hand_zone,
    /// DCB value: `r_leg01_zone`
    r_leg01_zone,
    /// DCB value: `r_leg02_zone`
    r_leg02_zone,
    /// DCB value: `r_leg03_zone`
    r_leg03_zone,
    /// DCB value: `r_leg04_zone`
    r_leg04_zone,
    /// DCB value: `r_foot_zone`
    r_foot_zone,
    /// DCB value: `r_eye_zone`
    r_eye_zone,
    /// DCB value: `omega_core_zone`
    omega_core_zone,
    /// DCB value: `omega_arms_zone`
    omega_arms_zone,
    /// DCB value: `omega_legs_zone`
    omega_legs_zone,
    /// DCB value: `theta_head_zone`
    theta_head_zone,
    /// DCB value: `theta_core_zone`
    theta_core_zone,
    /// DCB value: `theta_arms_zone`
    theta_arms_zone,
    /// DCB value: `theta_legs_zone`
    theta_legs_zone,
    /// DCB value: `theta_l_arm_zone`
    theta_l_arm_zone,
    /// DCB value: `theta_r_arm_zone`
    theta_r_arm_zone,
    /// DCB value: `theta_l_leg_zone`
    theta_l_leg_zone,
    /// DCB value: `theta_r_leg_zone`
    theta_r_leg_zone,
    /// DCB value: `theta_body_chest_zone`
    theta_body_chest_zone,
    /// DCB value: `pcg_eyebrows`
    pcg_eyebrows,
    /// DCB value: `pcg_r_ear`
    pcg_r_ear,
    /// DCB value: `pcg_l_ear`
    pcg_l_ear,
    /// DCB value: `pcg_nose`
    pcg_nose,
    /// DCB value: `pcg_mouth`
    pcg_mouth,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMisfireType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMisfireType {
    /// DCB value: `None`
    None,
    /// DCB value: `Minor`
    Minor,
    /// DCB value: `Major`
    Major,
    /// DCB value: `Critical`
    Critical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMissionObjectiveCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMissionObjectiveCategory {
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Optional`
    Optional,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMissionPhaseStates`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMissionPhaseStates {
    /// DCB value: `NotStarted`
    NotStarted,
    /// DCB value: `Active`
    Active,
    /// DCB value: `Completed`
    Completed,
    /// DCB value: `Failed`
    Failed,
    /// DCB value: `Abandoned`
    Abandoned,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMissionResult`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMissionResult {
    /// DCB value: `Completed`
    Completed,
    /// DCB value: `InProgress`
    InProgress,
    /// DCB value: `Failed`
    Failed,
    /// DCB value: `Abandoned`
    Abandoned,
    /// DCB value: `Withdrawn`
    Withdrawn,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMobiGlasAfterActionReportRank`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMobiGlasAfterActionReportRank {
    /// DCB value: `Acceptable`
    Acceptable,
    /// DCB value: `Good`
    Good,
    /// DCB value: `Great`
    Great,
    /// DCB value: `Extraordinary`
    Extraordinary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMobiGlasPersonalStatusSkillsDisplayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMobiGlasPersonalStatusSkillsDisplayType {
    /// DCB value: `None`
    None,
    /// DCB value: `Attribute`
    Attribute,
    /// DCB value: `Technique`
    Technique,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EModifierSignatureType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EModifierSignatureType {
    /// DCB value: `GainPercentage`
    GainPercentage,
    /// DCB value: `LossPercentage`
    LossPercentage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EModuleType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EModuleType {
    /// DCB value: `PersistentUniverse`
    PersistentUniverse,
    /// DCB value: `ElectronicAccess`
    ElectronicAccess,
    /// DCB value: `Squadron42`
    Squadron42,
    /// DCB value: `Frontend`
    Frontend,
    /// DCB value: `Undefined`
    Undefined,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMolehillDecayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMolehillDecayType {
    /// DCB value: `ShrinkOrInflate`
    ShrinkOrInflate,
    /// DCB value: `FlattenOrBuild`
    FlattenOrBuild,
    /// DCB value: `SinkOrRise`
    SinkOrRise,
    /// DCB value: `PopInOut`
    PopInOut,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMotionProcessorLimiterType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMotionProcessorLimiterType {
    /// DCB value: `NormalizedUniform`
    NormalizedUniform,
    /// DCB value: `NormalizedUniformWithMaxRadius`
    NormalizedUniformWithMaxRadius,
    /// DCB value: `NormalizedUniformLateral`
    NormalizedUniformLateral,
    /// DCB value: `Ellipsoid`
    Ellipsoid,
    /// DCB value: `PerAxis`
    PerAxis,
    /// DCB value: `PerAxisWithMaxRadius`
    PerAxisWithMaxRadius,
    /// DCB value: `PerAxisLateral`
    PerAxisLateral,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMovementProcessor`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMovementProcessor {
    /// DCB value: `None`
    None,
    /// DCB value: `Basic`
    Basic,
    /// DCB value: `Locomotion`
    Locomotion,
    /// DCB value: `SubmergedCreature`
    SubmergedCreature,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EMyEnum`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EMyEnum {
    /// DCB value: `First`
    First,
    /// DCB value: `Second`
    Second,
    /// DCB value: `Third`
    Third,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ENavPointType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ENavPointType {
    /// DCB value: `General`
    General,
    /// DCB value: `QTTracePoint`
    QTTracePoint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ENavigationAgentType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ENavigationAgentType {
    /// DCB value: `MediumSizedCharacters`
    MediumSizedCharacters,
    /// DCB value: `LargeSizedCharacters`
    LargeSizedCharacters,
    /// DCB value: `VehicleMedium`
    VehicleMedium,
    /// DCB value: `MediumSizedSpaceships`
    MediumSizedSpaceships,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ENavigationLinkLinkingType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ENavigationLinkLinkingType {
    /// DCB value: `LinksInsideSameZone`
    LinksInsideSameZone,
    /// DCB value: `LinksInsideSameExternalZone`
    LinksInsideSameExternalZone,
    /// DCB value: `LinksToExternalZone`
    LinksToExternalZone,
    /// DCB value: `LinksFromExternalZone`
    LinksFromExternalZone,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EObjectDataBankEntryTrackerType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EObjectDataBankEntryTrackerType {
    /// DCB value: `None`
    None,
    /// DCB value: `LandingArea`
    LandingArea,
    /// DCB value: `FPSMineable`
    FPSMineable,
    /// DCB value: `Mineable`
    Mineable,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `Mission`
    Mission,
    /// DCB value: `NavPoint`
    NavPoint,
    /// DCB value: `PartyMember`
    PartyMember,
    /// DCB value: `QuantumTravel`
    QuantumTravel,
    /// DCB value: `RaceCheckpoint`
    RaceCheckpoint,
    /// DCB value: `Transponder`
    Transponder,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `Debris`
    Debris,
    /// DCB value: `Actor`
    Actor,
    /// DCB value: `Unknown`
    Unknown,
    /// DCB value: `BlobContact`
    BlobContact,
    /// DCB value: `InteractionPoint`
    InteractionPoint,
    /// DCB value: `Interactable`
    Interactable,
    /// DCB value: `Hint`
    Hint,
    /// DCB value: `Explosive`
    Explosive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EObjectiveInteractionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EObjectiveInteractionType {
    /// DCB value: `Defend`
    Defend,
    /// DCB value: `Capture`
    Capture,
    /// DCB value: `Damage`
    Damage,
    /// DCB value: `Overload`
    Overload,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EOperationsType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EOperationsType {
    /// DCB value: `Add`
    Add,
    /// DCB value: `Multiply`
    Multiply,
    /// DCB value: `Subtract`
    Subtract,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EOperatorMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EOperatorMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `Flight`
    Flight,
    /// DCB value: `Combat`
    Combat,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `Scanning`
    Scanning,
    /// DCB value: `Mining`
    Mining,
    /// DCB value: `QuantumNavigation`
    QuantumNavigation,
    /// DCB value: `Refuel`
    Refuel,
    /// DCB value: `AirTrafficController`
    AirTrafficController,
    /// DCB value: `Salvage`
    Salvage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EOrdnanceRelativeDetachType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EOrdnanceRelativeDetachType {
    /// DCB value: `Ordnance`
    Ordnance,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `WeaponHost`
    WeaponHost,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EOutfitPieceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EOutfitPieceType {
    /// DCB value: `Head`
    Head,
    /// DCB value: `Torso`
    Torso,
    /// DCB value: `Arms`
    Arms,
    /// DCB value: `Legs`
    Legs,
    /// DCB value: `Undersuit`
    Undersuit,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EParticleInputs`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EParticleInputs {
    /// DCB value: `Count`
    Count,
    /// DCB value: `Size`
    Size,
    /// DCB value: `Speed`
    Speed,
    /// DCB value: `Time`
    Time,
    /// DCB value: `Pulse`
    Pulse,
    /// DCB value: `Strength`
    Strength,
    /// DCB value: `Scale`
    Scale,
    /// DCB value: `Diffuse`
    Diffuse,
    /// DCB value: `Radius`
    Radius,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EParticleModifierSource`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EParticleModifierSource {
    /// DCB value: `None`
    None,
    /// DCB value: `Power`
    Power,
    /// DCB value: `Heat`
    Heat,
    /// DCB value: `BarrelAttachment`
    BarrelAttachment,
    /// DCB value: `MiningLaserThrottle`
    MiningLaserThrottle,
    /// DCB value: `WeaponState`
    WeaponState,
    /// DCB value: `SalvageRepair`
    SalvageRepair,
    /// DCB value: `Wear`
    Wear,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EParticleProperties`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EParticleProperties {
    /// DCB value: `strength`
    strength,
    /// DCB value: `alpha`
    alpha,
    /// DCB value: `color`
    color,
    /// DCB value: `count`
    count,
    /// DCB value: `size`
    size,
    /// DCB value: `speed`
    speed,
    /// DCB value: `time`
    time,
    /// DCB value: `pulse`
    pulse,
    /// DCB value: `radius`
    radius,
    /// DCB value: `distribution`
    distribution,
    /// DCB value: `glowScale`
    glowScale,
    /// DCB value: `emissionSize`
    emissionSize,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPerceptionSenses`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPerceptionSenses {
    /// DCB value: `Vision`
    Vision,
    /// DCB value: `Hearing`
    Hearing,
    /// DCB value: `Radar`
    Radar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPerceptionStatus`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPerceptionStatus {
    /// DCB value: `Dead`
    Dead,
    /// DCB value: `Downed`
    Downed,
    /// DCB value: `Unconscious`
    Unconscious,
    /// DCB value: `Conscious`
    Conscious,
    /// DCB value: `Invalid`
    Invalid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPerceptionTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPerceptionTypes {
    /// DCB value: `Explosive`
    Explosive,
    /// DCB value: `Agent`
    Agent,
    /// DCB value: `Player`
    Player,
    /// DCB value: `Interesting`
    Interesting,
    /// DCB value: `SearchSpots`
    SearchSpots,
    /// DCB value: `SabotageEquipment`
    SabotageEquipment,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Spaceship`
    Spaceship,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `Torpedo`
    Torpedo,
    /// DCB value: `Bomb`
    Bomb,
    /// DCB value: `FiringRangeTarget`
    FiringRangeTarget,
    /// DCB value: `LargeObject`
    LargeObject,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `ExplosiveDestructible`
    ExplosiveDestructible,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPersistentDataPolicy`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPersistentDataPolicy {
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `ReadOnly`
    ReadOnly,
    /// DCB value: `WriteFull`
    WriteFull,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPhysFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPhysFlag {
    /// DCB value: `none`
    none,
    /// DCB value: `geom_colltype_default`
    geom_colltype_default,
    /// DCB value: `geom_colltype_player`
    geom_colltype_player,
    /// DCB value: `geom_colltype_explosion`
    geom_colltype_explosion,
    /// DCB value: `geom_colltype_vehicle`
    geom_colltype_vehicle,
    /// DCB value: `geom_colltype_foliage`
    geom_colltype_foliage,
    /// DCB value: `geom_colltype_debris`
    geom_colltype_debris,
    /// DCB value: `geom_colltype_shield`
    geom_colltype_shield,
    /// DCB value: `geom_colltype_terrain_mesh`
    geom_colltype_terrain_mesh,
    /// DCB value: `geom_colltype_foliage_proxy`
    geom_colltype_foliage_proxy,
    /// DCB value: `geom_colltype_obstruct`
    geom_colltype_obstruct,
    /// DCB value: `geom_colltype_ray`
    geom_colltype_ray,
    /// DCB value: `geom_car_wheel`
    geom_car_wheel,
    /// DCB value: `geom_interior`
    geom_interior,
    /// DCB value: `geom_exterior`
    geom_exterior,
    /// DCB value: `geom_interior_grid_part`
    geom_interior_grid_part,
    /// DCB value: `geom_voxelization_proxy`
    geom_voxelization_proxy,
    /// DCB value: `geom_cluster_mesh`
    geom_cluster_mesh,
    /// DCB value: `geom_interaction`
    geom_interaction,
    /// DCB value: `geom_disabled`
    geom_disabled,
    /// DCB value: `geom_floats`
    geom_floats,
    /// DCB value: `geom_deprecated_21`
    geom_deprecated_21,
    /// DCB value: `geom_internal_deprecated_22`
    geom_internal_deprecated_22,
    /// DCB value: `geom_internal_23`
    geom_internal_23,
    /// DCB value: `geom_squashy`
    geom_squashy,
    /// DCB value: `geom_log_interactions`
    geom_log_interactions,
    /// DCB value: `geom_monitor_contacts`
    geom_monitor_contacts,
    /// DCB value: `geom_manually_breakable`
    geom_manually_breakable,
    /// DCB value: `geom_no_coll_response`
    geom_no_coll_response,
    /// DCB value: `geom_mat_substitutor`
    geom_mat_substitutor,
    /// DCB value: `geom_deprecated_30`
    geom_deprecated_30,
    /// DCB value: `geom_no_particle_impulse`
    geom_no_particle_impulse,
    /// DCB value: `geom_destroyed_on_break`
    geom_destroyed_on_break,
    /// DCB value: `geom_sdf_proxy`
    geom_sdf_proxy,
    /// DCB value: `geom_internal_34`
    geom_internal_34,
    /// DCB value: `geom_internal_deprecated_35`
    geom_internal_deprecated_35,
    /// DCB value: `geom_unpushable`
    geom_unpushable,
    /// DCB value: `geom_foot_coll`
    geom_foot_coll,
    /// DCB value: `geom_kinematic_part`
    geom_kinematic_part,
    /// DCB value: `geom_stairs_ramps`
    geom_stairs_ramps,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPipeClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPipeClass {
    /// DCB value: `Power`
    Power,
    /// DCB value: `Heat`
    Heat,
    /// DCB value: `Avionics`
    Avionics,
    /// DCB value: `Fuel`
    Fuel,
    /// DCB value: `QuantumFuel`
    QuantumFuel,
    /// DCB value: `Oxygen`
    Oxygen,
    /// DCB value: `Shield`
    Shield,
    /// DCB value: `Decibel`
    Decibel,
    /// DCB value: `Charge`
    Charge,
    /// DCB value: `Health`
    Health,
    /// DCB value: `Input`
    Input,
    /// DCB value: `Output`
    Output,
    /// DCB value: `Atmosphere`
    Atmosphere,
    /// DCB value: `WeaponRegen`
    WeaponRegen,
    /// DCB value: `WeaponAmmoLoad`
    WeaponAmmoLoad,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPipePriorityGroup`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPipePriorityGroup {
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `Shield`
    Shield,
    /// DCB value: `Thruster`
    Thruster,
    /// DCB value: `Other`
    Other,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerActionAnimType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerActionAnimType {
    /// DCB value: `NoAnim`
    NoAnim,
    /// DCB value: `OneShot`
    OneShot,
    /// DCB value: `Continuous`
    Continuous,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerAnimatedInteractionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerAnimatedInteractionType {
    /// DCB value: `Instantaneous`
    Instantaneous,
    /// DCB value: `Reactive`
    Reactive,
    /// DCB value: `Bluetooth`
    Bluetooth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerAttachmentSlots`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerAttachmentSlots {
    /// DCB value: `Barrel`
    Barrel,
    /// DCB value: `IronSight`
    IronSight,
    /// DCB value: `UnderBarrel`
    UnderBarrel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerGroupType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerGroupType {
    /// DCB value: `System`
    System,
    /// DCB value: `Party`
    Party,
    /// DCB value: `Group`
    Group,
    /// DCB value: `Server`
    Server,
    /// DCB value: `DirectMessage`
    DirectMessage,
    /// DCB value: `GameEntity`
    GameEntity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerItemSlots`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerItemSlots {
    /// DCB value: `ArmorArms`
    ArmorArms,
    /// DCB value: `ArmorHelmet`
    ArmorHelmet,
    /// DCB value: `ArmorLegs`
    ArmorLegs,
    /// DCB value: `ArmorTorso`
    ArmorTorso,
    /// DCB value: `ArmorUndersuit`
    ArmorUndersuit,
    /// DCB value: `CharAccessoryEyes`
    CharAccessoryEyes,
    /// DCB value: `CharAccessoryHead`
    CharAccessoryHead,
    /// DCB value: `CharBody`
    CharBody,
    /// DCB value: `CharFlair`
    CharFlair,
    /// DCB value: `CharHead`
    CharHead,
    /// DCB value: `CharHeadBeard`
    CharHeadBeard,
    /// DCB value: `CharHeadEyebrow`
    CharHeadEyebrow,
    /// DCB value: `CharHeadEyelash`
    CharHeadEyelash,
    /// DCB value: `CharHeadEyes`
    CharHeadEyes,
    /// DCB value: `CharHeadHair`
    CharHeadHair,
    /// DCB value: `Grenade`
    Grenade,
    /// DCB value: `Medpen`
    Medpen,
    /// DCB value: `Radar`
    Radar,
    /// DCB value: `WeaponPrimary`
    WeaponPrimary,
    /// DCB value: `WeaponSecondary`
    WeaponSecondary,
    /// DCB value: `WeaponSidearm`
    WeaponSidearm,
    /// DCB value: `WeaponMelee`
    WeaponMelee,
    /// DCB value: `Utility`
    Utility,
    /// DCB value: `Visor`
    Visor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPlayerStateActions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPlayerStateActions {
    /// DCB value: `None`
    None,
    /// DCB value: `Carry`
    Carry,
    /// DCB value: `Consume`
    Consume,
    /// DCB value: `Drop`
    Drop,
    /// DCB value: `Equip`
    Equip,
    /// DCB value: `FinishConsume`
    FinishConsume,
    /// DCB value: `FirstSelect`
    FirstSelect,
    /// DCB value: `Inspect`
    Inspect,
    /// DCB value: `Interact`
    Interact,
    /// DCB value: `OffHandStore`
    OffHandStore,
    /// DCB value: `OffHandStow`
    OffHandStow,
    /// DCB value: `OpenMobiGlas`
    OpenMobiGlas,
    /// DCB value: `Place`
    Place,
    /// DCB value: `PlaceReady`
    PlaceReady,
    /// DCB value: `Stow`
    Stow,
    /// DCB value: `Store`
    Store,
    /// DCB value: `SwapAttachments`
    SwapAttachments,
    /// DCB value: `ThrowReady`
    ThrowReady,
    /// DCB value: `ThrowV2`
    ThrowV2,
    /// DCB value: `Unequip`
    Unequip,
    /// DCB value: `UnprimeItem`
    UnprimeItem,
    /// DCB value: `Unstow`
    Unstow,
    /// DCB value: `VisorWipe`
    VisorWipe,
    /// DCB value: `SelfTarget`
    SelfTarget,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPoolFilterCombineMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPoolFilterCombineMode {
    /// DCB value: `Cumulative`
    Cumulative,
    /// DCB value: `Additive`
    Additive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPopupType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPopupType {
    /// DCB value: `GameModeDisclaimer`
    GameModeDisclaimer,
    /// DCB value: `Reconnect`
    Reconnect,
    /// DCB value: `ForceLaunch`
    ForceLaunch,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPowerSourceState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPowerSourceState {
    /// DCB value: `None`
    None,
    /// DCB value: `Online`
    Online,
    /// DCB value: `Offline`
    Offline,
    /// DCB value: `Inoperable`
    Inoperable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPreferredSpawnLocationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPreferredSpawnLocationType {
    /// DCB value: `CriminalRespawnWithoutArrest`
    CriminalRespawnWithoutArrest,
    /// DCB value: `PreferredCriminalSpawn`
    PreferredCriminalSpawn,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPresetFStopValues`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPresetFStopValues {
    /// DCB value: `FStop_0_1`
    FStop_0_1,
    /// DCB value: `FStop_0_5`
    FStop_0_5,
    /// DCB value: `FStop_1`
    FStop_1,
    /// DCB value: `FStop_2`
    FStop_2,
    /// DCB value: `FStop_3`
    FStop_3,
    /// DCB value: `FStop_4`
    FStop_4,
    /// DCB value: `FStop_5`
    FStop_5,
    /// DCB value: `FStop_6`
    FStop_6,
    /// DCB value: `FStop_8`
    FStop_8,
    /// DCB value: `FStop_14`
    FStop_14,
    /// DCB value: `FStop_22`
    FStop_22,
    /// DCB value: `FStop_Disabled`
    FStop_Disabled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EPresetLensSizes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EPresetLensSizes {
    /// DCB value: `LensSize_8mm`
    LensSize_8mm,
    /// DCB value: `LensSize_9mm46`
    LensSize_9mm46,
    /// DCB value: `LensSize_12mm`
    LensSize_12mm,
    /// DCB value: `LensSize_14mm`
    LensSize_14mm,
    /// DCB value: `LensSize_16mm`
    LensSize_16mm,
    /// DCB value: `LensSize_18mm`
    LensSize_18mm,
    /// DCB value: `LensSize_21mm`
    LensSize_21mm,
    /// DCB value: `LensSize_25mm`
    LensSize_25mm,
    /// DCB value: `LensSize_27mm`
    LensSize_27mm,
    /// DCB value: `LensSize_32mm`
    LensSize_32mm,
    /// DCB value: `LensSize_35mm`
    LensSize_35mm,
    /// DCB value: `LensSize_40mm`
    LensSize_40mm,
    /// DCB value: `LensSize_50mm`
    LensSize_50mm,
    /// DCB value: `LensSize_65mm`
    LensSize_65mm,
    /// DCB value: `LensSize_75mm`
    LensSize_75mm,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProcLeanPoseType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProcLeanPoseType {
    /// DCB value: `Standing`
    Standing,
    /// DCB value: `Sitting`
    Sitting,
    /// DCB value: `Crouching`
    Crouching,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectedHudAlignmentType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectedHudAlignmentType {
    /// DCB value: `NoRotation`
    NoRotation,
    /// DCB value: `AngledToPitchSource`
    AngledToPitchSource,
    /// DCB value: `AngledToView`
    AngledToView,
    /// DCB value: `AngledToYawLineOrigin`
    AngledToYawLineOrigin,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectedHudPositionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectedHudPositionType {
    /// DCB value: `OnAngle`
    OnAngle,
    /// DCB value: `OnYawLine`
    OnYawLine,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectedHudYawLineAnchorType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectedHudYawLineAnchorType {
    /// DCB value: `InForward`
    InForward,
    /// DCB value: `InWorld`
    InWorld,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectileType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectileType {
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Secondary`
    Secondary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectionSelection1`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectionSelection1 {
    /// DCB value: `PS1_NoProjection`
    PS1_NoProjection,
    /// DCB value: `PS1_ShortarcRotation`
    PS1_ShortarcRotation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectionSelection2`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectionSelection2 {
    /// DCB value: `PS2_NoProjection`
    PS2_NoProjection,
    /// DCB value: `PS2_ShortarcRotation`
    PS2_ShortarcRotation,
    /// DCB value: `PS2_DirectedRotation`
    PS2_DirectedRotation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectionSelection3`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectionSelection3 {
    /// DCB value: `PS3_NoProjection`
    PS3_NoProjection,
    /// DCB value: `PS3_ShortvecTranslation`
    PS3_ShortvecTranslation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EProjectionSelection4`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EProjectionSelection4 {
    /// DCB value: `PS4_NoProjection`
    PS4_NoProjection,
    /// DCB value: `PS4_ShortvecTranslation`
    PS4_ShortvecTranslation,
    /// DCB value: `PS4_DirectedTranslation`
    PS4_DirectedTranslation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQTEPriority`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQTEPriority {
    /// DCB value: `QTELowPriority`
    QTELowPriority,
    /// DCB value: `QTEMediumPriority`
    QTEMediumPriority,
    /// DCB value: `QTEHighPriority`
    QTEHighPriority,
    /// DCB value: `QTEHighestPriority`
    QTEHighestPriority,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQTEPriorityGivewayBehaviour`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQTEPriorityGivewayBehaviour {
    /// DCB value: `Cancel`
    Cancel,
    /// DCB value: `Silent`
    Silent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQedVisualGraphState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQedVisualGraphState {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Charging`
    Charging,
    /// DCB value: `Discharging`
    Discharging,
    /// DCB value: `Ready`
    Ready,
    /// DCB value: `Active`
    Active,
    /// DCB value: `Tethering`
    Tethering,
    /// DCB value: `Cooldown`
    Cooldown,
    /// DCB value: `Jamming`
    Jamming,
    /// DCB value: `Off`
    Off,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQedVisualGraphTrackedVariable`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQedVisualGraphTrackedVariable {
    /// DCB value: `ChargePercentage`
    ChargePercentage,
    /// DCB value: `CooldownPercentage`
    CooldownPercentage,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQedVisualGraphTrackedVariableContext`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQedVisualGraphTrackedVariableContext {
    /// DCB value: `TrackAscending`
    TrackAscending,
    /// DCB value: `TrackDescending`
    TrackDescending,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EQuantumProcessFailure`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EQuantumProcessFailure {
    /// DCB value: `None`
    None,
    /// DCB value: `TargetTooClose`
    TargetTooClose,
    /// DCB value: `TargetTooFar`
    TargetTooFar,
    /// DCB value: `SolarObstruction`
    SolarObstruction,
    /// DCB value: `QuantumObstruction`
    QuantumObstruction,
    /// DCB value: `MovingBackwards`
    MovingBackwards,
    /// DCB value: `OutOfFuel`
    OutOfFuel,
    /// DCB value: `GroupNotReady`
    GroupNotReady,
    /// DCB value: `GroupHasObstruction`
    GroupHasObstruction,
    /// DCB value: `NavpointBlocked`
    NavpointBlocked,
    /// DCB value: `RotationNotAligned`
    RotationNotAligned,
    /// DCB value: `RotationUnstable`
    RotationUnstable,
    /// DCB value: `BlockedByEvent`
    BlockedByEvent,
    /// DCB value: `LandingGear`
    LandingGear,
    /// DCB value: `Unknown`
    Unknown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarChargeLevels`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarChargeLevels {
    /// DCB value: `LowCharge`
    LowCharge,
    /// DCB value: `FullCharge`
    FullCharge,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarContactBehaviourCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarContactBehaviourCategory {
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `FPS`
    FPS,
    /// DCB value: `Both`
    Both,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarContactHighlightLayer`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarContactHighlightLayer {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Skeleton`
    Skeleton,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarContactHighlightState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarContactHighlightState {
    /// DCB value: `TargetScanBlocked`
    TargetScanBlocked,
    /// DCB value: `TargetScannable`
    TargetScannable,
    /// DCB value: `TargetScanned`
    TargetScanned,
    /// DCB value: `TargetScanning`
    TargetScanning,
    /// DCB value: `PingDetected_Hostile`
    PingDetected_Hostile,
    /// DCB value: `PingDetected_Neutral`
    PingDetected_Neutral,
    /// DCB value: `PingDetected_Friendly`
    PingDetected_Friendly,
    /// DCB value: `PingDetected_Objective`
    PingDetected_Objective,
    /// DCB value: `PingDetected_Unknown`
    PingDetected_Unknown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarContactState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarContactState {
    /// DCB value: `Alive`
    Alive,
    /// DCB value: `Incapacitated`
    Incapacitated,
    /// DCB value: `Dead`
    Dead,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERadarFocusLevels`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERadarFocusLevels {
    /// DCB value: `Level1`
    Level1,
    /// DCB value: `Level2`
    Level2,
    /// DCB value: `Level3`
    Level3,
    /// DCB value: `Level4`
    Level4,
    /// DCB value: `Level5`
    Level5,
    /// DCB value: `Level6`
    Level6,
    /// DCB value: `Level7`
    Level7,
    /// DCB value: `Level8`
    Level8,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERatingScoreCurveType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERatingScoreCurveType {
    /// DCB value: `TwoThousandFiveHundred`
    TwoThousandFiveHundred,
    /// DCB value: `FiveThousand`
    FiveThousand,
    /// DCB value: `SevenThousandFiveHundred`
    SevenThousandFiveHundred,
    /// DCB value: `TenThousand`
    TenThousand,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReflexImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReflexImprovementType {
    /// DCB value: `ADSSpeed`
    ADSSpeed,
    /// DCB value: `SoundGeneration`
    SoundGeneration,
    /// DCB value: `StanceTransition`
    StanceTransition,
    /// DCB value: `BackpackReloadSpeed`
    BackpackReloadSpeed,
    /// DCB value: `WeaponSwap`
    WeaponSwap,
    /// DCB value: `ReloadSpeed`
    ReloadSpeed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERenderLayer`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERenderLayer {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Wall`
    Wall,
    /// DCB value: `Floor`
    Floor,
    /// DCB value: `Bulkhead`
    Bulkhead,
    /// DCB value: `Skeleton`
    Skeleton,
    /// DCB value: `Exterior`
    Exterior,
    /// DCB value: `Hologram`
    Hologram,
    /// DCB value: `NonPlayable`
    NonPlayable,
    /// DCB value: `UIObject`
    UIObject,
    /// DCB value: `DoorUnlocked`
    DoorUnlocked,
    /// DCB value: `DoorLocked`
    DoorLocked,
    /// DCB value: `SurfaceWater`
    SurfaceWater,
    /// DCB value: `Nominal`
    Nominal,
    /// DCB value: `Moderate`
    Moderate,
    /// DCB value: `Critical`
    Critical,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `HangarFloor`
    HangarFloor,
    /// DCB value: `HangarWall`
    HangarWall,
    /// DCB value: `PlayerVehicle`
    PlayerVehicle,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERenderType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERenderType {
    /// DCB value: `FORWARD`
    FORWARD,
    /// DCB value: `TILEDFORWARD`
    TILEDFORWARD,
    /// DCB value: `DEFERRED`
    DEFERRED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationChangeReason`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationChangeReason {
    /// DCB value: `UNSPECIFIED`
    UNSPECIFIED,
    /// DCB value: `INITIALIZED`
    INITIALIZED,
    /// DCB value: `INCREASED`
    INCREASED,
    /// DCB value: `DECREASED`
    DECREASED,
    /// DCB value: `SET`
    SET,
    /// DCB value: `RESET`
    RESET,
    /// DCB value: `STATE`
    STATE,
    /// DCB value: `STANDING`
    STANDING,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationComparisonOperator`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationComparisonOperator {
    /// DCB value: `GreaterThan`
    GreaterThan,
    /// DCB value: `GreaterThanOrEqualTo`
    GreaterThanOrEqualTo,
    /// DCB value: `EqualTo`
    EqualTo,
    /// DCB value: `LessThanOrEqualTo`
    LessThanOrEqualTo,
    /// DCB value: `LessThan`
    LessThan,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationEntityType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationEntityType {
    /// DCB value: `Organization`
    Organization,
    /// DCB value: `MissionGiver`
    MissionGiver,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationSortOrderEntity`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationSortOrderEntity {
    /// DCB value: `Alphabetical`
    Alphabetical,
    /// DCB value: `RecentActivity`
    RecentActivity,
    /// DCB value: `PrimaryProgress`
    PrimaryProgress,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationSortOrderScope`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationSortOrderScope {
    /// DCB value: `Alphabetical`
    Alphabetical,
    /// DCB value: `ListOrder`
    ListOrder,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EReputationStateValueModifier`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EReputationStateValueModifier {
    /// DCB value: `Increment`
    Increment,
    /// DCB value: `Decrement`
    Decrement,
    /// DCB value: `Set`
    Set,
    /// DCB value: `SetEqualToState`
    SetEqualToState,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EResourceContainerMutabilityLevel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EResourceContainerMutabilityLevel {
    /// DCB value: `Static`
    Static,
    /// DCB value: `ReadOnly`
    ReadOnly,
    /// DCB value: `ReadWrite`
    ReadWrite,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERespawnLocationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERespawnLocationType {
    /// DCB value: `None`
    None,
    /// DCB value: `Hospital`
    Hospital,
    /// DCB value: `Prison`
    Prison,
    /// DCB value: `PrisonExit`
    PrisonExit,
    /// DCB value: `CriminalLocation`
    CriminalLocation,
    /// DCB value: `CriminalHospital`
    CriminalHospital,
    /// DCB value: `Other`
    Other,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ERuntimeImageSourceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ERuntimeImageSourceType {
    /// DCB value: `None`
    None,
    /// DCB value: `CommsCall`
    CommsCall,
    /// DCB value: `FuelCamera`
    FuelCamera,
    /// DCB value: `UserInterface`
    UserInterface,
    /// DCB value: `InteriorMap`
    InteriorMap,
    /// DCB value: `Slot_1`
    Slot_1,
    /// DCB value: `Slot_2`
    Slot_2,
    /// DCB value: `Slot_3`
    Slot_3,
    /// DCB value: `Slot_4`
    Slot_4,
    /// DCB value: `Slot_5`
    Slot_5,
    /// DCB value: `Slot_6`
    Slot_6,
    /// DCB value: `Slot_7`
    Slot_7,
    /// DCB value: `Slot_8`
    Slot_8,
    /// DCB value: `Slot_9`
    Slot_9,
    /// DCB value: `Slot_10`
    Slot_10,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ES42StatsComparisonType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ES42StatsComparisonType {
    /// DCB value: `Equal`
    Equal,
    /// DCB value: `NotEqual`
    NotEqual,
    /// DCB value: `Greater`
    Greater,
    /// DCB value: `Less`
    Less,
    /// DCB value: `GreaterOrEqual`
    GreaterOrEqual,
    /// DCB value: `LessOrEqual`
    LessOrEqual,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ES42StatsOperationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ES42StatsOperationType {
    /// DCB value: `Add`
    Add,
    /// DCB value: `Sub`
    Sub,
    /// DCB value: `Mul`
    Mul,
    /// DCB value: `Div`
    Div,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ES42StatsPlayerState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ES42StatsPlayerState {
    /// DCB value: `Any`
    Any,
    /// DCB value: `OnFoot`
    OnFoot,
    /// DCB value: `InShip`
    InShip,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ES42StatsType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ES42StatsType {
    /// DCB value: `EnemyKills`
    EnemyKills,
    /// DCB value: `ShipKills`
    ShipKills,
    /// DCB value: `GroundKills`
    GroundKills,
    /// DCB value: `FriendlyFireHits`
    FriendlyFireHits,
    /// DCB value: `WeaponShots`
    WeaponShots,
    /// DCB value: `WeaponHits`
    WeaponHits,
    /// DCB value: `MissileShots`
    MissileShots,
    /// DCB value: `MissileHits`
    MissileHits,
    /// DCB value: `HeadShots`
    HeadShots,
    /// DCB value: `Consumable`
    Consumable,
    /// DCB value: `FriendlyFireMelee`
    FriendlyFireMelee,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESCItemDisplayScreenLightType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESCItemDisplayScreenLightType {
    /// DCB value: `None`
    None,
    /// DCB value: `Omni`
    Omni,
    /// DCB value: `SoftOmni`
    SoftOmni,
    /// DCB value: `Projector`
    Projector,
    /// DCB value: `Planar`
    Planar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESCItemDisplayScreenState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESCItemDisplayScreenState {
    /// DCB value: `None`
    None,
    /// DCB value: `NoSignal`
    NoSignal,
    /// DCB value: `NoPower`
    NoPower,
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Emergency`
    Emergency,
    /// DCB value: `Auxiliary`
    Auxiliary,
    /// DCB value: `CustomOverride`
    CustomOverride,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESCItemDisplayScreenUIModel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESCItemDisplayScreenUIModel {
    /// DCB value: `Provider`
    Provider,
    /// DCB value: `Consumer`
    Consumer,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESCItemElevatorPathNodeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESCItemElevatorPathNodeType {
    /// DCB value: `Stop`
    Stop,
    /// DCB value: `WayPoint`
    WayPoint,
    /// DCB value: `Teleport`
    Teleport,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESCScreenType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESCScreenType {
    /// DCB value: `MFD_16_9`
    MFD_16_9,
    /// DCB value: `MFD_4_3`
    MFD_4_3,
    /// DCB value: `Support_16_9`
    Support_16_9,
    /// DCB value: `Support_1_1`
    Support_1_1,
    /// DCB value: `Support_Bespoke_1`
    Support_Bespoke_1,
    /// DCB value: `Support_Bespoke_2`
    Support_Bespoke_2,
    /// DCB value: `HeadUpDisplay`
    HeadUpDisplay,
    /// DCB value: `Annunciator`
    Annunciator,
    /// DCB value: `Visor`
    Visor,
    /// DCB value: `Radar3DScreen`
    Radar3DScreen,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESDFSetTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESDFSetTypes {
    /// DCB value: `Shield`
    Shield,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESalvageRepairMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESalvageRepairMode {
    /// DCB value: `Salvage`
    Salvage,
    /// DCB value: `Repair`
    Repair,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanCategory {
    /// DCB value: `Affiliation`
    Affiliation,
    /// DCB value: `General`
    General,
    /// DCB value: `Physical`
    Physical,
    /// DCB value: `Signatures`
    Signatures,
    /// DCB value: `ScanSystem`
    ScanSystem,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanDisplaySection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanDisplaySection {
    /// DCB value: `Boxout`
    Boxout,
    /// DCB value: `Overview`
    Overview,
    /// DCB value: `Exterior`
    Exterior,
    /// DCB value: `Interior`
    Interior,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanDisplayVariable`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanDisplayVariable {
    /// DCB value: `Header`
    Header,
    /// DCB value: `Body`
    Body,
    /// DCB value: `Capacity`
    Capacity,
    /// DCB value: `Extension`
    Extension,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanDisplayVariableAuxiliaryType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanDisplayVariableAuxiliaryType {
    /// DCB value: `None`
    None,
    /// DCB value: `ShipRegNumber`
    ShipRegNumber,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanInformation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanInformation {
    /// DCB value: `ActorStatus`
    ActorStatus,
    /// DCB value: `ArmorWeightRating`
    ArmorWeightRating,
    /// DCB value: `ArmorMixedWeight`
    ArmorMixedWeight,
    /// DCB value: `CargoAmount`
    CargoAmount,
    /// DCB value: `CargoAmountLoose`
    CargoAmountLoose,
    /// DCB value: `Classification`
    Classification,
    /// DCB value: `Commodity`
    Commodity,
    /// DCB value: `ContactType`
    ContactType,
    /// DCB value: `Callout1`
    Callout1,
    /// DCB value: `Callout2`
    Callout2,
    /// DCB value: `Callout3`
    Callout3,
    /// DCB value: `CauseOfDeath`
    CauseOfDeath,
    /// DCB value: `CommsChannelStatus`
    CommsChannelStatus,
    /// DCB value: `DeltaSignature`
    DeltaSignature,
    /// DCB value: `Description`
    Description,
    /// DCB value: `DisplaySection`
    DisplaySection,
    /// DCB value: `DoorLocked`
    DoorLocked,
    /// DCB value: `DoorOpen`
    DoorOpen,
    /// DCB value: `DriverOrganization`
    DriverOrganization,
    /// DCB value: `DriverName`
    DriverName,
    /// DCB value: `EntityId`
    EntityId,
    /// DCB value: `EntityClass`
    EntityClass,
    /// DCB value: `ID`
    ID,
    /// DCB value: `Instability`
    Instability,
    /// DCB value: `InventoryAmount`
    InventoryAmount,
    /// DCB value: `ItemType`
    ItemType,
    /// DCB value: `ItemStatus`
    ItemStatus,
    /// DCB value: `Jurisdiction`
    Jurisdiction,
    /// DCB value: `LastScanned`
    LastScanned,
    /// DCB value: `LegalOwnerEntityId`
    LegalOwnerEntityId,
    /// DCB value: `LegalOwnerFlagged`
    LegalOwnerFlagged,
    /// DCB value: `LegalOwnerName`
    LegalOwnerName,
    /// DCB value: `LegalOwnerOrganization`
    LegalOwnerOrganization,
    /// DCB value: `Hackable`
    Hackable,
    /// DCB value: `Health`
    Health,
    /// DCB value: `Mass`
    Mass,
    /// DCB value: `MasterMode`
    MasterMode,
    /// DCB value: `MineablePercentageTotal`
    MineablePercentageTotal,
    /// DCB value: `MineablePercentageValuables`
    MineablePercentageValuables,
    /// DCB value: `Model`
    Model,
    /// DCB value: `Name`
    Name,
    /// DCB value: `OptimalPowerWindowLow`
    OptimalPowerWindowLow,
    /// DCB value: `OptimalPowerWindowHigh`
    OptimalPowerWindowHigh,
    /// DCB value: `OtherTrauma1`
    OtherTrauma1,
    /// DCB value: `OtherTrauma2`
    OtherTrauma2,
    /// DCB value: `OtherTrauma3`
    OtherTrauma3,
    /// DCB value: `OtherTrauma4`
    OtherTrauma4,
    /// DCB value: `PassengerCount`
    PassengerCount,
    /// DCB value: `PowerAvailability`
    PowerAvailability,
    /// DCB value: `PowerSwitch`
    PowerSwitch,
    /// DCB value: `PowerTransferResistance`
    PowerTransferResistance,
    /// DCB value: `Priority`
    Priority,
    /// DCB value: `Resource`
    Resource,
    /// DCB value: `Role`
    Role,
    /// DCB value: `ScanFullyCompleted`
    ScanFullyCompleted,
    /// DCB value: `ScanProcedureBits`
    ScanProcedureBits,
    /// DCB value: `ScanTimeStamp`
    ScanTimeStamp,
    /// DCB value: `ScanToken`
    ScanToken,
    /// DCB value: `SelfDestructFlagged`
    SelfDestructFlagged,
    /// DCB value: `SignatureIR`
    SignatureIR,
    /// DCB value: `SignatureCS`
    SignatureCS,
    /// DCB value: `SignatureEM`
    SignatureEM,
    /// DCB value: `SignatureRS`
    SignatureRS,
    /// DCB value: `Species`
    Species,
    /// DCB value: `Stolen`
    Stolen,
    /// DCB value: `SubType`
    SubType,
    /// DCB value: `Type`
    Type,
    /// DCB value: `Value`
    Value,
    /// DCB value: `Volatility`
    Volatility,
    /// DCB value: `WantedLevel`
    WantedLevel,
    /// DCB value: `MissionId`
    MissionId,
    /// DCB value: `Quality`
    Quality,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanProcedure`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanProcedure {
    /// DCB value: `Armor`
    Armor,
    /// DCB value: `CargoCommodities`
    CargoCommodities,
    /// DCB value: `Classification`
    Classification,
    /// DCB value: `CommsTap`
    CommsTap,
    /// DCB value: `ContactType`
    ContactType,
    /// DCB value: `Custom`
    Custom,
    /// DCB value: `Cryopod`
    Cryopod,
    /// DCB value: `Death`
    Death,
    /// DCB value: `Description`
    Description,
    /// DCB value: `Door`
    Door,
    /// DCB value: `Driver`
    Driver,
    /// DCB value: `Emissions`
    Emissions,
    /// DCB value: `Faction`
    Faction,
    /// DCB value: `Instability`
    Instability,
    /// DCB value: `InventoryContents`
    InventoryContents,
    /// DCB value: `Jurisdiction`
    Jurisdiction,
    /// DCB value: `LegalOwner`
    LegalOwner,
    /// DCB value: `Hackable`
    Hackable,
    /// DCB value: `Health`
    Health,
    /// DCB value: `Mass`
    Mass,
    /// DCB value: `MineableCommodities`
    MineableCommodities,
    /// DCB value: `Name`
    Name,
    /// DCB value: `OptimalPowerWindow`
    OptimalPowerWindow,
    /// DCB value: `Passenger`
    Passenger,
    /// DCB value: `Power`
    Power,
    /// DCB value: `PowerTransferResistance`
    PowerTransferResistance,
    /// DCB value: `Species`
    Species,
    /// DCB value: `Stolen`
    Stolen,
    /// DCB value: `SubItems`
    SubItems,
    /// DCB value: `VehicleMasterMode`
    VehicleMasterMode,
    /// DCB value: `VehicleModel`
    VehicleModel,
    /// DCB value: `VehicleRole`
    VehicleRole,
    /// DCB value: `VehicleSelfDestruct`
    VehicleSelfDestruct,
    /// DCB value: `WantedLevel`
    WantedLevel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanSortType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanSortType {
    /// DCB value: `Less`
    Less,
    /// DCB value: `Greater`
    Greater,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanTable`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanTable {
    /// DCB value: `Self`
    Self_,
    /// DCB value: `CargoCommodities`
    CargoCommodities,
    /// DCB value: `Custom`
    Custom,
    /// DCB value: `MineableCommodities`
    MineableCommodities,
    /// DCB value: `Passengers`
    Passengers,
    /// DCB value: `SubItems`
    SubItems,
    /// DCB value: `InventoryContents`
    InventoryContents,
    /// DCB value: `CommsChannels`
    CommsChannels,
    /// DCB value: `Armor`
    Armor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScanType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScanType {
    /// DCB value: `AIAutoScan`
    AIAutoScan,
    /// DCB value: `FocalPointScan`
    FocalPointScan,
    /// DCB value: `PassiveScan`
    PassiveScan,
    /// DCB value: `PingBroadScan`
    PingBroadScan,
    /// DCB value: `PingFocusScan`
    PingFocusScan,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScopeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScopeType {
    /// DCB value: `None`
    None,
    /// DCB value: `Zoom`
    Zoom,
    /// DCB value: `Nightvision`
    Nightvision,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScoreDisplayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScoreDisplayType {
    /// DCB value: `Score`
    Score,
    /// DCB value: `Rounds`
    Rounds,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScoreType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScoreType {
    /// DCB value: `Kill`
    Kill,
    /// DCB value: `TeamKill`
    TeamKill,
    /// DCB value: `KillAssist`
    KillAssist,
    /// DCB value: `KillAssistCrew`
    KillAssistCrew,
    /// DCB value: `ForcedError`
    ForcedError,
    /// DCB value: `ForcedEject`
    ForcedEject,
    /// DCB value: `CrashRoberts`
    CrashRoberts,
    /// DCB value: `PlayerBledOut`
    PlayerBledOut,
    /// DCB value: `DistortionDisabledShip`
    DistortionDisabledShip,
    /// DCB value: `ControlTerminalCaptured`
    ControlTerminalCaptured,
    /// DCB value: `ControlTerminalCaptureAssist`
    ControlTerminalCaptureAssist,
    /// DCB value: `CaptureAreaCaptureBegin`
    CaptureAreaCaptureBegin,
    /// DCB value: `CaptureAreaCaptureComplete`
    CaptureAreaCaptureComplete,
    /// DCB value: `CaptureAreaNeutralized`
    CaptureAreaNeutralized,
    /// DCB value: `Accident`
    Accident,
    /// DCB value: `Suicide`
    Suicide,
    /// DCB value: `VehicleDestruction`
    VehicleDestruction,
    /// DCB value: `TeamVehicleDestruction`
    TeamVehicleDestruction,
    /// DCB value: `DestroyPhaseObjective`
    DestroyPhaseObjective,
    /// DCB value: `DamagePhaseObjective`
    DamagePhaseObjective,
    /// DCB value: `DamageSentToObjective`
    DamageSentToObjective,
    /// DCB value: `MatchVictory`
    MatchVictory,
    /// DCB value: `MatchDefeat`
    MatchDefeat,
    /// DCB value: `SecondPlace`
    SecondPlace,
    /// DCB value: `ThirdPlace`
    ThirdPlace,
    /// DCB value: `KillCollected`
    KillCollected,
    /// DCB value: `KillDenied`
    KillDenied,
    /// DCB value: `Hemorrhage`
    Hemorrhage,
    /// DCB value: `PlayerBleeding`
    PlayerBleeding,
    /// DCB value: `DistortionDamage`
    DistortionDamage,
    /// DCB value: `VehicleDamage`
    VehicleDamage,
    /// DCB value: `TeamVehicleDamage`
    TeamVehicleDamage,
    /// DCB value: `Untouchable`
    Untouchable,
    /// DCB value: `Unaided`
    Unaided,
    /// DCB value: `Ace`
    Ace,
    /// DCB value: `AceBonus`
    AceBonus,
    /// DCB value: `AceKiller`
    AceKiller,
    /// DCB value: `KillingSpree`
    KillingSpree,
    /// DCB value: `KillingSpreeBonus`
    KillingSpreeBonus,
    /// DCB value: `KillingSpreeKiller`
    KillingSpreeKiller,
    /// DCB value: `NemesisKill`
    NemesisKill,
    /// DCB value: `Redemption`
    Redemption,
    /// DCB value: `Regurgence`
    Regurgence,
    /// DCB value: `Revenge`
    Revenge,
    /// DCB value: `FirstBlood`
    FirstBlood,
    /// DCB value: `SquadronRevengeKill`
    SquadronRevengeKill,
    /// DCB value: `Savior`
    Savior,
    /// DCB value: `UnderdogKill`
    UnderdogKill,
    /// DCB value: `CheapShot`
    CheapShot,
    /// DCB value: `GoodNight`
    GoodNight,
    /// DCB value: `LightsOut`
    LightsOut,
    /// DCB value: `ControlTerminalDefended`
    ControlTerminalDefended,
    /// DCB value: `ControlTerminalHackerKilled`
    ControlTerminalHackerKilled,
    /// DCB value: `ControlTerminalDomination`
    ControlTerminalDomination,
    /// DCB value: `CaptureAreaContesting`
    CaptureAreaContesting,
    /// DCB value: `CaptureAreaCapturing`
    CaptureAreaCapturing,
    /// DCB value: `CaptureCloseCallKill`
    CaptureCloseCallKill,
    /// DCB value: `CaptureReversing`
    CaptureReversing,
    /// DCB value: `DefenderKill`
    DefenderKill,
    /// DCB value: `AttackerKill`
    AttackerKill,
    /// DCB value: `MartyrKill`
    MartyrKill,
    /// DCB value: `Headshot`
    Headshot,
    /// DCB value: `MeleeKill`
    MeleeKill,
    /// DCB value: `TakeDown`
    TakeDown,
    /// DCB value: `VehiclePartDetached`
    VehiclePartDetached,
    /// DCB value: `CompletedLap`
    CompletedLap,
    /// DCB value: `INVALID`
    INVALID,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScoreUIType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScoreUIType {
    /// DCB value: `NONE`
    NONE,
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Secondary`
    Secondary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EScoreboardType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EScoreboardType {
    /// DCB value: `TotalScore`
    TotalScore,
    /// DCB value: `ObjectivePoints`
    ObjectivePoints,
    /// DCB value: `Kills`
    Kills,
    /// DCB value: `Deaths`
    Deaths,
    /// DCB value: `Assists`
    Assists,
    /// DCB value: `CompletedLaps`
    CompletedLaps,
    /// DCB value: `FastestLapTime`
    FastestLapTime,
    /// DCB value: `RacePosition`
    RacePosition,
    /// DCB value: `GunGameLevel`
    GunGameLevel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESeatAccessConditionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESeatAccessConditionType {
    /// DCB value: `True`
    True,
    /// DCB value: `False`
    False,
    /// DCB value: `Either`
    Either,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESeatAccessEntranceDependencyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESeatAccessEntranceDependencyType {
    /// DCB value: `Never`
    Never,
    /// DCB value: `Exit`
    Exit,
    /// DCB value: `Enter`
    Enter,
    /// DCB value: `Always`
    Always,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESeatAccessPassageCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESeatAccessPassageCondition {
    /// DCB value: `InteriorOnly`
    InteriorOnly,
    /// DCB value: `ExteriorOnly`
    ExteriorOnly,
    /// DCB value: `Either`
    Either,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESeatViewType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESeatViewType {
    /// DCB value: `Background`
    Background,
    /// DCB value: `Comms`
    Comms,
    /// DCB value: `Cooler`
    Cooler,
    /// DCB value: `Energy`
    Energy,
    /// DCB value: `Flight`
    Flight,
    /// DCB value: `Missile`
    Missile,
    /// DCB value: `Quantum`
    Quantum,
    /// DCB value: `Shield`
    Shield,
    /// DCB value: `Target`
    Target,
    /// DCB value: `Weapon`
    Weapon,
    /// DCB value: `Wheeled`
    Wheeled,
    /// DCB value: `Scanner`
    Scanner,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESecurityNetworkPermissionValue`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESecurityNetworkPermissionValue {
    /// DCB value: `Inherit`
    Inherit,
    /// DCB value: `Yes`
    Yes,
    /// DCB value: `No`
    No,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESelectionBehaviour`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESelectionBehaviour {
    /// DCB value: `KeepAllSelected`
    KeepAllSelected,
    /// DCB value: `EquipAndUnselectAll`
    EquipAndUnselectAll,
    /// DCB value: `EquipAndKeepItemPortSelected`
    EquipAndKeepItemPortSelected,
    /// DCB value: `EquipAndKeepItemSelected`
    EquipAndKeepItemSelected,
    /// DCB value: `EquipAndKeepFirstSelection`
    EquipAndKeepFirstSelection,
    /// DCB value: `EquipAndKeepLastSelection`
    EquipAndKeepLastSelection,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESequenceMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESequenceMode {
    /// DCB value: `Individually`
    Individually,
    /// DCB value: `Automatically`
    Automatically,
    /// DCB value: `Looping`
    Looping,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EServiceBeaconType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EServiceBeaconType {
    /// DCB value: `None`
    None,
    /// DCB value: `PersonalTransport`
    PersonalTransport,
    /// DCB value: `CombatAssistance`
    CombatAssistance,
    /// DCB value: `Escort`
    Escort,
    /// DCB value: `Refuel`
    Refuel,
    /// DCB value: `Revive`
    Revive,
    /// DCB value: `Heal`
    Heal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EShipComputerMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EShipComputerMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Race`
    Race,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EShipState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EShipState {
    /// DCB value: `Grounded`
    Grounded,
    /// DCB value: `Flying`
    Flying,
    /// DCB value: `Both`
    Both,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESignatureType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESignatureType {
    /// DCB value: `Infrared`
    Infrared,
    /// DCB value: `Electromagnetic`
    Electromagnetic,
    /// DCB value: `CrossSection`
    CrossSection,
    /// DCB value: `Decibel`
    Decibel,
    /// DCB value: `Resource`
    Resource,
    /// DCB value: `Identity`
    Identity,
    /// DCB value: `CommsSignal`
    CommsSignal,
    /// DCB value: `Interactable`
    Interactable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESilhouetteColourSource`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESilhouetteColourSource {
    /// DCB value: `HDRTarget`
    HDRTarget,
    /// DCB value: `PerObject`
    PerObject,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESkillType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESkillType {
    /// DCB value: `Fitness`
    Fitness,
    /// DCB value: `Strength`
    Strength,
    /// DCB value: `Reflex`
    Reflex,
    /// DCB value: `Fortitude`
    Fortitude,
    /// DCB value: `VaultingTechnique`
    VaultingTechnique,
    /// DCB value: `TakedownTechnique`
    TakedownTechnique,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESlottingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESlottingMode {
    /// DCB value: `EnabledOnAndOffSlotting`
    EnabledOnAndOffSlotting,
    /// DCB value: `EnabledOnSlotting`
    EnabledOnSlotting,
    /// DCB value: `DisabledOnSlotting`
    DisabledOnSlotting,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESoftbodySubstepMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESoftbodySubstepMode {
    /// DCB value: `FullSubstep`
    FullSubstep,
    /// DCB value: `FullSubstep_SingleCD`
    FullSubstep_SingleCD,
    /// DCB value: `Iteration`
    Iteration,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESoftbodyVisualBindingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESoftbodyVisualBindingMode {
    /// DCB value: `Planar`
    Planar,
    /// DCB value: `Volumetric`
    Volumetric,
    /// DCB value: `Spline`
    Spline,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESoundInputs`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESoundInputs {
    /// DCB value: `FloatParam`
    FloatParam,
    /// DCB value: `Enable`
    Enable,
    /// DCB value: `Disable`
    Disable,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESpecialEffectsType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESpecialEffectsType {
    /// DCB value: `ReloadFull`
    ReloadFull,
    /// DCB value: `ReloadCheck`
    ReloadCheck,
    /// DCB value: `ReloadEmpty`
    ReloadEmpty,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESpectatorMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESpectatorMode {
    /// DCB value: `SM_Fixed`
    SM_Fixed,
    /// DCB value: `SM_Free`
    SM_Free,
    /// DCB value: `SM_Cinematic`
    SM_Cinematic,
    /// DCB value: `SM_Follow`
    SM_Follow,
    /// DCB value: `SM_FirstPerson`
    SM_FirstPerson,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESpinActivationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESpinActivationMode {
    /// DCB value: `Fire`
    Fire,
    /// DCB value: `Procclip`
    Procclip,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EStarGalaPaths`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EStarGalaPaths {
    /// DCB value: `formationPath1`
    formationPath1,
    /// DCB value: `formationPath2`
    formationPath2,
    /// DCB value: `formationPath3`
    formationPath3,
    /// DCB value: `formationPath4`
    formationPath4,
    /// DCB value: `challengeStage1`
    challengeStage1,
    /// DCB value: `challengeStage2`
    challengeStage2,
    /// DCB value: `challengeStage3`
    challengeStage3,
    /// DCB value: `challengeStage4`
    challengeStage4,
    /// DCB value: `challengeStage5`
    challengeStage5,
    /// DCB value: `challengeStage6`
    challengeStage6,
    /// DCB value: `challengeStage7`
    challengeStage7,
    /// DCB value: `challengeStage8`
    challengeStage8,
    /// DCB value: `challengeStage9_1`
    challengeStage9_1,
    /// DCB value: `challengeStage9_2`
    challengeStage9_2,
    /// DCB value: `challengeStage10_1`
    challengeStage10_1,
    /// DCB value: `challengeStage10_2`
    challengeStage10_2,
    /// DCB value: `challengeStage11`
    challengeStage11,
    /// DCB value: `challengeStage12`
    challengeStage12,
    /// DCB value: `challengeStage13_1`
    challengeStage13_1,
    /// DCB value: `challengeStage13_2`
    challengeStage13_2,
    /// DCB value: `challengeStage13_3`
    challengeStage13_3,
    /// DCB value: `challengeStage13_4`
    challengeStage13_4,
    /// DCB value: `challengeStage14`
    challengeStage14,
    /// DCB value: `challengeStage15`
    challengeStage15,
    /// DCB value: `challengeStage16`
    challengeStage16,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EStatCompareMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EStatCompareMode {
    /// DCB value: `MoreIsBetter`
    MoreIsBetter,
    /// DCB value: `LessIsBetter`
    LessIsBetter,
    /// DCB value: `Neutral`
    Neutral,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EStatType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EStatType {
    /// DCB value: `OnFoot`
    OnFoot,
    /// DCB value: `Seated`
    Seated,
    /// DCB value: `Size`
    Size,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EStrengthImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EStrengthImprovementType {
    /// DCB value: `CarryWheight`
    CarryWheight,
    /// DCB value: `WeightImpact`
    WeightImpact,
    /// DCB value: `MeleeDamage`
    MeleeDamage,
    /// DCB value: `TrolleyHandling`
    TrolleyHandling,
    /// DCB value: `ThrowingForce`
    ThrowingForce,
    /// DCB value: `ForcedReactionsResistance`
    ForcedReactionsResistance,
    /// DCB value: `BodyCarryPickUpSpeedScale`
    BodyCarryPickUpSpeedScale,
    /// DCB value: `BodyCarryDropSpeedScale`
    BodyCarryDropSpeedScale,
    /// DCB value: `BodyStowingSpeedScale`
    BodyStowingSpeedScale,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESuggestedFOVMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESuggestedFOVMode {
    /// DCB value: `SuggestedFOVMode_None`
    SuggestedFOVMode_None,
    /// DCB value: `SuggestedFOVMode_ForceAlways`
    SuggestedFOVMode_ForceAlways,
    /// DCB value: `SuggestedFOVMode_ApplyIfBigger`
    SuggestedFOVMode_ApplyIfBigger,
    /// DCB value: `SuggestedFOVMode_ApplyIfLower`
    SuggestedFOVMode_ApplyIfLower,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ESunShadowMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ESunShadowMode {
    /// DCB value: `Upto_20m`
    Upto_20m,
    /// DCB value: `Upto_100m`
    Upto_100m,
    /// DCB value: `Upto_400m`
    Upto_400m,
    /// DCB value: `Upto_2000m`
    Upto_2000m,
    /// DCB value: `Upto_2000m_AND_Heightmap`
    Upto_2000m_AND_Heightmap,
    /// DCB value: `Heightmap_Only`
    Heightmap_Only,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETakedownTechniqueImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETakedownTechniqueImprovementType {
    /// DCB value: `Speed`
    Speed,
    /// DCB value: `SoundGenerated`
    SoundGenerated,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETattooMirrorMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETattooMirrorMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Both`
    Both,
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrowMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrowMode {
    /// DCB value: `Undefined`
    Undefined,
    /// DCB value: `OverhandThrow`
    OverhandThrow,
    /// DCB value: `UnderhandThrow`
    UnderhandThrow,
    /// DCB value: `TwoHandedThrow`
    TwoHandedThrow,
    /// DCB value: `Drop`
    Drop,
    /// DCB value: `Place`
    Place,
    /// DCB value: `InteractionMode`
    InteractionMode,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrowPerceptionSound`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrowPerceptionSound {
    /// DCB value: `SmallSound`
    SmallSound,
    /// DCB value: `LargeSound`
    LargeSound,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrusterAnimDriver`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrusterAnimDriver {
    /// DCB value: `ThrustWithAfterburner`
    ThrustWithAfterburner,
    /// DCB value: `Thrust`
    Thrust,
    /// DCB value: `Velocity`
    Velocity,
    /// DCB value: `ForwardOnlyVelocity`
    ForwardOnlyVelocity,
    /// DCB value: `Nutcracker`
    Nutcracker,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrusterFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrusterFlag {
    /// DCB value: `StrafeForward`
    StrafeForward,
    /// DCB value: `StrafeBack`
    StrafeBack,
    /// DCB value: `StrafeLeft`
    StrafeLeft,
    /// DCB value: `StrafeRight`
    StrafeRight,
    /// DCB value: `StrafeUp`
    StrafeUp,
    /// DCB value: `StrafeDown`
    StrafeDown,
    /// DCB value: `YawLeft`
    YawLeft,
    /// DCB value: `YawRight`
    YawRight,
    /// DCB value: `PitchUp`
    PitchUp,
    /// DCB value: `PitchDown`
    PitchDown,
    /// DCB value: `RollLeft`
    RollLeft,
    /// DCB value: `RollRight`
    RollRight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrusterOutputData`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrusterOutputData {
    /// DCB value: `FireStrength`
    FireStrength,
    /// DCB value: `FireStrengthSmoothed`
    FireStrengthSmoothed,
    /// DCB value: `AfterburnerEnabled`
    AfterburnerEnabled,
    /// DCB value: `LastFuelRequested`
    LastFuelRequested,
    /// DCB value: `LastFuelReceived`
    LastFuelReceived,
    /// DCB value: `HealthRatio`
    HealthRatio,
    /// DCB value: `MisfireThrustRatio`
    MisfireThrustRatio,
    /// DCB value: `MisfireState`
    MisfireState,
    /// DCB value: `TemperatureRatio`
    TemperatureRatio,
    /// DCB value: `IsOverheating`
    IsOverheating,
    /// DCB value: `IsOverheated`
    IsOverheated,
    /// DCB value: `PowerRatio`
    PowerRatio,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EThrusterType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EThrusterType {
    /// DCB value: `Maneuver`
    Maneuver,
    /// DCB value: `Main`
    Main,
    /// DCB value: `Retro`
    Retro,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETriggerDefaultActions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETriggerDefaultActions {
    /// DCB value: `pc_item_primary`
    pc_item_primary,
    /// DCB value: `pc_item_secondary`
    pc_item_secondary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETurbulentPassEA`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETurbulentPassEA {
    /// DCB value: `EA_BattleRoyale`
    EA_BattleRoyale,
    /// DCB value: `EA_FreeFlight`
    EA_FreeFlight,
    /// DCB value: `EA_PirateSwarm`
    EA_PirateSwarm,
    /// DCB value: `EA_SquadronBattle`
    EA_SquadronBattle,
    /// DCB value: `EA_VanduulSwarm`
    EA_VanduulSwarm,
    /// DCB value: `EA_ClassicRace`
    EA_ClassicRace,
    /// DCB value: `EA_Elimination`
    EA_Elimination,
    /// DCB value: `EA_TeamElimination`
    EA_TeamElimination,
    /// DCB value: `EA_Control`
    EA_Control,
    /// DCB value: `EA_TheatersOfWar`
    EA_TheatersOfWar,
    /// DCB value: `EA_IterativeTesting`
    EA_IterativeTesting,
    /// DCB value: `EA_Duel`
    EA_Duel,
    /// DCB value: `EA_FPSGunGame`
    EA_FPSGunGame,
    /// DCB value: `EA_Horde`
    EA_Horde,
    /// DCB value: `EA_VanduulInvasion`
    EA_VanduulInvasion,
    /// DCB value: `EA_ExperimentalMode_1`
    EA_ExperimentalMode_1,
    /// DCB value: `EA_ExperimentalMode_2`
    EA_ExperimentalMode_2,
    /// DCB value: `EA_ExperimentalMode_3`
    EA_ExperimentalMode_3,
    /// DCB value: `EA_ExperimentalMode_4`
    EA_ExperimentalMode_4,
    /// DCB value: `EA_ExperimentalMode_5`
    EA_ExperimentalMode_5,
    /// DCB value: `EA_ExperimentalMode_6`
    EA_ExperimentalMode_6,
    /// DCB value: `EA_ExperimentalMode_7`
    EA_ExperimentalMode_7,
    /// DCB value: `EA_ExperimentalMode_8`
    EA_ExperimentalMode_8,
    /// DCB value: `EA_ExperimentalMode_9`
    EA_ExperimentalMode_9,
    /// DCB value: `EA_ExperimentalMode_10`
    EA_ExperimentalMode_10,
    /// DCB value: `EA_ExperimentalMode_11`
    EA_ExperimentalMode_11,
    /// DCB value: `EA_ExperimentalMode_12`
    EA_ExperimentalMode_12,
    /// DCB value: `EA_GravRace`
    EA_GravRace,
    /// DCB value: `Subscriber`
    Subscriber,
    /// DCB value: `EA_SpecialEvent`
    EA_SpecialEvent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETurretGunSafetyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETurretGunSafetyType {
    /// DCB value: `Off`
    Off,
    /// DCB value: `PlayerOnly`
    PlayerOnly,
    /// DCB value: `All`
    All,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ETurretRotationStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ETurretRotationStyle {
    /// DCB value: `SingleAxis`
    SingleAxis,
    /// DCB value: `MultiAxis`
    MultiAxis,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EUsableEntrySelectionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EUsableEntrySelectionType {
    /// DCB value: `FirstFound`
    FirstFound,
    /// DCB value: `Closest`
    Closest,
    /// DCB value: `Farthest`
    Farthest,
    /// DCB value: `Random`
    Random,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EUsableSelectionMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EUsableSelectionMethod {
    /// DCB value: `Priority`
    Priority,
    /// DCB value: `Random`
    Random,
    /// DCB value: `Closest`
    Closest,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EUsableStance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EUsableStance {
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Seated`
    Seated,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `ProneBack`
    ProneBack,
    /// DCB value: `Unset`
    Unset,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EUseSlotReenablePoint`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EUseSlotReenablePoint {
    /// DCB value: `UseChannelCompletion`
    UseChannelCompletion,
    /// DCB value: `ExitAnimStart`
    ExitAnimStart,
    /// DCB value: `UsableDelink`
    UsableDelink,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVAStateType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVAStateType {
    /// DCB value: `StandIdle`
    StandIdle,
    /// DCB value: `IronmanRelaxed`
    IronmanRelaxed,
    /// DCB value: `IronmanTurn`
    IronmanTurn,
    /// DCB value: `IronmanMove`
    IronmanMove,
    /// DCB value: `StandMove`
    StandMove,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVaultingTechniqueImprovementType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVaultingTechniqueImprovementType {
    /// DCB value: `Speed`
    Speed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVehicleDamageModifier`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVehicleDamageModifier {
    /// DCB value: `AI_HitBy_AI`
    AI_HitBy_AI,
    /// DCB value: `AI_HitBy_Player`
    AI_HitBy_Player,
    /// DCB value: `Player_HitBy_AI`
    Player_HitBy_AI,
    /// DCB value: `Player_HitBy_Player`
    Player_HitBy_Player,
    /// DCB value: `Uncontrolled_HitBy_AI`
    Uncontrolled_HitBy_AI,
    /// DCB value: `Uncontrolled_HitBy_Player`
    Uncontrolled_HitBy_Player,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVehicleElevatorBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVehicleElevatorBehavior {
    /// DCB value: `None`
    None,
    /// DCB value: `DisableNetSync`
    DisableNetSync,
    /// DCB value: `Attach`
    Attach,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVehicleMovementClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVehicleMovementClass {
    /// DCB value: `ArcadeWheeled`
    ArcadeWheeled,
    /// DCB value: `Dummy`
    Dummy,
    /// DCB value: `Spaceship`
    Spaceship,
    /// DCB value: `TrackWheeled`
    TrackWheeled,
    /// DCB value: `Boat`
    Boat,
    /// DCB value: `PhysicalWheeled`
    PhysicalWheeled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVibrationAudioCalculationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVibrationAudioCalculationType {
    /// DCB value: `Additive`
    Additive,
    /// DCB value: `MaxOfAll`
    MaxOfAll,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVibrationSource`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVibrationSource {
    /// DCB value: `Collision`
    Collision,
    /// DCB value: `Damage`
    Damage,
    /// DCB value: `Explosion`
    Explosion,
    /// DCB value: `IFCSMasterModeDrag`
    IFCSMasterModeDrag,
    /// DCB value: `Thruster`
    Thruster,
    /// DCB value: `ThrusterAfterburner`
    ThrusterAfterburner,
    /// DCB value: `ThrustersAcceleration`
    ThrustersAcceleration,
    /// DCB value: `AerodynamicsAcceleration`
    AerodynamicsAcceleration,
    /// DCB value: `ExternalAcceleration`
    ExternalAcceleration,
    /// DCB value: `Wheels`
    Wheels,
    /// DCB value: `WeaponFire`
    WeaponFire,
    /// DCB value: `WindArea`
    WindArea,
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `MagLaunch_Charge`
    MagLaunch_Charge,
    /// DCB value: `MagLaunch_Catapult`
    MagLaunch_Catapult,
    /// DCB value: `Quantum`
    Quantum,
    /// DCB value: `ElectricalCharge`
    ElectricalCharge,
    /// DCB value: `ProjectileImpact`
    ProjectileImpact,
    /// DCB value: `JumpDrive`
    JumpDrive,
    /// DCB value: `JumpPoint`
    JumpPoint,
    /// DCB value: `JumpTunnel`
    JumpTunnel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVibrationVehiclePlayerRole`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVibrationVehiclePlayerRole {
    /// DCB value: `None`
    None,
    /// DCB value: `Driver`
    Driver,
    /// DCB value: `Crew`
    Crew,
    /// DCB value: `Passenger`
    Passenger,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EViewType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EViewType {
    /// DCB value: `eView_Off`
    eView_Off,
    /// DCB value: `eView_SelfStatus`
    eView_SelfStatus,
    /// DCB value: `eView_TargetStatus`
    eView_TargetStatus,
    /// DCB value: `eView_Communications`
    eView_Communications,
    /// DCB value: `eView_Configuration`
    eView_Configuration,
    /// DCB value: `eView_IFCS`
    eView_IFCS,
    /// DCB value: `eView_Diagnostics`
    eView_Diagnostics,
    /// DCB value: `eView_ResourceNetwork`
    eView_ResourceNetwork,
    /// DCB value: `eView_Scanning`
    eView_Scanning,
    /// DCB value: `eView_Weapons`
    eView_Weapons,
    /// DCB value: `eView_Shields`
    eView_Shields,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EVoteLocType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EVoteLocType {
    /// DCB value: `Passed`
    Passed,
    /// DCB value: `Failed`
    Failed,
    /// DCB value: `Yes`
    Yes,
    /// DCB value: `No`
    No,
    /// DCB value: `StartedBy`
    StartedBy,
    /// DCB value: `ActionPrompts`
    ActionPrompts,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWantedLevelComparison`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWantedLevelComparison {
    /// DCB value: `Ignore`
    Ignore,
    /// DCB value: `Equal`
    Equal,
    /// DCB value: `EqualOrLess`
    EqualOrLess,
    /// DCB value: `EqualOrGreater`
    EqualOrGreater,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWave`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWave {
    /// DCB value: `EWave_1`
    EWave_1,
    /// DCB value: `EWave_2`
    EWave_2,
    /// DCB value: `EWave_3`
    EWave_3,
    /// DCB value: `EWave_4`
    EWave_4,
    /// DCB value: `EWave_5`
    EWave_5,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponDisplayStat`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponDisplayStat {
    /// DCB value: `Damage`
    Damage,
    /// DCB value: `Handling`
    Handling,
    /// DCB value: `Range`
    Range,
    /// DCB value: `Weight`
    Weight,
    /// DCB value: `Velocity`
    Velocity,
    /// DCB value: `FireRate`
    FireRate,
    /// DCB value: `AmmoCapacity`
    AmmoCapacity,
    /// DCB value: `Modes`
    Modes,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponGroup`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponGroup {
    /// DCB value: `Gun1`
    Gun1,
    /// DCB value: `Gun2`
    Gun2,
    /// DCB value: `Missiles`
    Missiles,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponProjectileType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponProjectileType {
    /// DCB value: `Ballistic`
    Ballistic,
    /// DCB value: `Flak`
    Flak,
    /// DCB value: `Incendiary`
    Incendiary,
    /// DCB value: `Explosive`
    Explosive,
    /// DCB value: `Disarray`
    Disarray,
    /// DCB value: `MassDriver`
    MassDriver,
    /// DCB value: `Laser`
    Laser,
    /// DCB value: `Neutron`
    Neutron,
    /// DCB value: `Plasma`
    Plasma,
    /// DCB value: `Electron`
    Electron,
    /// DCB value: `Tachyon`
    Tachyon,
    /// DCB value: `Beam`
    Beam,
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `Mining`
    Mining,
    /// DCB value: `Tractor`
    Tractor,
    /// DCB value: `Towing`
    Towing,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponRangeCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponRangeCategory {
    /// DCB value: `Short`
    Short,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Long`
    Long,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponRole`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponRole {
    /// DCB value: `Sniper`
    Sniper,
    /// DCB value: `AntiMaterial`
    AntiMaterial,
    /// DCB value: `Default`
    Default,
    /// DCB value: `AntiAir`
    AntiAir,
    /// DCB value: `CloseQuarter`
    CloseQuarter,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponToggleMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponToggleMode {
    /// DCB value: `IsNotToggle`
    IsNotToggle,
    /// DCB value: `IsToggle`
    IsToggle,
    /// DCB value: `UsePlayerOptions`
    UsePlayerOptions,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeaponType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeaponType {
    /// DCB value: `Gatling`
    Gatling,
    /// DCB value: `Repeater`
    Repeater,
    /// DCB value: `GrenadeLauncher`
    GrenadeLauncher,
    /// DCB value: `Cannon`
    Cannon,
    /// DCB value: `Artillery`
    Artillery,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EWeekday`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EWeekday {
    /// DCB value: `Sunday`
    Sunday,
    /// DCB value: `Monday`
    Monday,
    /// DCB value: `Tuesday`
    Tuesday,
    /// DCB value: `Wednesday`
    Wednesday,
    /// DCB value: `Thursday`
    Thursday,
    /// DCB value: `Friday`
    Friday,
    /// DCB value: `Saturday`
    Saturday,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ElectricalCalculationPropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricalCalculationPropertyType {
    /// DCB value: `Distance`
    Distance,
    /// DCB value: `ElectricalCharge`
    ElectricalCharge,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ElectricalStatePropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricalStatePropertyType {
    /// DCB value: `Charge`
    Charge,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EngineeringState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineeringState {
    /// DCB value: `Baseline`
    Baseline,
    /// DCB value: `Warning`
    Warning,
    /// DCB value: `Critical`
    Critical,
    /// DCB value: `Disabled`
    Disabled,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `EulerAngles`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EulerAngles {
    /// DCB value: `Pitch`
    Pitch,
    /// DCB value: `Roll`
    Roll,
    /// DCB value: `Yaw`
    Yaw,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ExpirationActivationType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpirationActivationType {
    /// DCB value: `AtCreation`
    AtCreation,
    /// DCB value: `OnInteraction`
    OnInteraction,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ExplosionZone`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExplosionZone {
    /// DCB value: `AdamHosted`
    AdamHosted,
    /// DCB value: `Adam`
    Adam,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FactionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionType {
    /// DCB value: `LawEnforcement`
    LawEnforcement,
    /// DCB value: `PrivateSecurity`
    PrivateSecurity,
    /// DCB value: `Lawful`
    Lawful,
    /// DCB value: `Unlawful`
    Unlawful,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireBehavior {
    /// DCB value: `VoxelDamage`
    VoxelDamage,
    /// DCB value: `Radiation`
    Radiation,
    /// DCB value: `Convection`
    Convection,
    /// DCB value: `Equalization`
    Equalization,
    /// DCB value: `DamageIgnition`
    DamageIgnition,
    /// DCB value: `FlashIgnition`
    FlashIgnition,
    /// DCB value: `OxygenStrength`
    OxygenStrength,
    /// DCB value: `OxygenConsumption`
    OxygenConsumption,
    /// DCB value: `LowOxygenExtinguish`
    LowOxygenExtinguish,
    /// DCB value: `FuelConsumption`
    FuelConsumption,
    /// DCB value: `RequiresFuel`
    RequiresFuel,
    /// DCB value: `SmokeProduction`
    SmokeProduction,
    /// DCB value: `FogGeneration`
    FogGeneration,
    /// DCB value: `DamageToHealth`
    DamageToHealth,
    /// DCB value: `AmbientTemperature`
    AmbientTemperature,
    /// DCB value: `AutoRepairing`
    AutoRepairing,
    /// DCB value: `Repairing`
    Repairing,
    /// DCB value: `Extinguishing`
    Extinguishing,
    /// DCB value: `RoomConnectors`
    RoomConnectors,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireEnabledMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireEnabledMode {
    /// DCB value: `Enabled`
    Enabled,
    /// DCB value: `Disabled`
    Disabled,
    /// DCB value: `EnabledOnVehicles`
    EnabledOnVehicles,
    /// DCB value: `EnabledOnTag`
    EnabledOnTag,
    /// DCB value: `EnabledOnAllVehicles`
    EnabledOnAllVehicles,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireFilterMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireFilterMode {
    /// DCB value: `Include`
    Include,
    /// DCB value: `Exclude`
    Exclude,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireHazardFogNoiseTextures`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireHazardFogNoiseTextures {
    /// DCB value: `Noise0`
    Noise0,
    /// DCB value: `Noise1`
    Noise1,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireHazardMaterials`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireHazardMaterials {
    /// DCB value: `Baseline`
    Baseline,
    /// DCB value: `MaxSpecular`
    MaxSpecular,
    /// DCB value: `MaxGloss`
    MaxGloss,
    /// DCB value: `MaxSpecularAndGloss`
    MaxSpecularAndGloss,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FireHazardSurfaceDirections`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FireHazardSurfaceDirections {
    /// DCB value: `Floors`
    Floors,
    /// DCB value: `Walls`
    Walls,
    /// DCB value: `Ceilings`
    Ceilings,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FlashValueUpdateMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlashValueUpdateMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Offset`
    Offset,
    /// DCB value: `Overwrite`
    Overwrite,
    /// DCB value: `AttachContent`
    AttachContent,
    /// DCB value: `AttachScreen`
    AttachScreen,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FlightHUDMessageType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlightHUDMessageType {
    /// DCB value: `DefaultEmptyMessageDoesNotDisplay`
    DefaultEmptyMessageDoesNotDisplay,
    /// DCB value: `VehicleEjectWarning`
    VehicleEjectWarning,
    /// DCB value: `ShieldCritical`
    ShieldCritical,
    /// DCB value: `ShieldDown`
    ShieldDown,
    /// DCB value: `ShieldRecharge`
    ShieldRecharge,
    /// DCB value: `ProximityWarning`
    ProximityWarning,
    /// DCB value: `FuelEmptyWarning`
    FuelEmptyWarning,
    /// DCB value: `FuelLevel10Warning`
    FuelLevel10Warning,
    /// DCB value: `FuelLevel25Warning`
    FuelLevel25Warning,
    /// DCB value: `FuelLevel50Warning`
    FuelLevel50Warning,
    /// DCB value: `FuelLevel75Warning`
    FuelLevel75Warning,
    /// DCB value: `LowPowerWeaponsWarning`
    LowPowerWeaponsWarning,
    /// DCB value: `LowPowerShieldsWarning`
    LowPowerShieldsWarning,
    /// DCB value: `LowPowerThrustersWarning`
    LowPowerThrustersWarning,
    /// DCB value: `LowPowerGenericWarning`
    LowPowerGenericWarning,
    /// DCB value: `OverheatingWeaponsWarning`
    OverheatingWeaponsWarning,
    /// DCB value: `OverheatingShieldsWarning`
    OverheatingShieldsWarning,
    /// DCB value: `OverheatingThrustersWarning`
    OverheatingThrustersWarning,
    /// DCB value: `OverheatingGenericWarning`
    OverheatingGenericWarning,
    /// DCB value: `OverheatedWeaponsWarning`
    OverheatedWeaponsWarning,
    /// DCB value: `OverheatedShieldsWarning`
    OverheatedShieldsWarning,
    /// DCB value: `OverheatedThrustersWarning`
    OverheatedThrustersWarning,
    /// DCB value: `OverheatedGenericWarning`
    OverheatedGenericWarning,
    /// DCB value: `QuantumDriveInterdictionAlert`
    QuantumDriveInterdictionAlert,
    /// DCB value: `IncomingEMPDetected`
    IncomingEMPDetected,
    /// DCB value: `GreenZoneWarning`
    GreenZoneWarning,
    /// DCB value: `SelfDestructWarning`
    SelfDestructWarning,
    /// DCB value: `Repairing`
    Repairing,
    /// DCB value: `RepairFinished`
    RepairFinished,
    /// DCB value: `RequestAlignment`
    RequestAlignment,
    /// DCB value: `ObstructingLandingArea`
    ObstructingLandingArea,
    /// DCB value: `EMPOffline`
    EMPOffline,
    /// DCB value: `PingWarningMessage`
    PingWarningMessage,
    /// DCB value: `ScannedWarningMessage`
    ScannedWarningMessage,
    /// DCB value: `PortsUnlocked`
    PortsUnlocked,
    /// DCB value: `CatastrophicFailureWarning`
    CatastrophicFailureWarning,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ForceKnockdownDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForceKnockdownDirection {
    /// DCB value: `None`
    None,
    /// DCB value: `Forward`
    Forward,
    /// DCB value: `Backward`
    Backward,
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ForceReactionFilterType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForceReactionFilterType {
    /// DCB value: `Standing`
    Standing,
    /// DCB value: `Crouching`
    Crouching,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `JumpFallLand`
    JumpFallLand,
    /// DCB value: `Usable`
    Usable,
    /// DCB value: `EVA`
    EVA,
    /// DCB value: `Dead`
    Dead,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FriendlyFireType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FriendlyFireType {
    /// DCB value: `None`
    None,
    /// DCB value: `Self`
    Self_,
    /// DCB value: `Team`
    Team,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `FuelTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuelTypes {
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `QuantumFuel`
    QuantumFuel,
    /// DCB value: `HydrogenFuel`
    HydrogenFuel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GRID_TYPE`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GRID_TYPE {
    /// DCB value: `Global`
    Global,
    /// DCB value: `Planetary`
    Planetary,
    /// DCB value: `Small`
    Small,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Large`
    Large,
    /// DCB value: `SolarSystem`
    SolarSystem,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GameplayTrigger_SelectionMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameplayTrigger_SelectionMode {
    /// DCB value: `All`
    All,
    /// DCB value: `Any`
    Any,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GameplayTrigger_Toggle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameplayTrigger_Toggle {
    /// DCB value: `Enable`
    Enable,
    /// DCB value: `Disable`
    Disable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GenderType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenderType {
    /// DCB value: `NotSet`
    NotSet,
    /// DCB value: `Male`
    Male,
    /// DCB value: `Female`
    Female,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GeomForm`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeomForm {
    /// DCB value: `Vertices`
    Vertices,
    /// DCB value: `Edges`
    Edges,
    /// DCB value: `Surface`
    Surface,
    /// DCB value: `Volume`
    Volume,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `GeomType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeomType {
    /// DCB value: `None`
    None,
    /// DCB value: `BoundingBox`
    BoundingBox,
    /// DCB value: `Physics`
    Physics,
    /// DCB value: `Render`
    Render,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HUDPalleteEntry`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HUDPalleteEntry {
    /// DCB value: `Moderate`
    Moderate,
    /// DCB value: `Positive`
    Positive,
    /// DCB value: `Neutral`
    Neutral,
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Critical`
    Critical,
    /// DCB value: `Unknown`
    Unknown,
    /// DCB value: `Highlight`
    Highlight,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `SubItemTarget`
    SubItemTarget,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HandholdType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HandholdType {
    /// DCB value: `Fixed`
    Fixed,
    /// DCB value: `Cylinder`
    Cylinder,
    /// DCB value: `Corner`
    Corner,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HarvestableOverrideAreaType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HarvestableOverrideAreaType {
    /// DCB value: `AutoRegisterWithZoneHost`
    AutoRegisterWithZoneHost,
    /// DCB value: `ManualEntityLink`
    ManualEntityLink,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HintEventType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HintEventType {
    /// DCB value: `None`
    None,
    /// DCB value: `ActorStatusBuffApplied`
    ActorStatusBuffApplied,
    /// DCB value: `ActorStatusClothingChanged`
    ActorStatusClothingChanged,
    /// DCB value: `ActorStatusDeadlyInjuryStarted`
    ActorStatusDeadlyInjuryStarted,
    /// DCB value: `ActorStatusDigestionAbsorptionEmpty`
    ActorStatusDigestionAbsorptionEmpty,
    /// DCB value: `ActorStatusDownedEnter`
    ActorStatusDownedEnter,
    /// DCB value: `ActorStatusEffectArmsLockCannotClimbLadder`
    ActorStatusEffectArmsLockCannotClimbLadder,
    /// DCB value: `ActorStatusEffectArmsLockCannotMantle`
    ActorStatusEffectArmsLockCannotMantle,
    /// DCB value: `ActorStatusEffectArmsLockCannotSteerShip`
    ActorStatusEffectArmsLockCannotSteerShip,
    /// DCB value: `ActorStatusEffectArmsLockCannotRestrain`
    ActorStatusEffectArmsLockCannotRestrain,
    /// DCB value: `ActorStatusEffectArmsLockCannotTakeDown`
    ActorStatusEffectArmsLockCannotTakeDown,
    /// DCB value: `ActorStatusEffectArmsLockCannotUseMountedGun`
    ActorStatusEffectArmsLockCannotUseMountedGun,
    /// DCB value: `ActorStatusEffectArmsLockCannotUseTrolley`
    ActorStatusEffectArmsLockCannotUseTrolley,
    /// DCB value: `ActorStatusEffectArmsLockDropItem`
    ActorStatusEffectArmsLockDropItem,
    /// DCB value: `ActorStatusEffectArmsLockLowerWeapon`
    ActorStatusEffectArmsLockLowerWeapon,
    /// DCB value: `ActorStatusEffectArmsLockStart`
    ActorStatusEffectArmsLockStart,
    /// DCB value: `ActorStatusEffectBleedStart`
    ActorStatusEffectBleedStart,
    /// DCB value: `ActorStatusEffectProneLockStart`
    ActorStatusEffectProneLockStart,
    /// DCB value: `ActorStatusFoodDrinkItemHeld`
    ActorStatusFoodDrinkItemHeld,
    /// DCB value: `ActorStatusFoodDrinkItemInInventory`
    ActorStatusFoodDrinkItemInInventory,
    /// DCB value: `ActorStatusHospitalEnter`
    ActorStatusHospitalEnter,
    /// DCB value: `ActorStatusHospitalRoomReserved`
    ActorStatusHospitalRoomReserved,
    /// DCB value: `ActorStatusHungerThirstDamageStarted`
    ActorStatusHungerThirstDamageStarted,
    /// DCB value: `ActorStatusHungerThirstDeath`
    ActorStatusHungerThirstDeath,
    /// DCB value: `ActorStatusHungerThirstFull`
    ActorStatusHungerThirstFull,
    /// DCB value: `ActorStatusHungerThirstStatusStarted`
    ActorStatusHungerThirstStatusStarted,
    /// DCB value: `ActorStatusHyperthermiaDamageStarted`
    ActorStatusHyperthermiaDamageStarted,
    /// DCB value: `ActorStatusHyperthermiaStarted`
    ActorStatusHyperthermiaStarted,
    /// DCB value: `ActorStatusHypothermiaDamageStarted`
    ActorStatusHypothermiaDamageStarted,
    /// DCB value: `ActorStatusHypothermiaStarted`
    ActorStatusHypothermiaStarted,
    /// DCB value: `ActorStatusInjuryStarted`
    ActorStatusInjuryStarted,
    /// DCB value: `ActorStatusIntoxicatedEnter`
    ActorStatusIntoxicatedEnter,
    /// DCB value: `ActorStatusMajorInjuryStarted`
    ActorStatusMajorInjuryStarted,
    /// DCB value: `ActorStatusMedBedEnter`
    ActorStatusMedBedEnter,
    /// DCB value: `ActorStatusOverdoseDamageStarted`
    ActorStatusOverdoseDamageStarted,
    /// DCB value: `ActorStatusOverdoseDeath`
    ActorStatusOverdoseDeath,
    /// DCB value: `ActorStatusOverdoseEnter`
    ActorStatusOverdoseEnter,
    /// DCB value: `ActorStatusPITMenuDrugs`
    ActorStatusPITMenuDrugs,
    /// DCB value: `ActorStatusRespawnMedBed`
    ActorStatusRespawnMedBed,
    /// DCB value: `ActorStatusRespawnLocationSet`
    ActorStatusRespawnLocationSet,
    /// DCB value: `ActorStatusRespawnCriminal`
    ActorStatusRespawnCriminal,
    /// DCB value: `ActorStatusRespawnPrison`
    ActorStatusRespawnPrison,
    /// DCB value: `ActorStatusTemperatureAboveMaxResistance`
    ActorStatusTemperatureAboveMaxResistance,
    /// DCB value: `ActorStatusTemperatureBelowMinResistance`
    ActorStatusTemperatureBelowMinResistance,
    /// DCB value: `ActorStatusTemperatureDeath`
    ActorStatusTemperatureDeath,
    /// DCB value: `ActorStatusWearingHelmetConsume`
    ActorStatusWearingHelmetConsume,
    /// DCB value: `ActorStatusWearingNoUndersuitConsume`
    ActorStatusWearingNoUndersuitConsume,
    /// DCB value: `ActorStatusWeatherLocomotionEntered`
    ActorStatusWeatherLocomotionEntered,
    /// DCB value: `AlignShipForCargoTransfer`
    AlignShipForCargoTransfer,
    /// DCB value: `ASOPShipSpawned`
    ASOPShipSpawned,
    /// DCB value: `BodyCarry_Start`
    BodyCarry_Start,
    /// DCB value: `BodyDrag_Start`
    BodyDrag_Start,
    /// DCB value: `CargoTransferInterruptions`
    CargoTransferInterruptions,
    /// DCB value: `ChatOpened`
    ChatOpened,
    /// DCB value: `CollectCargoFromLoadingArea`
    CollectCargoFromLoadingArea,
    /// DCB value: `ConsumableFirstMiningConsumableBought`
    ConsumableFirstMiningConsumableBought,
    /// DCB value: `ConsumableFirstMiningWithConsumablesEquipped`
    ConsumableFirstMiningWithConsumablesEquipped,
    /// DCB value: `ConsumableMiningConsumableExpired`
    ConsumableMiningConsumableExpired,
    /// DCB value: `ConsumableMiningConsumableExpiredNoCharges`
    ConsumableMiningConsumableExpiredNoCharges,
    /// DCB value: `ConsumableMiningConsumableUsed`
    ConsumableMiningConsumableUsed,
    /// DCB value: `ContactsOpened`
    ContactsOpened,
    /// DCB value: `CustomizeACLoadoutMenuEntered`
    CustomizeACLoadoutMenuEntered,
    /// DCB value: `CustomizeSMLoadoutMenuEntered`
    CustomizeSMLoadoutMenuEntered,
    /// DCB value: `DeliverCargoToLoadingArea`
    DeliverCargoToLoadingArea,
    /// DCB value: `FPSMiningAttachmentPurchased`
    FPSMiningAttachmentPurchased,
    /// DCB value: `FPSMiningFinishedRockScan`
    FPSMiningFinishedRockScan,
    /// DCB value: `FPSMiningRockFracturedBad`
    FPSMiningRockFracturedBad,
    /// DCB value: `FPSMiningRockFracturedGood`
    FPSMiningRockFracturedGood,
    /// DCB value: `FPSMiningRockNearby`
    FPSMiningRockNearby,
    /// DCB value: `FPSMiningRockPowerDangerLevel`
    FPSMiningRockPowerDangerLevel,
    /// DCB value: `FPSMiningRockPowerOptimalLevel`
    FPSMiningRockPowerOptimalLevel,
    /// DCB value: `FPSMiningRockTargetedNotADS`
    FPSMiningRockTargetedNotADS,
    /// DCB value: `FPSMiningShotEnvironment`
    FPSMiningShotEnvironment,
    /// DCB value: `FPSMiningShotRockADS`
    FPSMiningShotRockADS,
    /// DCB value: `FPSMiningWeaponSelected`
    FPSMiningWeaponSelected,
    /// DCB value: `GreenZoneEntered`
    GreenZoneEntered,
    /// DCB value: `GreenZoneExited`
    GreenZoneExited,
    /// DCB value: `HailCargoServicesForLoading`
    HailCargoServicesForLoading,
    /// DCB value: `HarvestableInHandBackpackNotFull`
    HarvestableInHandBackpackNotFull,
    /// DCB value: `HarvestableNotInHand`
    HarvestableNotInHand,
    /// DCB value: `HarvestablePlayerLooksAtHarvestableBackpackNotFull`
    HarvestablePlayerLooksAtHarvestableBackpackNotFull,
    /// DCB value: `HarvestablePlayerLooksAtHarvestableNoBackpack`
    HarvestablePlayerLooksAtHarvestableNoBackpack,
    /// DCB value: `HintAreaEntered`
    HintAreaEntered,
    /// DCB value: `HintAreaExited`
    HintAreaExited,
    /// DCB value: `HullC_CargoRequiresSpindlesExtended`
    HullC_CargoRequiresSpindlesExtended,
    /// DCB value: `InGameStateBegin`
    InGameStateBegin,
    /// DCB value: `InteractionModeEntered`
    InteractionModeEntered,
    /// DCB value: `InteractionModeExited`
    InteractionModeExited,
    /// DCB value: `InteractionConditionGeneric`
    InteractionConditionGeneric,
    /// DCB value: `InteractionConditionSpecific`
    InteractionConditionSpecific,
    /// DCB value: `KnockdownStarted`
    KnockdownStarted,
    /// DCB value: `LegallyLandedShipStarted`
    LegallyLandedShipStarted,
    /// DCB value: `LogoutEntered`
    LogoutEntered,
    /// DCB value: `LogoutEnteredNonOwned`
    LogoutEnteredNonOwned,
    /// DCB value: `LogoutEnteredNoPlayersAround`
    LogoutEnteredNoPlayersAround,
    /// DCB value: `LogoutEnteredPlayersAround`
    LogoutEnteredPlayersAround,
    /// DCB value: `LogoutEnteredPlayersInShip`
    LogoutEnteredPlayersInShip,
    /// DCB value: `LogoutRespawnFailed`
    LogoutRespawnFailed,
    /// DCB value: `LogoutEnteredShipInShip`
    LogoutEnteredShipInShip,
    /// DCB value: `LogoutEnteredNonPersistent`
    LogoutEnteredNonPersistent,
    /// DCB value: `MedBeamEquippedMedgun`
    MedBeamEquippedMedgun,
    /// DCB value: `MedBeamEquippedMultiTool`
    MedBeamEquippedMultiTool,
    /// DCB value: `MedBeamSelfHealPrompt`
    MedBeamSelfHealPrompt,
    /// DCB value: `MedBeamSelfHealDescription`
    MedBeamSelfHealDescription,
    /// DCB value: `MedBeamValidTargetAcquired`
    MedBeamValidTargetAcquired,
    /// DCB value: `MedBeamValidTargetAcquiredIsHurt`
    MedBeamValidTargetAcquiredIsHurt,
    /// DCB value: `MedBeamBDLTooHighMedgun`
    MedBeamBDLTooHighMedgun,
    /// DCB value: `MedBeamBDLTooHighMultiTool`
    MedBeamBDLTooHighMultiTool,
    /// DCB value: `MedBeamMedgunToggleAdvancedMode`
    MedBeamMedgunToggleAdvancedMode,
    /// DCB value: `MedBeamMedgunOnAdvancedModeActivated`
    MedBeamMedgunOnAdvancedModeActivated,
    /// DCB value: `MedBeamMedgunAdvancedModeButtons`
    MedBeamMedgunAdvancedModeButtons,
    /// DCB value: `MedBeamMedgunAdvancedModeSafeBDLBypass`
    MedBeamMedgunAdvancedModeSafeBDLBypass,
    /// DCB value: `MedBeamMedgunAdvancedModeCriticalBDLBypass`
    MedBeamMedgunAdvancedModeCriticalBDLBypass,
    /// DCB value: `MedBedClearAllRespawns`
    MedBedClearAllRespawns,
    /// DCB value: `MedBedEraseDNA`
    MedBedEraseDNA,
    /// DCB value: `MedBedRespawnNotAvailable`
    MedBedRespawnNotAvailable,
    /// DCB value: `MedBedRespawnOutOfRange`
    MedBedRespawnOutOfRange,
    /// DCB value: `MedBedUploadDNA`
    MedBedUploadDNA,
    /// DCB value: `MedpenStabBlockedInGreenZone`
    MedpenStabBlockedInGreenZone,
    /// DCB value: `MeleeEquip`
    MeleeEquip,
    /// DCB value: `MeleeLightAttacks`
    MeleeLightAttacks,
    /// DCB value: `MeleeHooks`
    MeleeHooks,
    /// DCB value: `MeleeHaymaker`
    MeleeHaymaker,
    /// DCB value: `MeleeUppercut`
    MeleeUppercut,
    /// DCB value: `MeleeCombinationAttacks`
    MeleeCombinationAttacks,
    /// DCB value: `MeleeBlocking`
    MeleeBlocking,
    /// DCB value: `MeleeDodging`
    MeleeDodging,
    /// DCB value: `MeleeStun`
    MeleeStun,
    /// DCB value: `MeleeKnifeEquip`
    MeleeKnifeEquip,
    /// DCB value: `MeleeKnifeLight`
    MeleeKnifeLight,
    /// DCB value: `MeleeKnifeHeavy`
    MeleeKnifeHeavy,
    /// DCB value: `MeleeTakedownGun`
    MeleeTakedownGun,
    /// DCB value: `MeleeTakedownFists`
    MeleeTakedownFists,
    /// DCB value: `MeleeTakedownWeapon`
    MeleeTakedownWeapon,
    /// DCB value: `MeleeTakedownStop`
    MeleeTakedownStop,
    /// DCB value: `MFDFocusEntered`
    MFDFocusEntered,
    /// DCB value: `MFDFocusExited`
    MFDFocusExited,
    /// DCB value: `MiningExtractableRockTargeted`
    MiningExtractableRockTargeted,
    /// DCB value: `MiningFracturableRockTargeted`
    MiningFracturableRockTargeted,
    /// DCB value: `MiningModeSwitch`
    MiningModeSwitch,
    /// DCB value: `MiningNoProgress`
    MiningNoProgress,
    /// DCB value: `MiningRadarBlobFound`
    MiningRadarBlobFound,
    /// DCB value: `MiningRockExtracted`
    MiningRockExtracted,
    /// DCB value: `MiningRockFractured`
    MiningRockFractured,
    /// DCB value: `MiningRockInPassiveRadar`
    MiningRockInPassiveRadar,
    /// DCB value: `MiningRockPowerDangerLevel`
    MiningRockPowerDangerLevel,
    /// DCB value: `MiningRockPowerOptimalLevel`
    MiningRockPowerOptimalLevel,
    /// DCB value: `MiningShipCargoFull`
    MiningShipCargoFull,
    /// DCB value: `MiningShipCargoHalfFull`
    MiningShipCargoHalfFull,
    /// DCB value: `MiningShipIsFlightReady`
    MiningShipIsFlightReady,
    /// DCB value: `MiningShipLowMoonOrbit`
    MiningShipLowMoonOrbit,
    /// DCB value: `MobiGlasOpened`
    MobiGlasOpened,
    /// DCB value: `MobiGlasClosed`
    MobiGlasClosed,
    /// DCB value: `MobiGlasJournalOpened`
    MobiGlasJournalOpened,
    /// DCB value: `MobiGlasMissionManagerOpened`
    MobiGlasMissionManagerOpened,
    /// DCB value: `MonitoredZoneEntered`
    MonitoredZoneEntered,
    /// DCB value: `MonitoredZoneExited`
    MonitoredZoneExited,
    /// DCB value: `OxygenDroppedTo50pcHasCapsules`
    OxygenDroppedTo50pcHasCapsules,
    /// DCB value: `OxygenDroppedTo50pcNoCapsules`
    OxygenDroppedTo50pcNoCapsules,
    /// DCB value: `OxygenDroppedTo25pcHasCapsules`
    OxygenDroppedTo25pcHasCapsules,
    /// DCB value: `OxygenDroppedTo25pcNoCapsules`
    OxygenDroppedTo25pcNoCapsules,
    /// DCB value: `OxygenRefilled`
    OxygenRefilled,
    /// DCB value: `PersonalInnerThoughtClosed`
    PersonalInnerThoughtClosed,
    /// DCB value: `PersonalInnerThoughtOpened`
    PersonalInnerThoughtOpened,
    /// DCB value: `PersonalInnerThoughtQuickSelectionMode`
    PersonalInnerThoughtQuickSelectionMode,
    /// DCB value: `PersonalInnerThoughtQuickSelectionWeaponsOpened`
    PersonalInnerThoughtQuickSelectionWeaponsOpened,
    /// DCB value: `PersonalInnerThoughtQuickSelectionModeOpened`
    PersonalInnerThoughtQuickSelectionModeOpened,
    /// DCB value: `PersonalInnerThoughtGreyedOutActions`
    PersonalInnerThoughtGreyedOutActions,
    /// DCB value: `PersonalInnerThoughtAddToFavorites`
    PersonalInnerThoughtAddToFavorites,
    /// DCB value: `PersonalInnerThoughtTriedToCustomizeDefaultActions`
    PersonalInnerThoughtTriedToCustomizeDefaultActions,
    /// DCB value: `PersonalInnerThoughtCancelRebind`
    PersonalInnerThoughtCancelRebind,
    /// DCB value: `PersonalInventoryBackpackFull`
    PersonalInventoryBackpackFull,
    /// DCB value: `PersonalInventoryHarvestableLootedIntoBackpack`
    PersonalInventoryHarvestableLootedIntoBackpack,
    /// DCB value: `PersonalInventoryItemStowed`
    PersonalInventoryItemStowed,
    /// DCB value: `PersonalInventoryShardLootedIntoBackpack`
    PersonalInventoryShardLootedIntoBackpack,
    /// DCB value: `PersonalInventoryHarvestableDirectlyStowedIntoBackpack`
    PersonalInventoryHarvestableDirectlyStowedIntoBackpack,
    /// DCB value: `PersonalInventoryUndersuitClothingIncompatible`
    PersonalInventoryUndersuitClothingIncompatible,
    /// DCB value: `PersonalInventoryOpen`
    PersonalInventoryOpen,
    /// DCB value: `PersonalInventoryHomeItemAdded`
    PersonalInventoryHomeItemAdded,
    /// DCB value: `PersonalInventoryItemPurchased`
    PersonalInventoryItemPurchased,
    /// DCB value: `PersonalInventoryContainerSizeLimits`
    PersonalInventoryContainerSizeLimits,
    /// DCB value: `PersonalInventoryContainerFull`
    PersonalInventoryContainerFull,
    /// DCB value: `PersonalInventoryVehicleEnter`
    PersonalInventoryVehicleEnter,
    /// DCB value: `PersonalInventoryOpenInVehicle`
    PersonalInventoryOpenInVehicle,
    /// DCB value: `PersonalInventoryVehicleStorageUsed`
    PersonalInventoryVehicleStorageUsed,
    /// DCB value: `PersonalInventoryVehicleDestroyed`
    PersonalInventoryVehicleDestroyed,
    /// DCB value: `PersonalInventoryOpenWithNoAvailableContainer`
    PersonalInventoryOpenWithNoAvailableContainer,
    /// DCB value: `PersonalInventoryInvalidClothingArmorEquip`
    PersonalInventoryInvalidClothingArmorEquip,
    /// DCB value: `PersonalInventoryOutOfRange`
    PersonalInventoryOutOfRange,
    /// DCB value: `PickupCollected`
    PickupCollected,
    /// DCB value: `PlayerCombatHealed`
    PlayerCombatHealed,
    /// DCB value: `PlayerEnteredEVA`
    PlayerEnteredEVA,
    /// DCB value: `PlayerGotAKill`
    PlayerGotAKill,
    /// DCB value: `PlayerLowHealth`
    PlayerLowHealth,
    /// DCB value: `PlayerPickupItem`
    PlayerPickupItem,
    /// DCB value: `PlayerStowItem`
    PlayerStowItem,
    /// DCB value: `PlayerThrewItem`
    PlayerThrewItem,
    /// DCB value: `PlayerWasKilled`
    PlayerWasKilled,
    /// DCB value: `PlayerWasKilledByGrenade`
    PlayerWasKilledByGrenade,
    /// DCB value: `PrivateMatchMenuEntered`
    PrivateMatchMenuEntered,
    /// DCB value: `RepairBeam_AmmoEmpty`
    RepairBeam_AmmoEmpty,
    /// DCB value: `RepairBeam_HullOverview`
    RepairBeam_HullOverview,
    /// DCB value: `RepairBeam_Repairing`
    RepairBeam_Repairing,
    /// DCB value: `RepairBeam_VehicleTargetAquired`
    RepairBeam_VehicleTargetAquired,
    /// DCB value: `RestrictedAreaTunnelActivatedLanding`
    RestrictedAreaTunnelActivatedLanding,
    /// DCB value: `RestrictedAreaTunnelActivatedTakeoff`
    RestrictedAreaTunnelActivatedTakeoff,
    /// DCB value: `RestrictedAreaTunnelEdge`
    RestrictedAreaTunnelEdge,
    /// DCB value: `RestrictedAreaTunnelNearby`
    RestrictedAreaTunnelNearby,
    /// DCB value: `SalvageBeam_AmmoFull`
    SalvageBeam_AmmoFull,
    /// DCB value: `SalvageBeam_HullOverview`
    SalvageBeam_HullOverview,
    /// DCB value: `SalvageBeam_MaterialDepleeted`
    SalvageBeam_MaterialDepleeted,
    /// DCB value: `SalvageBeam_Salvaging`
    SalvageBeam_Salvaging,
    /// DCB value: `SalvageBeam_VehicleTargetAquired`
    SalvageBeam_VehicleTargetAquired,
    /// DCB value: `SalvageRepairBeam_Shields`
    SalvageRepairBeam_Shields,
    /// DCB value: `SalvageRepairBeam_SwitchFireMode`
    SalvageRepairBeam_SwitchFireMode,
    /// DCB value: `ScanModeEntered`
    ScanModeEntered,
    /// DCB value: `ScanModeExited`
    ScanModeExited,
    /// DCB value: `ShipAfterburnerOn`
    ShipAfterburnerOn,
    /// DCB value: `ShipBoostOn`
    ShipBoostOn,
    /// DCB value: `ShipCollision`
    ShipCollision,
    /// DCB value: `ShipCritPartHealthDroppedTo25pc`
    ShipCritPartHealthDroppedTo25pc,
    /// DCB value: `ShipOverallHealthDroppedTo75pc`
    ShipOverallHealthDroppedTo75pc,
    /// DCB value: `ShipAnyPartAndItemDamaged`
    ShipAnyPartAndItemDamaged,
    /// DCB value: `ShipDecoupledOn`
    ShipDecoupledOn,
    /// DCB value: `ShipGSafeToggle`
    ShipGSafeToggle,
    /// DCB value: `ShipHitByGun`
    ShipHitByGun,
    /// DCB value: `ShipHitByMissile`
    ShipHitByMissile,
    /// DCB value: `ShipIsFlightReady`
    ShipIsFlightReady,
    /// DCB value: `ShipEnteringLandingArea`
    ShipEnteringLandingArea,
    /// DCB value: `ShipLandingGearRaised`
    ShipLandingGearRaised,
    /// DCB value: `ShipLandingGearLowered`
    ShipLandingGearLowered,
    /// DCB value: `ShipPitchedOrYawedOnce`
    ShipPitchedOrYawedOnce,
    /// DCB value: `ShipHydrogenFuelDroppedTo25pc`
    ShipHydrogenFuelDroppedTo25pc,
    /// DCB value: `ShipHydrogenFuelDroppedTo75pc`
    ShipHydrogenFuelDroppedTo75pc,
    /// DCB value: `ShipHydrogenFuelRefilled`
    ShipHydrogenFuelRefilled,
    /// DCB value: `ShipQuantumFuelDroppedTo25pc`
    ShipQuantumFuelDroppedTo25pc,
    /// DCB value: `ShipQuantumFuelDroppedTo75pc`
    ShipQuantumFuelDroppedTo75pc,
    /// DCB value: `ShipQuantumFuelRefilled`
    ShipQuantumFuelRefilled,
    /// DCB value: `ShipQuantumTravelObstructed`
    ShipQuantumTravelObstructed,
    /// DCB value: `ShipQuantumDriveSpoolingStarted`
    ShipQuantumDriveSpoolingStarted,
    /// DCB value: `ShipQuantumDriveSpoolingOff`
    ShipQuantumDriveSpoolingOff,
    /// DCB value: `ShipQuantumDriveTriedSpoolingInCooldown`
    ShipQuantumDriveTriedSpoolingInCooldown,
    /// DCB value: `ShipQuantumCalibrationStarted`
    ShipQuantumCalibrationStarted,
    /// DCB value: `ShipQuantumCalibrationCompletedButNotSpooled`
    ShipQuantumCalibrationCompletedButNotSpooled,
    /// DCB value: `ShipQuantumCalibrationStalled`
    ShipQuantumCalibrationStalled,
    /// DCB value: `ShipQuantumNoCalibrationButSpooled`
    ShipQuantumNoCalibrationButSpooled,
    /// DCB value: `ShipQuantumCalibratedSpooledAlone`
    ShipQuantumCalibratedSpooledAlone,
    /// DCB value: `ShipQuantumCalibratedSpooledInGroup`
    ShipQuantumCalibratedSpooledInGroup,
    /// DCB value: `ShipSeatEntered`
    ShipSeatEntered,
    /// DCB value: `ShipShieldsDown`
    ShipShieldsDown,
    /// DCB value: `ShipZoneEntered`
    ShipZoneEntered,
    /// DCB value: `ShipTakenOff`
    ShipTakenOff,
    /// DCB value: `ShipThrottledUpOnce`
    ShipThrottledUpOnce,
    /// DCB value: `ShipStartedPreRampUp`
    ShipStartedPreRampUp,
    /// DCB value: `ShipStartedQuantumTravel`
    ShipStartedQuantumTravel,
    /// DCB value: `ShipEndedQuantumTravel`
    ShipEndedQuantumTravel,
    /// DCB value: `ShipEndedPostRampDown`
    ShipEndedPostRampDown,
    /// DCB value: `ShipAbortedQuantumTravel`
    ShipAbortedQuantumTravel,
    /// DCB value: `ShipWeaponGroup2Fired`
    ShipWeaponGroup2Fired,
    /// DCB value: `ShipWeaponsFired`
    ShipWeaponsFired,
    /// DCB value: `ShipMissilesDroppedTo25pc`
    ShipMissilesDroppedTo25pc,
    /// DCB value: `ShipMissilesRefilled`
    ShipMissilesRefilled,
    /// DCB value: `ShipBulletsDroppedTo25pc`
    ShipBulletsDroppedTo25pc,
    /// DCB value: `ShipBulletsRefilled`
    ShipBulletsRefilled,
    /// DCB value: `ShoppingTryOnInspectEnter`
    ShoppingTryOnInspectEnter,
    /// DCB value: `ShoppingTryOnInspectExit`
    ShoppingTryOnInspectExit,
    /// DCB value: `SignatureSystemPingAngleChanged`
    SignatureSystemPingAngleChanged,
    /// DCB value: `SignatureSystemPingTriggered`
    SignatureSystemPingTriggered,
    /// DCB value: `SignatureSystemScanAbandonned`
    SignatureSystemScanAbandonned,
    /// DCB value: `SignatureSystemScanCompleted`
    SignatureSystemScanCompleted,
    /// DCB value: `SignatureSystemScanModeEntered`
    SignatureSystemScanModeEntered,
    /// DCB value: `SignatureSystemScanModeExited`
    SignatureSystemScanModeExited,
    /// DCB value: `SignatureSystemScanStarted`
    SignatureSystemScanStarted,
    /// DCB value: `SignatureSystemScanZoomChanged`
    SignatureSystemScanZoomChanged,
    /// DCB value: `SpawnPointEnter`
    SpawnPointEnter,
    /// DCB value: `SpawnPointExit`
    SpawnPointExit,
    /// DCB value: `StaminaDroppedTo90pc`
    StaminaDroppedTo90pc,
    /// DCB value: `StaminaDroppedTo25pc`
    StaminaDroppedTo25pc,
    /// DCB value: `StartGame`
    StartGame,
    /// DCB value: `TrackviewButtonPressedAllowSwitch`
    TrackviewButtonPressedAllowSwitch,
    /// DCB value: `TrackviewButtonPressedDontAllowSwitch`
    TrackviewButtonPressedDontAllowSwitch,
    /// DCB value: `TrackviewCameraSwitchTimeOut`
    TrackviewCameraSwitchTimeOut,
    /// DCB value: `TrackviewCameraSwitchedTo1P`
    TrackviewCameraSwitchedTo1P,
    /// DCB value: `TractorBeamPlayerEquipped`
    TractorBeamPlayerEquipped,
    /// DCB value: `TractorBeamTetheredToTarget`
    TractorBeamTetheredToTarget,
    /// DCB value: `TractorBeamDistControlUsed`
    TractorBeamDistControlUsed,
    /// DCB value: `TractorBeamTetherBrokenDistance`
    TractorBeamTetherBrokenDistance,
    /// DCB value: `TractorBeamTargetedTooHeavyObject`
    TractorBeamTargetedTooHeavyObject,
    /// DCB value: `TractorBeamTargetedTooLargeObject`
    TractorBeamTargetedTooLargeObject,
    /// DCB value: `TractorBeamEnteredZeroG`
    TractorBeamEnteredZeroG,
    /// DCB value: `TractorBeamLineOfSightBroken`
    TractorBeamLineOfSightBroken,
    /// DCB value: `TractorBeamTetherBrokenFastMovement`
    TractorBeamTetherBrokenFastMovement,
    /// DCB value: `UnboundKeyShown`
    UnboundKeyShown,
    /// DCB value: `UsableEntered`
    UsableEntered,
    /// DCB value: `UsableExited`
    UsableExited,
    /// DCB value: `VolatileCargoCollected`
    VolatileCargoCollected,
    /// DCB value: `VolatileCargoCollectedFirstTime`
    VolatileCargoCollectedFirstTime,
    /// DCB value: `VolatileCargoCritical`
    VolatileCargoCritical,
    /// DCB value: `VolatileCargoExplodedNoDeath`
    VolatileCargoExplodedNoDeath,
    /// DCB value: `VolatileCargoJettisoned`
    VolatileCargoJettisoned,
    /// DCB value: `VolatileCargoLightOnCritical`
    VolatileCargoLightOnCritical,
    /// DCB value: `VolatileCargoLightOnWarning`
    VolatileCargoLightOnWarning,
    /// DCB value: `WantedLevelIncreased`
    WantedLevelIncreased,
    /// DCB value: `WeaponADSActivateNightvision`
    WeaponADSActivateNightvision,
    /// DCB value: `WeaponADSActivateZoom`
    WeaponADSActivateZoom,
    /// DCB value: `WeaponADSUsed`
    WeaponADSUsed,
    /// DCB value: `WeaponADSZoomedOut`
    WeaponADSZoomedOut,
    /// DCB value: `WeaponZeroingEnteredADSManual`
    WeaponZeroingEnteredADSManual,
    /// DCB value: `WeaponZeroingEnteredADSAuto`
    WeaponZeroingEnteredADSAuto,
    /// DCB value: `WeaponZeroingAutoSet`
    WeaponZeroingAutoSet,
    /// DCB value: `EVENTSCOUNT`
    EVENTSCOUNT,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HitReactionPart`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HitReactionPart {
    /// DCB value: `Head`
    Head,
    /// DCB value: `Torso`
    Torso,
    /// DCB value: `Crotch`
    Crotch,
    /// DCB value: `LegLeft`
    LegLeft,
    /// DCB value: `LegRight`
    LegRight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HitReactionRegion`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HitReactionRegion {
    /// DCB value: `Head`
    Head,
    /// DCB value: `UpperBody`
    UpperBody,
    /// DCB value: `LowerBody`
    LowerBody,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `HospitalizationReason`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HospitalizationReason {
    /// DCB value: `none`
    none,
    /// DCB value: `selfCheckin`
    selfCheckin,
    /// DCB value: `died`
    died,
    /// DCB value: `injuredAndBroughtToHangar`
    injuredAndBroughtToHangar,
    /// DCB value: `incapacitatedAndBroughtToHangar`
    incapacitatedAndBroughtToHangar,
    /// DCB value: `incapacitatedInLandingZone`
    incapacitatedInLandingZone,
    /// DCB value: `incapacitatedAndBroughtToHospital`
    incapacitatedAndBroughtToHospital,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Hostility`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hostility {
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Neutral`
    Neutral,
    /// DCB value: `Friendly`
    Friendly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `IconShowCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IconShowCondition {
    /// DCB value: `DefaultShowNever`
    DefaultShowNever,
    /// DCB value: `ShowAlways`
    ShowAlways,
    /// DCB value: `IsFPSRadar`
    IsFPSRadar,
    /// DCB value: `ContainsMatchingAmmo`
    ContainsMatchingAmmo,
    /// DCB value: `ContainsFPSDevice`
    ContainsFPSDevice,
    /// DCB value: `ContainsFPSConsumable`
    ContainsFPSConsumable,
    /// DCB value: `IsDeadOrIncapacitatedActor`
    IsDeadOrIncapacitatedActor,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InnerThoughtDisplayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerThoughtDisplayType {
    /// DCB value: `Show`
    Show,
    /// DCB value: `Hide`
    Hide,
    /// DCB value: `ForeignOnly`
    ForeignOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InnerThoughtJustification`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerThoughtJustification {
    /// DCB value: `Left`
    Left,
    /// DCB value: `Middle`
    Middle,
    /// DCB value: `Right`
    Right,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InnerThoughtOrientation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerThoughtOrientation {
    /// DCB value: `RotateToPlayer`
    RotateToPlayer,
    /// DCB value: `FixedRotation`
    FixedRotation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InnerThought_ForceCase`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerThought_ForceCase {
    /// DCB value: `None`
    None,
    /// DCB value: `Upper`
    Upper,
    /// DCB value: `Lower`
    Lower,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InputDeviceType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputDeviceType {
    /// DCB value: `Keyboard`
    Keyboard,
    /// DCB value: `Mouse`
    Mouse,
    /// DCB value: `Joystick`
    Joystick,
    /// DCB value: `Gamepad`
    Gamepad,
    /// DCB value: `Headmounted`
    Headmounted,
    /// DCB value: `Count`
    Count,
    /// DCB value: `AllInputs`
    AllInputs,
    /// DCB value: `Unknown`
    Unknown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InstancedInteriorSizeEnum`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstancedInteriorSizeEnum {
    /// DCB value: `Small`
    Small,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Large`
    Large,
    /// DCB value: `XLarge`
    XLarge,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractionBindingsMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionBindingsMethod {
    /// DCB value: `None`
    None,
    /// DCB value: `Name`
    Name,
    /// DCB value: `States`
    States,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractionGenericCursor`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionGenericCursor {
    /// DCB value: `SingleAction`
    SingleAction,
    /// DCB value: `MultiAction`
    MultiAction,
    /// DCB value: `Button`
    Button,
    /// DCB value: `Grab`
    Grab,
    /// DCB value: `Conversation`
    Conversation,
    /// DCB value: `Invalid`
    Invalid,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractionModifier`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionModifier {
    /// DCB value: `DefaultStanding`
    DefaultStanding,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `Linked`
    Linked,
    /// DCB value: `EVA`
    EVA,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractionPromptBoundTo`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionPromptBoundTo {
    /// DCB value: `ActorEyes`
    ActorEyes,
    /// DCB value: `EntityRoot`
    EntityRoot,
    /// DCB value: `TopBoundingBoxZSurface`
    TopBoundingBoxZSurface,
    /// DCB value: `NearestBoundingBoxSurface`
    NearestBoundingBoxSurface,
    /// DCB value: `NearestBoundingBoxIgnoreFurthestSurfaces`
    NearestBoundingBoxIgnoreFurthestSurfaces,
    /// DCB value: `InteractionPointFixedOffset`
    InteractionPointFixedOffset,
    /// DCB value: `Tmp_AngleConstraintForwardDirection`
    Tmp_AngleConstraintForwardDirection,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractionPromptStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionPromptStyle {
    /// DCB value: `Prompt`
    Prompt,
    /// DCB value: `Text`
    Text,
    /// DCB value: `Highlight`
    Highlight,
    /// DCB value: `PromptAndText`
    PromptAndText,
    /// DCB value: `PromptAndHighlight`
    PromptAndHighlight,
    /// DCB value: `TextAndHighlight`
    TextAndHighlight,
    /// DCB value: `PromptAndTextAndHighlight`
    PromptAndTextAndHighlight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InteractiveVariableLoopType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractiveVariableLoopType {
    /// DCB value: `NoLoop`
    NoLoop,
    /// DCB value: `Loop`
    Loop,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `InterpolationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InterpolationMode {
    /// DCB value: `Linear`
    Linear,
    /// DCB value: `EaseInQuad`
    EaseInQuad,
    /// DCB value: `EaseOutQuad`
    EaseOutQuad,
    /// DCB value: `EaseInOutQuad`
    EaseInOutQuad,
    /// DCB value: `EaseInCubic`
    EaseInCubic,
    /// DCB value: `EaseOutCubic`
    EaseOutCubic,
    /// DCB value: `EaseInOutCubic`
    EaseInOutCubic,
    /// DCB value: `EaseInExpo`
    EaseInExpo,
    /// DCB value: `EaseOutExpo`
    EaseOutExpo,
    /// DCB value: `EaseInOutExpo`
    EaseInOutExpo,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ItemExpiryState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemExpiryState {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Expiring`
    Expiring,
    /// DCB value: `Expired`
    Expired,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ItemJumpDriveState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemJumpDriveState {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `EntryChecks`
    EntryChecks,
    /// DCB value: `Tuning`
    Tuning,
    /// DCB value: `RequestingJump`
    RequestingJump,
    /// DCB value: `WaitingForOpen`
    WaitingForOpen,
    /// DCB value: `Entering`
    Entering,
    /// DCB value: `Transiting`
    Transiting,
    /// DCB value: `Exiting`
    Exiting,
    /// DCB value: `Failing`
    Failing,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ItemKioskMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKioskMode {
    /// DCB value: `Commodity`
    Commodity,
    /// DCB value: `Reference`
    Reference,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Player`
    Player,
    /// DCB value: `Refinery`
    Refinery,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ItemResourceDeltaType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemResourceDeltaType {
    /// DCB value: `Consumption`
    Consumption,
    /// DCB value: `Generation`
    Generation,
    /// DCB value: `Storage`
    Storage,
    /// DCB value: `Conversion`
    Conversion,
    /// DCB value: `NetworkReflection`
    NetworkReflection,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ItemResourceTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemResourceTypes {
    /// DCB value: `Vent`
    Vent,
    /// DCB value: `LifeSupport`
    LifeSupport,
    /// DCB value: `Light`
    Light,
    /// DCB value: `HeatSource`
    HeatSource,
    /// DCB value: `Cooler`
    Cooler,
    /// DCB value: `Relay`
    Relay,
    /// DCB value: `Misc`
    Misc,
    /// DCB value: `All`
    All,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `JumpVariant`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JumpVariant {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Vault`
    Vault,
    /// DCB value: `ZeroG`
    ZeroG,
    /// DCB value: `KnockdownForward`
    KnockdownForward,
    /// DCB value: `KnockdownBackward`
    KnockdownBackward,
    /// DCB value: `LadderDisembark`
    LadderDisembark,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `KeypadKeys`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeypadKeys {
    /// DCB value: `Key0`
    Key0,
    /// DCB value: `Key1`
    Key1,
    /// DCB value: `Key2`
    Key2,
    /// DCB value: `Key3`
    Key3,
    /// DCB value: `Key4`
    Key4,
    /// DCB value: `Key5`
    Key5,
    /// DCB value: `Key6`
    Key6,
    /// DCB value: `Key7`
    Key7,
    /// DCB value: `Key8`
    Key8,
    /// DCB value: `Key9`
    Key9,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `KioskShopType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KioskShopType {
    /// DCB value: `Commodity`
    Commodity,
    /// DCB value: `ShipItem`
    ShipItem,
    /// DCB value: `Refinery`
    Refinery,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LadderAnimationClimbSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LadderAnimationClimbSpeed {
    /// DCB value: `Walk`
    Walk,
    /// DCB value: `Run`
    Run,
    /// DCB value: `Sprint`
    Sprint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LandingCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandingCondition {
    /// DCB value: `None`
    None,
    /// DCB value: `ADSTriggered`
    ADSTriggered,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LandingExitStance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandingExitStance {
    /// DCB value: `Stand`
    Stand,
    /// DCB value: `Crouch`
    Crouch,
    /// DCB value: `Prone`
    Prone,
    /// DCB value: `ProneBack`
    ProneBack,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LandingStrength`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LandingStrength {
    /// DCB value: `Light`
    Light,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Heavy`
    Heavy,
    /// DCB value: `Impact`
    Impact,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LeanStateType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeanStateType {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `LeanLeft`
    LeanLeft,
    /// DCB value: `LeanRight`
    LeanRight,
    /// DCB value: `LeftToIdle`
    LeftToIdle,
    /// DCB value: `IdleToLeft`
    IdleToLeft,
    /// DCB value: `IdleToRight`
    IdleToRight,
    /// DCB value: `RightToIdle`
    RightToIdle,
    /// DCB value: `LeftToRight`
    LeftToRight,
    /// DCB value: `RightToLeft`
    RightToLeft,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LedgeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LedgeType {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Mantle`
    Mantle,
    /// DCB value: `Vault`
    Vault,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LegacyCraftingPortMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegacyCraftingPortMode {
    /// DCB value: `Both`
    Both,
    /// DCB value: `Input`
    Input,
    /// DCB value: `Output`
    Output,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LevelStreamingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LevelStreamingMode {
    /// DCB value: `Never`
    Never,
    /// DCB value: `Always`
    Always,
    /// DCB value: `ForTesting`
    ForTesting,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LightProperties`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightProperties {
    /// DCB value: `Radius`
    Radius,
    /// DCB value: `DiffuseColor`
    DiffuseColor,
    /// DCB value: `Intensity`
    Intensity,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LightningStrengthPropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightningStrengthPropertyType {
    /// DCB value: `Distance`
    Distance,
    /// DCB value: `ElectricalCharge`
    ElectricalCharge,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LinkedStatReverseSignFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkedStatReverseSignFlag {
    /// DCB value: `None`
    None,
    /// DCB value: `AlwaysPositive`
    AlwaysPositive,
    /// DCB value: `AlwaysNegative`
    AlwaysNegative,
    /// DCB value: `ReverseSign`
    ReverseSign,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LocalizationAvailability`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalizationAvailability {
    /// DCB value: `Available`
    Available,
    /// DCB value: `PendingRemoval`
    PendingRemoval,
    /// DCB value: `Removed`
    Removed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LocalizationLabel`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalizationLabel {
    /// DCB value: `None`
    None,
    /// DCB value: `PU`
    PU,
    /// DCB value: `S42`
    S42,
    /// DCB value: `Common`
    Common,
    /// DCB value: `PU_NotPublic`
    PU_NotPublic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LocalizationLanguage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalizationLanguage {
    /// DCB value: `None`
    None,
    /// DCB value: `enUS`
    enUS,
    /// DCB value: `zhHans`
    zhHans,
    /// DCB value: `zhHant`
    zhHant,
    /// DCB value: `frFR`
    frFR,
    /// DCB value: `deDE`
    deDE,
    /// DCB value: `itIT`
    itIT,
    /// DCB value: `jaJP`
    jaJP,
    /// DCB value: `koKR`
    koKR,
    /// DCB value: `plPL`
    plPL,
    /// DCB value: `ptBR`
    ptBR,
    /// DCB value: `esES`
    esES,
    /// DCB value: `es419`
    es419,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LocalizationStatus`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalizationStatus {
    /// DCB value: `WorkInProgress`
    WorkInProgress,
    /// DCB value: `ReadyForText`
    ReadyForText,
    /// DCB value: `TextApproved`
    TextApproved,
    /// DCB value: `Transcribed`
    Transcribed,
    /// DCB value: `LocalizationPreRequested`
    LocalizationPreRequested,
    /// DCB value: `LocalizationRequested`
    LocalizationRequested,
    /// DCB value: `LocalizationRequestRejected`
    LocalizationRequestRejected,
    /// DCB value: `LocalizationInProgress`
    LocalizationInProgress,
    /// DCB value: `LocalizationNeedsReview`
    LocalizationNeedsReview,
    /// DCB value: `LocalizationPassedReview`
    LocalizationPassedReview,
    /// DCB value: `LocalizationFailedReview`
    LocalizationFailedReview,
    /// DCB value: `DoNotLocalize`
    DoNotLocalize,
    /// DCB value: `LocalizationSourceUpdated`
    LocalizationSourceUpdated,
    /// DCB value: `LocalizationQAChecked`
    LocalizationQAChecked,
    /// DCB value: `LocalizationFinalTranslation`
    LocalizationFinalTranslation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `LocalizationUse`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LocalizationUse {
    /// DCB value: `General`
    General,
    /// DCB value: `Dialogue`
    Dialogue,
    /// DCB value: `DialogueAnimation`
    DialogueAnimation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MapDisplayFrameType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapDisplayFrameType {
    /// DCB value: `Rectangle2D`
    Rectangle2D,
    /// DCB value: `Rectangle2DRTT`
    Rectangle2DRTT,
    /// DCB value: `Ellipse2D`
    Ellipse2D,
    /// DCB value: `Ellipse2DRTT`
    Ellipse2DRTT,
    /// DCB value: `Sphere3D`
    Sphere3D,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MapMarkerTrackingPlaneAlignmentMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapMarkerTrackingPlaneAlignmentMode {
    /// DCB value: `GalacticPlane`
    GalacticPlane,
    /// DCB value: `Player`
    Player,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MapRadarMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapRadarMode {
    /// DCB value: `FullScreen`
    FullScreen,
    /// DCB value: `Landing`
    Landing,
    /// DCB value: `Small`
    Small,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MarkerClippingVolumeType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkerClippingVolumeType {
    /// DCB value: `Sphere`
    Sphere,
    /// DCB value: `Cuboid`
    Cuboid,
    /// DCB value: `Frustum`
    Frustum,
    /// DCB value: `Camera`
    Camera,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MarkerTrackingActions`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkerTrackingActions {
    /// DCB value: `None`
    None,
    /// DCB value: `SelectMarker`
    SelectMarker,
    /// DCB value: `FocusMarker`
    FocusMarker,
    /// DCB value: `LockTarget`
    LockTarget,
    /// DCB value: `PinTarget`
    PinTarget,
    /// DCB value: `HailContact`
    HailContact,
    /// DCB value: `QuantumTravel`
    QuantumTravel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_ARCullingCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_ARCullingCategory {
    /// DCB value: `Unassigned`
    Unassigned,
    /// DCB value: `Special_Hostile`
    Special_Hostile,
    /// DCB value: `UnattendedVehicle`
    UnattendedVehicle,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `Vehicle_Ground`
    Vehicle_Ground,
    /// DCB value: `Actor`
    Actor,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `Navpoint`
    Navpoint,
    /// DCB value: `Mineable`
    Mineable,
    /// DCB value: `Creature`
    Creature,
    /// DCB value: `Placeholder1`
    Placeholder1,
    /// DCB value: `Placeholder2`
    Placeholder2,
    /// DCB value: `Placeholder3`
    Placeholder3,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_DisplayMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_DisplayMode {
    /// DCB value: `FPS`
    FPS,
    /// DCB value: `VehicleSeat`
    VehicleSeat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_MapBoxoutSectionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_MapBoxoutSectionType {
    /// DCB value: `Amenities`
    Amenities,
    /// DCB value: `Galactapedia`
    Galactapedia,
    /// DCB value: `Jurisdiction`
    Jurisdiction,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_MapDisplayMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_MapDisplayMode {
    /// DCB value: `DefaultRadar`
    DefaultRadar,
    /// DCB value: `DefaultStarmap`
    DefaultStarmap,
    /// DCB value: `JumpTunnelRadar`
    JumpTunnelRadar,
    /// DCB value: `JumpTunnelStarmap`
    JumpTunnelStarmap,
    /// DCB value: `None`
    None,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_MapLabelDisplayType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_MapLabelDisplayType {
    /// DCB value: `Always`
    Always,
    /// DCB value: `OnInteracted`
    OnInteracted,
    /// DCB value: `OnParentSurface`
    OnParentSurface,
    /// DCB value: `OnSelected`
    OnSelected,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Marker_StackPositionAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Marker_StackPositionAlignment {
    /// DCB value: `Origin`
    Origin,
    /// DCB value: `Top`
    Top,
    /// DCB value: `Center`
    Center,
    /// DCB value: `Bottom`
    Bottom,
    /// DCB value: `TopAndBottom`
    TopAndBottom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MaterialProperties`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialProperties {
    /// DCB value: `Glow`
    Glow,
    /// DCB value: `Wear`
    Wear,
    /// DCB value: `Dirt`
    Dirt,
    /// DCB value: `Damage`
    Damage,
    /// DCB value: `Interference`
    Interference,
    /// DCB value: `Dissolve`
    Dissolve,
    /// DCB value: `Wetness`
    Wetness,
    /// DCB value: `UNDEFINED`
    UNDEFINED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MedBedTier`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MedBedTier {
    /// DCB value: `Hospital`
    Hospital,
    /// DCB value: `Clinic`
    Clinic,
    /// DCB value: `Ambulance`
    Ambulance,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MeleeAttackClass`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeleeAttackClass {
    /// DCB value: `Basic`
    Basic,
    /// DCB value: `Synced`
    Synced,
    /// DCB value: `Any`
    Any,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MeleeTargetType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeleeTargetType {
    /// DCB value: `Any`
    Any,
    /// DCB value: `Ground`
    Ground,
    /// DCB value: `Aerial`
    Aerial,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MessageState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageState {
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Moderate`
    Moderate,
    /// DCB value: `Critical`
    Critical,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MinimumInfluenceFactorOperation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MinimumInfluenceFactorOperation {
    /// DCB value: `GreaterThan`
    GreaterThan,
    /// DCB value: `TimesGreater`
    TimesGreater,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MissionLocationTagType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MissionLocationTagType {
    /// DCB value: `General`
    General,
    /// DCB value: `Produces`
    Produces,
    /// DCB value: `Consumes`
    Consumes,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `Month`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Month {
    /// DCB value: `January`
    January,
    /// DCB value: `February`
    February,
    /// DCB value: `March`
    March,
    /// DCB value: `April`
    April,
    /// DCB value: `May`
    May,
    /// DCB value: `June`
    June,
    /// DCB value: `July`
    July,
    /// DCB value: `August`
    August,
    /// DCB value: `September`
    September,
    /// DCB value: `October`
    October,
    /// DCB value: `November`
    November,
    /// DCB value: `December`
    December,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MotionControlType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotionControlType {
    /// DCB value: `Animation`
    Animation,
    /// DCB value: `Entity`
    Entity,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MotionStateType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotionStateType {
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Move`
    Move,
    /// DCB value: `IdleToMove`
    IdleToMove,
    /// DCB value: `MoveToIdle`
    MoveToIdle,
    /// DCB value: `Step`
    Step,
    /// DCB value: `Turn`
    Turn,
    /// DCB value: `Juke`
    Juke,
    /// DCB value: `InAirIdle`
    InAirIdle,
    /// DCB value: `InAirMove`
    InAirMove,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MovementSet`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementSet {
    /// DCB value: `Inactive`
    Inactive,
    /// DCB value: `BodyCarry`
    BodyCarry,
    /// DCB value: `Drunk`
    Drunk,
    /// DCB value: `Effort`
    Effort,
    /// DCB value: `Hurt`
    Hurt,
    /// DCB value: `Movable`
    Movable,
    /// DCB value: `Standard`
    Standard,
    /// DCB value: `Stumble`
    Stumble,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MovementSetCondition`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementSetCondition {
    /// DCB value: `StateMovable`
    StateMovable,
    /// DCB value: `StateBodyCarry`
    StateBodyCarry,
    /// DCB value: `StateBodyDragging`
    StateBodyDragging,
    /// DCB value: `StateSubmerged`
    StateSubmerged,
    /// DCB value: `ForceBrace`
    ForceBrace,
    /// DCB value: `ForceLean`
    ForceLean,
    /// DCB value: `ForceStumble`
    ForceStumble,
    /// DCB value: `StatusHurt`
    StatusHurt,
    /// DCB value: `StatusDrunk`
    StatusDrunk,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `MovementSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementSpeed {
    /// DCB value: `WalkSlow`
    WalkSlow,
    /// DCB value: `WalkMid`
    WalkMid,
    /// DCB value: `WalkFast`
    WalkFast,
    /// DCB value: `RunSlow`
    RunSlow,
    /// DCB value: `RunFast`
    RunFast,
    /// DCB value: `Sprint`
    Sprint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `NaturalMotionSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NaturalMotionSpeed {
    /// DCB value: `WalkSlow`
    WalkSlow,
    /// DCB value: `WalkMid`
    WalkMid,
    /// DCB value: `WalkFast`
    WalkFast,
    /// DCB value: `RunSlow`
    RunSlow,
    /// DCB value: `RunFast`
    RunFast,
    /// DCB value: `Sprint`
    Sprint,
    /// DCB value: `GreenZoneWalk`
    GreenZoneWalk,
    /// DCB value: `GreenZoneSprint`
    GreenZoneSprint,
    /// DCB value: `AimDownSight`
    AimDownSight,
    /// DCB value: `Conversation`
    Conversation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `NavPointIconEnum`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavPointIconEnum {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Star`
    Star,
    /// DCB value: `Planet`
    Planet,
    /// DCB value: `Moon`
    Moon,
    /// DCB value: `Station`
    Station,
    /// DCB value: `Outpost`
    Outpost,
    /// DCB value: `LandingZone`
    LandingZone,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ParticleAttachToZone`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticleAttachToZone {
    /// DCB value: `Parent`
    Parent,
    /// DCB value: `AboveParent`
    AboveParent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ParticleCPUVisAreaCullingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticleCPUVisAreaCullingMode {
    /// DCB value: `None`
    None,
    /// DCB value: `Local`
    Local,
    /// DCB value: `Dynamic`
    Dynamic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ParticleGPUVisAreaCullingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticleGPUVisAreaCullingMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `None`
    None,
    /// DCB value: `PerParticle`
    PerParticle,
    /// DCB value: `PerPixel`
    PerPixel,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ParticleTesselationOverride`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticleTesselationOverride {
    /// DCB value: `Default`
    Default,
    /// DCB value: `Off`
    Off,
    /// DCB value: `Low`
    Low,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `High`
    High,
    /// DCB value: `Ultra`
    Ultra,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PersistentItemGameModeFlag`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersistentItemGameModeFlag {
    /// DCB value: `Any`
    Any,
    /// DCB value: `StarMarine`
    StarMarine,
    /// DCB value: `ArenaCommander`
    ArenaCommander,
    /// DCB value: `PersistentUniverse`
    PersistentUniverse,
    /// DCB value: `SubscriberExclusive`
    SubscriberExclusive,
    /// DCB value: `Prison`
    Prison,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PersonalThoughtCameraEffets`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonalThoughtCameraEffets {
    /// DCB value: `None`
    None,
    /// DCB value: `InnerThoughts`
    InnerThoughts,
    /// DCB value: `Inventory`
    Inventory,
    /// DCB value: `ExternalInventory`
    ExternalInventory,
    /// DCB value: `Looting`
    Looting,
    /// DCB value: `InventoryV4`
    InventoryV4,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PersonalThoughtContext`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PersonalThoughtContext {
    /// DCB value: `EVA`
    EVA,
    /// DCB value: `Flight`
    Flight,
    /// DCB value: `GroundVehicle`
    GroundVehicle,
    /// DCB value: `OnFoot`
    OnFoot,
    /// DCB value: `Seated`
    Seated,
    /// DCB value: `Turret`
    Turret,
    /// DCB value: `Usable`
    Usable,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PickableCollision`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PickableCollision {
    /// DCB value: `None`
    None,
    /// DCB value: `Transparent`
    Transparent,
    /// DCB value: `Cutout`
    Cutout,
    /// DCB value: `Opaque`
    Opaque,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PlanetAreaFitting`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetAreaFitting {
    /// DCB value: `Loose`
    Loose,
    /// DCB value: `Tight`
    Tight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PlanetAreaType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetAreaType {
    /// DCB value: `Hole`
    Hole,
    /// DCB value: `ExcludeLarge`
    ExcludeLarge,
    /// DCB value: `ExcludeAll`
    ExcludeAll,
    /// DCB value: `ExcludeFaunaOnly`
    ExcludeFaunaOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PostEffectParams`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PostEffectParams {
    /// DCB value: `Global_DirectionalBlur_Vec`
    Global_DirectionalBlur_Vec,
    /// DCB value: `Blink_EyeClosure`
    Blink_EyeClosure,
    /// DCB value: `ColorGrading_Brightness`
    ColorGrading_Brightness,
    /// DCB value: `ColorGrading_Contrast`
    ColorGrading_Contrast,
    /// DCB value: `ColorGrading_Saturation`
    ColorGrading_Saturation,
    /// DCB value: `Dof_Active`
    Dof_Active,
    /// DCB value: `Dof_FocusDistance`
    Dof_FocusDistance,
    /// DCB value: `Dof_FocusRange`
    Dof_FocusRange,
    /// DCB value: `Dof_FocusMin`
    Dof_FocusMin,
    /// DCB value: `Dof_FocusMax`
    Dof_FocusMax,
    /// DCB value: `Dof_MaxCoC`
    Dof_MaxCoC,
    /// DCB value: `Dof_BlurAmount`
    Dof_BlurAmount,
    /// DCB value: `Dof_User_Active`
    Dof_User_Active,
    /// DCB value: `Dof_User_FocusDistance`
    Dof_User_FocusDistance,
    /// DCB value: `Dof_User_FocusRange`
    Dof_User_FocusRange,
    /// DCB value: `Dof_User_BlurAmount`
    Dof_User_BlurAmount,
    /// DCB value: `Dof_FocusMinZ`
    Dof_FocusMinZ,
    /// DCB value: `Dof_FocusMinZScale`
    Dof_FocusMinZScale,
    /// DCB value: `FilterChromaShift_User_Amount`
    FilterChromaShift_User_Amount,
    /// DCB value: `FilterArtifacts_ChromaShift`
    FilterArtifacts_ChromaShift,
    /// DCB value: `FilterGrain_Amount`
    FilterGrain_Amount,
    /// DCB value: `FilterArtifacts_Grain`
    FilterArtifacts_Grain,
    /// DCB value: `FilterArtifacts_GrainTile`
    FilterArtifacts_GrainTile,
    /// DCB value: `FilterBlurring_Amount`
    FilterBlurring_Amount,
    /// DCB value: `FilterRadialBlurring_Amount`
    FilterRadialBlurring_Amount,
    /// DCB value: `FilterRadialBlurring_ScreenPosX`
    FilterRadialBlurring_ScreenPosX,
    /// DCB value: `FilterRadialBlurring_ScreenPosY`
    FilterRadialBlurring_ScreenPosY,
    /// DCB value: `FilterRadialBlurring_Radius`
    FilterRadialBlurring_Radius,
    /// DCB value: `FlashBang_StartTime`
    FlashBang_StartTime,
    /// DCB value: `FlashBang_DifractionAmount`
    FlashBang_DifractionAmount,
    /// DCB value: `FlashBang_Time`
    FlashBang_Time,
    /// DCB value: `FlashBang_BlindAmount`
    FlashBang_BlindAmount,
    /// DCB value: `GForce_BlackoutValue`
    GForce_BlackoutValue,
    /// DCB value: `GForce_RedoutValue`
    GForce_RedoutValue,
    /// DCB value: `GForce_LatStressValue`
    GForce_LatStressValue,
    /// DCB value: `GForce_BlackoutRecovery`
    GForce_BlackoutRecovery,
    /// DCB value: `GForce_LatStressRecovery`
    GForce_LatStressRecovery,
    /// DCB value: `GForce_PulseAmplitude`
    GForce_PulseAmplitude,
    /// DCB value: `GForce_PulseMaskAmplitude`
    GForce_PulseMaskAmplitude,
    /// DCB value: `GForce_PulsePeriod`
    GForce_PulsePeriod,
    /// DCB value: `GForce_PulseDuration`
    GForce_PulseDuration,
    /// DCB value: `GForce_TunnelRadiusGrey`
    GForce_TunnelRadiusGrey,
    /// DCB value: `GForce_TunnelRadiusBlack`
    GForce_TunnelRadiusBlack,
    /// DCB value: `GForce_TunnelStrengthGrey`
    GForce_TunnelStrengthGrey,
    /// DCB value: `GForce_TunnelStrengthBlack`
    GForce_TunnelStrengthBlack,
    /// DCB value: `GForce_SaturationGrey`
    GForce_SaturationGrey,
    /// DCB value: `GForce_SaturationBlack`
    GForce_SaturationBlack,
    /// DCB value: `HudSilhouettes_Active`
    HudSilhouettes_Active,
    /// DCB value: `HudSilhouettes_Amount`
    HudSilhouettes_Amount,
    /// DCB value: `HudSilhouettes_FillStr`
    HudSilhouettes_FillStr,
    /// DCB value: `HudSilhouettes_EdgeWidth`
    HudSilhouettes_EdgeWidth,
    /// DCB value: `HudSilhouettes_BlurRadius`
    HudSilhouettes_BlurRadius,
    /// DCB value: `HudSilhouettes_Type`
    HudSilhouettes_Type,
    /// DCB value: `OcularMigraine_StrengthValue`
    OcularMigraine_StrengthValue,
    /// DCB value: `OcularMigraine_BlindspotSize`
    OcularMigraine_BlindspotSize,
    /// DCB value: `OcularMigraine_SpectralOpacity`
    OcularMigraine_SpectralOpacity,
    /// DCB value: `OcularMigraine_BlindspotOpacity`
    OcularMigraine_BlindspotOpacity,
    /// DCB value: `BloodVision_StrengthValue`
    BloodVision_StrengthValue,
    /// DCB value: `BloodVision_BloodAuraStrength`
    BloodVision_BloodAuraStrength,
    /// DCB value: `BloodVision_AngularVelocityScaleX`
    BloodVision_AngularVelocityScaleX,
    /// DCB value: `BloodVision_AngularVelocityScaleY`
    BloodVision_AngularVelocityScaleY,
    /// DCB value: `SunShafts_Active`
    SunShafts_Active,
    /// DCB value: `SunShafts_RaysAmount`
    SunShafts_RaysAmount,
    /// DCB value: `SunShafts_RaysAttenuation`
    SunShafts_RaysAttenuation,
    /// DCB value: `SunShafts_RaysCustomColor`
    SunShafts_RaysCustomColor,
    /// DCB value: `tex_VisualArtifacts_Mask`
    tex_VisualArtifacts_Mask,
    /// DCB value: `clr_VisualArtifacts_ColotTint`
    clr_VisualArtifacts_ColotTint,
    /// DCB value: `VisualArtifacts_Vsync`
    VisualArtifacts_Vsync,
    /// DCB value: `VisualArtifacts_VsyncFreq`
    VisualArtifacts_VsyncFreq,
    /// DCB value: `VisualArtifacts_Interlacing`
    VisualArtifacts_Interlacing,
    /// DCB value: `VisualArtifacts_InterlacingTile`
    VisualArtifacts_InterlacingTile,
    /// DCB value: `VisualArtifacts_InterlacingRot`
    VisualArtifacts_InterlacingRot,
    /// DCB value: `VisualArtifacts_Noise`
    VisualArtifacts_Noise,
    /// DCB value: `VisualArtifacts_SyncWaveFreq`
    VisualArtifacts_SyncWaveFreq,
    /// DCB value: `VisualArtifacts_SyncWavePhase`
    VisualArtifacts_SyncWavePhase,
    /// DCB value: `VisualArtifacts_SyncWaveAmplitude`
    VisualArtifacts_SyncWaveAmplitude,
    /// DCB value: `WaterDroplets_Amount`
    WaterDroplets_Amount,
    /// DCB value: `ImageGhosting_Amount`
    ImageGhosting_Amount,
    /// DCB value: `Letterboxing_WidthAspectRatio`
    Letterboxing_WidthAspectRatio,
    /// DCB value: `Letterboxing_HeightAspectRatio`
    Letterboxing_HeightAspectRatio,
    /// DCB value: `Letterboxing_Progress`
    Letterboxing_Progress,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PostureType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PostureType {
    /// DCB value: `Invalid`
    Invalid,
    /// DCB value: `Peek`
    Peek,
    /// DCB value: `Aim`
    Aim,
    /// DCB value: `BlindFire`
    BlindFire,
    /// DCB value: `Throw`
    Throw,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralAnimationBoneName`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralAnimationBoneName {
    /// DCB value: `Hips`
    Hips,
    /// DCB value: `Spine`
    Spine,
    /// DCB value: `Spine1`
    Spine1,
    /// DCB value: `Spine2`
    Spine2,
    /// DCB value: `Spine3`
    Spine3,
    /// DCB value: `Neck`
    Neck,
    /// DCB value: `Neck1`
    Neck1,
    /// DCB value: `Head`
    Head,
    /// DCB value: `HelmetCam`
    HelmetCam,
    /// DCB value: `Head_LowPass`
    Head_LowPass,
    /// DCB value: `LeftShoulder`
    LeftShoulder,
    /// DCB value: `RightShoulder`
    RightShoulder,
    /// DCB value: `LeftArm`
    LeftArm,
    /// DCB value: `RightArm`
    RightArm,
    /// DCB value: `LeftForeArm`
    LeftForeArm,
    /// DCB value: `RightForeArm`
    RightForeArm,
    /// DCB value: `LeftHand`
    LeftHand,
    /// DCB value: `RightHand`
    RightHand,
    /// DCB value: `LeftFoot`
    LeftFoot,
    /// DCB value: `RightFoot`
    RightFoot,
    /// DCB value: `LeftLeg`
    LeftLeg,
    /// DCB value: `RightLeg`
    RightLeg,
    /// DCB value: `LeftUpLeg`
    LeftUpLeg,
    /// DCB value: `RightUpLeg`
    RightUpLeg,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralAnimationBoneOperation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralAnimationBoneOperation {
    /// DCB value: `Offset`
    Offset,
    /// DCB value: `Rotation`
    Rotation,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralAnimationBoneSpace`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralAnimationBoneSpace {
    /// DCB value: `ParentBone`
    ParentBone,
    /// DCB value: `Character`
    Character,
    /// DCB value: `TPose`
    TPose,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralLandingStrengthFilter`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralLandingStrengthFilter {
    /// DCB value: `Light`
    Light,
    /// DCB value: `Medium`
    Medium,
    /// DCB value: `Heavy`
    Heavy,
    /// DCB value: `Impact`
    Impact,
    /// DCB value: `Any`
    Any,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralLayout_TagFilteringMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralLayout_TagFilteringMode {
    /// DCB value: `Inclusion`
    Inclusion,
    /// DCB value: `Exclusion`
    Exclusion,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralLayout_VerticalDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralLayout_VerticalDirection {
    /// DCB value: `Downwards`
    Downwards,
    /// DCB value: `Upwards`
    Upwards,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ProceduralPOILookAtType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProceduralPOILookAtType {
    /// DCB value: `Origin`
    Origin,
    /// DCB value: `EntityBB`
    EntityBB,
    /// DCB value: `CustomBBs`
    CustomBBs,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `PurchasableVehicleUsageType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchasableVehicleUsageType {
    /// DCB value: `Purchase`
    Purchase,
    /// DCB value: `Rent`
    Rent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `QDriveSplineRotationBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QDriveSplineRotationBehavior {
    /// DCB value: `NoRollback`
    NoRollback,
    /// DCB value: `Rollback`
    Rollback,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `QuantumDriveState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantumDriveState {
    /// DCB value: `Off`
    Off,
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `Aligning`
    Aligning,
    /// DCB value: `Pre_Ramp_Up`
    Pre_Ramp_Up,
    /// DCB value: `Ramp_Up`
    Ramp_Up,
    /// DCB value: `Flight_In_Progress`
    Flight_In_Progress,
    /// DCB value: `Ramp_Down`
    Ramp_Down,
    /// DCB value: `Post_Ramp_Down`
    Post_Ramp_Down,
    /// DCB value: `End_Travel`
    End_Travel,
    /// DCB value: `Abort`
    Abort,
    /// DCB value: `Cooldown`
    Cooldown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `QuantumState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuantumState {
    /// DCB value: `Off`
    Off,
    /// DCB value: `WaitingForInputDelay`
    WaitingForInputDelay,
    /// DCB value: `TargetLocking`
    TargetLocking,
    /// DCB value: `Charging`
    Charging,
    /// DCB value: `BoostingSlow`
    BoostingSlow,
    /// DCB value: `Travelling`
    Travelling,
    /// DCB value: `CancelingTravel`
    CancelingTravel,
    /// DCB value: `CancelingBoost`
    CancelingBoost,
    /// DCB value: `Cooldown`
    Cooldown,
    /// DCB value: `Done`
    Done,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RadarPriorityComparison`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RadarPriorityComparison {
    /// DCB value: `LessThan`
    LessThan,
    /// DCB value: `LessThanEqual`
    LessThanEqual,
    /// DCB value: `Equals`
    Equals,
    /// DCB value: `GreaterThanEqual`
    GreaterThanEqual,
    /// DCB value: `GreaterThan`
    GreaterThan,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RadiationStatePropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RadiationStatePropertyType {
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `IR`
    IR,
    /// DCB value: `EM`
    EM,
    /// DCB value: `CS`
    CS,
    /// DCB value: `RadiationHazard`
    RadiationHazard,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ReactionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionType {
    /// DCB value: `Hostile`
    Hostile,
    /// DCB value: `Neutral`
    Neutral,
    /// DCB value: `Friendly`
    Friendly,
    /// DCB value: `Unknown`
    Unknown,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RefiningQuality`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefiningQuality {
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Careful`
    Careful,
    /// DCB value: `Wasteful`
    Wasteful,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RefiningSpeed`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefiningSpeed {
    /// DCB value: `Normal`
    Normal,
    /// DCB value: `Slow`
    Slow,
    /// DCB value: `Fast`
    Fast,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RelativeDirection`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelativeDirection {
    /// DCB value: `Up`
    Up,
    /// DCB value: `Down`
    Down,
    /// DCB value: `Front`
    Front,
    /// DCB value: `Back`
    Back,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Left`
    Left,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RenderToTextureTarget`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RenderToTextureTarget {
    /// DCB value: `All`
    All,
    /// DCB value: `Primary`
    Primary,
    /// DCB value: `Secondary`
    Secondary,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ResourceNetworkAcessParameter`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceNetworkAcessParameter {
    /// DCB value: `Pressure`
    Pressure,
    /// DCB value: `Temperature`
    Temperature,
    /// DCB value: `CarbonDioxide`
    CarbonDioxide,
    /// DCB value: `Oxygen`
    Oxygen,
    /// DCB value: `Consumption`
    Consumption,
    /// DCB value: `Generation`
    Generation,
    /// DCB value: `FuncionalityRatio`
    FuncionalityRatio,
    /// DCB value: `Preference`
    Preference,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ResourceNetworkResource`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceNetworkResource {
    /// DCB value: `Power`
    Power,
    /// DCB value: `Fuel`
    Fuel,
    /// DCB value: `Coolant`
    Coolant,
    /// DCB value: `Shield`
    Shield,
    /// DCB value: `Gravity`
    Gravity,
    /// DCB value: `QuantumFuel`
    QuantumFuel,
    /// DCB value: `CPU`
    CPU,
    /// DCB value: `Gas`
    Gas,
    /// DCB value: `Filter`
    Filter,
    /// DCB value: `LifeSupport`
    LifeSupport,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RestraintStyle`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestraintStyle {
    /// DCB value: `CuffArmLeg`
    CuffArmLeg,
    /// DCB value: `CuffArmOnly`
    CuffArmOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RestrictedAreaDirectionalMessage`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestrictedAreaDirectionalMessage {
    /// DCB value: `Left`
    Left,
    /// DCB value: `Right`
    Right,
    /// DCB value: `Above`
    Above,
    /// DCB value: `Below`
    Below,
    /// DCB value: `Ahead`
    Ahead,
    /// DCB value: `Behind`
    Behind,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RestrictedAreaState`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestrictedAreaState {
    /// DCB value: `Allow`
    Allow,
    /// DCB value: `Disallow`
    Disallow,
    /// DCB value: `Despawn`
    Despawn,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RoomConnectorOrientationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomConnectorOrientationMode {
    /// DCB value: `AutoDetectWidest`
    AutoDetectWidest,
    /// DCB value: `ForwardBackward`
    ForwardBackward,
    /// DCB value: `UpDown`
    UpDown,
    /// DCB value: `RightLeft`
    RightLeft,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RoomStateModifyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomStateModifyType {
    /// DCB value: `Inherit`
    Inherit,
    /// DCB value: `Override`
    Override,
    /// DCB value: `Additive`
    Additive,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RoomStatePropertyType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomStatePropertyType {
    /// DCB value: `DebrisDensity`
    DebrisDensity,
    /// DCB value: `Pressure`
    Pressure,
    /// DCB value: `Temperature`
    Temperature,
    /// DCB value: `Humidity`
    Humidity,
    /// DCB value: `Charge`
    Charge,
    /// DCB value: `Distortion`
    Distortion,
    /// DCB value: `IR`
    IR,
    /// DCB value: `EM`
    EM,
    /// DCB value: `CS`
    CS,
    /// DCB value: `RadiationHazard`
    RadiationHazard,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RoomType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoomType {
    /// DCB value: `Area`
    Area,
    /// DCB value: `Planet`
    Planet,
    /// DCB value: `GasCloud`
    GasCloud,
    /// DCB value: `AsteroidField`
    AsteroidField,
    /// DCB value: `Helmet`
    Helmet,
    /// DCB value: `NavPoint`
    NavPoint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `RttOutputType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RttOutputType {
    /// DCB value: `Default`
    Default,
    /// DCB value: `SurfaceWater`
    SurfaceWater,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SCSeatActorAttachmentType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SCSeatActorAttachmentType {
    /// DCB value: `Host`
    Host,
    /// DCB value: `Seat`
    Seat,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SeatSkipStates`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeatSkipStates {
    /// DCB value: `None`
    None,
    /// DCB value: `SkipDeploy`
    SkipDeploy,
    /// DCB value: `SkipRetract`
    SkipRetract,
    /// DCB value: `SkipBoth`
    SkipBoth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SeatTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeatTypes {
    /// DCB value: `HOTAS_R_L`
    HOTAS_R_L,
    /// DCB value: `HOTAS_C_L`
    HOTAS_C_L,
    /// DCB value: `DUAL_STICK`
    DUAL_STICK,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ShadowQuality`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadowQuality {
    /// DCB value: `Full`
    Full,
    /// DCB value: `Half`
    Half,
    /// DCB value: `Quarter`
    Quarter,
    /// DCB value: `OneEighth`
    OneEighth,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ShockwaveType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShockwaveType {
    /// DCB value: `Cylinder`
    Cylinder,
    /// DCB value: `Sphere`
    Sphere,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ShopInventoryType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShopInventoryType {
    /// DCB value: `INVALID`
    INVALID,
    /// DCB value: `ITEM`
    ITEM,
    /// DCB value: `COMMODITY`
    COMMODITY,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ShoppingKioskVariant`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShoppingKioskVariant {
    /// DCB value: `LOW`
    LOW,
    /// DCB value: `MED`
    MED,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SinglePlayerOrMultiplayer`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SinglePlayerOrMultiplayer {
    /// DCB value: `Both`
    Both,
    /// DCB value: `SinglePlayerOnly`
    SinglePlayerOnly,
    /// DCB value: `MultiplayerOnly`
    MultiplayerOnly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SkeletonAnimationTaskTransitionType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkeletonAnimationTaskTransitionType {
    /// DCB value: `Linear`
    Linear,
    /// DCB value: `Cubic`
    Cubic,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SpawnWithMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpawnWithMode {
    /// DCB value: `MostSimilar`
    MostSimilar,
    /// DCB value: `Random`
    Random,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SpeedThrottleActiveMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeedThrottleActiveMode {
    /// DCB value: `Always`
    Always,
    /// DCB value: `NoWeapon`
    NoWeapon,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SpeedThrottleNoWeaponSpeedCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeedThrottleNoWeaponSpeedCategory {
    /// DCB value: `SlowWalk`
    SlowWalk,
    /// DCB value: `MidWalk`
    MidWalk,
    /// DCB value: `FastWalk`
    FastWalk,
    /// DCB value: `SlowRun`
    SlowRun,
    /// DCB value: `FastRun`
    FastRun,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SpeedThrottleWithWeaponSpeedCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeedThrottleWithWeaponSpeedCategory {
    /// DCB value: `SameAsNoWeapon`
    SameAsNoWeapon,
    /// DCB value: `SlowWalk`
    SlowWalk,
    /// DCB value: `MidWalk`
    MidWalk,
    /// DCB value: `FastWalk`
    FastWalk,
    /// DCB value: `SlowRun`
    SlowRun,
    /// DCB value: `FastRun`
    FastRun,
    /// DCB value: `AimDownSight`
    AimDownSight,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StaminaActionCategory`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StaminaActionCategory {
    /// DCB value: `MeleeJab`
    MeleeJab,
    /// DCB value: `MeleeHook`
    MeleeHook,
    /// DCB value: `MeleeOverhand`
    MeleeOverhand,
    /// DCB value: `MeleeArmed`
    MeleeArmed,
    /// DCB value: `MeleeUpperCut`
    MeleeUpperCut,
    /// DCB value: `MeleeBolo`
    MeleeBolo,
    /// DCB value: `MeleeHaymaker`
    MeleeHaymaker,
    /// DCB value: `BladeSlash`
    BladeSlash,
    /// DCB value: `BladeStab`
    BladeStab,
    /// DCB value: `BladeLightOver`
    BladeLightOver,
    /// DCB value: `BladeLightUnder`
    BladeLightUnder,
    /// DCB value: `BladeHeavyOver`
    BladeHeavyOver,
    /// DCB value: `BladeHeavyUnder`
    BladeHeavyUnder,
    /// DCB value: `LanceSwipeLeft`
    LanceSwipeLeft,
    /// DCB value: `LanceSwipeRight`
    LanceSwipeRight,
    /// DCB value: `LanceStab`
    LanceStab,
    /// DCB value: `LanceLeap`
    LanceLeap,
    /// DCB value: `LanceHammerDown`
    LanceHammerDown,
    /// DCB value: `LancePushBack`
    LancePushBack,
    /// DCB value: `LanceShoot`
    LanceShoot,
    /// DCB value: `TestAttack`
    TestAttack,
    /// DCB value: `Blocking`
    Blocking,
    /// DCB value: `SyringeStab`
    SyringeStab,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StarmapBoolOverride`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarmapBoolOverride {
    /// DCB value: `False`
    False,
    /// DCB value: `True`
    True,
    /// DCB value: `NoOverride`
    NoOverride,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StatBuffType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatBuffType {
    /// DCB value: `Hypertrophic`
    Hypertrophic,
    /// DCB value: `Atrophic`
    Atrophic,
    /// DCB value: `Fatiguing`
    Fatiguing,
    /// DCB value: `Energizing`
    Energizing,
    /// DCB value: `CognitiveBoost`
    CognitiveBoost,
    /// DCB value: `CognitiveImpair`
    CognitiveImpair,
    /// DCB value: `HypoMetabolic`
    HypoMetabolic,
    /// DCB value: `HyperMetabolic`
    HyperMetabolic,
    /// DCB value: `Hydrating`
    Hydrating,
    /// DCB value: `Dehydrating`
    Dehydrating,
    /// DCB value: `Healing`
    Healing,
    /// DCB value: `Toxic`
    Toxic,
    /// DCB value: `ImmuneBoost`
    ImmuneBoost,
    /// DCB value: `ImmuneSuppress`
    ImmuneSuppress,
    /// DCB value: `WeaponChargeMoveSpeed`
    WeaponChargeMoveSpeed,
    /// DCB value: `GForcePassOut`
    GForcePassOut,
    /// DCB value: `OverdoseRevival`
    OverdoseRevival,
    /// DCB value: `OverdoseRevivalBDLDecay`
    OverdoseRevivalBDLDecay,
    /// DCB value: `ReviveDamageMultiplier`
    ReviveDamageMultiplier,
    /// DCB value: `DownedDamageMultiplier`
    DownedDamageMultiplier,
    /// DCB value: `DrugDurationMultiplier`
    DrugDurationMultiplier,
    /// DCB value: `HealthPoolMask`
    HealthPoolMask,
    /// DCB value: `HurtLocomotionMask`
    HurtLocomotionMask,
    /// DCB value: `StunRecoveryMask`
    StunRecoveryMask,
    /// DCB value: `ImpactResistanceKnockdownMask`
    ImpactResistanceKnockdownMask,
    /// DCB value: `ImpactResistanceStaggerMask`
    ImpactResistanceStaggerMask,
    /// DCB value: `ImpactResistanceTwitchMask`
    ImpactResistanceTwitchMask,
    /// DCB value: `ImpactResistanceFlinchMask`
    ImpactResistanceFlinchMask,
    /// DCB value: `StaminaRegenMask`
    StaminaRegenMask,
    /// DCB value: `StaminaPoolMask`
    StaminaPoolMask,
    /// DCB value: `WheezingAudioMask`
    WheezingAudioMask,
    /// DCB value: `CoughBloodMask`
    CoughBloodMask,
    /// DCB value: `MoveSpeedMask`
    MoveSpeedMask,
    /// DCB value: `TraversalLockMask`
    TraversalLockMask,
    /// DCB value: `TraversalLockProneMask`
    TraversalLockProneMask,
    /// DCB value: `PainGruntMask`
    PainGruntMask,
    /// DCB value: `ArmsLockMask`
    ArmsLockMask,
    /// DCB value: `WeaponSwayMask`
    WeaponSwayMask,
    /// DCB value: `ADSEnterMask`
    ADSEnterMask,
    /// DCB value: `BloodVisionMask`
    BloodVisionMask,
    /// DCB value: `MuffledAudioInjuryMask`
    MuffledAudioInjuryMask,
    /// DCB value: `BlurredVisionMask`
    BlurredVisionMask,
    /// DCB value: `DrunkLocomotionMask`
    DrunkLocomotionMask,
    /// DCB value: `DrunkManoeuvringMask`
    DrunkManoeuvringMask,
    /// DCB value: `DoubleVisionMask`
    DoubleVisionMask,
    /// DCB value: `OrificeBloodMask`
    OrificeBloodMask,
    /// DCB value: `FlashEffect`
    FlashEffect,
    /// DCB value: `Slam`
    Slam,
    /// DCB value: `RadiationAntidote`
    RadiationAntidote,
    /// DCB value: `ThrowForceMask`
    ThrowForceMask,
    /// DCB value: `MeleeForceMask`
    MeleeForceMask,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StateTypeNetworkAuthority`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateTypeNetworkAuthority {
    /// DCB value: `Server`
    Server,
    /// DCB value: `Local`
    Local,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StatusEffectType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectType {
    /// DCB value: `ADSEnter`
    ADSEnter,
    /// DCB value: `ArmsLock`
    ArmsLock,
    /// DCB value: `Bleed`
    Bleed,
    /// DCB value: `BloodDrugLevelDecay`
    BloodDrugLevelDecay,
    /// DCB value: `BloodVision`
    BloodVision,
    /// DCB value: `BlurredVision`
    BlurredVision,
    /// DCB value: `BodyRadiationDecay`
    BodyRadiationDecay,
    /// DCB value: `CoughBlood`
    CoughBlood,
    /// DCB value: `Dead`
    Dead,
    /// DCB value: `DehydrationDamage`
    DehydrationDamage,
    /// DCB value: `DepressurizationDamage`
    DepressurizationDamage,
    /// DCB value: `DrugDuration`
    DrugDuration,
    /// DCB value: `DrunkLocomotion`
    DrunkLocomotion,
    /// DCB value: `DrunkManoeuvring`
    DrunkManoeuvring,
    /// DCB value: `DoubleVision`
    DoubleVision,
    /// DCB value: `DownedDamageDecay`
    DownedDamageDecay,
    /// DCB value: `EarRinging`
    EarRinging,
    /// DCB value: `ExternalDamageMultiplier`
    ExternalDamageMultiplier,
    /// DCB value: `FlashEffect`
    FlashEffect,
    /// DCB value: `HeadacheAudio`
    HeadacheAudio,
    /// DCB value: `HealthPool`
    HealthPool,
    /// DCB value: `HealthBoost`
    HealthBoost,
    /// DCB value: `HungerDecay`
    HungerDecay,
    /// DCB value: `HurtLocomotion`
    HurtLocomotion,
    /// DCB value: `HurtProne`
    HurtProne,
    /// DCB value: `HygieneDecay`
    HygieneDecay,
    /// DCB value: `HypothermiaDamage`
    HypothermiaDamage,
    /// DCB value: `HyperthermiaDamage`
    HyperthermiaDamage,
    /// DCB value: `ImpactResistanceKnockdown`
    ImpactResistanceKnockdown,
    /// DCB value: `ImpactResistanceStagger`
    ImpactResistanceStagger,
    /// DCB value: `ImpactResistanceTwitch`
    ImpactResistanceTwitch,
    /// DCB value: `ImpactResistanceFlinch`
    ImpactResistanceFlinch,
    /// DCB value: `LightSensitivity`
    LightSensitivity,
    /// DCB value: `MacularDegeneration`
    MacularDegeneration,
    /// DCB value: `MalfunctionDistortion`
    MalfunctionDistortion,
    /// DCB value: `MeleeDamage`
    MeleeDamage,
    /// DCB value: `MeleeForce`
    MeleeForce,
    /// DCB value: `MoveSpeed`
    MoveSpeed,
    /// DCB value: `MuffledAudio`
    MuffledAudio,
    /// DCB value: `Blink`
    Blink,
    /// DCB value: `OcularMigraine`
    OcularMigraine,
    /// DCB value: `OrificeBlood`
    OrificeBlood,
    /// DCB value: `OverdoseDamage`
    OverdoseDamage,
    /// DCB value: `PainGrunt`
    PainGrunt,
    /// DCB value: `PassOutDowned`
    PassOutDowned,
    /// DCB value: `PassOutUnconscious`
    PassOutUnconscious,
    /// DCB value: `PassOutGForce`
    PassOutGForce,
    /// DCB value: `RadiationDamageLow`
    RadiationDamageLow,
    /// DCB value: `RadiationDamageHigh`
    RadiationDamageHigh,
    /// DCB value: `Recoil`
    Recoil,
    /// DCB value: `Shivering`
    Shivering,
    /// DCB value: `StaminaCost`
    StaminaCost,
    /// DCB value: `StaminaPool`
    StaminaPool,
    /// DCB value: `StaminaRegen`
    StaminaRegen,
    /// DCB value: `StarvationDamage`
    StarvationDamage,
    /// DCB value: `StatusDamageCap`
    StatusDamageCap,
    /// DCB value: `StatusDamageMultiplier`
    StatusDamageMultiplier,
    /// DCB value: `StomachGroanAudio`
    StomachGroanAudio,
    /// DCB value: `StunDecay`
    StunDecay,
    /// DCB value: `Suffocation`
    Suffocation,
    /// DCB value: `SuffocationDamage`
    SuffocationDamage,
    /// DCB value: `TempAudioLoss`
    TempAudioLoss,
    /// DCB value: `ThirstDecay`
    ThirstDecay,
    /// DCB value: `ThrowForce`
    ThrowForce,
    /// DCB value: `TraversalLock`
    TraversalLock,
    /// DCB value: `TraversalLockProne`
    TraversalLockProne,
    /// DCB value: `Wheezing`
    Wheezing,
    /// DCB value: `WeaponSway`
    WeaponSway,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StatusEffectValueType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectValueType {
    /// DCB value: `Additive`
    Additive,
    /// DCB value: `Multiplier`
    Multiplier,
    /// DCB value: `MaxValue`
    MaxValue,
    /// DCB value: `MinValue`
    MinValue,
    /// DCB value: `DiminishingReturns`
    DiminishingReturns,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StatusHeadBleedingLocation`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusHeadBleedingLocation {
    /// DCB value: `LeftEye`
    LeftEye,
    /// DCB value: `LeftNostril`
    LeftNostril,
    /// DCB value: `LeftMouth`
    LeftMouth,
    /// DCB value: `LeftEar`
    LeftEar,
    /// DCB value: `RightEye`
    RightEye,
    /// DCB value: `RightNostril`
    RightNostril,
    /// DCB value: `RightMouth`
    RightMouth,
    /// DCB value: `RightEar`
    RightEar,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `StatusProbabilityCheckType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusProbabilityCheckType {
    /// DCB value: `Always`
    Always,
    /// DCB value: `StatValueIncrease`
    StatValueIncrease,
    /// DCB value: `StatValueDecrease`
    StatValueDecrease,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SubItemScanItemStatus`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubItemScanItemStatus {
    /// DCB value: `Default`
    Default,
    /// DCB value: `PowerOn`
    PowerOn,
    /// DCB value: `PowerOff`
    PowerOff,
    /// DCB value: `Destroyed`
    Destroyed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SurfaceRaindropEmitterType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceRaindropEmitterType {
    /// DCB value: `Rain`
    Rain,
    /// DCB value: `Snow`
    Snow,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `SyncedMeleeAttackResult`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyncedMeleeAttackResult {
    /// DCB value: `Hit`
    Hit,
    /// DCB value: `Countered`
    Countered,
    /// DCB value: `Dodged`
    Dodged,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `TacticalQuerySystemType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TacticalQuerySystemType {
    /// DCB value: `TacticalPointQuery`
    TacticalPointQuery,
    /// DCB value: `TacticalTargetQuery`
    TacticalTargetQuery,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `TakeDownQuadrant`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TakeDownQuadrant {
    /// DCB value: `AllQuadrants`
    AllQuadrants,
    /// DCB value: `BackOnly`
    BackOnly,
    /// DCB value: `FrontOnly`
    FrontOnly,
    /// DCB value: `NotPossible`
    NotPossible,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `TakeDownStance`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TakeDownStance {
    /// DCB value: `AnyStance`
    AnyStance,
    /// DCB value: `StandOnly`
    StandOnly,
    /// DCB value: `ProneOnly`
    ProneOnly,
    /// DCB value: `NotPossible`
    NotPossible,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `TestType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TestType {
    /// DCB value: `PhysicsImpact_Ocean`
    PhysicsImpact_Ocean,
    /// DCB value: `PhysicsImpact_WaterVolume`
    PhysicsImpact_WaterVolume,
    /// DCB value: `MFXHit`
    MFXHit,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `TransportDestinationOrderingMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportDestinationOrderingMethod {
    /// DCB value: `Height`
    Height,
    /// DCB value: `Priority`
    Priority,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UI3DDisplayInputType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UI3DDisplayInputType {
    /// DCB value: `Pan`
    Pan,
    /// DCB value: `Rotate`
    Rotate,
    /// DCB value: `Zoom`
    Zoom,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIBlockingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIBlockingMode {
    /// DCB value: `ScanMode`
    ScanMode,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIDisplayActivationTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIDisplayActivationTypes {
    /// DCB value: `Manual`
    Manual,
    /// DCB value: `AlwaysOn`
    AlwaysOn,
    /// DCB value: `OnWhenUsed`
    OnWhenUsed,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIDisplayEnvironmentAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIDisplayEnvironmentAlignment {
    /// DCB value: `GalacticPlane`
    GalacticPlane,
    /// DCB value: `World`
    World,
    /// DCB value: `Owner`
    Owner,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIElementAlignMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIElementAlignMode {
    /// DCB value: `dynamic`
    dynamic,
    /// DCB value: `fullscreen`
    fullscreen,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIGraph_BackBehavior`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIGraph_BackBehavior {
    /// DCB value: `Unsuported`
    Unsuported,
    /// DCB value: `NoAction`
    NoAction,
    /// DCB value: `LastDoNothing`
    LastDoNothing,
    /// DCB value: `LastRequestContextEnd`
    LastRequestContextEnd,
    /// DCB value: `LastRequestClose`
    LastRequestClose,
    /// DCB value: `LastCustomCallback`
    LastCustomCallback,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIGraph_BlockingMessagePopUpProvider`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIGraph_BlockingMessagePopUpProvider {
    /// DCB value: `GlobalGame`
    GlobalGame,
    /// DCB value: `ElectronicAccess`
    ElectronicAccess,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIGraph_SimpleComponentType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIGraph_SimpleComponentType {
    /// DCB value: `ElectronicAccessFullscreen`
    ElectronicAccessFullscreen,
    /// DCB value: `Hints`
    Hints,
    /// DCB value: `PlayerInteraction`
    PlayerInteraction,
    /// DCB value: `PlayerLens`
    PlayerLens,
    /// DCB value: `ChatWidget`
    ChatWidget,
    /// DCB value: `FpsCombat`
    FpsCombat,
    /// DCB value: `ACLoadoutContextSwitch`
    ACLoadoutContextSwitch,
    /// DCB value: `EALoadoutWarningPopUp`
    EALoadoutWarningPopUp,
    /// DCB value: `EAShipDetailsKickNoticePopUp`
    EAShipDetailsKickNoticePopUp,
    /// DCB value: `EAShipDetailsKickWarningPopUp`
    EAShipDetailsKickWarningPopUp,
    /// DCB value: `EALeaderboards`
    EALeaderboards,
    /// DCB value: `GGULoadingScreen`
    GGULoadingScreen,
    /// DCB value: `GGUNotificationScreen`
    GGUNotificationScreen,
    /// DCB value: `FrontendViewTransition`
    FrontendViewTransition,
    /// DCB value: `mobiGlasLauncherDock`
    mobiGlasLauncherDock,
    /// DCB value: `mobiGlasHomeDock`
    mobiGlasHomeDock,
    /// DCB value: `mobiGlasBeacon`
    mobiGlasBeacon,
    /// DCB value: `mobiGlasRouteInfo`
    mobiGlasRouteInfo,
    /// DCB value: `ShopApp`
    ShopApp,
    /// DCB value: `ShopDock`
    ShopDock,
    /// DCB value: `Popup`
    Popup,
    /// DCB value: `ContactsCommsApp`
    ContactsCommsApp,
    /// DCB value: `BackClick`
    BackClick,
    /// DCB value: `ShipList`
    ShipList,
    /// DCB value: `PortCategories`
    PortCategories,
    /// DCB value: `ItemKioskDock`
    ItemKioskDock,
    /// DCB value: `VehicleQuery`
    VehicleQuery,
    /// DCB value: `ShipSelectOrRental`
    ShipSelectOrRental,
    /// DCB value: `ShopKiosk`
    ShopKiosk,
    /// DCB value: `AreaMapContextComponent`
    AreaMapContextComponent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIWorldDisplayFollowRotationMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIWorldDisplayFollowRotationMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `None`
    None,
    /// DCB value: `Owner`
    Owner,
    /// DCB value: `User`
    User,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIWorldDisplayPathTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIWorldDisplayPathTypes {
    /// DCB value: `Highlighted`
    Highlighted,
    /// DCB value: `Selected`
    Selected,
    /// DCB value: `Destination`
    Destination,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIWorldDisplayPlaneAlignment`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIWorldDisplayPlaneAlignment {
    /// DCB value: `Default`
    Default,
    /// DCB value: `GalacticPlane`
    GalacticPlane,
    /// DCB value: `Owner`
    Owner,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `UIWorldDisplayUseInputMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UIWorldDisplayUseInputMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `UseInput`
    UseInput,
    /// DCB value: `NoInput`
    NoInput,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `VectorBases`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VectorBases {
    /// DCB value: `Right`
    Right,
    /// DCB value: `Forward`
    Forward,
    /// DCB value: `Up`
    Up,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `WeaponPoseType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponPoseType {
    /// DCB value: `RightHand`
    RightHand,
    /// DCB value: `Zoom`
    Zoom,
    /// DCB value: `LeftHand`
    LeftHand,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `WingmanTargetTypes`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WingmanTargetTypes {
    /// DCB value: `None`
    None,
    /// DCB value: `Player`
    Player,
    /// DCB value: `Target`
    Target,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `WorldDisplayObjectFacingMode`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldDisplayObjectFacingMode {
    /// DCB value: `Default`
    Default,
    /// DCB value: `World`
    World,
    /// DCB value: `Camera`
    Camera,
    /// DCB value: `AwayFromParent`
    AwayFromParent,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `ZeroGTraversalAction`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZeroGTraversalAction {
    /// DCB value: `None`
    None,
    /// DCB value: `Attach`
    Attach,
    /// DCB value: `Detach`
    Detach,
    /// DCB value: `Idle`
    Idle,
    /// DCB value: `MoveForward`
    MoveForward,
    /// DCB value: `LedgeTraversal`
    LedgeTraversal,
    /// DCB value: `IdleToTurn`
    IdleToTurn,
    /// DCB value: `KneeDrop`
    KneeDrop,
    /// DCB value: `LaunchTurn`
    LaunchTurn,
    /// DCB value: `Launch`
    Launch,
    /// DCB value: `Sprint`
    Sprint,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `eCommunicationChannelType`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum eCommunicationChannelType {
    /// DCB value: `Global`
    Global,
    /// DCB value: `Group`
    Group,
    /// DCB value: `Personal`
    Personal,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `eCommunicationChoiceMethod`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum eCommunicationChoiceMethod {
    /// DCB value: `Random`
    Random,
    /// DCB value: `Sequence`
    Sequence,
    /// DCB value: `RandomSequence`
    RandomSequence,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `eCommunicationCriteriaOperant`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum eCommunicationCriteriaOperant {
    /// DCB value: `None`
    None,
    /// DCB value: `Equals`
    Equals,
    /// DCB value: `LessThan`
    LessThan,
    /// DCB value: `LessThanOrEquals`
    LessThanOrEquals,
    /// DCB value: `GreaterThan`
    GreaterThan,
    /// DCB value: `GreaterThanOrEquals`
    GreaterThanOrEquals,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `eContextualCommunicationConcept`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum eContextualCommunicationConcept {
    /// DCB value: `Custom`
    Custom,
    /// DCB value: `OnIdleChatter`
    OnIdleChatter,
    /// DCB value: `OnHit`
    OnHit,
    /// DCB value: `OnVehicleHit`
    OnVehicleHit,
    /// DCB value: `OnFriendlyDied`
    OnFriendlyDied,
    /// DCB value: `OnFriendlyKilledEnemy`
    OnFriendlyKilledEnemy,
    /// DCB value: `OnTargetKilled`
    OnTargetKilled,
    /// DCB value: `OnVehiclePartDestroyed`
    OnVehiclePartDestroyed,
    /// DCB value: `OnRespawn`
    OnRespawn,
    /// DCB value: `OnKilled`
    OnKilled,
    /// DCB value: `OnVehicleEnemySpotted`
    OnVehicleEnemySpotted,
    /// DCB value: `OnVehicleEnemyMissileLockingOn`
    OnVehicleEnemyMissileLockingOn,
    /// DCB value: `OnVehicleEnemyMissileLockedOn`
    OnVehicleEnemyMissileLockedOn,
    /// DCB value: `OnVehicleEnemyMissileLockLost`
    OnVehicleEnemyMissileLockLost,
    /// DCB value: `OnVehicleEnemyMissileLaunched`
    OnVehicleEnemyMissileLaunched,
    /// DCB value: `OnVehicleMissileLockingOn`
    OnVehicleMissileLockingOn,
    /// DCB value: `OnVehicleMissileLaunched`
    OnVehicleMissileLaunched,
    /// DCB value: `OnResponseFinished`
    OnResponseFinished,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

/// DCB enum: `eContextualCommunicationCriteria`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum eContextualCommunicationCriteria {
    /// DCB value: `Custom`
    Custom,
    /// DCB value: `Who`
    Who,
    /// DCB value: `LevelName`
    LevelName,
    /// DCB value: `LastResponse`
    LastResponse,
    /// DCB value: `LastDialog`
    LastDialog,
    /// DCB value: `Vehicle`
    Vehicle,
    /// DCB value: `VehicleHealth`
    VehicleHealth,
    /// DCB value: `VehicleShield`
    VehicleShield,
    /// DCB value: `VehicleSpeed`
    VehicleSpeed,
    /// DCB value: `VehicleHitTime`
    VehicleHitTime,
    /// DCB value: `VehicleHitShield`
    VehicleHitShield,
    /// DCB value: `VehicleHitDamage`
    VehicleHitDamage,
    /// DCB value: `VehicleStartFireTime`
    VehicleStartFireTime,
    /// DCB value: `VehicleStopFireTime`
    VehicleStopFireTime,
    /// DCB value: `VehicleFiringWeapons`
    VehicleFiringWeapons,
    /// DCB value: `Attacker_Vehicle`
    Attacker_Vehicle,
    /// DCB value: `Attacker_VehicleHealth`
    Attacker_VehicleHealth,
    /// DCB value: `Attacker_VehicleShield`
    Attacker_VehicleShield,
    /// DCB value: `Attacker_VehicleSpeed`
    Attacker_VehicleSpeed,
    /// DCB value: `Attacker_VehicleFiringWeapons`
    Attacker_VehicleFiringWeapons,
    /// DCB value: `Target_Vehicle`
    Target_Vehicle,
    /// DCB value: `Target_VehicleHealth`
    Target_VehicleHealth,
    /// DCB value: `Target_VehicleShield`
    Target_VehicleShield,
    /// DCB value: `Target_VehicleSpeed`
    Target_VehicleSpeed,
    /// DCB value: `Target_VehicleFiringWeapons`
    Target_VehicleFiringWeapons,
    /// DCB value: `ActorHealth`
    ActorHealth,
    /// DCB value: `IsDriving`
    IsDriving,
    /// DCB value: `IsOnFoot`
    IsOnFoot,
    /// DCB value: `IsEjecting`
    IsEjecting,
    /// DCB value: `IsEjected`
    IsEjected,
    /// DCB value: `IsDead`
    IsDead,
    /// DCB value: `Attacker_Who`
    Attacker_Who,
    /// DCB value: `Attacker_ActorHealth`
    Attacker_ActorHealth,
    /// DCB value: `Attacker_IsDriving`
    Attacker_IsDriving,
    /// DCB value: `Attacker_IsOnFoot`
    Attacker_IsOnFoot,
    /// DCB value: `Attacker_IsEjecting`
    Attacker_IsEjecting,
    /// DCB value: `Attacker_IsEjected`
    Attacker_IsEjected,
    /// DCB value: `Attacker_IsDead`
    Attacker_IsDead,
    /// DCB value: `Attacker_IsFriendly`
    Attacker_IsFriendly,
    /// DCB value: `Target_Who`
    Target_Who,
    /// DCB value: `Target_ActorHealth`
    Target_ActorHealth,
    /// DCB value: `Target_IsDriving`
    Target_IsDriving,
    /// DCB value: `Target_IsOnFoot`
    Target_IsOnFoot,
    /// DCB value: `Target_IsEjecting`
    Target_IsEjecting,
    /// DCB value: `Target_IsEjected`
    Target_IsEjected,
    /// DCB value: `Target_IsDead`
    Target_IsDead,
    /// DCB value: `Target_IsFriendly`
    Target_IsFriendly,
    /// Unrecognised / newly-added enum value.
    Unrecognized(String),
}

