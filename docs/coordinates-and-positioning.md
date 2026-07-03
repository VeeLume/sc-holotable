# Coordinates & positioning — the frame model behind `Place::global_position`

Status: **investigation notes** (started 2026-07-01). Explains the Star Citizen
coordinate system as it appears in *our* data, characterizes exactly what
[`crate::Universe`]'s `global_position` computes (and its error), and records the
cross-checks against community datasets. Companion to
[`object-containers.md`](object-containers.md) (the placement-graph mechanics) —
this doc is about the *coordinate math*, not the socpak walk.

Sources triangulated: the community "Coordinates" guide (frames + `/showlocation`),
Murphy's Navigation Tool (the reverse-engineered time-driven model), the
starmap.space POI API, and a direct sweep of our LIVE 4.8 socpak/DCB data
(`examples/position_audit.rs`, `examples/objcontainer_positions.rs --diag`).

## 1. The layered-frame model

A point in the 'verse is expressed through a chain of nested frames:

| Frame | Origin / axes | Static or moving | Units |
|---|---|---|---|
| **System-global** (absolute) | origin near the system barycentre; **Z = "up"** (the star sits at large +Z, most bodies at Z ≈ 0 — the ecliptic) | absolute; this is what the `/showlocation` chat command copies | **metres** |
| **Object container** (planet / moon / Lagrange point / station) | body centre; its own axes | **position orbits** the star (slow); **orientation spins** (planet day/night) — both time-dependent | — |
| **Surface-local** (an outpost / cave / facility on a body) | body-local; expressed as lat/long/height *or* local XYZ (‖v‖ ≈ body radius) | static *within* its body | **km** (community) |

The composition, in full, is:

```
global(P, t) = body_global(t)  +  R_body(t) · local_pos(P)
```

- `body_global(t)` — the body centre in system coordinates. Bodies orbit, so this
  is technically a function of time, but the orbit is slow and the authored value
  is a fixed snapshot (see §4).
- `R_body(t)` — the body's orientation at time `t`. Planets spin (day/night), so
  for a **surface** point this rotation is the dominant time-dependent term.
- `local_pos(P)` — the point's fixed offset in the body frame.

### Coordinate conventions worth pinning down

