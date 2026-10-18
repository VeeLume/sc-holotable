# Changelog

Consumers pin **`sc-holotable/vX.Y.Z`** tags, tracked in this file. Releases
are cut by `.github/workflows/release.yml`, automatically after a bindings
regen merges or by hand ("Run workflow"); it moves `[Unreleased]` into a
dated section, with a *Bindings* subsection per regen. Pre-1.0 convention:
`0.X.0` when the section has Added, Changed, Removed or Deprecated entries
or a regen removed record types; `0.x.Y` otherwise.

**`datacore/<label>`** tags mark the commit whose generated bindings came from
a game build. The same workflow cuts them when a regen merges; they are not
pinned. Both tag kinds are monotonic and immutable.

## [Unreleased]

### Added

- **`sc-extract`: vehicle XML reader** (`VehicleDefinition`), the first piece of the ship model for Starlume's loadout calculator.
  - `VehicleDefinition::read(assets, path, modification)` reads a ship's hull XML, named by `VehicleComponentParams.vehicleDefinition`. It applies the record's modification: first the `patchFile`, whose top-level elements replace the base elements with the same `id`, then the `<Elem idRef name value>` patches.
  - It gives the `Part` tree: name, class, parent, mass, `damageMax`, detach ratio / probability.
  - Each `ItemPort` gives size range, flags, port and required tags, display name, weapon group, typed `EItemType` / `EItemSubType` types, and pitch / yaw limits.
  - Also the ship's `itemPortTags` / `requiredItemTags` and the vehicle damage multipliers.
  - `hull_health()` sums `damageMax`; `hull_mass()` is the root part's mass.
  - A patch file missing from the archive (three LIVE 4.10 wreck records) logs a warning and keeps the base.
  - Probe `vehicle_xml_probe`: the RSI Constellation Mk V Centaurus reproduces spviewer / erkul (170,500 hull HP over 31 parts, 45,000 vital body, 1 / 12 / 14 / 4 part split, 1,240,000 kg). All 1,147 vehicle records read.
- **`sc-extract`: `Error::VehicleXml`**.
- **`sc-vehicles` (new domain crate; umbrella feature `vehicles`)**: ships and ground vehicles as a loadout sees them.
  - `Vehicles::build(datacore, items, containers, assets)` gives every entity with `VehicleComponentParams` (1,147 in LIVE 4.10):
    - its hull parts with HP and mass (`hull_health()`, `hull_mass()`), crew size and bounding box;
    - its ports: the hull XML's hardpoints plus the entity's own;
    - its `stock` loadout as a tree of `Mounted { port, item, children }`.
  - Stock loadout rule: an entry's item, or else the port's `defaultItem`; below it the entry's nested loadout, or else the item's own default loadout. Docked vehicles bring their hull's ports.
  - `Vehicle::insurance` gives the insurance claim terms: wait minutes and expediting fee (Centaurus 13.5 min, 6,750 aUEC).
  - `Vehicle::cross_section` gives the radar cross-section per axis (Centaurus 8,350 / 11,320 / 25,430; erkul shows the largest).
  - `PortContainers::build(datacore)` holds every entity's own ports and default loadout.
  - `fits(port, item, offered_tags)` checks the type / sub-type, size and tag rule. On the player-editable stock attachments of non-AI vehicles it rejects 1.5 %; locked ports are filled by force.
  - `Vehicle::offered_tags` collects the tags offered from above.
  - `has_flag` / `is_editable` follow the client's flag inheritance: `uneditable` locks a subtree, `$uneditable` only its port.
  - Probe `vehicles_probe`: the Constellation Mk V Centaurus resolves to every component spviewer lists, with its gimbals locked and their guns swappable.
- **`sc-vehicles`: resource network inputs** for the power and cooling model.
  - `ResourceProfiles::build(datacore)` holds every entity's `ItemResourceComponentParams` as `ResourceProfile`:
    - states with their generation / consumption / conversion / storage deltas (`ResourceUnits` as authored), minimum fractions and dynamic-amount overrides;
    - EM / IR signatures, power ranges and the power-plant modifier override.
  - `NetworkParams::build(datacore)` holds `ItemResourceNetworkGlobal`: per-size plant modifiers and the power-on distribution defaults.
  - `PortContainer::power_pools` lists the vehicle's `FixedPowerPool` / `DynamicPowerPool`s.
  - `network_items(loadout, vehicles)` lists the items on a vehicle's own network; a docked vehicle runs its own.
  - `plant_output` gives the client's share of several online plants. `signature_ratio` / `em_signature` give the power ratio with its range modifier and the EM it settles at.
  - `NetworkParams::mode_exclusions` / `excluded` give the master modes each item type is off in (quantum drives in SCM, shields in Navigation). `ResourceProfile` gains the item's own override and `on_by_default`.
  - Probe `ship_systems_probe`: the stock Centaurus gets 24 power from two Sedulity plants, as spviewer shows, and its nominal EMs and pools match.
- **`sc-vehicles`: `PowerNetwork`**, a loadout's power network as the client runs it (LIVE 4.10.197):
  - plants, consumers (request and minimum in micro-units, master-mode exclusions) and pools (members sorted as the client sorts them);
  - `supply`, `pool_size`, `pool_minimum`;
  - `power_on(mode)` gives the client's power-on distribution:
    - minimums, pool minimums, coolers, pool shares of half for weapons / engines / shields, then a top-up;
    - the client walks a runtime hash map, so the last segments can land on a different item than in game;
  - `split_pools` shares a pool target out to its members, so a player-set target can be applied;
  - `ratios` gives each item's power ratio.
  - `readout(allocation, mode)` gives what the engineering screen shows once temperatures settle (`Readout`, `ItemReadout`, from the client's `Signals`):
    - each item's power ratio with its range modifier, and its EM;
    - coolant made by the coolers, and coolant asked for by everything that heats (`P × ratio`);
    - cooler IR, which follows what is drawn from each cooler;
    - shield resource made by the generators, which is what the shield regenerates from;
    - `Readout::cooling()` gives the cooling bar.
  - `Signature::curve` holds the functionality curve; LIVE 4.10 has none.
  - `PortContainer::max_default_distribution`.
  - The probe reproduces the stock Centaurus at power-on in SCM and Navigation: coolers 4 + 4, flight controller 3 of 6, shields 2 + 2 (off in Navigation), guns 2 of 12.2, quantum drive off in SCM.
  - At power-on in SCM the probe's cooling bar is 33.65 / 82 = 41 %. spviewer's 65.9 % uses its own static formula.
- **`sc-items-ship-components`: per-kind sheets**, read through the typed surface:
  - `shield` (`ShieldGenerator`: HP, regen, delays, reserve pool, per-type resistance / absorption ranges);
  - `emitter` (`ShieldEmitter`: face layout, reallocation);
  - `armor` (`VehicleArmor`: damage multiplier, deflection, penetration, signal multipliers);
  - `quantum` (`QuantumDrive`: travel and spline-jump parameters, fuel, ranges);
  - `thruster` (`Thruster`: thrust, type, fuel burn);
  - `health` (`ComponentHealth`: HP and per-type damage multipliers);
  - `capacity` (`ResourceContainer.capacity` of fuel and quantum fuel tanks).
  - Probe `component_sheets_probe` re-proves them on the Centaurus's components.
- **`sc-vehicles`: `Loadout`**, an editable loadout:
  - `stock`, `at(path)`;
  - `chain(path)` gives the ports along a path and the items holding them;
  - `check(path, item)` returns `Refusal::Locked`, `NoPort` or `Misfit` when the item can't go in;
  - `candidates(path)` lists every item that fits;
  - `equip(path, item)` puts the item in with its own default loadout, dropping what the old one carried.
  - Probe `loadout_probe`: the Centaurus's gimbals are locked while their guns take the 11 S5 weapons. Two QuadraCell MT plants give 26, as erkul shows.
- **`sc-vehicles`: `Defense`** sums up a loadout's protection:
  - hull HP;
  - the armour sheet and its own HP;
  - the shield generators combined as the client combines them (`Shield::combine`: HP and regen add, delays averaged, resistance and absorption weighted by HP);
  - the emitter's face layout.
  - `flight_probe` re-proves it on the Centaurus: hull 170,500, armour 25,740 HP at 0.7 / 0.5, 2 × 5CA = 200,000 HP and 38,000/s, quadrant faces.
- **`sc-weapons`: `ShipWeapon::powered(&WeaponPower)`** gives an energy weapon as it runs at its power ratio, under the ship's `SRegenConsumerModifier` (client `FUN_145ce27e0`):
  - capacitor shots = round(load × `maxAmmoLoad` × multiplier);
  - regen = load × `maxRegenPerSec` × multiplier;
  - fire rate and damage per shot are unchanged.
- **`sc-vehicles`: `ItemportModifiers` and `weapon_power(…)`**: the attach-active item-port modifiers that change a vehicle's guns, and each gun's `WeaponPower` (its power ratio plus those multipliers).
  - Probe `weapons_probe`, the Centaurus at erkul's default (weapon pool 4 of 12.2, the engineering buff's ×9): CF-557 221 shots, 4.918/s, sustained 341.7; CF-337 152.3. erkul shows the same figures.
- **`sc-vehicles`: `Quantum`**, a loadout's quantum drive and quantum fuel:
  - `time(distance)` follows the client's travel envelope (`quantum_time`): five mirrored, jerk-limited phases, solving for the peak on short trips. Times are truncated to 1e-7 s ticks; spool and calibration are not included.
  - `fuel(distance)` uses the drive's travelling-state QuantumFuel per Mm; `range()` divides the tank by it.
  - The Centaurus's Khaos: 0.022 SCU per GM, range 390.9 GM, as spviewer shows. 31.92 GM takes 170.6 s; spviewer shows 2:46 from an older model, rounded up.
