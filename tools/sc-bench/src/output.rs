//! Benchmark result structure and formatting.

use serde::Serialize;

/// Complete benchmark result emitted as JSON on stdout.
///
/// The bench script (`tools/bench/bench.ps1`) parses this via
/// `ConvertFrom-Json`. Fields may be zero when the corresponding
/// phase is skipped or when no features are enabled (0 records).
#[derive(Debug, Default, Serialize)]
pub struct BenchResult {
    // ── Core counts ──────────────────────────────────────────────────
    pub records: usize,
    pub locale_entries: usize,
    pub manufacturers: usize,
    pub display_names: usize,
    pub tag_nodes: usize,
    pub graph_edges: usize,

    // ── Phase timings (seconds) ──────────────────────────────────────
    pub asset_open_time: f64,
    pub asset_extract_time: f64,
    pub parse_time: f64,

    // ── Tag tree exercise ────────────────────────────────────────────
    pub tag_roots: usize,
    pub tag_max_depth: usize,
    pub tag_exercise_time: f64,

    // ── Manufacturer exercise ────────────────────────────────────────
    pub manufacturers_with_name_key: usize,
    pub manufacturer_exercise_time: f64,

    // ── Display name exercise ────────────────────────────────────────
    pub display_names_non_empty: usize,
    pub display_name_exercise_time: f64,

    // ── Locale exercise ──────────────────────────────────────────────
    pub locale_exercise_time: f64,

    // ── Graph exercise (only with --all) ─────────────────────────────
    pub graph_sources: usize,
    pub graph_targets: usize,
    pub graph_depth2_reachable: usize,
    pub graph_exercise_time: f64,

    // ── Filter predicates ────────────────────────────────────────────
    pub total_entities: usize,
    pub playable_weapons: usize,
    pub playable_ships: usize,
    pub filter_exercise_time: f64,

    // ── Raw escape hatch counts ──────────────────────────────────────
    pub raw_entity_count: usize,
    pub raw_manufacturer_count: usize,
    pub raw_tag_count: usize,
    pub raw_ammo_count: usize,
    pub raw_query_time: f64,

    // ── Snapshot round-trip ──────────────────────────────────────────
    pub snapshot_capture_time: f64,
    pub snapshot_size_mb: f64,
    pub snapshot_save_time: f64,
    pub snapshot_load_time: f64,
    pub snapshot_hydrate_time: f64,
    pub snapshot_hydrated_records: usize,
}

impl BenchResult {
    /// Print a human-readable summary table to stdout.
    pub fn print_human(&self) {
        println!();
        println!("sc-bench results");
        println!("================");
        println!();

        println!("Timings");
        println!("-------");
        println!("  asset open     : {:.2}s", self.asset_open_time);
        println!("  asset extract  : {:.2}s", self.asset_extract_time);
        println!("  parse          : {:.2}s", self.parse_time);
        if self.snapshot_capture_time > 0.0 {
            println!("  snap capture   : {:.2}s", self.snapshot_capture_time);
            println!("  snap save      : {:.2}s", self.snapshot_save_time);
            println!("  snap load      : {:.2}s", self.snapshot_load_time);
            println!("  snap hydrate   : {:.2}s", self.snapshot_hydrate_time);
        }

        println!();
        println!("Counts");
        println!("------");
        println!("  records        : {}", self.records);
        println!("  locale entries : {}", self.locale_entries);
        println!("  manufacturers  : {}", self.manufacturers);
        println!("  display names  : {}", self.display_names);
        println!("  tag nodes      : {}", self.tag_nodes);
        println!("  graph edges    : {}", self.graph_edges);

        println!();
        println!("Exercises");
        println!("---------");
        println!(
            "  tags               : roots={} max_depth={} ({:.3}s)",
            self.tag_roots, self.tag_max_depth, self.tag_exercise_time
        );
        println!(
            "  manufacturers      : w/ name_key={} ({:.3}s)",
            self.manufacturers_with_name_key, self.manufacturer_exercise_time
        );
        println!(
            "  display names      : non_empty={} ({:.3}s)",
            self.display_names_non_empty, self.display_name_exercise_time
        );
        println!("  locale             : ({:.3}s)", self.locale_exercise_time);
        println!(
            "  filters            : entities={} weapons={} ships={} ({:.3}s)",
            self.total_entities, self.playable_weapons, self.playable_ships,
            self.filter_exercise_time
        );

        if self.graph_edges > 0 {
            println!(
                "  graph              : sources={} targets={} depth2={} ({:.3}s)",
                self.graph_sources, self.graph_targets, self.graph_depth2_reachable,
                self.graph_exercise_time
            );
        }

        println!();
        println!("Raw escape hatch ({:.3}s)", self.raw_query_time);
        println!("----------------");
        println!("  EntityClassDefinition  : {}", self.raw_entity_count);
        println!("  SCItemManufacturer     : {}", self.raw_manufacturer_count);
        println!("  Tag                    : {}", self.raw_tag_count);
        println!("  AmmoParams             : {}", self.raw_ammo_count);

        if self.snapshot_size_mb > 0.0 {
            println!();
            println!("Snapshot");
            println!("--------");
            println!("  size on disk   : {:.2} MB", self.snapshot_size_mb);
            println!("  hydrated recs  : {}", self.snapshot_hydrated_records);
        }

        println!();
        println!("Done.");
    }
}
