//! Which DCB struct fields hold socpak paths, counted per (struct, field),
//! with an example; `--grep <text>` lists the instances whose path
//! contains it.
//!
//! ```bash
//! cargo run -p sc-loot --release --example socpak_refs_dig -- --grep flagship/stanton
//! ```

use std::collections::BTreeMap;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let db = datacore.db();
    let grep = std::env::args()
        .skip_while(|a| a != "--grep")
        .nth(1)
        .map(|s| s.to_ascii_lowercase());
    let mut fields: BTreeMap<(String, String), (usize, String)> = BTreeMap::new();
    for m in db.data_mappings() {
        let Ok(index) = usize::try_from(m.struct_index) else {
            continue;
        };
        let sname = db.struct_name(index).unwrap_or("?").to_string();
        for i in 0..m.struct_count {
            for p in db.instance(index as u32, i).properties() {
                let Value::String(s) = p.value else { continue };
                let low = s.to_ascii_lowercase();
                if !low.contains(".socpak") {
                    continue;
                }
                let e = fields
                    .entry((sname.clone(), p.name.to_string()))
                    .or_insert((0, s.to_string()));
                e.0 += 1;
                if let Some(g) = &grep
                    && low.contains(g.as_str())
                {
                    println!("  {sname}.{} = {s}", p.name);
                }
            }
        }
    }
    for ((s, f), (n, ex)) in &fields {
        println!("{n:>6}  {s}.{f}   e.g. {ex}");
    }
    Ok(())
}
