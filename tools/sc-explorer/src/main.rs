//! sc-explorer — interactive TUI for browsing parsed sc-holotable data.
//!
//! Three tabs:
//!
//! 1. **Pools** — record-pool census, surfaces missing-feature gaps.
//!    A `0` next to a `required` row means the consumer's feature set isn't
//!    pulling that pool's leaf feature in (the "data just wasn't there" mode).
//! 2. **Contracts** — filter + drill-down into [`sc_contracts::Contract`]s,
//!    with the locale-key sibling cluster surfaced inline.
//! 3. **Weapons** — filter + drill-down into [`sc_weapons::ShipWeapon`]s.
//!
//! The two domain tabs delegate their entire view to the per-crate `tui`
//! modules, so when the underlying data type grows a field, the view
//! updates next to it rather than here.
//!
//! Note: this binary depends only on `sc-installs`, `sc-contracts`, and
//! `sc-weapons` — not on `sc-extract` directly. The aggregation crates'
//! narrow-consumer re-exports cover everything we need to build the
//! `Datacore` pipeline. That's deliberate, and validates the narrow-
//! consumer dep story.

use std::time::Instant;

use anyhow::{Context as _, Result};
use sc_contracts::{
    AssetConfig, AssetData, AssetSource, ContractIndex, Datacore, DatacoreConfig, LocaleMap,
};
use slt::{Border, Color, Context, KeyCode, KeyModifiers, RunConfig, ScrollState, TabsState, Theme};

const TAB_POOLS: usize = 0;
const TAB_CONTRACTS: usize = 1;
const TAB_CLUSTERS: usize = 2;
const TAB_WEAPONS: usize = 3;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    eprintln!("sc-explorer: discovering primary install...");
    let install = sc_installs::discover_primary().context("discover SC install")?;
    eprintln!("  {} v{} at {}", install.channel, install.short_version(), install.root.display());

    eprintln!("sc-explorer: opening {}", install.data_p4k().display());
    let assets = AssetSource::from_install(&install).context("open Data.p4k")?;

    eprintln!("sc-explorer: extracting locale + DCB bytes...");
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())
        .context("extract asset data")?;

    let parse_started = Instant::now();
    eprintln!("sc-explorer: parsing Datacore (~25s)...");
    let datacore = Datacore::parse(&assets, &asset_data, &DatacoreConfig::standard())
        .context("parse Datacore")?;
    eprintln!(
        "sc-explorer: parsed {} record(s) in {:.1}s",
        datacore.records().len(),
        parse_started.elapsed().as_secs_f32()
    );

    eprintln!("sc-explorer: building ContractIndex...");
    let index = ContractIndex::build(&datacore, &asset_data.locale);
    eprintln!("  {} contract(s)", index.contracts.len());

    eprintln!("sc-explorer: materialising weapon set...");
    let weapon_set = sc_weapons::tui::WeaponSet::build(&datacore);
    eprintln!("  {} ship weapon(s)", weapon_set.ships.len());

    let pool_checks = build_pool_checks(&datacore);

    let mut app = AppState {
        index,
        locale: asset_data.locale,
        weapon_set,
        pool_checks,
        tabs: TabsState::new(vec![
            "Pools".to_string(),
            "Contracts".to_string(),
            "Clusters".to_string(),
            "Weapons".to_string(),
        ]),
        contracts_state: sc_contracts::tui::ExplorerState::new(),
        clusters_state: sc_contracts::tui::clusters::ClustersState::new(),
        weapons_state: sc_weapons::tui::ExplorerState::new(),
        pools_scroll: ScrollState::new(),
        dark_mode: true,
        version_label: format!(
            "{} v{}",
            install.channel,
            install.short_version()
        ),
    };

    eprintln!("sc-explorer: starting TUI (Ctrl+Q to quit)");
    // Mouse enabled so the detail-pane scrollable receives wheel events —
    // SLT's `auto_scroll_nested` only consumes mouse events, so without
    // this the scroll indicator paints but the viewport never moves.
    slt::run_with(RunConfig::default().mouse(true), |ui: &mut Context| draw(ui, &mut app))
        .context("run SLT")?;
    Ok(())
}

struct AppState {
    index: ContractIndex,
    locale: LocaleMap,
    weapon_set: sc_weapons::tui::WeaponSet,
    pool_checks: Vec<PoolRow>,
    tabs: TabsState,
    contracts_state: sc_contracts::tui::ExplorerState,
    clusters_state: sc_contracts::tui::clusters::ClustersState,
    weapons_state: sc_weapons::tui::ExplorerState,
    pools_scroll: ScrollState,
    dark_mode: bool,
    version_label: String,
}

/// One row in the Pools tab. Aggregates the per-aggregation-crate
/// `pool_checks` outputs under a unified shape so we can render one
/// table over the whole set.
struct PoolRow {
    /// "sc-contracts" / "sc-weapons" — which crate cares about this pool.
    owner: &'static str,
    name: &'static str,
    feature: &'static str,
    count: usize,
    required: bool,
}

