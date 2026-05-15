//! Interactive explorer view for sc-weapons data.
//!
//! Gated behind the `tui` feature. Sister module to
//! `sc_contracts::tui` — the workspace binary `tools/sc-explorer`
//! composes both into a tabbed UI alongside its own pool census.
//!
//! Layout: a list of ship weapons on the left, a detail panel on the
//! right showing sizing, damage breakdown, sustain model, fire actions,
//! and ammo. State (filter, list scroll) is owned by [`ExplorerState`].

use sc_extract::Datacore;
use slt::{Border, Color, Context, KeyCode, ListState, ScrollState, TextInputState};

use crate::ship::ShipWeapon;
use crate::sustain::SustainKind;

/// Persistent state for the weapons explorer view. Held across frames
/// by the embedding binary.
pub struct ExplorerState {
    pub filter: TextInputState,
    pub list: ListState,
    pub detail_scroll: ScrollState,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            filter: TextInputState::with_placeholder("filter record / manufacturer"),
            list: ListState::new(Vec::<String>::new()),
            detail_scroll: ScrollState::new(),
        }
    }
}

impl Default for ExplorerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Materialised weapon data the explorer renders. Built once when the
/// Datacore is loaded (the iterator under `iter_ship_weapons` allocates
/// per-frame, so the binary caches the result).
pub struct WeaponSet {
    pub ships: Vec<ShipWeapon>,
}

impl WeaponSet {
    pub fn build(datacore: &Datacore) -> Self {
        Self {
            ships: crate::iter_ship_weapons(datacore).collect(),
        }
    }
}

/// One pool-census row for the explorer's "Pools" tab. Mirrors
/// `sc_contracts::tui::PoolCheck`.
pub struct PoolCheck {
    pub name: &'static str,
    pub feature: &'static str,
    pub count: usize,
    pub required: bool,
}

/// Pool counts the weapons pipeline depends on. A `0` next to a
/// `required: true` entry means the corresponding feature isn't
/// effectively enabled at parse time and `iter_*_weapons` is going to
/// come back empty.
pub fn pool_checks(datacore: &Datacore) -> Vec<PoolCheck> {
    let idx = &datacore.records().records.multi_feature;
    vec![
        PoolCheck {
            name: "EntityClassDefinition",
            feature: "entityclassdefinition",
            count: idx.entity_class_definition.len(),
            required: true,
        },
        PoolCheck {
            name: "AmmoParams",
            feature: "ammoparams",
            count: idx.ammo_params.len(),
            required: true,
        },
    ]
}

// ── Render ──────────────────────────────────────────────────────────────────

pub fn render(ui: &mut Context, state: &mut ExplorerState, set: &WeaponSet) {
    let filter = state.filter.value.to_lowercase();
    let filtered: Vec<usize> = set
        .ships
        .iter()
        .enumerate()
        .filter(|(_, w)| matches_filter(w, &filter))
        .map(|(i, _)| i)
        .collect();

    if filtered.is_empty() {
        state.list.selected = 0;
    } else {
        state.list.selected = state.list.selected.min(filtered.len().saturating_sub(1));
    }

    let _ = ui.container().grow(1).col(|ui| {
        let _ = ui.row(|ui| {
            ui.text("Ship Weapons").bold().fg(Color::Cyan);
            ui.spacer();
            ui.text(format!(
                "{} matching / {} total",
                filtered.len(),
                set.ships.len()
            ))
            .dim();
        });
        // Fixed-height filter (text_input bakes grow(1) internally).
        let _ = ui.container().h(3).col(|ui| {
            let _ = ui.text_input(&mut state.filter);
        });
        ui.separator();

        let _ = ui.container().grow(1).row(|ui| {
            let _ = ui
                .bordered(Border::Rounded)
                .title("Weapons")
                .pad(1)
                .grow(1)
                .col(|ui| {
                    if filtered.is_empty() {
                        ui.text("(no matches)").dim();
                    } else {
                        let items: Vec<String> = filtered
                            .iter()
                            .map(|&i| format_list_item(&set.ships[i]))
                            .collect();
                        state.list.set_items(items.clone());
                        let visible = list_visible_rows(ui);
                        scroll_list(ui, &mut state.list, &items, visible);
                    }
                });

            let _ = ui
                .bordered(Border::Rounded)
                .title("Detail")
                .pad(1)
                .grow(2)
                .col(|ui| match filtered.get(state.list.selected).copied() {
                    Some(idx) => render_detail(ui, &mut state.detail_scroll, &set.ships[idx]),
                    None => {
                        ui.text("(nothing selected)").dim();
                    }
                });
        });
    });
}

fn matches_filter(w: &ShipWeapon, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    w.record_name.to_lowercase().contains(filter)
}

fn format_list_item(w: &ShipWeapon) -> String {
    let dmg = w
        .damage
        .as_ref()
        .map(|d| format!("{:>4.0}", d.total()))
        .unwrap_or_else(|| "  ?".into());
    format!("S{}  {dmg} dmg  {}", w.size.max(0), w.record_name)
}