- **`sc-vehicles`: `Flight::hydrogen_burn` / `hydrogen_capacity`** give the full-thrust burn per thruster type and the tanks. The Centaurus mains burn 0.5925 SCU/min from 46.8 SCU; nothing burns at rest.
- **`sc-vehicles`: `Flight`**, a loadout's flight figures under the legacy IFCS the PU runs:
  - speeds and angular velocity (boosted × the afterburner multipliers);
  - mass: hull, everything mounted and docked vehicles;
  - accelerations per direction in G, base and boosted. Each thruster counts once, along the axis it is most aligned with.
  - `port_axes(assets, vehicle)` reads each port's push axis from the hull geometry's helper nodes. `Vehicle::geometry` gives the hull geometry path.
  - Probe `flight_probe`: the Centaurus reproduces spviewer: 1.63 Mkg; forward 4.94, back 1.98, strafe 2.48, up 2.01, down 1.98 G; boosted 7.41 / 2.38 / 3.34 / 2.61 / 2.38; SCM 170, boost 365, NAV 1075; 22 / 35 / 18 deg/s, boosted 33 / 70 / 27.
- **`sc-items-ship-components`: `flight` sheet** (`FlightControl`, `Afterburner`): the flight controller's legacy `IFCSParams`.
- **`sc-resources`: `CargoQuantity::from_ptr`** projects a `SBaseCargoUnitPtr`. `sc_crafting::cargo_quantity_from_ptr` now delegates to it.

### Changed

- **`sc-items-ship-components`: `ShipComponentKind` gains** `ShieldController`, `Armor`, `MainThruster`, `ManeuverThruster`, `LifeSupportGenerator`, `FlightController`, `FuelTank`, `QuantumFuelTank` and `JumpDrive`. The collection now covers them, and exhaustive matches need the new arms.

- **`sc-items`: `Item` gains `tags` and `required_tags`** (`AttachDef.Tags` / `RequiredTags`, split on spaces), so struct literals need them.
- **`sc-weapons`: `LoadoutContext::power_per_slot` is replaced by `power: Option<WeaponPower>`.** `effective_dps` no longer scales burst DPS linearly with power: power changes how long an energy weapon fires and refills, not its rate or damage.
- **`sc-weapons`: sequence fire rate = entries / max(Σ entry delays, slowest inner cooldown)**, the client's per-barrel gating and erkul's rule. It was `min(first delay, inner rate)`; the BEHR ballistic repeater now fires 750 rpm, not 500.
- **`sc-weapons`: sequence heat = the inner actions' `heatPerShot` × the sequence rate.** It was `heatRateOnline`, which the client doesn't read on the shot path. The BEHR repeater now overheats after 10 s (125 shots), as erkul shows, not 4.2 s.
- **`sc-items-ship-components`: `Thruster::fuel_burn_rate_per_10kn` (the legacy float) is replaced by `fuel_burn_per_10kn`**, read from `fuelBurnRatePer10KNewtonRN` in SCU. That is the field the client burns.
- **`sc-items`: `Item` gains `mass`** (`SEntityPhysicsControllerParams.PhysType.Mass`, kg) and no longer derives `Eq`.
- **`sc-extract` / `sc-vehicles`: `hull_mass()` sums every part's mass**, not just the root's. The Skylark's 11 detachable panels add 115.5 kg; the Centaurus has none.

## [v0.26.0] - 2026-10-18

### Added

- **`sc-mining`: beam cuts in the simulation.**
  - `LaserControl` gains `firing` (on before the first tick). A cut beam sends no new hits, but the hit time already queued on the rock keeps draining at the laser's current throttle and distance (client 4.10.197: the drain has no firing check).
  - Hit queues are per laser.
  - A laser's modifiers stay on the rock until `modifier_persistence_time` (0.25 s) after its last hit drains, then come off until the next hit.
  - `ModifiedRock::with_lasers(ruleset, rock, setup, contributing)` gives the rock with only some lasers' modifiers on.
  - The simulation caches these rocks per timeline span and hitting set.
  - Each laser has its own hit consistency, and the wave's damping is the mean over the lasers that delivered hit time that tick.
  - `LaserSample` gains `firing`, `queued_s` and `contributing`.
  - With every beam always on, results are bit-identical to v0.25.0, checked by trace fingerprints over solo and crew setups, three tick / hit-interval pairs and three controllers.

### Changed

- **`sc-mining`: `ModifiedRock` carries a private field** (its modifier sources), so it can no longer be built as a struct literal. Build it with `ModifiedRock::new` or `with_lasers`. A deserialized `ModifiedRock` has no sources and ignores beam cuts.
- **`sc-mining`: `LaserControl` and `LaserSample` gain fields** (`firing`; `firing`, `queued_s`, `contributing`). Struct literals need them.

## [v0.25.0] - 2026-10-18

### Added

- **`sc-mining`: what an overcharge keeps** (client 4.10.197). The danger
  pool at fracture drops pieces. A fracture keeps every `base` piece plus
  the first `round(n * (1 - danger))` of the others in mesh order, and the
  dropped pieces' ore is lost. `Shape::kept_pieces(danger)` and
  `Shape::yield_fraction(danger)` give the count and the share of the ore
  left; `Piece` gains `base`. Probe: `mining_overcharge_probe`.
- **`sc-mining`: per-laser throttle with the client's lerp.** Each laser's
  throttle follows its own setting by `clamp(throttleLerpSpeed * dt,
  throttleMinimum, 1)` per tick (`LaserPower::next_throttle`).
- **`sc-mining`: distance per tick.** A controller can move each laser
  (`LaserControl::distance_m`), and the beam's range falloff is taken there
  each tick (`LaserPower::power_at`).
- **`sc-mining`: active modules over time.** `Setup::activations` lists
  `ModuleActivation { laser, module, start_s }`. `Timeline::new(ruleset,
  rock, setup)` turns them into a rock per stretch: modifiers and power
  multiplier apply over `[start, start + lifetime)`, with uses capped by
  charges. `simulate_timeline` runs an attempt over a `Timeline`.
  `Trace::lasers` records each laser per tick, and `Sample` gains `energy`.
- **`sc-items-mining`: `MiningModule::lifetime_s()` / `charges()`**.

### Changed

- **`sc-mining`: `Spawn::fracture` and `Spawn::fracture_odds` take
  `danger_pool`** (before the seed / run count). Only the kept pieces spawn,
  and each element fills them at its parent share. Pass `0.0` for the old
  clean-fracture result.
- **`sc-mining`: `Throttle` sets every laser.** The trait is now `fn
  throttle(&mut self, view, lasers: &mut [LaserControl])`: set each laser's
  `throttle` (and optionally its `distance_m`). `View` gains `throttles`,
  each laser's throttle now. `Fixed` and `Player` set every laser alike.
  With the lerp a `Player`'s results shift slightly; `Fixed` results are
  unchanged up to float rounding.
- **`sc-mining`: `Setup` gains `activations`** and **`LaserPower` gains
  `throttle_lerp_speed`, `distance_m`, `power_in_range` and the beam's
  `full_damage_range_m` / `zero_damage_range_m`.** Struct literals need the
  new fields (`..Default::default()` works for `Setup`).
  `ModifiedRock::new` still takes the setup as fitted, without activations.

## [v0.24.0] - 2026-10-17

### Added

- **`sc-mining`: `FractureBlast`**, the explosion every fracture makes, a
  clean one included. Radii, damage and push are scaled from
  `defaultExplosionParams` by the rock's volume, composition
  (`explosion_multiplier`), shatter-damage modifier and the danger pool at
  the moment of fracture (client 4.10). `radius_max` at the current danger
  pool is the HUD's striped zone on the mining distance gauge. Build one
  with `FractureBlast::new(ruleset, volume, explosion_multiplier, shatter,
  danger_pool)` or `Rock::fracture_blast(ruleset, shatter, danger_pool)`.
  `damage_at(distance)` / `damage_falloff` give what an item or other
  non-vehicle target takes: full inside `radius_min`, then falling with the
  squared distance to 0 at `radius_max`. A ship's hull takes no distance
  falloff, so it takes the full `damage` anywhere in reach. `pressure_at`
  gives the push, which falls off linearly.

### Changed

- **`sc-mining`: `Ruleset`** gains `danger_pool_factor` and `blast:
  BlastParams` (min/max radius, pressure, damage from
  `mineableExplosionParams.defaultExplosionParams`), and
  **`ModifiedRock`** gains `shatter_damage: Combined`, folded from the
  lasers', modules' and gadgets' `shatterdamageModifier`. Code that builds
  either struct by literal must set the new fields.

## [v0.23.0] - 2026-10-16

### Added

- **`sc-locations`: `ObjectContainers::walk`** cooks only what given root
  socpaks reach (level by level, each level read in parallel) and hands
  every decoded member to a visitor, so a caller reads other components in
  the same pass. `ObjectContainers::nesting` lists every placement that
  includes a socpak, named or not; `ObjectContainers::parent_entity`
  resolves a placement's `parentGUID`.

### Changed

