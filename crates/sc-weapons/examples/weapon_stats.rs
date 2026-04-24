//! Dump all weapon stats from sc-weapons to verify against spviewer.
//!
//! ```bash
//! cargo run -p sc-weapons --release --example weapon_stats
//! cargo run -p sc-weapons --release --example weapon_stats -- --fps
//! cargo run -p sc-weapons --release --example weapon_stats -- --filter "Bulldog"
//!
//! # Phase 3 comparison stats with engagement window (seconds) and
//! # power allocation (units/slot). Sorts ship weapons by effective_dps.
//! cargo run -p sc-weapons --release --example weapon_stats -- \
//!     --window 30 --power 1.0 --sort
//! ```

use std::time::Instant;

use sc_extract::{AssetConfig, AssetData, AssetSource, DatacoreConfig};
use sc_weapons::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let show_fps = args.iter().any(|a| a == "--fps");
    let sort_by_effective = args.iter().any(|a| a == "--sort");
    let filter: Option<&str> = arg_value(&args, "--filter");
    let window: f32 = arg_value(&args, "--window")
        .and_then(|s| s.parse().ok())
        .unwrap_or(30.0);
    let power: Option<f32> = arg_value(&args, "--power").and_then(|s| s.parse().ok());

    let ctx = LoadoutContext {
        window_seconds: window,
        power_per_slot: power,
    };

    let t0 = Instant::now();
    let install = sc_installs::discover_primary()?;
    println!(
        "Found {} v{} at {}",
        install.channel,
        install.short_version(),
        install.root.display()
    );

    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = sc_extract::Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())?;
    let parse_secs = t0.elapsed().as_secs_f64();

    let snap = datacore.snapshot();

    println!(
        "\nLoadoutContext: window={}s, power_per_slot={}",
        ctx.window_seconds,
        ctx.power_per_slot
            .map(|p| format!("{p}"))
            .unwrap_or_else(|| "unconstrained".into())
    );

    if show_fps {
        let weapons: Vec<FpsWeapon> = iter_fps_weapons(&datacore).collect();
        println!(
            "\nFPS weapons: {} (parsed in {parse_secs:.1}s)\n",
            weapons.len()
        );
        for w in &weapons {
            if let Some(f) = filter
                && !w.record_name.contains(f)
                && !snap.display_names.get(&w.guid).unwrap_or("").contains(f)
            {
                continue;
            }
            print_fps_weapon(w, snap);
        }
        if filter.is_none() {
            println!("\nTotal: {} FPS weapons", weapons.len());
        }
    } else {
        let mut weapons: Vec<ShipWeapon> = iter_ship_weapons(&datacore).collect();
        if sort_by_effective {
            weapons.sort_by(|a, b| {
                let ea = a.effective_dps(&ctx).unwrap_or(0.0);
                let eb = b.effective_dps(&ctx).unwrap_or(0.0);
                eb.partial_cmp(&ea).unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        println!(
            "\nShip weapons: {} (parsed in {parse_secs:.1}s)\n",
            weapons.len()
        );
        for w in &weapons {
            if let Some(f) = filter
                && !w.record_name.contains(f)
                && !snap.display_names.get(&w.guid).unwrap_or("").contains(f)
            {
                continue;
            }
            if sort_by_effective {
                print_ship_weapon_summary(w, snap, &ctx);
            } else {
                print_ship_weapon(w, snap, &ctx);
            }
        }
        if filter.is_none() {
            println!("\nTotal: {} ship weapons", weapons.len());
        }
    }

    Ok(())
}

fn arg_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

/// One-line summary used by the `--sort` ranked view.
fn print_ship_weapon_summary(
    w: &ShipWeapon,
    snap: &sc_extract::DatacoreSnapshot,
    ctx: &LoadoutContext,
) {
    let display = snap.display_names.get(&w.guid).unwrap_or("");
    let eff = w.effective_dps(ctx).unwrap_or(0.0);
    let burst = w.burst_dps().unwrap_or(0.0);
    let sust = w.sustained_dps().unwrap_or(0.0);
    let retention = w.dps_retention_pct(ctx.window_seconds).unwrap_or(0.0);
    println!(
        "  [S{} eff={eff:8.1}] burst={burst:7.1}  sust={sust:7.1}  ret@{:.0}s={retention:5.1}%  {} ({})",
        w.size, ctx.window_seconds, display, w.record_name,
    );
}

fn print_ship_weapon(w: &ShipWeapon, snap: &sc_extract::DatacoreSnapshot, ctx: &LoadoutContext) {
    let display = snap.display_names.get(&w.guid).unwrap_or("");
    let mfg = w
        .manufacturer_guid
        .and_then(|g| snap.manufacturers.get(&g))
        .map(|m| m.code.as_str())
        .unwrap_or("");

    println!("--- {} ---", w.record_name);
    println!("  Display:      {display}");
    println!("  Manufacturer: {mfg}");
    println!("  Size/Grade:   S{} G{}", w.size, w.grade);
    println!("  Type:         {:?} / {:?}", w.item_type, w.item_sub_type);
    print_fire_actions(&w.fire_actions);

    // Damage
    if let Some(ref d) = w.damage {
        let parts: Vec<String> = [
            ("phys", d.physical),
            ("energy", d.energy),
            ("dist", d.distortion),
            ("thermal", d.thermal),
            ("biochem", d.biochemical),
            ("stun", d.stun),
        ]
        .iter()
        .filter(|(_, v)| *v > 0.0)
        .map(|(name, v)| format!("{name}={v:.1}"))
        .collect();
        println!(
            "  Alpha:        {} (total={:.1})",
            parts.join(", "),
            d.total()
        );
    } else {
        println!("  Alpha:        (no ammo resolved)");
    }

    if let Some(spd) = w.ammo_speed {
        let lt = w.ammo_lifetime.unwrap_or(0.0);
        println!(
            "  Ammo:         speed={spd:.0} m/s, lifetime={lt:.1}s, range={:.0}m",
            spd * lt
        );
    }

    if let Some(p) = w.pellet_count {
        println!("  Pellets:      {p}");
    }
    if let Some(m) = w.total_ammo {
        println!("  Total ammo:   {m}");
    }

    // Sustain
    match &w.sustain {
        SustainKind::Heat(h) => {
            println!("  Sustain:      HEAT");
            println!(
                "    overheat={:.0} cool/s={:.1} fix={:.1}s after_fix={:.0} cool_delay={:.1}s",
                h.overheat_temperature,
                h.cooling_per_second,
                h.overheat_fix_time,
                h.temperature_after_overheat_fix,
                h.time_till_cooling_starts
            );
            if let Some(t) = w.time_to_overheat() {
                println!("    time_to_overheat={t:.2}s");
            }
        }
        SustainKind::Energy(e) => {
            println!("  Sustain:      ENERGY");
            println!(
                "    max_load={:.0} regen/s={:.1} cooldown={:.1}s cost/bullet={:.2} requested_regen={:.1} requested_load={:.0}",
                e.max_ammo_load,
                e.max_regen_per_sec,
                e.regeneration_cooldown,
                e.regeneration_cost_per_bullet,
                e.requested_regen_per_sec,
                e.requested_ammo_load
            );
            // Cycle analysis (needs burst_rpm from primary fire action)
            let burst_rpm = w.fire_actions.first().and_then(|a| a.effective_rpm());
            if let Some(rpm) = burst_rpm {
                if let Some(b) = e.burst_seconds(rpm) {
                    println!(
                        "    burst_seconds={b:.2} refill_seconds={:.2} cycle_seconds={:.2}",
                        e.refill_seconds().unwrap_or(0.0),
                        e.cycle_seconds(rpm).unwrap_or(0.0)
                    );
                }
                if let Some(f) = e.asymptotic_dps_fraction(rpm) {
                    println!("    asymptotic_dps_fraction={:.1}%", f * 100.0);
                }
            }
        }
        SustainKind::None => {
            println!("  Sustain:      none");
        }
    }

    // Tier 2 — burst stats
    if let Some(burst) = w.burst_dps() {
        println!("  Burst DPS:    {burst:.1}");
        if let Some(bs) = w.burst_seconds() {
            println!("  Burst s:      {bs:.2}");
        }
        if let Some(vd) = w.volley_damage() {
            println!("  Volley dmg:   {vd:.0}");
        }
        if let Some(rs) = w.recovery_seconds() {
            println!("  Recovery s:   {rs:.2}");
        }
        if let Some(cs) = w.cycle_seconds() {
            println!("  Cycle s:      {cs:.2}");
        }

        // Tier 3 — normalised
        let sustained = w.sustained_dps().unwrap_or(burst);
        println!(
            "  Sust. DPS:    {sustained:.1} ({:.0}% of burst)",
            100.0 * sustained / burst.max(f32::EPSILON)
        );
        if let Some(ret) = w.dps_retention_pct(ctx.window_seconds) {
            println!("  Retention@{:.0}s: {ret:.1}%", ctx.window_seconds);
        }
        if let Some(ft) = w.firing_time_pct() {
            println!("  Firing time:  {ft:.1}% (long-run)");
        }
        if let Some(te) = w.thermal_efficiency() {
            println!("  Therm eff:    {te:.2} dmg/heat");
        }
        if let Some(pe) = w.power_efficiency() {
            println!("  Power eff:    {pe:.1} burst_dps/power");
        }
        if let Some(eff) = w.effective_dps(ctx) {
            println!(
                "  Effective:    {eff:.1} DPS (score at window={}s)",
                ctx.window_seconds
            );
        }
    }

    if let Some(pd) = w.power_draw {
        println!("  Power draw:   {pd:.3}/s");
    }
    if let Some(hp) = w.health {
        println!("  Health:       {hp:.0}");
    }
    println!();
}

fn print_fps_weapon(w: &FpsWeapon, snap: &sc_extract::DatacoreSnapshot) {
    let display = snap.display_names.get(&w.guid).unwrap_or("");
    let mfg = w
        .manufacturer_guid
        .and_then(|g| snap.manufacturers.get(&g))
        .map(|m| m.code.as_str())
        .unwrap_or("");

    println!("--- {} ---", w.record_name);
    println!("  Display:      {display}");
    println!("  Manufacturer: {mfg}");
    println!("  Size/Grade:   S{} G{}", w.size, w.grade);
    println!("  Type:         {:?} / {:?}", w.item_type, w.item_sub_type);
    print_fire_actions(&w.fire_actions);

    if let Some(ref d) = w.damage {
        let parts: Vec<String> = [
            ("phys", d.physical),
            ("energy", d.energy),
            ("dist", d.distortion),
            ("thermal", d.thermal),
            ("biochem", d.biochemical),
            ("stun", d.stun),
        ]
        .iter()
        .filter(|(_, v)| *v > 0.0)
        .map(|(name, v)| format!("{name}={v:.1}"))
        .collect();
        println!(
            "  Alpha:        {} (total={:.1})",
            parts.join(", "),
            d.total()
        );
    } else {
        println!("  Alpha:        (no ammo resolved)");
    }

    if let Some(spd) = w.ammo_speed {
        let lt = w.ammo_lifetime.unwrap_or(0.0);
        println!(
            "  Ammo:         speed={spd:.0} m/s, lifetime={lt:.1}s, range={:.0}m",
            spd * lt
        );
    }

    if let Some(p) = w.pellet_count {
        println!("  Pellets:      {p}");
    }
    if let Some(m) = w.total_ammo {
        println!("  Total ammo:   {m}");
    }
    println!();
}

fn print_fire_actions(actions: &[FireActionKind]) {
    println!("  Fire actions: {}", actions.len());
    for (i, fa) in actions.iter().enumerate() {
        let label = if i == 0 { "primary" } else { "alt" };
        println!("    [{i}] {label}: {}", format_fire_action(fa));
        if let Some(rpm) = fa.effective_rpm() {
            println!("        rpm={rpm:.0}");
        }
        if let FireActionKind::Charged {
            max_modifier: Some(m),
            ..
        } = fa
        {
            println!("        charge_mod: {}", format_charge_modifier(m));
        }
    }
}

fn format_fire_action(fa: &FireActionKind) -> String {
    match fa {
        FireActionKind::Rapid {
            fire_rate,
            heat_per_shot,
            spin_up,
            spin_down,
        } => {
            let mut s = format!("Rapid({fire_rate}rpm");
            if *heat_per_shot > 0.0 {
                s += &format!(", hps={heat_per_shot:.2}");
            }
            if *spin_up > 0.0 {
                s += &format!(", spin={spin_up:.2}/{spin_down:.2}");
            }
            s + ")"
        }
        FireActionKind::Single {
            fire_rate,
            heat_per_shot,
        } => {
            let mut s = format!("Single({fire_rate}rpm");
            if *heat_per_shot > 0.0 {
                s += &format!(", hps={heat_per_shot:.2}");
            }
            s + ")"
        }
        FireActionKind::Sequence {
            effective_rpm,
            barrel_count,
        } => {
            format!("Sequence({effective_rpm:.0}rpm, {barrel_count} barrels)")
        }
        FireActionKind::Burst {
            fire_rate,
            shot_count,
            cooldown,
        } => {
            format!("Burst({shot_count}x @ {fire_rate}rpm, cd={cooldown:.1}s)")
        }
        FireActionKind::Charged {
            charge_time,
            overcharge_time,
            auto_fire,
            full_only,
            ..
        } => {
            format!(
                "Charged({charge_time:.1}s+{overcharge_time:.1}s, auto={auto_fire}, full_only={full_only})"
            )
        }
        FireActionKind::Beam {
            dps,
            full_range,
            zero_range,
            heat_per_sec,
        } => {
            format!(
                "Beam(dps={:.0}, range={full_range:.0}-{zero_range:.0}m, hps={heat_per_sec:.0})",
                dps.total()
            )
        }
        FireActionKind::Unknown => "Unknown".into(),
    }
}

fn format_charge_modifier(m: &ChargeModifier) -> String {
    let mut parts: Vec<String> = Vec::new();
    if (m.damage_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("dmg×{:.2}", m.damage_multiplier));
    }
    if (m.projectile_speed_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("speed×{:.2}", m.projectile_speed_multiplier));
    }
    if (m.fire_rate_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("rpm×{:.2}", m.fire_rate_multiplier));
    }
    if (m.charge_time_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("charge×{:.2}", m.charge_time_multiplier));
    }
    if (m.heat_generation_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("heat×{:.2}", m.heat_generation_multiplier));
    }
    if (m.ammo_cost_multiplier - 1.0).abs() > f32::EPSILON {
        parts.push(format!("ammo×{:.2}", m.ammo_cost_multiplier));
    }
    if m.pellets > 0 {
        parts.push(format!("pellets={}", m.pellets));
    }
    if m.burst_shots > 0 {
        parts.push(format!("burst={}", m.burst_shots));
    }
    if parts.is_empty() {
        "(no overrides)".into()
    } else {
        parts.join(", ")
    }
}
