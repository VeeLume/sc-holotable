//! Locale-key cluster explorer pane — groups contracts by their
//! `title_key` or `description_key` and surfaces per-axis divergence.
//!
//! Two sub-tabs: **Title** and **Description**. Each lists the
//! clusters for the chosen key, sorted with divergent ones first
//! (most actionable for patcher tools). The detail pane shows the
//! cluster's divergence summary plus a per-variant breakdown so the
//! consumer can see at a glance which fields disagree across members.

use std::collections::HashMap;

use sc_extract::{Guid, LocaleMap};
use slt::{Border, Color, Context, KeyCode, ListState, ScrollState, TabsState, TextInputState};

use crate::clusters::{KeyCluster, cluster_by_description_key, cluster_by_title_key};
use crate::expand::RewardAmount;
use crate::index::ContractIndex;
use crate::locality::LocalityRegistry;
use crate::merge::Contract;

const SUB_TITLE: usize = 0;
const SUB_DESC: usize = 1;

/// State for the cluster explorer pane. Filter, sub-tab selection,
/// and list/detail scroll positions persist across frames.
pub struct ClustersState {
    pub sub_tabs: TabsState,
    pub filter: TextInputState,
    pub list: ListState,
    pub detail_scroll: ScrollState,
}

impl ClustersState {
    pub fn new() -> Self {
        Self {
            sub_tabs: TabsState::new(vec!["Title keys".to_string(), "Description keys".to_string()]),
            filter: TextInputState::with_placeholder("filter key / resolved text"),
            list: ListState::new(Vec::<String>::new()),
            detail_scroll: ScrollState::new(),
        }
    }
}

impl Default for ClustersState {
    fn default() -> Self {
        Self::new()
    }
}

/// Render the cluster explorer pane.
///
/// The pane reads the sub-tab selection from `state.sub_tabs.selected`
/// and groups `index.contracts` accordingly. `localities` is used to
/// resolve `mission_span` GUIDs to a short region label in the
/// per-variant table; pass `&index.localities` from the calling
/// context.
pub fn render(
    ui: &mut Context,
    state: &mut ClustersState,
    index: &ContractIndex,
    locale: &LocaleMap,
) {
    // Compute clusters for the selected grouping. Cheap to redo every
    // frame on 1.6k contracts; revisit if it shows up in a profile.
    let clusters: Vec<KeyCluster<'_>> = match state.sub_tabs.selected {
        SUB_TITLE => cluster_by_title_key(&index.contracts),
        _ => cluster_by_description_key(&index.contracts),
    };
    let _ = locale; // reserved for future per-key live resolution

    let filter = state.filter.value.to_lowercase();
    let filtered: Vec<usize> = clusters
        .iter()
        .enumerate()
        .filter(|(_, c)| matches_filter(c, &filter))
        .map(|(i, _)| i)
        .collect();

    let divergent_count = clusters.iter().filter(|c| c.divergence.any()).count();
    let multi_member_count = clusters.iter().filter(|c| c.members.len() > 1).count();

    if filtered.is_empty() {
        state.list.selected = 0;
    } else {
        state.list.selected = state.list.selected.min(filtered.len().saturating_sub(1));
    }

    let _ = ui.container().grow(1).col(|ui| {
        let _ = ui.row(|ui| {
            ui.text("Clusters").bold().fg(Color::Cyan);
            ui.spacer();
            ui.text(format!(
                "{} matching · {} divergent · {} multi-member · {} total",
                filtered.len(),
                divergent_count,
                multi_member_count,
                clusters.len(),
            ))
            .dim();
        });
        let _ = ui.tabs(&mut state.sub_tabs);
        let _ = ui.container().h(3).col(|ui| {
            let _ = ui.text_input(&mut state.filter);
        });
        ui.separator();

        let _ = ui.container().grow(1).row(|ui| {
            // Left: cluster list
            let _ = ui
                .bordered(Border::Rounded)
                .title(if state.sub_tabs.selected == SUB_TITLE {
                    "Title clusters"
                } else {
                    "Description clusters"
                })
                .pad(1)
                .grow(1)
                .col(|ui| {
                    if filtered.is_empty() {
                        ui.text("(no matches)").dim();
                    } else {
                        let items: Vec<String> = filtered
                            .iter()
                            .map(|&i| format_list_item(&clusters[i], state.sub_tabs.selected))
                            .collect();
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
                        &clusters[idx],
                        state.sub_tabs.selected,
                        &index.localities,
                    ),
                    None => {
                        ui.text("(nothing selected)").dim();
                    }
                });
        });
    });
}

