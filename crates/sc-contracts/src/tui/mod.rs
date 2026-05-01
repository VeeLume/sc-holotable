//! Interactive explorer views for sc-contracts data.
//!
//! Gated behind the `tui` feature. The workspace binary
//! `tools/sc-explorer` composes this module with `sc_weapons::tui` plus
//! its own pool-census tab. Each domain crate owns its own view so when
//! `Contract` grows a field, the rendering updates next to it.
//!
//! The crate-level [`render`]/[`ExplorerState`] below is the contracts
//! list-detail view; [`clusters`] adds the locale-key cluster pane
//! that groups contracts by `title_key` / `description_key` and
//! surfaces per-axis divergence, informing patcher tools where tag /
//! description annotations are safe vs misleading.

pub mod clusters;

use std::collections::HashMap;

use sc_extract::{Datacore, TagTree};
use slt::{Border, Color, Context, KeyCode, ListState, ScrollState, TextInputState};

use crate::expand::{EncounterSlot, RewardAmount};
use crate::index::ContractIndex;
use crate::merge::Contract;

/// Persistent state for the contracts explorer view.
///
/// Held by the embedding binary across frames. `new` initialises with
/// the contract list visible; the filter is empty by default.
pub struct ExplorerState {
    pub filter: TextInputState,
    pub list: ListState,
    pub detail_scroll: ScrollState,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            filter: TextInputState::with_placeholder("filter title / debug name"),
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

/// One pool-census row for the explorer's "Pools" tab. Each entry maps
/// a record-pool name → leaf feature → live count, plus whether the
/// crate's code requires it to be non-empty.
pub struct PoolCheck {
    pub name: &'static str,
    pub feature: &'static str,
    pub count: usize,
    pub required: bool,
}

/// Pool counts the contracts pipeline depends on. The Pools tab
/// renders these alongside `sc-weapons::tui::pool_checks` to surface
/// missing-feature bugs at a glance — a `0` next to a `required: true`
/// entry means the feature is not effectively enabled even if the type
/// compiles.
pub fn pool_checks(datacore: &Datacore) -> Vec<PoolCheck> {
    let idx = &datacore.records().records.multi_feature;
    vec![
        PoolCheck {
            name: "ContractGenerator",
            feature: "contracts",
            count: idx.contract_generator.len(),
            required: true,
        },
        PoolCheck {
            name: "BlueprintPoolRecord",
            feature: "contracts",
            count: idx.blueprint_pool_record.len(),
            required: true,
        },
        PoolCheck {
            name: "MissionLocality",
            feature: "contracts",
            count: idx.mission_locality.len(),
            required: true,
        },
        PoolCheck {
            name: "EntityClassDefinition",
            feature: "entityclassdefinition",
            count: idx.entity_class_definition.len(),
            required: true,
        },
        PoolCheck {
            name: "StarMapObject",
            feature: "servicebeacon",
            count: idx.star_map_object.len(),
            required: true,
        },
    ]
}

// ── Render ──────────────────────────────────────────────────────────────────

/// Render the contracts explorer into `ui`.
///
/// Layout: a bordered column with the filter input + count header, then
/// a row split between the contract list and the detail panel for the
/// currently selected contract.
pub fn render(ui: &mut Context, state: &mut ExplorerState, index: &ContractIndex) {
    let filter = state.filter.value.to_lowercase();
    let filtered: Vec<usize> = index
        .contracts
        .iter()
        .enumerate()
        .filter(|(_, c)| matches_filter(c, &filter))
        .map(|(i, _)| i)
        .collect();

    if filtered.is_empty() {
        state.list.selected = 0;
    } else {
        state.list.selected = state.list.selected.min(filtered.len().saturating_sub(1));
    }

    let _ = ui.container().grow(1).col(|ui| {
        // Header
        let _ = ui.row(|ui| {
            ui.text("Contracts").bold().fg(Color::Cyan);
            ui.spacer();
            ui.text(format!(
                "{} matching / {} total",
                filtered.len(),
                index.contracts.len()
            ))
            .dim();
        });
        // Fixed-height filter (text_input bakes grow(1) internally — pin
        // its height here so the body container below claims the leftover
        // flex space).
        let _ = ui.container().h(3).col(|ui| {
            let _ = ui.text_input(&mut state.filter);
        });
        ui.separator();

        // Body
        let _ = ui.container().grow(1).row(|ui| {
            // Left: list
            let _ = ui
                .bordered(Border::Rounded)
                .title("Contracts")
                .pad(1)
                .grow(1)
                .col(|ui| {
                    if filtered.is_empty() {
                        ui.text("(no matches)").dim();
                    } else {
                        let items: Vec<String> = filtered
                            .iter()
                            .map(|&i| format_list_item(&index.contracts[i]))
                            .collect();
                        // Sync ListState's items so other widgets that read
                        // from it (and the height-clamp below) stay correct.
                        state.list.set_items(items.clone());
                        let visible = list_visible_rows(ui);
                        scroll_list(ui, &mut state.list, &items, visible);
                    }
                });

            // Right: detail
            let _ = ui
                .bordered(Border::Rounded)
                .title("Detail")
                .pad(1)
                .grow(2)
                .col(|ui| match filtered.get(state.list.selected).copied() {
                    Some(idx) => render_detail(
                        ui,
                        &mut state.detail_scroll,
                        &index.contracts,
                        idx,
                        &index.tag_tree,
                    ),
                    None => {
                        ui.text("(nothing selected)").dim();
                    }
                });
        });
    });
}

fn matches_filter(c: &Contract, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    if let Some(t) = &c.title
        && t.to_lowercase().contains(filter)
    {
        return true;
    }
    c.debug_name.to_lowercase().contains(filter)
}

fn format_list_item(c: &Contract) -> String {
    let title = c.title.as_deref().unwrap_or(&c.debug_name);
    let marker = match (c.title_siblings.is_empty(), c.has_runtime_substitution) {
        (false, _) => '⋈', // sibling cluster
        (true, true) => '~', // runtime substitution
        (true, false) => ' ',
    };
    let bp = if c.rewards.blueprint.is_some() { "[BP]" } else { "    " };
    format!("{marker} {bp} {title}")
}

fn render_detail(
    ui: &mut Context,
    scroll: &mut ScrollState,
    contracts: &[Contract],
    idx: usize,
    tree: &TagTree,
) {
    let c = &contracts[idx];

    // Build a sibling-by-id map once so we don't index_of every render.
    let by_id: HashMap<_, _> = contracts.iter().map(|c| (c.id, c)).collect();

    let _ = ui.scrollable(scroll).grow(1).col(|ui| {
        // Title block
        ui.text(c.title.as_deref().unwrap_or("(no title)"))
            .bold()
            .fg(Color::Cyan);
        ui.text(&c.debug_name).dim();
        ui.separator();

        // Identity
        kv(ui, "handler", &format!("{:?}", c.handler_kind));
        kv(ui, "handler_debug", &c.handler_debug_name);
        if c.has_runtime_substitution {
            ui.text("title contains ~mission(...) runtime substitution")
                .fg(Color::Yellow);
        }
        kv(
            ui,
            "title_key",
            c.title_key
                .as_ref()
                .map(|k| k.as_str())
                .unwrap_or("(none)"),
        );
        kv(
            ui,
            "description_key",
            c.description_key
                .as_ref()
                .map(|k| k.as_str())
                .unwrap_or("(none)"),
        );
        kv(
            ui,
            "shareable",
            if c.shareable { "yes" } else { "no" },
        );
        kv(
            ui,
            "illegal_flag",
            if c.illegal_flag { "yes" } else { "no" },
        );
        ui.separator();

        // Variations
        let _ = ui.row(|ui| {
            ui.text("Variations").bold();
            ui.spacer();
            ui.text(format!("{}", c.variations.len())).dim();
        });
        for v in &c.variations {
            ui.text(format!(
                "  • {:?}  extras: {}",
                v.origin,
                v.extra_prerequisites.len()
            ))
            .dim();
        }
        ui.separator();

        // Sibling cluster — the locale-key headline view
        if !c.title_siblings.is_empty() {
            let _ = ui.row(|ui| {
                ui.text("Title siblings").bold();
                ui.spacer();
                ui.text(format!("{}", c.title_siblings.len())).dim();
            });
            ui.text("(other contracts sharing this title/description)")
                .dim();
            for sib_id in &c.title_siblings {
                if let Some(sib) = by_id.get(sib_id) {
                    let bp = if sib.rewards.blueprint.is_some() { "[BP] " } else { "" };
                    ui.text(format!(
                        "  • {bp}{}  ({:?})",
                        sib.debug_name, sib.handler_kind
                    ));
                } else {
                    ui.text(format!("  • <missing> {sib_id:?}")).fg(Color::Red);
                }
            }
            ui.separator();
        }

        // Encounters — full per-slot breakdown so we can investigate
        // typed signals (initial_damage_settings, spawn_identifiers,
        // markup / entity / negative tag bags, ...) interactively
        // against real contracts.
        if !c.encounters.is_empty() {
            let _ = ui.row(|ui| {
                ui.text("Encounters").bold();
                ui.spacer();
                ui.text(format!("{}", c.encounters.len())).dim();
            });
            for g in &c.encounters {
                ui.text(format!("  ▸ {}", g.variable_name)).bold();
                if !g.extended_text_token.is_empty() {
                    ui.text(format!("      ext_token: {}", g.extended_text_token))
                        .fg(Color::Yellow);
                }
                for w in &g.waves {
                    let wave_label = if w.name.is_empty() {
                        "(unnamed wave)".to_string()
                    } else {
                        format!("wave \"{}\"", w.name)
                    };
                    ui.text(format!("      {wave_label} — {} slot(s)", w.slots.len()))
                        .dim();
                    for (i, s) in w.slots.iter().enumerate() {
                        render_slot(ui, i, s, tree);
                    }
                }
            }
            ui.separator();
        }

        // Rewards
        let _ = ui.row(|ui| {
            ui.text("Rewards").bold();
        });
        let uec = match c.rewards.uec {
            RewardAmount::None => "none".to_string(),
            RewardAmount::Calculated => "calculated (engine-computed)".to_string(),
            RewardAmount::Fixed(n) => format!("fixed {n} aUEC"),
        };
        kv(ui, "uec", &uec);
        for s in &c.rewards.scrip {
            ui.text(format!("  scrip: {} × {}", s.amount, s.display_name));
        }
        for r in &c.rewards.reputation {
            let amount = r.amount.map(|n| n.to_string()).unwrap_or_else(|| "calc".into());
            ui.text(format!("  rep: {amount}"));
        }
        for it in &c.rewards.items {
            ui.text(format!("  item: {} × {}", it.amount, it.display_name));
        }
        if let Some(bp) = &c.rewards.blueprint {
            ui.text(format!(
                "  bp pool: {} ({:.0}% chance, {} item(s))",
                bp.pool_name,
                bp.chance * 100.0,
                bp.items.len()
            ));
        }
        if !c.mission_span.is_empty() {
            ui.separator();
            kv(ui, "mission_span", &format!("{} locality refs", c.mission_span.len()));
        }
    });
}

/// Render one [`EncounterSlot`] with all four tag bags surfaced.
/// `tree` is required for the subtree-classified iterators on
/// [`crate::TagBag`].
fn render_slot(ui: &mut Context, i: usize, s: &EncounterSlot, tree: &TagTree) {
    // Slot header: concurrency / weight / candidate count + flag markers.
    let damage_marker = if s.initial_damage_settings.is_some() {
        " [pre-damaged]"
    } else {
        ""
    };
    let loc_marker = if s.include_location_ai_spawn_tags {
        " [+location-tags]"
    } else {
        ""
    };
    let recovery_marker = if s.positive.is_cargo_recovery() {
        " [cargo-recovery]"
    } else if s.positive.is_pre_damaged_wreck() {
        " [wreck]"
    } else {
        ""
    };
    ui.text(format!(
        "        slot {i}: {}× w={:.2} candidates={}{damage_marker}{loc_marker}{recovery_marker}",
        s.concurrent,
        s.weight,
        s.candidates.len(),
    ));

    // Positive bag — classified into subtree categories.
    let spawn_ids: Vec<&str> = s.positive.spawn_identifiers(tree).collect();
    if !spawn_ids.is_empty() {
        ui.text(format!("            spawn_identifiers: {}", spawn_ids.join(", ")))
            .fg(Color::Cyan);
    }
    let factions: Vec<&str> = s.positive.factions(tree).collect();
    if !factions.is_empty() {
        ui.text(format!("            factions: {}", factions.join(", "))).dim();
    }
    let cargo: Vec<&str> = s.positive.cargo(tree).collect();
    if !cargo.is_empty() {
        ui.text(format!("            cargo: {}", cargo.join(", "))).dim();
    }
    let ai_traits: Vec<&str> = s.positive.ai_traits(tree).collect();
    if !ai_traits.is_empty() {
        ui.text(format!("            ai_traits: {}", ai_traits.join(", "))).dim();
    }
    let mission_tags: Vec<&str> = s.positive.mission_tags(tree).collect();
    if !mission_tags.is_empty() {
        ui.text(format!("            mission_tags: {}", mission_tags.join(", "))).dim();
    }
    if let Some(skill) = s.positive.ai_skill() {
        let ace = if s.positive.ace_pilot() { " + AcePilot" } else { "" };
        ui.text(format!("            ai_skill: HumanPilot{skill}{ace}")).dim();
    }
    let directives: Vec<&str> = s.positive.directives().collect();
    if !directives.is_empty() {
        ui.text(format!("            directives: {}", directives.join(", "))).dim();
    }
    let other: Vec<&str> = s.positive.other(tree).collect();
    if !other.is_empty() {
        ui.text(format!("            other: {}", other.join(", "))).dim();
    }

    // Sibling tag bags — flat names; consumers can classify if needed.
    if !s.negative.is_empty() {
        ui.text(format!("            negative: {}", s.negative.names.join(", "))).dim();
    }
    if !s.markup.is_empty() {
        ui.text(format!("            markup: {}", s.markup.names.join(", "))).dim();
    }
    if !s.entity.is_empty() {
        ui.text(format!("            entity: {}", s.entity.names.join(", "))).dim();
    }

    // Candidate ships — first 4 to keep the panel readable.
    let preview: Vec<String> = s
        .candidates
        .iter()
        .take(4)
        .map(|cand| cand.display_name.clone())
        .collect();
    let extra = s.candidates.len().saturating_sub(preview.len());
    if !preview.is_empty() {
        let suffix = if extra > 0 {
            format!(" (+{extra} more)")
        } else {
            String::new()
        };
        ui.text(format!("            ships: {}{suffix}", preview.join(", "))).dim();
    }
}

fn kv(ui: &mut Context, key: &str, value: &str) {
    let _ = ui.row(|ui| {
        ui.text(format!("  {key:<16}")).dim();
        ui.text(value);
    });
}

/// Approximate row budget for the contracts list. The contracts tab
/// stacks: outer border (2) + header row (1) + filter (3) + sep (1) +
/// list border (2) + list pad (2) + tab tabs/help/sep above the body
/// (~5). Take terminal height minus that. The fudge is intentional —
/// over-budgeting (by ~2 rows) means the "↓ N more" indicator fits.
fn list_visible_rows(ui: &Context) -> usize {
    (ui.height() as usize).saturating_sub(18).max(5)
}

/// Custom selection-aware list with a viewport that always contains
/// `state.selected`. Replaces SLT's `ui.list` (which renders every item
/// without scrolling — invisible cursor when selection moves off-panel)
/// and `ui.virtual_list` (whose closure can't see the post-keypress
/// `state.selected` so PageUp/PageDown render the highlight on a stale
/// row). Owns key handling + viewport + render together.
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

    // Reserve a row for each indicator so they don't push items out.
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
            ui.text(format!("► {text}"))
                .bold()
                .fg(if focused { Color::Cyan } else { Color::White });
        } else {
            ui.text(format!("  {text}"));
        }
    }
    let remaining = total.saturating_sub(end);
    if remaining > 0 {
        ui.text(format!("  ↓ {remaining} more")).dim();
    }
}
