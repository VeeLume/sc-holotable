//! Interactive mining calculator: load the game data once, then type the
//! rock you scanned and the loadout you fly, and get the crack verdict, the
//! best gadget, module swaps and the Prospector support that would close the
//! gap. A stop-gap until Starlume has the mining overlay.
//!
//! ```bash
//! cargo build -p sc-mining --release --example mining_calc
//! target/release/examples/mining_calc                        # interactive
//! target/release/examples/mining_calc "laser pitman, focus iii, rieger-c3; gadget sabir"
//! echo "rock 20998, bexalite 6.22 597, bexalite 43.65 302, gold 2.49, borase 4.13" \
//!   | target/release/examples/mining_calc "laser pitman, focus iii, rieger-c3; gadget sabir"
//! ```
//!
//! Arguments are commands separated by `;`, run before reading stdin (a
//! prompt when it is a terminal, one command per line otherwise). Names are
//! the in-game ones or record names, matched loosely (case, spaces and
//! punctuation ignored; a unique substring is enough). Commands:
//!
//! - `rock <mass t>, <element> <percent> [quality], ...` — the scan; `inert`
//!   rows are ignored (inert is the remainder).
//! - `laser <laser>, <module>, ...` — your ship: one laser with its modules.
//! - `add <laser>, <module>, ...` — another laser on the same ship (MOLE).
//! - `gadget <gadget>|none` — the gadget on the rock.
//! - `sim <rock hz> [beam ticks per raycast]` — hit timing for the player
//!   model (default `30 1`: one even hit per rock update). The rock's update
//!   rate is the owning machine's frame rate, which the data does not give;
//!   other values show how much uneven hits cost (see `SimConfig`).
//! - `list lasers|modules|gadgets|elements`, `show`, `help`, `quit`.
//!
//! The Bexalite rock scanned on 2026-10-16 from a Golem (Focus III +
//! Rieger-C3, Sabir on) read resistance 30 % and instability 440.59 with the
//! beam on; this computes 30.7 % (the scanner truncates) and 440.65.

use std::io::{BufRead, IsTerminal, Write};

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_items::Items;
use sc_items_mining::{Activation, MiningGadget, MiningGadgets, MiningModule, MiningModules};
use sc_items_ship_weapons::{MiningLaserStats, ShipWeapons};
use sc_mining::{
    Compositions, Difficulty, Elements, Entry, Laser, ModifiedRock, Player, Rock, RockTypes,
    Ruleset, Rulesets, Setup, SimConfig, difficulty,
};
use sc_resources::Resources;

/// Player-model attempts per evaluated setup.
const SIM_RUNS: u32 = 100;

/// A catalog entry the user can name.
struct Named<T> {
    display: String,
    keys: Vec<String>,
    value: T,
}

/// Lower-case letters and digits only.
fn norm(s: &str) -> String {
    s.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

impl<T> Named<T> {
    fn new(display: String, record: &str, value: T) -> Self {
        // `Passive_Rieger_MK3` also answers to `rieger3`.
        let short = norm(
            &record
                .replace("Mining_Modules_", "")
                .replace("Mining_Gadget_", "")
                .replace("Mining_Laser_", "")
                .replace("Passive_", "")
                .replace("Active_", "")
                .replace("_MK", ""),
        );
        let keys = vec![norm(&display), norm(record), short];
        Self {
            display,
            keys,
            value,
        }
    }
}

/// The entry `query` names: an exact key, else a unique substring.
fn find<'a, T>(items: &'a [Named<T>], what: &str, query: &str) -> Result<&'a Named<T>, String> {
    let q = norm(query);
    if q.is_empty() {
        return Err(format!("no {what} given"));
    }
    let exact: Vec<_> = items.iter().filter(|i| i.keys.contains(&q)).collect();
    let pool = if exact.is_empty() {
        items
            .iter()
            .filter(|i| i.keys.iter().any(|k| k.contains(&q)))
            .collect()
    } else {
        exact
    };
    match pool.as_slice() {
        [one] => Ok(one),
        [] => Err(format!("no {what} matches '{query}' (try `list`)")),
        many => Err(format!(
            "'{query}' matches several {what}s: {}",
            many.iter()
                .map(|i| i.display.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        )),
    }
}

