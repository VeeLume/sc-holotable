/// Errors that can occur during weapon data extraction.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WeaponError {
    /// A required component was not found on the entity.
    #[error("missing component `{component}` on entity")]
    ComponentMissing { component: &'static str },

    /// The ammo reference chain could not be resolved to an `AmmoParams` record.
    #[error("ammo resolution failed for `{weapon_name}`")]
    AmmoResolutionFailed { weapon_name: String },
}