// ── Filter / list rendering ─────────────────────────────────────────────────

fn matches_filter(cluster: &KeyCluster<'_>, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    if cluster.key.as_str().to_lowercase().contains(filter) {
        return true;
    }
    if let Some(t) = cluster.resolved_text
        && t.to_lowercase().contains(filter)
    {
        return true;
    }
    false
}

fn format_list_item(cluster: &KeyCluster<'_>, sub_tab: usize) -> String {
    let count = cluster.members.len();
    let warn = if (sub_tab == SUB_TITLE && cluster.divergence.any_title())
        || (sub_tab == SUB_DESC && cluster.divergence.any_description())
        || cluster.divergence.handler_kind_mixed
    {
        '⚠'
    } else if cluster.divergence.any() {
        // Other-axis divergence — flag mildly so it isn't completely silent.
        '·'
    } else {
        ' '
    };
    let runtime = if cluster.divergence.has_runtime_substitution {
        '~'
    } else {
        ' '
    };
    let label = cluster.resolved_text.unwrap_or(cluster.key.as_str());
    format!("{count:>3} {warn}{runtime} {label}")
}

// ── Detail pane ─────────────────────────────────────────────────────────────

fn render_detail(
    ui: &mut Context,
    scroll: &mut ScrollState,
    cluster: &KeyCluster<'_>,
    sub_tab: usize,
    localities: &LocalityRegistry,
) {
    let _ = ui.scrollable(scroll).grow(1).col(|ui| {
        // Header
        ui.text(cluster.resolved_text.unwrap_or("(no resolved text)"))
            .bold()
            .fg(Color::Cyan);
        ui.text(cluster.key.as_str()).dim();
        ui.text(format!("{} variant(s)", cluster.members.len()));
        if cluster.divergence.has_runtime_substitution {
            ui.text("contains ~mission(...) runtime substitution")
                .fg(Color::Yellow);
        }
        ui.separator();

        // Two divergence groups, with the active sub-tab's group emphasised.
        let title_color = if sub_tab == SUB_TITLE {
            Color::Cyan
        } else {
            Color::Indexed(245)
        };
        let desc_color = if sub_tab == SUB_DESC {
            Color::Cyan
        } else {
            Color::Indexed(245)
        };

        ui.text("Title-relevant divergence").bold().fg(title_color);
        ui.text("  (informs which title tags are safe to add)").dim();
        flag_row(ui, "blueprint mixed", cluster.divergence.blueprint_mixed);
        flag_row(ui, "once_only mixed", cluster.divergence.once_only_mixed);
        flag_row(ui, "shareable mixed", cluster.divergence.shareable_mixed);
        flag_row(ui, "illegal_flag mixed", cluster.divergence.illegal_mixed);
        ui.separator();

        ui.text("Description-relevant divergence").bold().fg(desc_color);
        ui.text("  (informs which info-block lines are safe)").dim();
        flag_row(ui, "reward_uec", cluster.divergence.reward_uec);
        flag_row(ui, "reward_scrip", cluster.divergence.reward_scrip);
        flag_row(ui, "reward_rep", cluster.divergence.reward_rep);
        flag_row(ui, "blueprint_pool", cluster.divergence.blueprint_pool);
        flag_row(ui, "mission_span", cluster.divergence.mission_span);
        flag_row(ui, "encounters", cluster.divergence.encounters);
        flag_row(ui, "cooldowns", cluster.divergence.cooldowns);
        ui.separator();

        if cluster.divergence.handler_kind_mixed {
            ui.text("handler_kind mixed across members")
                .fg(Color::Yellow);
            ui.separator();
        }

        // Cross-key breakdown: how do this cluster's members split
        // across the *other* key? Surfaces the bounty-hunter case
        // where one description_key is shared by 12 variants spread
        // across multiple title_keys — a description-block patch goes
        // to all 12, but title-tag patches only land on the ones in
        // each title cluster, so a `[BP]` mention in the description
        // can be stranded for variants whose title cluster doesn't
        // get the marker.
        render_cross_key_breakdown(ui, cluster, sub_tab);

        // Variants table
        ui.text("Variants").bold();
        let _ = ui.row(|ui| {
            ui.text(format!("  {:<32}", "DEBUG NAME")).dim();
            ui.text(format!("{:<8}", "HANDLER")).dim();
            ui.text(format!("{:<5}", "BP")).dim();
            ui.text(format!("{:<11}", "UEC")).dim();
            ui.text(format!("{:<10}", "SCRIP")).dim();
            ui.text(format!("{:<5}", "REP")).dim();
            ui.text(format!("{:<5}", "ILL")).dim();
            ui.text("REGION").dim();
        });
        for c in &cluster.members {
            let bp = if c.blueprint_reward.is_some() { "yes" } else { "no" };
            let uec = match c.reward_uec {
                RewardAmount::None => "none".to_string(),
                RewardAmount::Calculated => "calc".to_string(),
                RewardAmount::Fixed(n) => format!("{n}"),
            };
            let scrip = scrip_label(&c.reward_scrip);
            let rep = if c.reward_rep.is_empty() {
                "-".to_string()
            } else {
                format!("{}", c.reward_rep.len())
            };
            let illegal = if c.illegal_flag { "yes" } else { "no" };
            let region = region_label(&c.mission_span, localities);
            let _ = ui.row(|ui| {
                ui.text(format!("  {:<32}", clip(&c.debug_name, 32)));
                ui.text(format!("{:<8}", short_handler(c)));
                ui.text(format!("{bp:<5}"));
                ui.text(format!("{uec:<11}"));
                ui.text(format!("{:<10}", clip(&scrip, 10)));
                ui.text(format!("{rep:<5}"));
                ui.text(format!("{illegal:<5}"));
                ui.text(clip(&region, 40));
            });
        }
    });
}