struct Catalog {
    ruleset: Ruleset,
    elements: Elements,
    element_names: Vec<Named<Guid>>,
    lasers: Vec<Named<MiningLaserStats>>,
    modules: Vec<Named<MiningModule>>,
    gadgets: Vec<Named<MiningGadget>>,
}

#[derive(Default)]
struct State {
    lasers: Vec<Laser>,
    labels: Vec<String>,
    gadget: Option<usize>,
    rock: Option<Rock>,
    sim: SimConfig,
}

/// `sim <rock updates per second> [beam ticks per raycast]`: how often the
/// rock updates (the owning machine's frame rate) and how many 1/30 s beam
/// ticks a raycast takes, which together set how unevenly hits arrive.
fn set_sim(state: &mut State, rest: &str) -> Result<bool, String> {
    let mut parts = rest.split_whitespace();
    let parse = |s: Option<&str>, default: f32| match s {
        None => Ok(default),
        Some(s) => s
            .parse::<f32>()
            .ok()
            .filter(|v| *v > 0.0)
            .ok_or_else(|| format!("'{s}' is not a positive number")),
    };
    let hz = parse(parts.next(), 30.0)?;
    let raycast_ticks = parse(parts.next(), 1.0)?;
    state.sim.tick_s = 1.0 / hz;
    state.sim.hit_interval_s = raycast_ticks / 30.0;
    Ok(true)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("loading game data ...");
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let locale = &data.locale;
    let resources = Resources::build(datacore.records());
    let items = Items::build(datacore.records());
    let db = datacore.db();
    let record = |guid: &Guid| {
        db.record(guid)
            .and_then(|r| {
                r.name()
                    .map(|n| n.rsplit('.').next().unwrap_or(n).to_string())
            })
            .unwrap_or_default()
    };
    let text = |key: Option<&sc_extract::LocaleKey>| {
        key.and_then(|k| locale.resolve(k))
            .filter(|t| !t.is_empty() && !t.starts_with('@') && !t.contains("PLACEHOLDER"))
            .map(str::to_string)
    };
    // Test and template records carry real display names; nothing typed
    // marks them, so they are dropped by record name.
    let junk = |guid: &Guid| {
        let r = record(guid).to_ascii_lowercase();
        r.contains("test") || r.contains("template") || r.contains("mpuv")
    };
    let rulesets = Rulesets::build(&datacore, &resources);
    let (ruleset_guid, ruleset) = rulesets
        .iter()
        .find(|(g, _)| record(g) == "MiningGlobalParamsShip")
        .map(|(g, r)| (*g, r.clone()))
        .ok_or("no ship mining ruleset")?;
    let elements = Elements::build(&datacore, &resources);
    // The elements ship rocks spawn with: rock types on the ship ruleset,
    // through their compositions.
    let compositions = Compositions::build(&datacore);
    let ship_elements: std::collections::HashSet<Guid> = RockTypes::build(&datacore)
        .values()
        .filter(|t| t.ruleset == Some(ruleset_guid))
        .filter_map(|t| t.composition.and_then(|c| compositions.get(&c)))
        .flat_map(|c| c.parts.iter().filter_map(|p| p.element))
        .collect();
    let mut element_names: Vec<Named<Guid>> = elements
        .iter()
        .filter(|(g, e)| e.density.is_some() && ship_elements.contains(g))
        .map(|(g, e)| {
            let display = e
                .resource
                .and_then(|r| resources.get(&r))
                .and_then(|r| text(Some(&r.name_key)))
                .unwrap_or_else(|| record(g));
            Named::new(format!("{display} [{}]", record(g)), &record(g), *g)
        })
        .collect();
    element_names.sort_by(|a, b| a.display.cmp(&b.display));
    // Only items with an in-game name: drops test and template records.
    let mut lasers: Vec<Named<MiningLaserStats>> = ShipWeapons::build(&datacore, &items)
        .iter()
        .filter_map(|(g, w)| {
            let m = w.mining.clone()?;
            // Size 0 are FPS and ROC lasers, on other rulesets.
            if w.size < 1 || junk(g) {
                return None;
            }
            let display = text(w.name_key.as_ref())?;
            Some(Named::new(
                format!("{display} (S{})", w.size),
                &record(g),
                m,
            ))
        })
        .collect();
    lasers.sort_by(|a, b| a.display.cmp(&b.display));
    let mut modules: Vec<Named<MiningModule>> = MiningModules::build(&datacore, &items)
        .iter()
        // An active module without a duration is a base record or the ROC's.
        .filter(|(g, m)| {
            !junk(g)
                && !matches!(
                    m.activation,
                    Activation::Active {
                        duration_s: None,
                        ..
                    }
                )
        })
        .filter_map(|(g, m)| {
            Some(Named::new(
                text(m.name_key.as_ref())?,
                &record(g),
                m.clone(),
            ))
        })
        .collect();
    modules.sort_by(|a, b| a.display.cmp(&b.display));
    let mut gadgets: Vec<Named<MiningGadget>> = MiningGadgets::build(&datacore, &items)
        .iter()
        .filter(|(g, _)| !junk(g))
        .filter_map(|(g, m)| {
            Some(Named::new(
                text(m.name_key.as_ref())?,
                &record(g),
                m.clone(),
            ))
        })
        .collect();
    gadgets.sort_by(|a, b| a.display.cmp(&b.display));
    let catalog = Catalog {
        ruleset,
        elements,
        element_names,
        lasers,
        modules,
        gadgets,
    };
    eprintln!(
        "ready: {} lasers, {} modules, {} gadgets, {} elements. `help` for commands.",
        catalog.lasers.len(),
        catalog.modules.len(),
        catalog.gadgets.len(),
        catalog.element_names.len()
    );

    let mut state = State::default();
    let startup = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    // Startup commands set things up quietly; one report at the end.
    let mut quit = false;
    for command in startup.split(';') {
        if !run(&catalog, &mut state, command, true) {
            quit = true;
            break;
        }
    }
    if !startup.trim().is_empty() {
        report(&catalog, &state);
    }
    if quit {
        return Ok(());
    }
    let stdin = std::io::stdin();
    let interactive = stdin.is_terminal();
    let mut line = String::new();
    loop {
        if interactive {
            print!("mining> ");
            std::io::stdout().flush()?;
        }
        line.clear();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }
        if !run(&catalog, &mut state, &line, false) {
            break;
        }
    }
    Ok(())
}