- **`sc-gathering`: `ProviderLocations::cook(&AssetSource, &RecordStore)`**
  walks only the object containers of the solar systems the `MegaMap`
  records list: about 9 s and 0.8 GB over the DCB parse on LIVE, instead of
  a full `ObjectContainers::cook` (about 200 s) plus a second pass.
  `ProviderLocations::cook_from` takes other roots. A provider now applies to
  the nearest named placement above its socpak, so nested fields get a place
  (the Nyx belts map to Nyx, Pyro's Lagrange clouds to their Lagrange
  points), and every provider in a socpak counts (Yela's ring). The places
  that only the old full walk found come from content no live system
  includes. `provider_at` is replaced by `providers_at`, which returns every
  provider at a place. `compose` takes `HashMap<String, Vec<Guid>>`.
  `COOK_SCHEMA_VERSION` is now 2.
- **`sc-locations`: `ObjectContainers::cook` reads the socpaks in parallel**
  (about 16 s instead of 200 s on LIVE), with the same result.

## [v0.22.0] - 2026-10-16

### Changed

- **`sc-mining`: `simulate` meters hits through the client's hit queue.**
  Each laser's hits wait in a queue, and every rock update drains at most
  `HIT_DRAIN` (1.2) updates' worth of hit time, at the throttle set then.
  The queue holds `HIT_QUEUE_CAP_S` (1 s); hits beyond that are dropped.
  `SimConfig` gained `hit_interval_s` (default 1/30 s), and `tick_s` is now
  the rock's update step. When hits arrive at a different rate than the rock
  updates, ticks alternate between strong and weak, and the instability wave
  amplifies the strong ones even at the balance throttle. The defaults (both
  1/30 s) give the same results as before. Code constructing `SimConfig`
  literals must add the field.

## [v0.21.0] - 2026-10-15

### Added

- **`sc-extract`: render inputs for 3D models.** `MeshGeometry::read` reads a
  geometry file and its stream companion into everything needed to draw it:
  positions, normals, tangents, UVs and vertex colours, the triangle list,
  `Submesh`es (material index, node, index range), the node tree and the
  `.mtl` it names. `Material::read` parses a `.mtl` into `SubMaterial`s
  (shader, `StringGenMask` features, textures by slot, `LayerBlend` layers,
  public params). `Texture::read` reassembles a split DDS (`.dds` + `.1`…`.8`)
  and its separate gloss texture (`.a` + `.1a`…`.8a`) into complete DDS files;
  `Texture::is_srgb` reports the engine's sRGB-read flag (set on colour and
  blend maps, which the block formats cannot express).
  `MeshGeometry::material_path` resolves a bare `MtlName` (no folder) next to
  the geometry file. `Skeleton::read` reads a `.chr`'s bones (both
  `CompiledBones` layouts) in model space, for parts attached to bones.
  `MeshGeometry::bone_weights` carries a `.skin`'s per-vertex skinning
  (`BoneWeights`: up to eight joints into its own bone list, weights summing
  to 255). `CharacterDefinition::read` parses a `.cdf` (base model plus `CA_SKIN` /
  `CA_BONE` attachments). Probe: `render_assets_probe`.
- **`sc-items-visuals` (umbrella feature `visuals`): item render recipes.**
  `Visuals` holds, per item, the geometry node tree with its tagged
  variants, the material node, the skin tag (`geometryTags`), the ports with
  their attach helpers and offsets (and the attached item's own helper,
  `ItemOffsetHelperName`) or whether the attached item is skinned to the
  item's skeleton (`Port::skinned`: a body's head, a head's hair, worn
  armor), the default loadout and the inventory grid cell;
  `Visual::look` resolves the geometry, material and tint palette for the
  item's tags. `Palettes` reads the `TintPaletteTree` records.
  `InventoryView` reads the inventory's orientation and light-group tables
  and the light rigs, and picks an item's entries by type, sub type and
  required tag. Probe: `visual_probe` re-derives the values sc-item-render
  calibrated against in-game screenshots. `Visuals::scene` assembles an
  item for drawing (`Scene`): its `.cdf` parts, what hangs on its ports
  (the default loadout or the caller's `Attached` items), skinned children
  sharing the parent's skeleton, and the variant tags per part (`Body`, the
  stored prop unless worn, port flags); what it cannot resolve comes back as
  `SceneNote`s. Probe: `scene_probe`.

### Changed

- **`sc-extract`: `MeshNode` gained `transform`** (the node's 3 × 4 placement
  relative to its parent). Code constructing `MeshNode` literals must add it.

## [v0.20.0] - 2026-10-15

### Added

- **`sc-discovery`: launcher patch state.** `Installation::patch_state()`
  reads the launcher log, the launcher store and `build_manifest.id` afresh
  and returns `PatchState::Settled`, `Patching(LauncherRun)` (a launcher
  install / update / verify run is open on the install, possibly paused) or
  `Unsettled` (`RunDidNotComplete`: the last run that wrote files failed, was
  cancelled or died; `NotTheRecordedBuild`: the store records another build
  than the manifest). The launcher rewrites `Data.p4k` in place and moves
  `build_manifest.id` to the new build minutes before the patch ends, so check
  before reading the p4k or writing into the install. `launcher_runs(path)`
  lists every run in a launcher log with its kind, target label, start time,
  whether it wrote files and how it ended.

### Fixed

- **`sc-discovery`: log fallback reads launcher 2.14+ lines.** The
  `[Pipeline]`, `[PatcherPhase]` and `[InitialDownloadPhase]` install lines
  that replaced `[Installer]` are recognised again, so the log fallback finds
  installs patched since April 2026 that were never launched.
- **`sc-discovery`: the launcher store key is scanned once per process.**
  Repeated store reads no longer stream the ~430 MB `app.asar` each time; a
  rotated key triggers one fresh scan.
- **`sc-regen` skips an install the launcher is patching**, and fails the run
  when the build changed while the generator read it.

## [v0.19.0] - 2026-10-13

### Added

- **`sc-items-mining`: mining modules and gadgets** (umbrella feature
  `mining-items`, module `mining_items`). `MiningModules` /
  `MiningGadgets::build(&Datacore, &Items)` read each module's power
  multiplier per laser fire action (`MiningModule::damage_multiplier(i)`; the
  Surge is x1.5 on the fracturing beam and x1.0 on extraction), its mining
  modifiers, inert filter and activation (`Passive`, or `Active` with
  duration and uses), and each gadget's rock modifiers. The shared
  `MiningModifiers` / `FloatModifier` types keep each value's kind
  (multiplicative or additive), as the client combines them differently.
- **`sc-items-ship-weapons`: mining-laser stats.** `ShipWeaponStats::mining`
  (`MiningLaserStats`) carries every beam (`MiningBeam`: energy per second,
  `full_damage_range` / `zero_damage_range`, hit type), the fracturing and
  extraction beam (`fracture_beam()` / `extraction_beam()`), the throttle
  minimum and lerp speed, `uses_power_throttle`, the laser's own mining
  modifiers and inert filter, and its number of module slots.
  `MiningBeam::falloff(distance)` is the client's range falloff: full power up
  to the optimal range, linear to 0 at the maximum range, no bonus closer in.
- **`sc-mining`: the rock side of mining** (umbrella feature `mining`,
  module `mining`). `Rulesets` / `Elements::build(&Datacore, &Resources)`
  read `MiningGlobalParams` and `MineableElement` (with densities). `Rock`
  computes a rock's resistance, instability, optimal window, mass and SCU from
  the composition the scanner lists; `ModifiedRock::new(ruleset, rock, setup)`
  applies a `Setup` (lasers with modules, gadgets) and answers breakability
  (`max_mass`, `is_breakable`), the balance throttle, time to the window and
  in it. `simulate` runs the charge bar tick by tick with the instability
  wave, hit consistency and both breaking pools; `difficulty` runs it many
  times with a `Player` model. `Laser::extraction_scu_per_s` gives extraction
  speed. Checked against in-game scans and Mort13's Breakability Chart
  (`mining_rocks_probe`).
- **`sc-mining`: rock spawns and fracture trees.** `RockTypes::build`
  (every `HarvestablePreset` mineable: scale range, meshes, composition),
  `Compositions::build` (`MineableComposition` with `Composition::roll`, the
  client's spawn roll), `Shapes::build(&AssetSource, &RockTypes)` (each
  rock mesh's `Shape`: the pieces it breaks into and theirs, with
  `cracks_to_collect` / `collectable_pieces`), and
  `RockType::geometries_for(&Tags, geometry_tags)` (which meshes a provider
  entry spawns). `Spawner` rolls rocks as the client does and `Survey`
  summarises how a setup fares on them, the pieces of a fracture included.
  `Spawn::fracture` / `fracture_odds` split a rock's elements across its
  pieces as the client does; `Composition::roll_with_quality` with a
  `QualityRoll` rolls each entry's quality (`Entry::quality`), and
  `Composition::odds` gives each part's chance and share. Sampling functions
  take a `seed` that picks one draw (the game seeds per spawn location at
  runtime); the odds functions take none. Checked against a fractured iron
  asteroid's pieces and the scanned qualities (`mining_mesh_probe`,
  `mining_spawn_probe`, `mining_fracture_probe`, `mining_quality_probe`).
- **`sc-extract`: `Mesh`.** `Mesh::read(&AssetSource, path)` reads an IVO
  geometry file's node tree with the volume each node's triangles enclose.

### Changed

- **`sc-items-ship-weapons`: `ShipWeaponStats` has a new `mining` field**
  (`None` for guns). Code that builds the struct with a literal must add it.
- **`sc-gathering`: `GatherableElement` has a new `geometry_tags` field**
  (the provider entry's model tags, which pick the rock's mesh). Code that
  builds the struct with a literal must add it.

## [v0.18.1] - 2026-10-13

### Bindings

- Regenerated from SC 4.10.2-live.12913470 (`datacore/4.10.2-live.12913470`), since `datacore/4.10.2-live.12881860`.

