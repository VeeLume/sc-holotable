//! Spot-check `~mission(...)` marker substitution: raw vs resolved title +
//! description, and the resolved variable map (cargo-grade, numeric, location).
//!
//! ```bash
//! cargo run -p sc-missions --release --example marker_check
//! ```

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, LocaleMap};
use sc_missions::{MissionVar, Missions, RecordCollection};

fn dump_vars(m: &sc_missions::Mission, locale: &LocaleMap) -> String {
    let mut parts = Vec::new();
    for (token, var) in &m.variables {
        let rendered = match var {
            MissionVar::Choice(opts) => {
                let labels: Vec<String> = opts
                    .iter()
                    .map(|o| {
                        let l = locale.resolve(&o.label_key).unwrap_or("?");
                        if o.gated {
                            format!("{l}(gated)")
                        } else {
                            l.to_string()
                        }
                    })
                    .collect();
                format!("Choice[{}]", labels.join(","))
            }
            MissionVar::Number(n) => format!("Number{n:?}"),
            MissionVar::Location { systems, settings } => {
                format!("Location{{sys={systems:?} set={settings:?}}}")
            }
        };
        parts.push(format!("{token}={rendered}"));
    }
    parts.join("  ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let asset_data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &asset_data)?;
    let locale = &asset_data.locale;
    let index = Missions::build(&datacore);

    let mut shown_loc = 0;
    let mut shown_eliminate = false;
    for m in index.values() {
        let title = m.title(locale).unwrap_or("");
        let desc = m.description(locale).unwrap_or("");
        let has_loc = title.contains("Location")
            || title.contains("Destination")
            || desc.contains("Location")
            || desc.contains("Destination");
        if !has_loc {
            continue;
        }
        // Always show Eliminate Annoyance (the worked example), then a sample.
        let is_eliminate = title.contains("Eliminate Annoyance");
        if is_eliminate {
            if shown_eliminate {
                continue;
            }
            shown_eliminate = true;
        } else if shown_loc >= 10 {
            continue;
        } else {
            shown_loc += 1;
        }
        let rt = index.title_text(m, locale).unwrap_or_default();
        let rd = index.description_text(m, locale).unwrap_or_default();
        println!("TITLE raw : {title}");
        println!("TITLE new : {rt}");
        if desc.contains("~mission(") {
            let d1: String = rd.chars().take(160).collect();
            println!("DESC  new : {d1}");
        }
        println!("  vars: {}\n", dump_vars(m, locale));
        if shown_eliminate && shown_loc >= 10 {
            break;
        }
    }

    Ok(())
}