/// Run one command, reporting after a change unless `quiet`; `false` to quit.
fn run(catalog: &Catalog, state: &mut State, command: &str, quiet: bool) -> bool {
    let command = command.trim();
    if command.is_empty() {
        return true;
    }
    let (verb, rest) = command.split_once(' ').unwrap_or((command, ""));
    let result = match verb.to_ascii_lowercase().as_str() {
        "quit" | "exit" | "q" => return false,
        "help" | "?" => {
            println!(
                "rock <mass>, <element> <pct> [quality], ...   e.g. rock 20998, bexalite 6.22 597, gold 2.49\n\
                 laser <laser>, <module>, ...                   e.g. laser pitman, focus iii, rieger-c3\n\
                 add <laser>, <module>, ...                     another laser on your ship (MOLE)\n\
                 gadget <gadget>|none\n\
                 sim <rock hz> [beam ticks per raycast]         hit timing for the player model, default 30 1\n\
                 list lasers|modules|gadgets|elements\n\
                 show | quit"
            );
            Ok(false)
        }
        "list" => {
            list(catalog, rest.trim());
            Ok(false)
        }
        "show" => Ok(true),
        "laser" => set_laser(catalog, state, rest, false),
        "add" => set_laser(catalog, state, rest, true),
        "gadget" => {
            if norm(rest) == "none" || rest.trim().is_empty() {
                state.gadget = None;
                Ok(true)
            } else {
                find(&catalog.gadgets, "gadget", rest).map(|g| {
                    state.gadget = catalog.gadgets.iter().position(|x| std::ptr::eq(x, g));
                    true
                })
            }
        }
        "sim" => set_sim(state, rest),
        "rock" => parse_rock(catalog, rest).map(|rock| {
            state.rock = Some(rock);
            true
        }),
        other => Err(format!("unknown command '{other}' (try `help`)")),
    };
    match result {
        Ok(true) if !quiet => report(catalog, state),
        Ok(_) => {}
        Err(e) => println!("! {e}"),
    }
    true
}