/// Group the cluster's members by the **other** key (description_key
/// for a title cluster, title_key for a description cluster) and show
/// per-group counts plus how many members in that group have a
/// blueprint reward. The crucial signal: if a description cluster has
/// a single title_key with `bp 0/N` but the description cluster's own
/// `blueprint_mixed` is true (so the description block could mention
/// BP), the title for those N members would never get a `[BP]` tag —
/// resulting in misinformation.
fn render_cross_key_breakdown(ui: &mut Context, cluster: &KeyCluster<'_>, sub_tab: usize) {
    let (heading, group_label) = if sub_tab == SUB_DESC {
        ("Title clusters touching this description", "title_key")
    } else {
        ("Description clusters touching this title", "description_key")
    };

    // Group members by the other key. None-keyed members go into a
    // single "(no key)" bucket so the user sees them too.
    let mut groups: HashMap<Option<&str>, Vec<&Contract>> = HashMap::new();
    for &m in &cluster.members {
        let other = if sub_tab == SUB_DESC {
            m.title_key.as_ref().map(|k| k.as_str())
        } else {
            m.description_key.as_ref().map(|k| k.as_str())
        };
        groups.entry(other).or_default().push(m);
    }

    let total_groups = groups.len();
    let any_bp_present = cluster
        .members
        .iter()
        .any(|c| c.blueprint_reward.is_some());

    let _ = ui.row(|ui| {
        ui.text(heading).bold();
        ui.spacer();
        ui.text(format!(
            "{} distinct {group_label}{}",
            total_groups,
            if total_groups == 1 { "" } else { "s" }
        ))
        .dim();
    });

    // Sort groups by size desc, with the (no key) bucket pinned to the end.
    let mut entries: Vec<(Option<&str>, Vec<&Contract>)> = groups.into_iter().collect();
    entries.sort_by(|a, b| match (a.0.is_some(), b.0.is_some()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => b.1.len().cmp(&a.1.len()),
    });

    for (key, members) in entries {
        let bp_count = members.iter().filter(|c| c.blueprint_reward.is_some()).count();
        let n = members.len();
        let bp_summary = format!("BP {bp_count}/{n}");
        let key_label = key.unwrap_or("(no key)");

        // Highlight the cross-key inconsistency: if the cluster as a
        // whole could attract a description-side BP mention (any
        // member has BP) but THIS group has zero BP-having members,
        // the title cluster for these members will never get `[BP]`
        // even though the shared description might describe one.
        let warn = sub_tab == SUB_DESC && any_bp_present && bp_count == 0;
        let _ = ui.row(|ui| {
            ui.text(format!("  {n:>3} × ")).dim();
            if warn {
                ui.text(clip(key_label, 60)).fg(Color::Yellow);
                ui.text("  ").dim();
                ui.text(&bp_summary).fg(Color::Yellow);
                ui.spacer();
                ui.text("⚠ no BP — title would lack [BP]")
                    .fg(Color::Yellow);
            } else {
                ui.text(clip(key_label, 60));
                ui.text("  ").dim();
                let bp_color = if bp_count == n && n > 0 && any_bp_present {
                    Color::Green
                } else if bp_count == 0 {
                    Color::Indexed(245)
                } else {
                    Color::Yellow
                };
                ui.text(&bp_summary).fg(bp_color);
            }
        });
    }
    ui.separator();
}