## [v0.18.0] - 2026-10-11

### Added

- **`sc-extract`: stale bindings are visible in release builds.**
  `Datacore::unknown_record_types() -> &[UnknownRecordType]` lists the DCB
  record types the generated bindings have never seen, with how many records
  of each were skipped. `Datacore::parse` logs the game build the bindings
  came from on every parse, and a `warn!` naming the unknown types when the
  list is non-empty. Previously release builds dropped those records without
  a trace; debug builds still panic.
- **`tools/sc-regen`: bindings regen without anyone running it.** A
  scheduled task notices a new installed game build, regenerates in its own
  worktree off `origin/main`, runs the tests and smoke examples, and opens a
  PR with a change report (record types added/removed, bucket sizes). A
  safe regen (LIVE, checks green, no record type removed, no bucket drift)
  merges itself once CI passes; the rest wait for review.
- **Releases cut themselves.** `.github/workflows/release.yml` tags a merged
  regen `datacore/<label>` and then releases `sc-holotable/vX.Y.Z`
  (CHANGELOG commit, tag, GitHub release); "Run workflow" releases work
  without a regen.

### Changed

- **Consumers pin `sc-holotable/v*` only.** `datacore/*` tags now just mark
  which commit's bindings came from a game build; a regen ships as a library
  release.

### Removed

- **`tools/regenerate.ps1`** — replaced by `sc-regen`.
- **`tools/release.ps1`** — replaced by `release.yml`; there is no local
  release path any more.

### Fixed

- **`sc-discovery`: no launcher label for a build it doesn't describe.** The
  launcher store labels channel slots, so a LIVE folder renamed to PTU (to
  seed a PTU patch) took the PTU label while still holding the LIVE build.
  `Installation::launcher_version_label` is now `None` when the label's
  changelist differs from the manifest on disk.

## [v0.17.0] - 2026-10-11

### Added

- **`sc-missions`: NPC spawn counts.** `NpcSlot::spawn_counts:
  Option<NpcSpawnCounts>` (`group_size`, `max_concurrent`, `max_spawns`
  from `AutoSpawnSettings`) and `NpcSlot::character_tags: TagBag`
  (`positiveCharacterTags`: archetype + faction). Previously an NPC slot
  carried no count at all; consumers parsed `x N` out of designer phase
  names, which disagree with the data on ~6% of named slots. Proven by
  `examples/npc_spawns.rs` (every NPC slot on 4.10 LIVE carries counts).
- **`sc-missions`: `Mission::integer_properties`** — every integer-valued
  mission property by variable name, resolved across template → handler →
  contract → sub-contract (most-specific wins), not gated on a text token.
  Carries the engine flags a mission never prints, notably the crimestat
  signal `DontHarmAllies` / `DontHarmCivs`: 172 contracts on 4.10 LIVE
  override a template's flag to 0, which a template-only read gets wrong.
  Proven by `examples/dont_harm.rs`.
- **`sc-missions`: `TagBag::archetypes` and `TagBag::npc_classes`** —
  `AI ▸ Archetype ▸ *` (`PU_Soldier`, `PU_CQC`, …) and `AI ▸ NpcClass ▸ *`
  (`Light` … `Boss`) classifiers for character tags.