fn list(catalog: &Catalog, what: &str) {
    match norm(what).as_str() {
        "lasers" | "laser" => {
            for l in &catalog.lasers {
                let s = &l.value;
                println!(
                    "  {:<34} {:>5.0}/s  slots {}  R {:>4}  I {:>4}  W {:>4}",
                    l.display,
                    s.fracture_beam().map_or(0.0, |b| b.energy_per_second),
                    s.module_slots,
                    pct(s.modifiers.resistance.as_ref().map(|m| m.value)),
                    pct(s.modifiers.laser_instability.as_ref().map(|m| m.value)),
                    pct(s
                        .modifiers
                        .optimal_charge_window_size
                        .as_ref()
                        .map(|m| m.value)),
                );
            }
        }
        "modules" | "module" => {
            for m in &catalog.modules {
                let v = &m.value;
                println!(
                    "  {:<26} power x{:.2}  R {:>4}  I {:>4}  W {:>4}  {}",
                    m.display,
                    v.damage_multiplier(0),
                    pct(v.modifiers.resistance.as_ref().map(|m| m.value)),
                    pct(v.modifiers.laser_instability.as_ref().map(|m| m.value)),
                    pct(v
                        .modifiers
                        .optimal_charge_window_size
                        .as_ref()
                        .map(|m| m.value)),
                    activation(v)
                );
            }
        }
        "gadgets" | "gadget" => {
            for g in &catalog.gadgets {
                let v = &g.value;
                println!(
                    "  {:<20} R {:>4}  I {:>4}  W {:>4}",
                    g.display,
                    pct(v.modifiers.resistance.as_ref().map(|m| m.value)),
                    pct(v.modifiers.laser_instability.as_ref().map(|m| m.value)),
                    pct(v
                        .modifiers
                        .optimal_charge_window_size
                        .as_ref()
                        .map(|m| m.value)),
                );
            }
        }
        "elements" | "element" => {
            for e in &catalog.element_names {
                println!("  {}", e.display);
            }
        }
        _ => println!("! list lasers|modules|gadgets|elements"),
    }
}

fn pct(v: Option<f32>) -> String {
    v.filter(|v| *v != 0.0)
        .map_or("-".into(), |v| format!("{v:+.0}%"))
}

fn activation(m: &MiningModule) -> String {
    match m.activation {
        Activation::Passive => "passive".into(),
        Activation::Active { duration_s, uses } => format!(
            "active {}s x{}",
            duration_s.map_or("?".into(), |d| format!("{d:.0}")),
            uses.map_or("?".into(), |u| u.to_string())
        ),
    }
}

fn is_active(m: &MiningModule) -> bool {
    matches!(m.activation, Activation::Active { .. })
}

