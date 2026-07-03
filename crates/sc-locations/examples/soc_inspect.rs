//! Diagnostic: how is the within-`stantonsystem.socpak` OOC hierarchy encoded?
//! Walk the entity tree PRESERVING nesting (track each Entity's nearest ancestor
//! Entity), and dump the top OOC nodes + the attrs of the chain nodes
//! `4a269932` (Stanton) / `4ce1f681` / `448d7bcf`. Answers: XML nesting vs
//! objectContainer vs a parent attr we mislabeled.
//!
//! ```bash
//! cargo run -p sc-locations --release --example soc_inspect
//! ```
use sc_extract::AssetSource;
use sc_extract::object_container::{Socpak, XmlNode, decode};

const TARGETS: &[&str] = &["4a269932", "4ce1f681", "448d7bcf", "4c557c37"];

/// Recursive walk tracking the nearest ancestor `Entity`'s cry.
fn walk(node: &XmlNode, parent_cry: Option<&str>, depth: usize, out: &mut Vec<String>) {
    let mut next_parent = parent_cry;
    let mut this_cry = None;
    if node.tag == "Entity" {
        let cry = node
            .attr("EntityCryGUID")
            .unwrap_or("")
            .to_ascii_lowercase();
        let class = node.attr("EntityClass").unwrap_or("");
        let _name = node.attr("Name").unwrap_or("");
        // objectContainer + any parent-ish attr anywhere in this entity's subtree
        let oc = node.descendants().find_map(|n| n.attr("objectContainer"));
        let parentish: Vec<String> = node
            .descendants()
            .flat_map(|n| n.attrs.iter())
            .filter(|(k, _)| {
                let kl = k.to_ascii_lowercase();
                kl.contains("parent")
                    || (kl.contains("guid") && k != "EntityCryGUID" && k != "EntityClassGUID")
            })
            .map(|(k, v)| format!("{k}={}", v.get(..13).unwrap_or(v)))
            .collect();
        if class.contains("ObjectContainer") {
            let short = |s: &str| s.get(..8).unwrap_or(s).to_string();
            let hit = TARGETS.contains(&short(&cry).as_str());
            if depth < 6 || hit {
                out.push(format!(
                    "{}{}{} [{}] xmlparent={} oc={} {}",
                    "  ".repeat(depth.min(8)),
                    if hit { "★ " } else { "" },
                    short(&cry),
                    class,
                    parent_cry.map(short).unwrap_or_else(|| "—".into()),
                    oc.map(|s| s.rsplit('/').next().unwrap_or(s).to_string())
                        .unwrap_or_else(|| "—".into()),
                    if parentish.is_empty() {
                        String::new()
                    } else {
                        format!("attrs:[{}]", parentish.join(","))
                    },
                ));
            }
        }
        this_cry = Some(cry);
        next_parent = this_cry.as_deref();
    }
    let _ = this_cry;
    for c in &node.children {
        walk(c, next_parent, depth + (node.tag == "Entity") as usize, out);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let sp = assets
        .find(|n| {
            n.to_ascii_lowercase()
                .replace('\\', "/")
                .ends_with("system/stanton/stantonsystem.socpak")
        })
        .next()
        .map(|e| e.name.to_string())
        .or_else(|| {
            assets
                .find(|n| n.to_ascii_lowercase().ends_with("stantonsystem.socpak"))
                .next()
                .map(|e| e.name.to_string())
        })
        .expect("stantonsystem.socpak not found");
    eprintln!("opening {sp}");
    let mut pak = Socpak::open(assets.read(&sp)?)?;
    eprintln!("members: {}", pak.len());

    // member-kind histogram + walk each .soc/.entxml tree
    let mut out = Vec::new();
    let mut kinds = std::collections::BTreeMap::new();
    for m in 0..pak.len() {
        let nm = pak.name(m).unwrap_or_default().to_ascii_lowercase();
        let ext = nm.rsplit('.').next().unwrap_or("").to_string();
        *kinds.entry(ext.clone()).or_insert(0) += 1;
        if !(nm.ends_with(".soc") || nm.ends_with(".pla") || nm.ends_with(".entxml")) {
            continue;
        }
        let Ok(b) = pak.read(m) else { continue };
        let Ok(Some(root)) = decode(&b) else { continue };
        walk(&root, None, 0, &mut out);
    }
    eprintln!("member kinds: {kinds:?}\n");
    eprintln!("OOC entities (depth<6 or ★target), {} lines:", out.len());
    for l in out.iter().take(120) {
        println!("{l}");
    }
    Ok(())
}