- **`sc-missions`: event ladders and points.** `Missions::events: Events`
  — each limited-time event (`ScenarioProgress` record) with its ladders:
  the overall one and the category ones (Transport / Collection / Defense),
  each with its reward tiers (`min_points`, badge).
  `MissionRewards::event_points: Vec<EventPoints>` — what a contract adds:
  overall `points` (`ContractResult_ScenarioProgress.PointsToAward`),
  `split_for_party` (`splitPointsForParty`, overall ladder only), and
  per-category `ladders` (the contract's completion-tag counts for the
  event's ladder tags; never split). On 4.10 LIVE, RSI Discovery Month has
  31 point contracts, each feeding the overall ladder and one category
  ladder; 3 split the overall points. Proven by `examples/event_points.rs`.

- **`sc-missions`: `OtherReward::VehicleRental`** — `ContractResult_VehicleRental`,
  new in the 4.10 bindings (Discovery Month's Constellation rental offer).

- **`sc-extract`: `examples/dcb_xml_export.rs`** — dumps every DCB record to
  XML for grepping data the bindings don't model yet.
- **`sc-extract`: `LocaleMap::resolve_name` and `is_placeholder`** — name
  resolution that treats empty text and CIG's `<= PLACEHOLDER =>` sentinel
  as unresolved (previously private to sc-crafting).

### Changed

- **`sc-missions`: `Ships::display_name`** returns `None` for placeholder
  names instead of `<= PLACEHOLDER =>` (it showed up in rendered ship pools).
- **`sc-missions`: NPC `SlotGroup::concurrent_range`** is now the slot's
  `max_concurrent` instead of a constant `(1, 1)` (still `(1, 1)` when no
  spawn settings are attached).
- **`sc-missions`: `OtherReward::ScenarioProgress` removed** — those
  rewards are now typed in `MissionRewards::event_points`.

## [v0.16.0] - 2026-07-03

### Added

- **`sc-holotable`: `missions-payout` umbrella feature** — passes through
  sc-missions' `payout` feature (`UecCurve` / `Mission::estimate_uec`, backed
  by the `gamemode` sc-extract pools) which previously had no umbrella
  passthrough, forcing consumers to hardcode the uecCurve constants. Kept
  separate from `missions` so contract-only consumers don't pay the gamemode
  extract cost; included in `full`.

## [v0.15.0] - 2026-07-03

### Added

- **`sc-extract::RecordCollection`** — the uniform read contract over every
  GUID-keyed cooked collection: `get(&Guid)` / `len` / `iter() -> (&Guid, &Item)`
  plus provided `is_empty` / `contains` / `values` / `guids`. Implemented by
  `Items`, `Tags`, `Manufacturers`, `Resources`, `Providers`, `Locations`,
  `Blueprints`, `Missions`, and the four `sc-items-*` stat sheets. Every
  collection crate re-exports the trait next to its collection;
  `sc_holotable::prelude` includes it. Normative spec: `docs/CONVENTIONS.md`
  (new — the API-conventions contract, checked in alongside the sweep).
- **`sc-missions`: hauling cargo manifests.** `Mission::cargo: Vec<HaulingLeg>` —
  one leg per `HaulingOrderContent_Resource` (commodity `ResourceType` GUID +
  min/max SCU + max box size), read from `MissionPropertyValue_HaulingOrders`
  across the template `contractProperties` and handler / contract / sub override
  layers, with an objective-tree fallback (`ObjectiveHandler_Hauling`) for
  procedural broker contracts. `HaulingLeg::commodity` / `commodity_name` join
  the leg to the `sc-resources` catalog (new dep). Split-delivery contracts
  (`SingleToMultiN` / `MultiNToSingle`) intentionally carry one leg per
  endpoint; validated on live data that leg counts match endpoint counts (no
  cross-layer duplication — see `examples/hauling_dup_check.rs`).
- **`sc-missions`: `Mission` is serde-capable.** The full facet tree
  (`MissionOrigin`, `MissionRewards`, encounters, prereqs, vars, `HaulingLeg`, …)
  derives `Serialize`/`Deserialize`; generated enums round-trip as DCB strings.
- **`tools/sc-cargo-viewer`** — new workspace member: 3D viewer for ship cargo
  grids + interior wall boxes (internal validation tool for the cargo-grid
  geometry pipeline).
- **`sc-locations::object_containers`** — one tolerant socpak placement-graph walk
  (`ObjectContainers::cook`) that the location-domain socpak products project from,
  replacing the parallel scanners. `ObjectContainers` is a navigable graph:
  `get` / `iter` / `placements_of(guid)` / `by_mission_crc` / `chain_of` /
  `parent_of` / `global_position` / `realized_socpaks` / `locations_in`, over a
  public `Placement` (`starmap_guid` / `position` / `orbit` / `socpak` /
  `entity_name` / `is_mission_leaf`) keyed by `PlacementId`. It is
  `Serialize`/`Deserialize` (persists `placements`; rebuilds indices on load), so a
  cooked graph re-`join`s into a `Universe` without re-walking the socpaks.
  `normalize_socpak_path` is public — the canonical key form for any consumer
  joining on socpak paths (`sc-gathering` uses it).
- **`sc-locations::Universe`** — the join of the DCB StarMap (`Locations`) and the
  socpak placement graph (`ObjectContainers`). `Universe::join`, lookups
  `by_guid` / `by_class_crc` / `by_mission_crc` / `by_name`, and the logical graph
  (`roots` / `parent` / `children` / `ancestors`) as `Place`s. `Place` exposes both
  facets: **direct** accessors (`name_key`, `position`, `kind`, `socpak`,
  `entity_name`) and opt-in derived helpers (`name` / `display_name` for
  nearest-named-ancestor resolution, `global_position` for a system-global
  coordinate composed up the containment chain, `anchor_position` for the
  parent-body fallback).

### Changed (breaking)

- **API-consistency sweep to the `RecordCollection` contract.** Canonical
  accessors are now trait-only — the inherent `get` / `iter` / `len` /
  `is_empty` copies were removed from every collection (bring the trait into
  scope: `use sc_extract::RecordCollection;` or the crate-local re-export).
  `iter()` yields `(&Guid, &Item)` pairs everywhere; value iteration is
  `values()` (replacing `Manufacturers::all()`, `Gathering::providers()`, …);
  `get` borrows its key everywhere (`Missions::get(Guid)` → `get(&Guid)`).
- **`sc-gathering`: `Gathering` → `Providers`** (per the one-noun rule;
  `provider()` → trait `get`). `Foundations` / `HolotableSnapshot` carry the
  new type; the serialized shape is unchanged.
- **`sc-gathering`: `ProviderLocations::cook` / `compose` take
  `&ObjectContainers`** (were `&LocationContainers`), following the
  `sc-locations` replacement below.
- **`sc-manufacturers`: `Manufacturer::{name_key, description_key}` are
  `Option<LocaleKey>`** (were `Option<String>`), per the typed-locale rule.

### Fixed

- **Place ↔ socpak binding on SC 4.8** — the pre-4.8 `SNavPointObjectMetadataParams`
  scan returned 0 bindings after the component was renamed `SObjectMetadataParams`.
  The tolerant placement graph fixes this and extends coverage to nested body OCs
  (`sc-gathering` now resolves providers again).
- **Mission-CRC stragglers** — locations whose `starmapRecord` sits on a
  non-`SObjectMetadataParams` node (e.g. MIC-L1 Shallow Frontier Station on `<Elem>`)
  now resolve, via the graph's `first_attr`-style search across all descendants.

### Removed

- **`MissionCrcCatalog` / `MissionLocation`** — superseded by
  `Universe::by_mission_crc` (a `Place` with both name and position). The consumer
  (sc-cargo-planner) is migrated; the `crc_catalog` example is dropped.
- **`LocationContainers`** — superseded by `ObjectContainers` (`realized_socpaks` /
  `locations_in` / `placements_of`) and `Universe` (`Place::socpak`). The
  in-workspace consumer `sc-gathering` is migrated; the throwaway `containers_dump`
  example is dropped.
- **`sc-weapons::WeaponError`** — dead error enum (defined, never returned)
  deleted, per the error policy in `docs/CONVENTIONS.md` §6.

## [v0.14.0] - 2026-06-08

### Added

- **Per-itemtype base-stat T1 crates + crafting product stats.** Four new data
  crates expose the *base* stats a crafted item declares, read from the entity's
  typed component structure:
  - `sc-items-fps-weapons` — FPS weapons (fire rate, damage, spread, recoil, magazine).
  - `sc-items-armor` — character armor (temperature/radiation resistance, per-type damage resistance).
  - `sc-items-ship-components` — coolers / power plants / quantum drives / shields / radars (integrity + per-domain stats).
  - `sc-items-ship-weapons` — ship guns + mining lasers (integrity + per-shot / beam damage).

  `sc-crafting` ties them to recipes: `GameplayStat` (the typed gameplay-stat
  vocabulary, mapped from a GPP's **record name** — never its churning GUID —
  with build-time drift validation), `GameplayProperty::stat()`, the
  `ProductStatSource` trait (one impl per domain), and
  `Blueprints::product_stats(entity, &gp, &source, quality) -> Vec<ProductStat>`,
  which aggregates a recipe's per-slot modifiers (additively, the in-game rule)
  and applies them to the item's base value. Verified to the decimal against the
  SC Crafter (scmdb) reference across all four domains. The umbrella
  `sc-holotable` exposes every surface behind `fps-weapons` / `armor` /
  `ship-components` / `ship-weapons` features (all pulled by `crafting`).

## [v0.13.0] - 2026-06-07

### Added

- **`sc-crafting`: per-material crafting-effect surface.** `Cost::Select` now
  carries `name_info: Option<SlotName>` (the slot label — "Frame", "Cabling",
  "Power Regulator", …) and every cost node carries `context: Vec<CostContext>`,
  exposing the gameplay-property modifiers that were previously dropped. New
  types: `SlotName`, `CostContext` (`GameplayPropertyModifiers` /
  `QuantityMultiplier` / `ResultCompositionInclusion` / `Other`),
  `CompositionInclusion`, `GameplayPropertyModifier`, `ValueRange`
  (`Linear` / `LinearIntegerAdditive` / `Other`), `ModifierValue`. Quality
  interpolation helpers: `ValueRange::{quality_band, contains, evaluate}`,
  `GameplayPropertyModifier::evaluate(quality)`, and
  `Cost::{context, gameplay_property_modifiers}` (subtree roll-up). This is the
  data a consumer needs to render a slot/material/quality crafting calculator.
  See `docs/sc-crafting.md` § "Update 2026-06-07".

## [v0.12.0] - 2026-06-06

### Added

- **`sc-missions`: contract enrichment — every `Mission` now carries its
  display-and-grouping facets.** `category` (the `MissionType` — SCMDB's "Mission
  Type"), `faction` (reward-or-prerequisite reputation faction), `difficulty`
  (the four `ContractDifficulty` axes + profile + weights), `buy_in`,
  `time_to_complete`, `mission_span` localities (now with a typed
  `LocationRef::kind` via `sc-locations`), and `grants_completion_tags`.
  `Missions::prerequisite_missions` walks a contract's required tags back to the
  missions that grant them (the mission-chain graph).

- **Reputation + mission-type registries on `Missions`.** `factions`
  (`FactionReputations` → `FactionRep`), `rep_standings` (`ReputationStandings` →
  `Standing`), and `mission_types` (`MissionTypes` → `MissionTypeInfo`) resolve
  the GUIDs a contract references to stable record names + localizable display
  keys. Reputation *prerequisites* now surface as `PrereqView::Reputation`,
  including career-contract gates (handler faction + contract min/max standing).

- **`~mission(...)` runtime-marker resolution.** `Missions::title_text` /
  `description_text` substitute the engine's `~mission(Token|Fmt)` markers in a
  contract's title/description: reputation rank (from the rep gate), cargo grade
  and numeric quantities (from the contract's `MissionProperty` values), and
  location-query facets (system · setting, from the `TagSearch` tags). Values the
  contract pins render verbatim; values the engine finalizes at spawn (a query
  scope, a gated / multi-option pick) render in `[brackets]`. Backed by
  `Mission::variables` (the resolved non-spawn `MissionProperty` map) with
  `MissionVar` / `VarOption`, plus `Missions::unresolved_markers`.

### Fixed

- **`Mission::time_to_complete` is minutes, not seconds.** The field and its doc
  comment were mislabeled as seconds; the DCB stores a bare minute value.
  Verified against the raw records — no numeric change, only the unit is
  corrected.

## [v0.11.0] - 2026-06-03

### Added

- **`sc-locations`: new crate — typed surface over `StarMapObject` records.**
  The universe's stable places (2,054 in LIVE: systems, planets, moons, landing
  zones, rest stops, stations, outposts, mining claims), the canonical "record
  type with no typed domain crate" getting one.

  - **`Location`** — materialized struct: localized `name`/`description`/
    `callouts` keys, typed `LocationKind` category, `nav_icon`, `respawn`,
    hierarchy `parent`, `jurisdiction`.
  - **`LocationKind`** — typed category enum (21 variants from
    `StarMapObjectType.name` + `Unrecognized` fallback), `from_dcb_str` /
    `as_dcb_str`. The category is the StarMapObjectType, not the nav icon (e.g.
    Levski is `Manmade`, not `LandingZone`).
  - **`Locations`** — index by GUID **and by class-CRC**:
    `by_crc(crc) -> Option<&Location>` / `guid_by_crc` resolve an EntityGraph
    `subject_id` to a typed location (verified: `3723364946` → "Levski").
    Hierarchy via `parent_of` / `children_of` / `ancestors` (cycle-guarded).
    Derived indices rebuild on deserialize (no snapshot bloat).
  - Whole feature closure is one flag: `sc-extract[starmap]`. Wired into
    `Foundations` + `HolotableSnapshot` (cook version 2 → 3) and the
    `sc-holotable` prelude (`Location`, `LocationKind`, `Locations`).

- **`sc-extract`: class-CRC resolution (`class_crc` + `CrcIndex`).** Star
  Citizen's EntityGraph gRPC service identifies records on the wire by a u32
  CRC (`class_guid_crc`, `guid_hash_crc`, resource-descriptor `resource_id`)
  rather than the full GUID. The CRC is `crc32c(guid.as_bytes())` — CRC32C over
  the GUID's 16 storage-order bytes (reverse-engineered, verified byte-exact
  against live `(guid, crc)` pairs).

  - **`class_crc(&Guid) -> u32`** — the forward function.
  - **`CrcIndex`** — reverse `crc → guid` over the **whole DCB record set**,
    built from a `RecordPaths` (`from_paths`, recommended) or a live `Datacore`
    (`from_datacore`). Derived data, not serialized: rebuilt on demand from the
    already-snapshotted `RecordPaths`, so it adds nothing to snapshot size.
    Collisions are logged (`warn`) but tolerated. Both are re-exported from the
    `sc-holotable` prelude.

- **`sc-items`: item-scoped class-CRC lookups.** `Items` now carries a reverse
  `class_crc → Item`/`Guid` index — `Items::guid_by_crc` and `Items::by_crc`
  (the latter returns `&Item`, which the global `CrcIndex` can't). The forward
  `sc_items::class_crc` is re-exported from `sc-extract`. The reverse index is
  rebuilt on deserialize, so it doesn't bloat the `Items` snapshot.

### Fixed

- **`sc-crafting`: two `redundant_closure` clippy lints** (`.map(|e|
  build_effect_kind(e))` → `.map(build_effect_kind)`), surfaced by the rust 1.94
  clippy. Internal only — no behavior change.

## [v0.10.0] - 2026-05-31

### Added

- **`sc-items`: `catalog` module + two-tier `ItemCatalog` index.** A
  gear catalog that recovers the design ↔ model ↔ colorway relationship
  CIG doesn't model cleanly, in two tiers so a UI can render either a
  whole matching design or a single item with its colorways:

  - **`Collection`** — the models that read as the **same design**:
    "Geist Armor" helmet + arms + core + legs; a gun with its magazine.
    Carries a human `name` ("Geist Armor") and its member model ids.
  - **`Model`** — one design + one slot: "Geist Armor Helmet" and all
    its colorways; "LH86 Pistol" and its paints; "LH86 Magazine" on its
    own. Carries `base: Guid` + `members: Vec<Guid>` (base first — the
    plain "design + slot" item, else a "…Base" default, else CIG's
    canonical first colorway `…_01_01_01`, else shortest name),
    `item_type`/`item_sub_type`, and an optional `collection` link.

  Classification is driven by the **display name** — the signal users
  actually read — not by CIG's `Armor / FPS / Set / …` tag, which proved
  unreliable (one set tag bundles visually-unrelated models: the
  `FieldRecon` tag mixes Geist, Field Recon, and FBL-8a helmets). The
  design name is the base display name's words up to the slot noun
  (Helmet/Arms/Core/…, known from `item_type`): "Geist Armor Helmet" →
  "Geist Armor". Colorways trail *after* the slot noun, so they fold
  into one model regardless of how the colorway or underlying record is
  named. A `Model` is `(design, item_type, item_sub_type, size, grade)`;
  a `Collection` is `(category, design)`. The `size`/`grade` split (from
  `SItemDefinition`, now surfaced on `Item`) keeps distinct ship-weapon
  size classes apart — "Deadbolt I Cannon" (S1) and "Deadbolt V Cannon"
  (S5) are separate models under one "Deadbolt" collection — robustly,
  where parsing the display numeral would fail ("Omnisky IX" is S3).
  Weapons/clothing have no slot noun, so their design is the leading word
  (grouping a gun with its magazine, or a cannon's whole size ladder).

  Scope: a **gear** catalog (armor, clothing, weapons), built over
  [`Items`] and restricted to real inventory items
  (`is_inventory_item()`). Excluded: non-inventory attachables (NPC
  archetypes, seat-access, doors, tattoos); non-gear items (ship
  components, world props); and dev-template items with placeholder
  names (`<= PLACEHOLDER =>`).

  API:
  ```rust
  let catalog = ItemCatalog::build(&items, &paths, &locale);
  catalog.model_of(guid)              // -> Option<&Model>      (item + colorways)
  catalog.collection_of(guid)         // -> Option<&Collection> (the design)
  catalog.models_in(collection)       // iterate a design's models
  catalog.members_of_collection(col)  // iterate every member of a design
  catalog.base_of(guid)               // -> Option<Guid>
  model.variants() / model.is_solo()
  ```

  Verified against live SC DCB (SC 1.0): the polluted `FieldRecon` tag
  dissolves into the "Geist Armor", "Field Recon Suit", and "FBL-8a"
  designs a user would actually name; mixed-type models and
  non-inventory models are both 0. Classification needs only `&Items` +
  `&LocaleMap` — no tag tree, so sc-items no longer depends on `sc-tags`
  (`&RecordPaths` is used only to pick a model's base/header). serde-clean
  — round-trips through `ProcessedSnapshot`.

## [v0.9.0] - 2026-05-31

### Added

- **New crate `sc-resources`** — typed catalog over the DCB's
  `ResourceType` records (`libs/foundry/records/resourcetypedatabase/`).
  Surfaces `Resources::build(&RecordStore)` + GUID lookup +
  `refined_version_of()` graph walk + `RecordVisitor` impl for bundled
  walks. `Resource` carries display name + description, thumbnail
  paths, RTT entity ref, `refined_version` (the raw→refined edge),
  `validate_default_cargo_box`, `density: Option<Density>`, and
  `volatility: Option<Volatility>`. SC 4.8: 206 records, 30 refining
  edges, 205 resolve names, 206/206 carry density with physically
  grounded values (Tungsten Ore = 19,300 kg/m³ ≈ real Tungsten).
  Plus the shared `CargoQuantity` primitive (Standard/Centi/Micro
  variants with `.to_scu()` normalization) and `DensityUnit`
  (GramsPerCm3 with `.to_kg_per_m3()`).

- **`sc-crafting` future-proofing pass — full recipe surface.** Owns
  the whole `libs/foundry/records/crafting/` subsystem (minus the
  legacy salvage/repair recipes). 8 typed surfaces, every polymorphic
  enum carries an `Other { type_name, struct_index }` fallback for
  dormant variants so future regen-after-population promotes them
  cleanly. Live-validated end-to-end against SC 4.8 LIVE.
  - **`Blueprints`** index + `Blueprint { category, process, tiers }`
    with by_record_guid / by_category / by_crafted_entity reverse
    lookups. `Process { Creation { entity_class } | Other }`. Tier
    carries `recipe: Option<Recipe>` + `research: Option<Research>`.
    Sample: P4-AR Rifle resolves to 150s craft time + Aluminum 0.04
    SCU + Hephaestanite 0.02 SCU + Iron 0.02 SCU.
  - **`Duration { days, hours, minutes, seconds }`** projected from
    `TimeValue_Partitioned` (the only populated `TimeValue_*` in 4.8).
  - **`Recipe { craft_time, costs, results, is_shared }`** with
    `RecipeCosts { mandatory, optional }` and the recursive
    `Cost { Resource(ResourceCost) | Item(ItemCost) | Select { count,
    options } | Other }` tree. SC 4.8 universally shapes mandatory
    as `Select(Select(Resource))`.
  - **`ResourceCost { resource: Guid, quantity: Option<CargoQuantity>,
    min_quality }`** and `ItemCost { entity_class, quantity, min_quality }`.
  - **`RecipeResult { Item | Resource | Other }`** — Vec preserved
    even though SC 4.8 results are universally empty (Creation's
    `entity_class` IS the output).
  - **`Research { unlock, costs }`** + `ResearchUnlock { Default |
    Other }`. Present-but-empty on 57% of tiers.
  - **`Categories`** — 20 marker-record categories (FPSWeapons /
    FPSArmours / VehicleWeaponsS1-6 / Medical / Refining/Dismantle
    Examples / …) + database_guid, built from `RecordPaths`.
  - **`GlobalParams`** singleton — refining_quality_unit_multiplier,
    default_composition_quality, dismantle blacklists, and
    `default_blueprint_whitelist` (the 9 default-unlocked blueprints
    at character start: basic dismantle, P4-AR, behr_pistol, light
    combat armor parts, ammo magazines).
  - **`GameplayProperties`** (29 records) — `GameplayProperty {
    property_name_key, unit_format_key, display_transformation,
    name_overrides }` with `DisplayTransformation { Scale { factor }
    | ConvertFactorToPercentChange | ConvertFactorToNegatedPercentChange
    | ConvertValueToFactorOfBaseValue | Sequence(Vec<Self>) | Other }`
    (recursive). `PropertyNameOverride` with `OverrideCondition::ItemType
    { match_item_types: Vec<EItemType>, match_sub_types: Vec<EItemSubType> }`
    via the enum_serde DCB-string adapter pattern.
  - **`Quality`** — standalone records under `qualitydistribution/`,
    `qualitylocationoverride/`, `qualityquantization/`. 10 / 12 / 38
    in SC 4.8. `QualityDistributionShape::Normal { min, max, mean,
    stddev }`, `LocationOverrideEntry { location, distribution:
    Option<DistributionRef> }` where `DistributionRef { Inline |
    Record(Guid) | Other }` captures both inline and shared-by-RecordRef
    forms, and `QuantizationBand { start, end, mapped_value }` maps
    quality ranges to discrete outputs. Per-resource quality data
    (also reachable via `ResourceType.properties` → `ResourceTypeCraftingData`)
    has been confirmed independent of the standalone records (134/134
    location-override entries inline, 0 RecordRef cross-links).
  - **`cargo_quantity_from_ptr`** projection helper. Lives here, not
    sc-resources, because the Centi/Micro pool types are gated under
    the `crafting` feature.
  - **`BlueprintsBuilder`** implementing `RecordVisitor` for bundled
    walks (interest = `["CraftingBlueprintRecord"]`).

- **Umbrella `sc-holotable` integration.** New `resources` feature,
  `resources` module re-exporting `sc_resources::*`. `crafting` now
  depends on `resources` transitively. Prelude grows the full
  sc-resources + sc-crafting type surface.

### Changed

- **`Foundations` grows a `resources: Resources` field.**
  `build_foundations` is now a 5-builder bundled `all_records` pass
  (was 4). `RecordPaths` still declares `AllRecords` so the walk shape
  doesn't change — Resources rides along for free.

- **`HolotableSnapshot` grows `resources: Option<Resources>`** and
  bumps `HOLOTABLE_COOK_VERSION` from 1 to 2. Old snapshots fall back
  per the `ProcessedSnapshot` version-guard machinery
  (rebuild from raw / live).

### Removed (breaking)

- **`sc_crafting::BlueprintItem`** and the free functions
  **`sc_crafting::all_blueprints` / `sc_crafting::resolve_blueprint`**.
  Use [`Blueprints::build`] + [`Blueprints::get`] + [`Blueprints::iter`]
  instead — the new types carry everything the old shims did, plus the
  full recipe surface. `sc-missions` re-exports of the same names also
  dropped; `sc_missions::BlueprintPoolEntry.blueprint` is now a
  `sc_crafting::Blueprint`.

### Fixed

- `holotable_snapshot_round_trip` test built `HolotableSnapshot`
  without its `items` field — was masked because workspace
  `cargo check --all-targets` runs with the umbrella's empty default
  features, so the test target never compiled.

## [v0.8.0] - 2026-05-29

### Changed (breaking)

- **Uniform API shape: domain-noun cooked indexes** (new rules 6–7 in
  `docs/workspace-structure.md`). Each data crate exposes one primary type
  named for its domain, built with `Type::build`, shape via methods, grouping
  folded in as a field. Renames: `ItemCache`→`Items`, `TagTree`→`Tags`,
  `ManufacturerRegistry`→`Manufacturers`, `MissionIndex`→`Missions`, and the
  sc-missions sub-types `ShipRegistry`/`LocationRegistry`/`LocalityRegistry`/
  `RewardCurrencyCatalog`/`BlueprintPoolRegistry` →
  `Ships`/`Locations`/`Localities`/`RewardCurrencies`/`BlueprintPools`. Each
  index also exposes a `{Type}Builder` for bundled walks.
- **Foundational builders take `&RecordStore`** (narrowest sufficient input):
  `Items`/`Tags`/`Manufacturers::build(&RecordStore)`. Builders needing the raw
  db (`RecordPaths`) or asset data keep `&Datacore`; dependency indices are
  appended by reference.
- **`sc-weapons`: `build_weapon_pools` (tuple) → `Weapons`** —
  `Weapons { ships, fps, missiles, pools }` via `Weapons::build(&Datacore,
  &Items)`. `WeaponPools` is now the `pools` field; `iter_*` remain for
  streaming.
- **Crate rename `sc-contracts` → `sc-missions`.**
- **Crate rename `sc-installs` → `sc-discovery`** — the `install` substring
  tripped Windows installer-detection (UAC), blocking the test binary from
  launching. The crate discovers/locates the install; it doesn't install.

### Removed

- **`DatacoreConfig`** — dissolved; cooked indices are explicit `X::build`
  calls in their owning crates.
- **`DatacoreSnapshot`, `Datacore::snapshot()`, `Datacore::into_snapshot()`** —
  `Datacore` owns a `RecordStore` directly; use `Datacore::records()`.

### Added

- **`RecordPaths`** (sc-extract) — every record's file path/name/type by GUID
  plus a `/`-segment trie (`get`/`at`/`under`/`children`/`roots`). The DCB path
  is a classification axis (e.g. manufacturer kind).
- **Bundled-walk API** (sc-extract) — `RecordVisitor`/`Interest`/`VisitItem`/
  `BundledWalk` + tuple `VisitorSet`. Build several indices in one
  `all_records` pass; `X::build` stays as the single-index path.
- **Generic `ProcessedSnapshot<T>`** (sc-extract) — serialize a cooked index
  (zstd + msgpack, envelope + cook-schema version guard) for sub-second load
  vs the raw snapshot's full re-parse. All four cooked indices are
  serde-capable (`LocaleKey` gains serde; the generated `EItemType`/
  `EItemSubType` round-trip as DCB strings via a new generator-emitted
  `as_dcb_str`).
- **`sc-holotable` umbrella crate** — feature-gated re-exports + `prelude`,
  `build_foundations` (one bundled pass over all foundational indices), and
  `HolotableSnapshot` (a serializable bundle of the cooked indices). The
  recommended public dependency.

## [v0.7.0] - 2026-05-28

### Added

- **`sc-extract`: item classification on `LocalizedItem` / `LocalizedItemCache`.**
  The per-entity cache now also captures the raw `AttachDef.Type` and
  `AttachDef.SubType` strings (e.g. `"Char_Armor_Helmet"`, `"WeaponPersonal"`)
  alongside the existing locale keys — they come from the same
  `SAttachableComponentParams.AttachDef` (`SItemDefinition`) walk, so no extra
  pass. New `LocalizedItem.item_type` / `.item_sub_type` fields and
  `LocalizedItemCache::item_type(guid)` / `::item_sub_type(guid)` accessors.
  Values are the DCB enum-value names verbatim, leaving category mapping to
  consumers. Hearth uses this to group its blueprint catalog by item type.
- The localization walk now tolerates a missing `Localization` block: an
  entity with a `Type` but no localized name is still cached (previously it
  was skipped entirely). Existing name-resolution behaviour is unchanged for
  entities that do have `Localization`.

## [v0.6.0] - 2026-05-28

### Added

- **`sc-discovery`: `Installation::platform_id: Option<String>`.** Authoritative
  `'prod'` / `'ptu'` tag from the launcher store, populated during
  `discover()` and `discover_default()`. Lets consumers route to the right
  services endpoint or scope personal state by environment without re-mapping
  from `Channel`. `None` when discovery fell back to log parsing (the log
  doesn't carry this field) or when the install was constructed via
  `Installation::from_root` / `from_parts`. Hearth uses this to keep
  PTU progress out of PU tables; downstream consumers may find it useful
  for any prod-vs-test split. Already exposed on the lower-level
  `StoreInstall`; this just plumbs it to the public `Installation`.

- **`sc-discovery`: `read_identity()` / `read_identity_from()`.** Opt-in API
  exposing the currently-logged-in RSI handle from the launcher store's
  `identity` block. New type: `LauncherIdentity { handle: String }`.
  Only the handle is exposed — other identity fields (email, Heap account
  ID, tracking UUIDs) are either PII or already derivable elsewhere, and
  stay internal. New error variant `Error::LauncherIdentityMissing`
  covers the "store decrypted but no nickname" case; callers can fall
  back to log parsing (`Handle[…]` in `<Legacy login response>` lines).
  Consumers should treat the value as a single-point-in-time snapshot:
  switching RSI accounts in the launcher overwrites it, so multi-account
  discovery is fundamentally log-based.

## [v0.5.0] - 2026-05-26

### Changed (breaking)

- **`sc-missions`: `MissionRewards.blueprint: Option<BlueprintReward>` → `blueprints: Vec<BlueprintReward>`.**
  Multi-pool missions in the DCB store multiple `BlueprintRewards`
  entries side-by-side in their `contractResults`; the old `Option`
  shape combined with an early `return` in `resolve_blueprint_reward`
  silently dropped every pool past the first. The new `Vec` shape
  preserves all entries. Field rename is intentional — every
  consumer's call site breaks at compile time, making the migration
  auditable rather than a silent semantic shift.
- **`sc-missions`: `BlueprintReward` slimmed to `{ chance, pool_guid }`.**
  Dropped the duplicated `pool_name` and cloned `items: Vec<BlueprintItem>`
  that previously lived on each reward. Consumers resolve the pool's
  name and items via [`BlueprintPools::get(pool_guid)`] at
  render time. Single source of truth for pool contents; no more
  silent drift between the registry and the mission's snapshot.
- **`sc-missions`: `expand_all` no longer takes `&BlueprintPools`.**
  Now that `materialise_blueprint` doesn't need to resolve items at
  build time, the param is dropped from the entire `expand_all` /
  `walk_handler` / `emit_*` / `build_expansion` chain. `Missions::build`
  populates the reverse index after expansion via the new
  `link_missions` call.

### Added

- **`sc-missions`: reverse-index lookups on `BlueprintPools`.**
  New methods unlock pool → missions and item → missions queries:
  - `missions_for_pool(pool_guid) -> &[Guid]` — mission GUIDs that
    award the given blueprint pool. Populated by `Missions::build`
    via the new `link_missions(&[Mission])` after `expand_all`.
  - `pools_containing_item(blueprint_record_guid) -> Vec<&BlueprintPool>` —
    every pool that contains the given blueprint record as one of
    its items.
  - `missions_for_item(blueprint_record_guid) -> Vec<Guid>` —
    convenience composition of the above two, dedup'd. Powers
    consumer queries like "missions that drop blueprint X" and
    "missions that drop blueprints I don't own yet" (the cross-
    domain query [`bulkhead`](https://github.com/VeeLume/bulkhead)
    needs once an inventory tracker lands).
- **`sc-extract`: `strip_locale_metadata` helper + universal normalization
  at parse / set time.** CIG ships some entries with a `,P` metadata
  suffix (probably *plural* or *pronoun-aware*) on weapon variant
  names — observed on ~12,873 entries in the SC 4.8 LIVE locale:
  ```
  item_Nameutfl_crossbow_ballistic_01_tint01,P=Novian "Nighthunter" Crossbow
  ```
  DCB references use the bare key, so the suffix silently broke
  lookup for every affected variant. Now stripped at every locale
  construction point: `LocaleMap::parse`, `LocaleMap::parse_utf8_bom`,
  `LocaleMap::set`. New `pub fn strip_locale_metadata(&str) -> &str`
  exposed for downstream consumers (e.g. sc-langpatch's INI parsers)
  so the same normalization can apply at their boundaries too.
- **`sc-missions`: `blueprint_item_probe` example.** Dumps every
  `BlueprintItem` in a matching mission's blueprint pools with full
  DCB metadata — record names, entity record + struct type,
  `LocalizedItemCache` entries, every component on the crafted entity
  + any `Localization` blocks they carry. Diagnostic tool for
  investigating display-name resolution gaps. Built during the locale
  `,P` investigation and kept around for future digging.

### Fixed

- **`sc-missions`: `blueprint_pool_consistent` switched to per-set
  equality.** With `MissionRewards.blueprints` now a `Vec`, the
  old "all members agree on one pool guid" check needs to compare
  the *set* of pool guids across members. Members A {X, Y} and B {X, Y}
  are consistent; A {X, Y} and C {X, Z} are not.

### Internal

- `cargo fmt` pass across `crates/sc-missions/src/axes.rs` and
  ~8 example files. Pure whitespace reflow, no semantic change.

## [v0.4.0] - 2026-05-24

### Added

- **`sc-missions`: tag-axis classifier.** New `axes` module exposes
  `AxisKind`, `AxisValues`, `AxisDiff`, `SharedTag`. Classifies tags
  by their position in the `Tags` (`AI/Ship/CombatClass/*`,
  `AI/SkillDefinitions/*`, `Missions/VehicleType/Ship/*`,
  `Ship/Model/*`, `EntityEffectSystem/Tags/*`, etc.) into a small
  set of player-meaningful families. Renderers use this to decide
  which axes to collapse silently (skill, combat-class agreement)
  vs surface as per-alternative variance (hull, ship class, effects
  like Distortion).

- **`Mission::combat_class()` / `Mission::ship_count_range()`.**
  `combat_class` returns the CombatClass tag shared by every ship-spawn
  alternative across the mission (e.g. `"VeryEasy"`), or `None` when
  groups disagree or have no CombatClass tag. `ship_count_range`
  returns the `(min, max)` total ship count summed across all
  ship/entity slot groups — honest min from picking the smallest
  alternative in each group, honest max from the largest.

### Changed (breaking)

- **`sc-missions`: `EncounterPhase` reshaped to preserve
  `ShipOptions` alternatives.** Old shape `EncounterPhase { name,
  slots: Vec<S> }` flattened the three-level DCB nesting
  (`ShipGroup → ShipOptions → Ship`) and lost the boundary between
  "concurrent slot groups" (all fire) and "weighted alternatives
  inside one group" (engine picks one). New shape:
  `EncounterPhase { name, groups: Vec<SlotGroup<S>> }`, with
  `SlotGroup { options, concurrent_range, weight_uniform, axes,
  shared_tags }`. Each `SlotGroup` corresponds to one
  `SpawnDescription_ShipOptions` and carries pre-computed
  `AxisDiff` + `shared_tags` (with cached `AxisKind`) so consumers
  don't have to re-walk the tag tree.
  - Fixes ~14% of ship-spawn `MissionProperty` displays that were
    over-counting by summing concurrent across alternatives (the
    "Settle a Score shows 6× Scythe instead of 1-3" pattern). See
    the investigation thread in chat history for the full census.
  - **Migration:** read access patterns like `phase.slots.iter()` /
    `phase.slots.len()` become `phase.all_options()` /
    `phase.option_count()` for "flatten everything" consumers, or
    `phase.groups.iter().flat_map(|g| g.options.iter())` for
    consumers that want the explicit grouping. Renderers should
    iterate `phase.groups` directly to respect the alternatives
    boundary.

## [v0.3.0] - 2026-05-03

### Added

- **`sc-discovery`: launcher-store discovery.** New module `launcher_store`
  reads `%APPDATA%/rsilauncher/launcher store.json` (electron-store /
  AES-256-CBC) for an authoritative list of installed channels — works
  even for channels never launched. Encryption key is extracted at
  runtime from the launcher's own `app.asar` (no embedded secret).
  - New `discover_default()` returns the channel from the launcher's
    `library.defaults[]` — better "default install" UX than
    `discover_primary` (hardcoded LIVE-first).
  - `discover()` and `discover_primary()` now go through the store
    first, fall back to log parsing on any failure.
  - New types: `StoreInstall`, `StoreSnapshot`. New free functions:
    `read_launcher_store`, `read_launcher_snapshot`, plus `_from`
    siblings.
  - New deps: `aes`, `cbc`, `pbkdf2`, `sha2` (pure-rust, no DPAPI).
  - See `docs/launcher-store.md` for the full launcher-store reference.

- **`sc-discovery`: broader log parsing.** Recognises `[Installer]` and
  `Deleting <root>\loginData.json` markers in addition to
  `[Launcher::launch]`, so log-fallback discovery finds channels that
  have been installed but never launched. New `LogEntry` /
  `LogEntryKind` types; `parse_launcher_log_entries` returns
  `Vec<LogEntry>`.

- **`Channel::install_dir_name()`** — on-disk directory name for a
  channel (notably `TECH-PREVIEW`, distinct from the display name `TECH`).

### Changed (breaking)

- **`sc-discovery`: `Installation::launcher_version_string()` removed.**
  Replaced with two explicit alternatives because the old auto-derivation
  was silently off-by-patch-number once any hotfix shipped on top of an
  X.Y.0 branch (the manifest's `Branch` field doesn't roll forward):
  - `Installation::launcher_version_label: Option<String>` (new public
    field) — authoritative store-provided label, e.g.
    `"4.7.2-live.11715810"`. Set by store-using discovery paths; `None`
    when discovery fell back to log parsing.
  - `Installation::launcher_version_string_derived()` (renamed) —
    locally derived from manifest fields; carries an explicit staleness
    caveat. Consumers that want fallback semantics must opt in
    explicitly.

  `sc-generator` now uses `launcher_version_label` exclusively and
  refuses to fall back to derivation, so a wrong tag like
  `datacore/4.7.0-live.X` for a 4.7.2 build can't slip through. The
  regen script gets this for free.

- **`sc-discovery`: `Error::NoLaunchEntries` renamed** to
  `Error::NoInstallEntries` — the error now also covers logs that have
  no `Installer` markers, not just no launch markers.

- **`sc-discovery`: `parse_launcher_log_entries` return type changed**
  from `Vec<(Channel, PathBuf)>` to `Vec<LogEntry>`.

- **`sc-discovery`: legacy plain-text manifest parsing removed.** Only
  the v2 nested `{"Data": {...}}` shape is parsed now. The legacy
  flat-format support was code path debt — every shipped Star Citizen
  build the workspace targets uses the v2 shape.

## [v0.2.0] - 2026-05-02

### Changed (breaking)

- **`sc-missions` v2 redesign — Mission-centric API.** Wholesale
  rename and reshape of the public surface; every consumer needs
  updates. Design doc: `docs/sc-missions-v2.md`; consumer guide:
  `docs/sc-missions-guide.md`.
  - `Contract` → `Mission`, `ContractIndex` → `Missions`,
    `EncounterWave` → `EncounterPhase` (now generic
    `EncounterPhase<S>`).
  - `MissionOrigin` consolidates the previously-flat handler fields
    (`MissionContractHandler` / `RandomMissionParams` / variation
    metadata) into one enum.
  - `MissionRewards` collapses six previously-flat reward fields
    (UEC, REC, reputation, …) into one struct.
  - `EncounterSlot` carries symmetric `TagBag`s (positive / negative
    / markup / entity) instead of the v1 ad-hoc tag plumbing.
  - `Encounter` enum replaces v1's ship-only encounter type. Variants
    `Ships` / `Npcs` / `Entities` / `Unknown` widen NPC and entity
    coverage; NPC slots expose `mission_allied_marker` (48 slots on
    SC 4.7), `is_critical`, `faction_override`.
  - Implicit BP merge step removed; `Variation`, `title_siblings`,
    and `find_bp_conflicts` deleted. Consumers walking the
    inheritance chain manually should switch to `Missions` /
    `MissionPools`.

### Added

- **`MissionPools`** precomputed on the index, with opt-in
  divergence helpers for consumers that need to reason about per-BP
  pool drift without re-walking the graph themselves.
- **`Ships::resolve_spawn`** — tag-tree subsumption + spawn-
  state filter. Recovered 80 of 335 previously-empty ship-encounter
  slots on SC 4.7 LIVE (24% drop). Three coupled bugs fixed: ancestor
  tag matching, `AI > Ship > SpawnFlags` / `AI > CargoManifest` state
  tags being misread as identity filters, and the dual-`Ship`-node
  intent gate over-firing. See `status.md` "Last worked on" for the
  data-driven derivation.
- **Narrow-consumer re-exports** on `sc-missions` and `sc-weapons`
  (`EntityClassDefinition`, locale-key cluster API) so callers don't
  have to depend on `sc-extract` directly for common types.
- **`tools/sc-explorer`** — interactive TUI binary with three tabs
  (Pools / Contracts / Weapons). Per-crate `tui` modules
  (`sc-missions/src/tui`, `sc-weapons/src/tui`) own their domain
  views behind a `tui` feature.
- **Investigation examples** (committed under
  `crates/sc-missions/examples/`) — `contract_dump`, `ambush_dig`,
  `damage_dig`, `salvage_pool`, `spawn_dig`, `encounter_analytics`,
  `encounter_kinds`, `role_investigation`, `tier_investigation`.
  Canonical way to run a quick dig against a fresh DCB regen.

### Fixed

- `EncounterSlot` now forwards typed signals on ship-spawn entries
  (previously dropped during extraction).
- TUI encounter-detail view surfaces the forwarded typed signals.

## [v0.1.0] - 2026-04-25

### Added

- `sc_missions::Contract` and `sc_missions::Variation` now carry
  `title_key: Option<LocaleKey>` and `description_key: Option<LocaleKey>`
  alongside the resolved `title` / `description` strings. Consumers
  patching `global.ini` (sc-langpatch, translation-extraction tooling)
  no longer need to re-walk the contract inheritance chain to recover
  the raw INI key the displayed text was resolved from.
- `sc_missions::ResolvedText` gained the same `title_key` /
  `description_key` fields; `resolve_contract_text` fills them during
  the existing inheritance walk at no extra cost.
- `sc_missions::ExpandedContract` propagates the keys so
  pre-merge consumers see them too.

### Changed

- `ResolvedText` is still constructible via field-init but now requires
  four fields instead of two. Call sites that built it manually should
  use `..Default::default()` to stay forward-compatible.