fn set_laser(catalog: &Catalog, state: &mut State, rest: &str, add: bool) -> Result<bool, String> {
    let mut parts = rest.split(',').map(str::trim).filter(|p| !p.is_empty());
    let laser = find(&catalog.lasers, "laser", parts.next().unwrap_or(""))?;
    let mut mods = Vec::new();
    for p in parts {
        mods.push(find(&catalog.modules, "module", p)?);
    }
    if mods.len() > laser.value.module_slots as usize {
        return Err(format!(
            "{} has {} module slots",
            laser.display, laser.value.module_slots
        ));
    }
    let mut l = Laser::new(laser.value.clone());
    l.modules = mods.iter().map(|m| m.value.clone()).collect();
    let label = format!(
        "{} [{}]",
        laser.display,
        mods.iter()
            .map(|m| m.display.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    if !add {
        state.lasers.clear();
        state.labels.clear();
    }
    state.lasers.push(l);
    state.labels.push(label);
    Ok(true)
}

fn parse_rock(catalog: &Catalog, rest: &str) -> Result<Rock, String> {
    let mut parts = rest.split(',').map(str::trim).filter(|p| !p.is_empty());
    let mass: f32 = parts
        .next()
        .map(|m| m.replace(['t', '.'], "").replace(' ', ""))
        .ok_or("rock <mass>, <element> <pct> [quality], ...")?
        .parse()
        .map_err(|_| "the first value is the mass in t (e.g. 20998)".to_string())?;
    let mut entries = Vec::new();
    for p in parts {
        let tokens: Vec<&str> = p.split_whitespace().collect();
        let numbers: Vec<f32> = tokens
            .iter()
            .rev()
            .map_while(|t| t.trim_end_matches('%').parse::<f32>().ok())
            .collect();
        let name = tokens[..tokens.len() - numbers.len()].join(" ");
        if norm(&name).starts_with("inert") {
            continue;
        }
        let (fraction, quality) = match numbers.as_slice() {
            [f] => (*f, None),
            [q, f] => (*f, Some(*q as u16)),
            _ => return Err(format!("'{p}': want <element> <percent> [quality]")),
        };
        let element = find(&catalog.element_names, "element", &name)?;
        entries.push(Entry {
            element: element.value,
            fraction: fraction / 100.0,
            quality,
        });
    }
    if entries.is_empty() {
        return Err("no elements given".into());
    }
    Rock::from_mass(&catalog.ruleset, &catalog.elements, &entries, mass)
        .ok_or_else(|| "an element has no density".into())
}

/// How a setup does on the rock.
struct Eval {
    margin: f32,
    power: f32,
    need: f32,
    rock: ModifiedRock,
    difficulty: Option<Difficulty>,
}

impl Eval {
    /// `sim`: also run the player model with this configuration.
    fn of(ruleset: &Ruleset, rock: &Rock, setup: &Setup, sim: Option<&SimConfig>) -> Self {
        let m = ModifiedRock::new(ruleset, rock, setup);
        let difficulty = sim
            .filter(|_| m.is_breakable(ruleset))
            .map(|config| difficulty(&m, ruleset, &Player::default(), config, SIM_RUNS));
        Self {
            margin: m.max_mass(ruleset) / rock.mass,
            power: m.power(),
            need: rock.mass * ruleset.decay_per_mass / (1.0 - m.resistance).max(1e-6),
            rock: m,
            difficulty,
        }
    }

    /// Ranking: a clean crack first, then margin.
    fn score(&self) -> (f32, f32) {
        (self.difficulty.map_or(-1.0, |d| d.controlled), self.margin)
    }

    fn line(&self, ruleset: &Ruleset) -> String {
        let verdict = match self.difficulty {
            None => format!("IMPOSSIBLE, {:.0}/s short", self.need - self.power),
            Some(d) => format!(
                "clean {:>3.0} %  overcharge {:>3.0} %  ~{}s",
                d.controlled * 100.0,
                d.overcharged * 100.0,
                d.median_time_s.map_or("?".into(), |t| format!("{t:.0}"))
            ),
        };
        format!(
            "{:>4.0} % | {:>5.0}/{:>5.0} power | R {:>4.1} % I {:>6.1} W {:.3} | balance {:>4} | {verdict}",
            self.margin * 100.0,
            self.power,
            self.need,
            self.rock.resistance * 100.0,
            self.rock.instability,
            self.rock.window.size(),
            self.rock
                .balance_throttle(ruleset)
                .map_or("-".into(), |t| format!("{:.0}%", t * 100.0)),
        )
    }
}

fn better(a: &Eval, b: &Eval) -> std::cmp::Ordering {
    let (a, b) = (a.score(), b.score());
    b.0.total_cmp(&a.0).then(b.1.total_cmp(&a.1))
}

/// Every multiset of up to `k` module indices.
fn kits(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut out = vec![vec![]];
    let mut frontier = vec![vec![]];
    for _ in 0..k {
        frontier = frontier
            .into_iter()
            .flat_map(|v: Vec<usize>| {
                let start = v.last().copied().unwrap_or(0);
                (start..n).map(move |i| {
                    let mut w = v.clone();
                    w.push(i);
                    w
                })
            })
            .collect();
        out.extend(frontier.iter().cloned());
    }
    out
}

fn report(catalog: &Catalog, state: &State) {
    let rs = &catalog.ruleset;
    let gadget = state.gadget.map(|g| &catalog.gadgets[g]);
    let gadgets: Vec<MiningGadget> = gadget.map(|g| g.value.clone()).into_iter().collect();
    println!(
        "\nship: {}  | gadget: {}  | sim: rock {:.0} Hz, hit every {:.0} beam tick(s)",
        if state.labels.is_empty() {
            "(none: `laser ...`)".into()
        } else {
            state.labels.join(" + ")
        },
        gadget.map_or("none", |g| g.display.as_str()),
        1.0 / state.sim.tick_s,
        state.sim.hit_interval_s * 30.0
    );
    let Some(rock) = &state.rock else {
        println!("(no rock yet: `rock <mass>, <element> <pct> [quality], ...`)");
        return;
    };
    let on = |lasers: Vec<Laser>, gadgets: Vec<MiningGadget>| Setup { lasers, gadgets };
    let gadget_only = ModifiedRock::new(rs, rock, &on(vec![], gadgets.clone()));
    println!(
        "rock: {:.0} t, {:.2} SCU | bare R {:.1} % I {:.2} W {:.3} | gadget only R {:.1} % I {:.2} | window {:.3}..{:.3}",
        rock.mass,
        rock.scu,
        rock.resistance * 100.0,
        rock.instability,
        rock.window.size(),
        gadget_only.resistance * 100.0,
        gadget_only.instability,
        rock.window.low,
        rock.window.high
    );
    if state.lasers.is_empty() {
        return;
    }
    let sim = Some(&state.sim);
    let mine = Eval::of(rs, rock, &on(state.lasers.clone(), gadgets.clone()), sim);
    println!(
        "scanner with the beam on should read R {:.0} % I {:.2}",
        (mine.rock.resistance * 100.0).floor(),
        mine.rock.instability
    );
    println!("\nlimit/mass | power/needed | modified rock | balance throttle | player model");
    println!("YOU        {}", mine.line(rs));

    // Gadgets with your loadout.
    let mut by_gadget: Vec<(String, Eval)> = std::iter::once(None)
        .chain(catalog.gadgets.iter().map(Some))
        .map(|g| {
            let gs = g.map(|g| g.value.clone()).into_iter().collect();
            (
                g.map_or("no gadget".into(), |g| g.display.clone()),
                Eval::of(rs, rock, &on(state.lasers.clone(), gs), sim),
            )
        })
        .collect();
    by_gadget.sort_by(|a, b| better(&a.1, &b.1));
    println!("\ngadget (your loadout):");
    for (name, e) in by_gadget.iter().take(4) {
        println!("  {}  | {name}", e.line(rs));
    }

    // Module swaps, single-laser ships only.
    if let [laser] = state.lasers.as_slice() {
        let fitted = |kit: &[usize]| {
            let mut l = laser.clone();
            l.modules = kit
                .iter()
                .map(|&i| catalog.modules[i].value.clone())
                .collect();
            on(vec![l], gadgets.clone())
        };
        let all = kits(catalog.modules.len(), laser.stats.module_slots as usize);
        for (active, title) in [(false, "passive"), (true, "with an active module running")] {
            // Simulate only the strongest few by margin, then rank by the sim.
            let mut rows: Vec<(&Vec<usize>, f32)> = all
                .iter()
                .filter(|kit| kit.iter().any(|&i| is_active(&catalog.modules[i].value)) == active)
                .map(|kit| (kit, Eval::of(rs, rock, &fitted(kit), None).margin))
                .collect();
            rows.sort_by(|a, b| b.1.total_cmp(&a.1));
            let mut rows: Vec<(&Vec<usize>, Eval)> = rows
                .into_iter()
                .take(8)
                .map(|(kit, _)| (kit, Eval::of(rs, rock, &fitted(kit), sim)))
                .collect();
            rows.sort_by(|a, b| better(&a.1, &b.1));
            println!(
                "
modules, {title} (gadget as set):"
            );
            for (kit, e) in rows.iter().take(3) {
                let names: Vec<&str> = kit
                    .iter()
                    .map(|&i| catalog.modules[i].display.as_str())
                    .collect();
                // An active module that runs out before the median fracture.
                let shortest = kit
                    .iter()
                    .filter_map(|&i| match catalog.modules[i].value.activation {
                        Activation::Active { duration_s, .. } => duration_s,
                        Activation::Passive => None,
                    })
                    .reduce(f32::min);
                let warn = match (shortest, e.difficulty.and_then(|d| d.median_time_s)) {
                    (Some(d), Some(t)) if t > d => format!("  (active lasts {d:.0}s only)"),
                    _ => String::new(),
                };
                println!("  {}  | [{}]{warn}", e.line(rs), names.join(", "));
            }
        }
    }

    // Support: one Prospector (any size-1 laser but the Golem's Pitman).
    let support: Vec<&Named<MiningLaserStats>> = catalog
        .lasers
        .iter()
        .filter(|l| l.display.ends_with("(S1)") && !l.keys.iter().any(|k| k.contains("golem")))
        .collect();
    let passive: Vec<usize> = (0..catalog.modules.len())
        .filter(|&i| !is_active(&catalog.modules[i].value))
        .collect();
    let mut rows: Vec<(String, Eval)> = Vec::new();
    for l in &support {
        let mut best: Option<(f32, Vec<usize>)> = None;
        for kit in kits(passive.len(), l.value.module_slots as usize) {
            let mut p = Laser::new(l.value.clone());
            p.modules = kit
                .iter()
                .map(|&i| catalog.modules[passive[i]].value.clone())
                .collect();
            p.owner = 1;
            let mut lasers = state.lasers.clone();
            lasers.push(p);
            let e = Eval::of(rs, rock, &on(lasers, gadgets.clone()), None);
            // The smallest kit that cracks with some margin, else the strongest.
            let key = if e.margin >= 1.1 {
                10.0 - kit.len() as f32 - e.margin / 100.0
            } else {
                e.margin
            };
            if best.as_ref().is_none_or(|(b, _)| key > *b) {
                best = Some((key, kit.iter().map(|&i| passive[i]).collect()));
            }
        }
        for kit in std::iter::once(vec![]).chain(best.map(|b| b.1)) {
            let mut p = Laser::new(l.value.clone());
            p.modules = kit
                .iter()
                .map(|&i| catalog.modules[i].value.clone())
                .collect();
            p.owner = 1;
            let mut lasers = state.lasers.clone();
            lasers.push(p);
            let names: Vec<&str> = kit
                .iter()
                .map(|&i| catalog.modules[i].display.as_str())
                .collect();
            let label = format!("{} [{}]", l.display, names.join(", "));
            if rows.iter().any(|(n, _)| *n == label) {
                continue;
            }
            rows.push((label, Eval::of(rs, rock, &on(lasers, gadgets.clone()), sim)));
        }
    }
    rows.sort_by(|a, b| better(&a.1, &b.1));
    println!(
        "\n+ one Prospector (your loadout and gadget unchanged; two ships double the wave, inferred):"
    );
    for (n, e) in rows.iter().take(6) {
        println!("  {}  | {n}", e.line(rs));
    }
    println!();
}