fn render_detail(ui: &mut Context, scroll: &mut ScrollState, w: &ShipWeapon) {
    let _ = ui.scrollable(scroll).grow(1).col(|ui| {
        ui.text(&w.record_name).bold().fg(Color::Cyan);
        ui.separator();
        kv(ui, "size", &format!("S{}", w.size));
        kv(ui, "grade", &format!("{}", w.grade));
        kv(ui, "item_type", &format!("{:?}", w.item_type));
        kv(ui, "item_sub_type", &format!("{:?}", w.item_sub_type));
        ui.separator();

        // Damage
        if let Some(d) = &w.damage {
            ui.text("Damage").bold();
            kv(ui, "alpha", &format!("{:.0}", d.total()));
            if d.physical > 0.0 {
                kv(ui, "  physical", &format!("{:.0}", d.physical));
            }
            if d.energy > 0.0 {
                kv(ui, "  energy", &format!("{:.0}", d.energy));
            }
            if d.distortion > 0.0 {
                kv(ui, "  distortion", &format!("{:.0}", d.distortion));
            }
            if d.thermal > 0.0 {
                kv(ui, "  thermal", &format!("{:.0}", d.thermal));
            }
            if d.biochemical > 0.0 {
                kv(ui, "  biochemical", &format!("{:.0}", d.biochemical));
            }
            if d.stun > 0.0 {
                kv(ui, "  stun", &format!("{:.0}", d.stun));
            }
        } else {
            ui.text("Damage").bold();
            ui.text("  (no damage data)").dim();
        }
        ui.separator();

        // Ballistics
        ui.text("Ballistics").bold();
        if let Some(s) = w.ammo_speed {
            kv(ui, "speed", &format!("{s:.0} m/s"));
        }
        if let Some(l) = w.ammo_lifetime {
            kv(ui, "lifetime", &format!("{l:.2} s"));
        }
        if let Some(p) = w.pellet_count {
            kv(ui, "pellets", &format!("{p}"));
        }
        if let Some(a) = w.total_ammo {
            kv(ui, "total_ammo", &format!("{a}"));
        }
        if let Some(p) = w.power_draw {
            kv(ui, "power_draw", &format!("{p:.1}/s"));
        }
        if let Some(h) = w.health {
            kv(ui, "health", &format!("{h:.0}"));
        }
        ui.separator();

        // Sustain
        ui.text("Sustain").bold();
        match &w.sustain {
            SustainKind::Heat(h) => {
                kv(ui, "kind", "heat");
                kv(ui, "heat_rate", &format!("{:.1}/s", h.heat_rate_per_second));
                if let Some(t) = h.time_to_overheat_cold() {
                    kv(ui, "to_overheat", &format!("{t:.2} s"));
                }
                if let Some(d) = h.duty_cycle_long_run() {
                    kv(ui, "duty_cycle", &format!("{:.0}%", d * 100.0));
                }
            }
            SustainKind::Energy(e) => {
                kv(ui, "kind", "energy");
                kv(ui, "capacitor", &format!("{:.0}", e.max_ammo_load));
                kv(ui, "regen", &format!("{:.1}/s", e.max_regen_per_sec));
            }
            SustainKind::None => {
                ui.text("  (no sustain model)").dim();
            }
        }
        ui.separator();

        // Fire actions
        ui.text("Fire actions").bold();
        for (i, fa) in w.fire_actions.iter().enumerate() {
            ui.text(format!("  [{i}] {fa:?}"));
        }
    });
}

fn kv(ui: &mut Context, key: &str, value: &str) {
    let _ = ui.row(|ui| {
        ui.text(format!("  {key:<14}")).dim();
        ui.text(value);
    });
}

/// Approximate row budget for the weapons list. Same logic as
/// `sc_contracts::tui::list_visible_rows`.
fn list_visible_rows(ui: &Context) -> usize {
    (ui.height() as usize).saturating_sub(18).max(5)
}

/// Selection-aware list with a viewport that always contains
/// `state.selected`. See `sc_contracts::tui::scroll_list` for the
/// full rationale; the two helpers are intentionally duplicated rather
/// than shared, because they're each simple enough that a shared crate
/// would cost more than it saves.
fn scroll_list(ui: &mut Context, state: &mut ListState, items: &[String], visible: usize) {
    let total = items.len();
    if total == 0 {
        return;
    }
    if state.selected >= total {
        state.selected = total - 1;
    }
    let focused = ui.register_focusable();
    if focused {
        if ui.consume_key('j') || ui.consume_key_code(KeyCode::Down) {
            state.selected = (state.selected + 1).min(total - 1);
        }
        if ui.consume_key('k') || ui.consume_key_code(KeyCode::Up) {
            state.selected = state.selected.saturating_sub(1);
        }
        if ui.consume_key_code(KeyCode::PageDown) {
            state.selected = (state.selected + visible).min(total - 1);
        }
        if ui.consume_key_code(KeyCode::PageUp) {
            state.selected = state.selected.saturating_sub(visible);
        }
        if ui.consume_key_code(KeyCode::Home) {
            state.selected = 0;
        }
        if ui.consume_key_code(KeyCode::End) {
            state.selected = total - 1;
        }
    }

    let body_rows = visible.saturating_sub(2).max(1);
    let start = if state.selected >= body_rows {
        state.selected + 1 - body_rows
    } else {
        0
    };
    let end = (start + body_rows).min(total);

    if start > 0 {
        ui.text(format!("  ↑ {start} more")).dim();
    }
    for (idx, text) in items.iter().enumerate().take(end).skip(start) {
        if idx == state.selected {
            ui.text(format!("► {text}")).bold().fg(if focused {
                Color::Cyan
            } else {
                Color::White
            });
        } else {
            ui.text(format!("  {text}"));
        }
    }
    let remaining = total.saturating_sub(end);
    if remaining > 0 {
        ui.text(format!("  ↓ {remaining} more")).dim();
    }
}
