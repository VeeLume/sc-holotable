//! Decode a standalone CryXMLB (or plain-XML) p4k file into the XmlNode tree
//! and print tags + attribute keys/values. Used to see whether standalone
//! Data/ObjectContainers and Data/Prefabs XMLs carry GUID attributes.
//!
//! ```bash
//! cargo run -p sc-locations --release --example cryxml_show -- <exact/path>
//! ```
use sc_extract::AssetSource;
use sc_extract::object_container;

fn walk(n: &object_container::XmlNode, depth: usize, out: &mut usize) {
    if *out > 120 {
        return;
    }
    let indent = "  ".repeat(depth.min(8));
    let attrs: Vec<String> = n.attrs.iter().map(|(k, v)| format!("{k}={v}")).collect();
    println!("{indent}<{}> {}", n.tag, attrs.join(" "));
    *out += 1;
    for c in &n.children {
        walk(c, depth + 1, out);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("path arg");
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let bytes = assets.read(&path)?;
    let node = object_container::decode(&bytes)?.ok_or("no cryxml node")?;
    let mut out = 0usize;
    walk(&node, 0, &mut out);
    Ok(())
}