fn flag_row(ui: &mut Context, label: &str, mixed: bool) {
    let (text, color) = if mixed {
        ("differs", Color::Yellow)
    } else {
        ("uniform", Color::Green)
    };
    let _ = ui.row(|ui| {
        ui.text(format!("  {label:<22}")).dim();
        ui.text(text).fg(color);
    });
}

fn short_handler(c: &Contract) -> &'static str {
    use crate::expand::HandlerKind::*;
    match c.handler_kind {
        Career => "Career",
        Legacy => "Legacy",
        List => "List",
        LinearSeries => "LinSer",
        Tutorial => "Tut",
        PVPBounty => "PvPB",
        ServiceBeacon => "Svc",
        Unknown => "?",
    }
}

fn scrip_label(scrip: &[crate::expand::ScripReward]) -> String {
    if scrip.is_empty() {
        return "-".into();
    }
    // Show the first reward; suffix with `+N` if there are more.
    let first = &scrip[0];
    if scrip.len() == 1 {
        format!("{}×{}", first.amount, abbreviate_currency(&first.display_name))
    } else {
        format!(
            "{}×{} +{}",
            first.amount,
            abbreviate_currency(&first.display_name),
            scrip.len() - 1
        )
    }
}

fn abbreviate_currency(name: &str) -> String {
    // "MG Scrip" → "MG", "Council Scrip" → "Cnc", anything else → first 3 chars.
    if name.is_empty() {
        return "?".into();
    }
    if name.starts_with("MG ") {
        return "MG".into();
    }
    if name.starts_with("Council") {
        return "Cnc".into();
    }
    name.chars().take(3).collect()
}

fn region_label(span: &[Guid], localities: &LocalityRegistry) -> String {
    if span.is_empty() {
        return "-".into();
    }
    // Pick the first locality with a region label; fall back to count.
    for guid in span {
        if let Some(view) = localities.get(guid)
            && !view.region_label.is_empty()
        {
            return view.region_label.clone();
        }
    }
    format!("{} loc", span.len())
}

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

// ── Scrolled list helper (mirrors the one in tui::mod) ──────────────────────

fn list_visible_rows(ui: &Context) -> usize {
    (ui.height() as usize).saturating_sub(20).max(5)
}

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