fn build_pool_checks(datacore: &Datacore) -> Vec<PoolRow> {
    let mut rows = Vec::new();
    for c in sc_contracts::tui::pool_checks(datacore) {
        rows.push(PoolRow {
            owner: "sc-contracts",
            name: c.name,
            feature: c.feature,
            count: c.count,
            required: c.required,
        });
    }
    for c in sc_weapons::tui::pool_checks(datacore) {
        // Skip duplicates — both crates declare entityclassdefinition
        // and the contracts row already shows it. Deduplicate on
        // (name, feature) keeping the first appearance.
        if rows
            .iter()
            .any(|r| r.name == c.name && r.feature == c.feature)
        {
            continue;
        }
        rows.push(PoolRow {
            owner: "sc-weapons",
            name: c.name,
            feature: c.feature,
            count: c.count,
            required: c.required,
        });
    }
    rows
}

fn draw(ui: &mut Context, app: &mut AppState) {
    if ui.key_mod('q', KeyModifiers::CONTROL) || ui.key_code(KeyCode::Esc) {
        ui.quit();
    }
    if ui.key_mod('t', KeyModifiers::CONTROL) {
        app.dark_mode = !app.dark_mode;
    }
    ui.set_theme(if app.dark_mode {
        Theme::dark()
    } else {
        Theme::light()
    });

    let _ = ui
        .bordered(Border::Rounded)
        .title("sc-explorer")
        .pad(1)
        .grow(1)
        .col(|ui| {
            // Header
            let _ = ui.row(|ui| {
                ui.text("sc-explorer").bold().fg(Color::Cyan);
                ui.spacer();
                ui.text(&app.version_label).dim();
            });
            let _ = ui.tabs(&mut app.tabs);
            ui.separator();

            match app.tabs.selected {
                TAB_POOLS => render_pools(ui, app),
                TAB_CONTRACTS => sc_contracts::tui::render(
                    ui,
                    &mut app.contracts_state,
                    &app.index,
                ),
                TAB_CLUSTERS => sc_contracts::tui::clusters::render(
                    ui,
                    &mut app.clusters_state,
                    &app.index,
                    &app.locale,
                ),
                TAB_WEAPONS => sc_weapons::tui::render(
                    ui,
                    &mut app.weapons_state,
                    &app.weapon_set,
                ),
                _ => {}
            }

            ui.separator();
            let _ = ui.help(&[
                ("Tab", "focus"),
                ("j/k", "list"),
                ("PgUp/Dn", "page"),
                ("wheel", "scroll detail"),
                ("Ctrl+T", "theme"),
                ("Ctrl+Q", "quit"),
            ]);
        });
}

fn render_pools(ui: &mut Context, app: &mut AppState) {
    let _ = ui.container().grow(1).col(|ui| {
        let _ = ui.row(|ui| {
            ui.text("Record-pool census").bold().fg(Color::Cyan);
            ui.spacer();
            let zeros = app
                .pool_checks
                .iter()
                .filter(|r| r.required && r.count == 0)
                .count();
            if zeros == 0 {
                ui.text("all required pools populated").fg(Color::Green);
            } else {
                ui.text(format!("{zeros} required pool(s) empty"))
                    .fg(Color::Red);
            }
        });
        ui.text(
            "A `0` next to a required row means the consumer's feature set isn't \
             pulling that leaf feature in. Aggregation crates own their feature \
             closures — investigate by enabling the listed feature on the crate \
             that owns the row.",
        )
        .dim();
        ui.separator();

        let _ = ui.scrollable(&mut app.pools_scroll).grow(1).col(|ui| {
            // Header row
            let _ = ui.row(|ui| {
                ui.text(format!("{:<12}", "OWNER")).bold().dim();
                ui.text(format!("{:<24}", "RECORD TYPE")).bold().dim();
                ui.text(format!("{:<22}", "FEATURE")).bold().dim();
                ui.text(format!("{:>10}", "COUNT")).bold().dim();
                ui.spacer();
                ui.text("STATUS").bold().dim();
            });
            for row in &app.pool_checks {
                let (status_text, status_color) = match (row.required, row.count) {
                    (true, 0) => ("MISSING", Color::Red),
                    (true, _) => ("ok", Color::Green),
                    (false, 0) => ("(opt)", Color::Indexed(245)),
                    (false, _) => ("(opt) ok", Color::Indexed(245)),
                };
                let _ = ui.row(|ui| {
                    ui.text(format!("{:<12}", row.owner));
                    ui.text(format!("{:<24}", row.name));
                    ui.text(format!("{:<22}", row.feature)).dim();
                    let count_color = if row.required && row.count == 0 {
                        Color::Red
                    } else {
                        Color::White
                    };
                    ui.text(format!("{:>10}", row.count)).fg(count_color);
                    ui.spacer();
                    ui.text(status_text).fg(status_color);
                });
            }
        });
    });
}