- **Units**: system-global is **metres**; community tools (Murphy, starmap.space)
  quote body-local in **km**. Murphy's system cartesian is **km** → `× 1000` gives
  our metres (verified: Murphy Stanton-star Z = `2 923 350` km ⇒ `2.92335e9` m,
  matching the guide's `/showlocation` `Z = 2 923 345 368`).
- **Z is up.** The star is at large +Z above the ecliptic; "up/down" in nav chatter
  = sign of Z.
- **Orbital markers (OM-1..6)** sit at **±OM_radius along the body-local X/Y/Z
  axes** — this falls straight out of Murphy's OM-distance formulas
  `√(x² + y² + (z ± OM_radius)²)` and permutations. OM-1 = +Z ("up"/North),
  OM-2 = −Z (South), the rest are the four equatorial axis points. Each body has
  its own `OM_radius` and `body_radius`.
- **Time**: the UEE game clock runs at **6× real time**
  (Murphy: `UEE = 383507 + (real − 43831) × 6`). It drives both orbit and spin.

## 2. What *our* data actually carries

Harvested from the placement entities in `*system.socpak`
(`examples/objcontainer_positions.rs --diag`, `examples/position_audit.rs`):

- **`Entity::Pos`** — a 3-vector in **metres**. For a **system-OC placement**
  (planet, moon, Lagrange station, asteroid base) this is **already
  system-global**. Verified: an asteroid base near Daymar has
  `Pos = (-1.8923e10, -2.6003e9, 0)`, essentially the guide's Daymar
  `/showlocation` sample `(-18 930 267 227, -2 610 218 512, …)`.
- **`Entity::RelativePos`** — a position relative to the orbit parent (present on
  ~25% of placements).
- **`EntityComponentOrbit`** — `OrbitalRadius`, `OrbitalAngle`, `parentGUID`. The
  orbit of the container about its parent.
- **`Entity::Radius`** — the container radius.
- For **sub-body placements** (surface outposts, in the *body* socpaks) `Pos` is
  **body-local**, not global — it must be composed with the parent body's frame.

### The decisive gap: no body rotation

A rot/quat/ang attribute sweep over all 3 system socpaks finds **only**
`Entity::Rotate` (198×, on *stations/props* — not bodies) and
`EntityComponentOrbit::OrbitalAngle` (104×). **Every celestial-body entity
carries just `Pos` + `Radius` + orbit** — no spin quaternion and no rotation rate.
The DCB `starmap` surface's `rotationSpeed` fields belong to
`StarMapMissionObject` / `StarMapPartyMemberObject` (HUD-marker spin, with
`facingMode` + `minimumDisplaySize`) — **not** planetary rotation.
`PlanetDayNightTemperatureParams` models temperature-vs-daytime but carries no
rate either.

**⇒ `R_body(t)` is not recoverable from our data.** This is why community tools
(Murphy's "Adjustment Log") **triangulate the rotation phase ("noon") empirically**
from in-game measurements rather than reading it from files.

## 3. What `global_position` computes — and its exact error

[`Universe::global_position`](../crates/sc-locations/src/object_containers.rs)
sums `Pos` up the containment chain and stops at the first `*system.socpak`
placement, using **R = identity** and **ignoring orbit**. Concretely:

- **Bodies / moons / stations placed at system level: correct.** Their `Pos`
  *is* the system-global coordinate; no summation or rotation is involved.
- **Surface points (sub-body placements): body-local added un-rotated.** Missing
  `R_body(t)` means the error is the *rotated* vs *un-rotated* local offset —
  bounded by a **body radius** (hundreds of km) and, worse, **time-dependent**
  (the point sweeps a body-radius circle over one planetary day).

This is exactly right for **ordering stops by distance / by which body they sit
at** (the cargo-routing use case) and wrong for **navigation**.

### Exact vs. approximate — a consumer's cheat-sheet

The missing term is the body's orientation `R_body(t)`. The decisive fact is that
`R_body` is a **rotation** — an isometry — so it **cancels in any quantity that
compares two points in the *same* body frame**. Anything that needs a point's
*absolute* placement, or compares across two *different* body frames, carries the
rotation and is only approximate.

| Query | Exactness | Why |
|---|---|---|
| Global position of a body / moon / station | **Exact** | its `Pos` is authored system-global; no rotation involved |
| Global position of a single **surface** point | **± ~body radius, time-dependent** | needs `R_body(t)`, which we don't have — the point is unknown on a sphere of radius `‖local‖` |
| Chord (straight-line) distance between two surface points on the **same body** | **Exact** | `‖R·(l₁−l₂)‖ = ‖l₁−l₂‖`; rotation cancels |
| Great-circle (surface) distance, same body | **Exact** | `body_radius · acos(l₁·l₂ / (‖l₁‖‖l₂‖))` — uses only the local vectors |
| Bearing / heading between two surface points, same body | **Exact** | angle is computed in the shared local frame |
| Distance from a surface point to its OMs | **Exact** | OMs are at `±OM_radius` on the body-local axes; all in one frame |
| Body-fixed latitude / longitude of a surface point | **Exact** | spherical coords of the local vector (pole = body axis, meridian body-fixed) |
| Sun-relative angle / time-of-day / "where is noon" | **Approximate** | this is exactly the rotation *phase* the files omit (Murphy triangulates it) |
| Distance between surface points on **different bodies** | **Exact for ordering** | each endpoint carries ± body-radius error, negligible vs. inter-body distance (10⁹ m) |

**Rule of thumb:** *within one body* the local frame is fully trustworthy —
distances, bearings, OM geometry, body-fixed lat/long are all exact. Only a
point's *absolute* placement (and hence precise cross-body surface-to-surface
vectors and anything sun-relative) needs the rotation we can't recover.

## 4. Cross-checks against community data

**Murphy's Nav Tool — body positions (✓ structure validated).** Murphy's
per-body cartesian (km) `× 1000` lines up with our system-global `Pos` (m) in
scale and layout (star at +Z, bodies on the ecliptic, moons clustered around
their planet). Exact values drift because their Stanton DB is 4.0/4.2 and ours is
4.8 — CIG re-places bodies across patches. Murphy additionally stores a per-body
**rotation quaternion + rotation speed + rotation adjust (phase)** — precisely the
`R_body(t)` we lack — plus `OM_radius`, `body_radius`, `orbital_radius`.

**starmap.space POI API — surface points (✗ direct GUID join does not land).**
1 885 POIs (Stanton 1289 / Pyro 588 / Nyx 8), 1 824 with a `GUID` and body-local
XYZ (km) + lat/long/height. **0 / 1 824 of those GUIDs resolve to our
`StarMapObject` `Locations`, and 1 822 / 1 824 are not DataForge records at all**
(`db().record()` → `<not a DCB record>`). The POIs (caves, underground
facilities, `HDRSO-*` surface outposts) are a *different population* keyed by a
GUID that is not a DCB record id — most likely the placed entity's CryEngine
`EntityCryGUID` in the *body* socpaks. The join to measure surface-POI error is
therefore an **`EntityCryGUID` join against the socpak placement graph**, not a
DataForge lookup (see `examples/starmap_join.rs`).

**The `EntityCryGUID` join lands, and validates our harvest exactly.**
`starmap_join.rs` walks all 9 526 socpaks, builds `EntityCryGUID → Pos`, and
joins: **1 753 / 1 885 POIs match** (in *cry* byte-order; 0 in std-order — so
starmap.space renders CryEngine GUIDs), and where both carry a `Pos` the vectors
are **byte-identical**: our entity `Pos` (m) equals starmap's `XYZ` (km × 1000)
component-for-component, `Δ = 0` (e.g. MT OpCenter TLI-4 = `[68364, 64276,
-996489]` on both). Two consequences:

1. **Our `Pos` harvest is correct** — independently reproduced, to the metre, by
   starmap.space's separate extraction.
2. **starmap.space stores only body-local `Pos`** (‖v‖ ≈ body radius: ~1 000 898 m
   at microTech's surface, ~295 000 m on a Cellin-class moon) — *the same values
   we already hold*. It has **no** global surface coordinate. Combined with
   Murphy (body-local + triangulated rotation), this confirms **no community
   source solves global-surface positioning** — they all stay body-local and
   recover lat/long via an empirical rotation phase, because the files don't carry
   `R_body(t)`.

The ~132 unmatched are version/coverage drift (removed POIs, Pyro/Nyx entities not
in the current socpaks). The few non-zero `Δ` are duplicate-`EntityCryGUID` clones
(same magnitude, sibling `Pos`) — key by `(socpak, cry)` to disambiguate, as the
`ObjectContainers` graph already does.

## 5. Guidance for consumers

- **Distance ordering / "which body is this at"** — `Place::global_position`
  (self, then `anchor_position` for the parent-body fallback) is sufficient today.
- **Faithful surface navigation** (lat/long, headings, OM-relative) is **not**
  achievable from our data alone: it needs `R_body(t)`, which requires either
  planetary rotation params CIG doesn't ship in the DCB/socpak, or an empirical
  phase calibration like Murphy's. Treat it as out of scope unless a consumer
  brings that rotation model.

## 6. Reproduce

- `examples/position_audit.rs <starmap_pois.json>` — GUID-join coverage,
  unmatched-GUID DCB-type resolution, and the rotation-attribute sweep.
- `examples/objcontainer_positions.rs --diag` / `--dump` — raw placement shape.
- starmap.space POIs: `curl -k https://starmap.space/api/v3/pois/index.php`
  (their TLS chain is incomplete → `-k`).
- Murphy's Navigation Tool (xlsx): open with `openpyxl data_only=True`; body table
  is the `Nav Database` / `Primary` (Stanton) / `Secondary` sheets.
