use sc_extract::generated::{EItemSubType, EItemType};

/// Weapon category determined from `SItemDefinition` type/subtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponCategory {
    /// Ship-mounted weapon: `WeaponGun` + `Gun` or `Rocket`.
    Ship,
    /// FPS personal weapon: `WeaponPersonal` + any subtype except `Gadget`.
    Fps,
    /// Ship missile / torpedo: `EItemType::Missile` + `Missile` or
    /// `Torpedo`. The two share enough of the data shape (explosion
    /// damage, GCS speed, optional tracking profile) that one
    /// `Missile` model covers both.
    Missile,
}

/// Classify a weapon from its `SItemDefinition` type and subtype.
///
/// Returns `None` for entities that have weapon components but aren't combat
/// weapons: countermeasure launchers, gadgets (tractor beams, fire
/// extinguishers), mining lasers, creature weapons, props, etc.
pub fn classify(item_type: &EItemType, sub_type: &EItemSubType) -> Option<WeaponCategory> {
    match item_type {
        EItemType::WeaponGun => match sub_type {
            EItemSubType::Gun | EItemSubType::Rocket | EItemSubType::NoseMounted => {
                Some(WeaponCategory::Ship)
            }
            _ => None,
        },
        EItemType::WeaponPersonal => match sub_type {
            EItemSubType::Gadget => None,
            _ => Some(WeaponCategory::Fps),
        },
        EItemType::Missile => match sub_type {
            EItemSubType::Missile | EItemSubType::Torpedo => Some(WeaponCategory::Missile),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ship_weapon_gun() {
        assert_eq!(
            classify(&EItemType::WeaponGun, &EItemSubType::Gun),
            Some(WeaponCategory::Ship)
        );
    }

    #[test]
    fn ship_weapon_rocket() {
        assert_eq!(
            classify(&EItemType::WeaponGun, &EItemSubType::Rocket),
            Some(WeaponCategory::Ship)
        );
    }

    #[test]
    fn ship_weapon_nose_mounted() {
        assert_eq!(
            classify(&EItemType::WeaponGun, &EItemSubType::NoseMounted),
            Some(WeaponCategory::Ship)
        );
    }

    #[test]
    fn fps_weapon_small() {
        assert_eq!(
            classify(&EItemType::WeaponPersonal, &EItemSubType::Small),
            Some(WeaponCategory::Fps)
        );
    }

    #[test]
    fn fps_weapon_medium() {
        assert_eq!(
            classify(&EItemType::WeaponPersonal, &EItemSubType::Medium),
            Some(WeaponCategory::Fps)
        );
    }

    #[test]
    fn gadget_excluded() {
        assert_eq!(
            classify(&EItemType::WeaponPersonal, &EItemSubType::Gadget),
            None
        );
    }

    #[test]
    fn countermeasure_excluded() {
        assert_eq!(
            classify(
                &EItemType::WeaponDefensive,
                &EItemSubType::CountermeasureLauncher
            ),
            None
        );
    }

    #[test]
    fn mining_excluded() {
        assert_eq!(classify(&EItemType::WeaponMining, &EItemSubType::Gun), None);
    }

    #[test]
    fn ship_missile() {
        assert_eq!(
            classify(&EItemType::Missile, &EItemSubType::Missile),
            Some(WeaponCategory::Missile)
        );
    }

    #[test]
    fn ship_torpedo() {
        assert_eq!(
            classify(&EItemType::Missile, &EItemSubType::Torpedo),
            Some(WeaponCategory::Missile)
        );
    }

    #[test]
    fn ground_vehicle_missile_excluded() {
        // GroundVehicleMissile is a ground-vehicle ordnance subtype, not a
        // ship missile. Filter it out of ship-missile coverage; if a future
        // consumer needs it, surface a sibling `WeaponCategory::GroundMissile`.
        assert_eq!(
            classify(&EItemType::Missile, &EItemSubType::GroundVehicleMissile),
            None
        );
    }
}
