//! Dump (decompress + CryXMLB-decode if needed) a single p4k file by exact path.
//! ```bash
//! cargo run -p sc-locations --release --example xml_dump -- <exact/path>
//! ```
use sc_extract::AssetSource;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("path arg");
    let install = sc_discovery::discover_primary()?;
    let assets = AssetSource::from_install(&install)?;
    let bytes = assets.read(&path)?;
    eprintln!(
        "len={} first16={:02x?}",
        bytes.len(),
        &bytes[..16.min(bytes.len())]
    );
    // CryXMLB magic?
    if bytes.starts_with(b"CryXmlB") || bytes.starts_with(b"CryXml") {
        eprintln!("(CryXMLB binary)");
    }
    // dump as text if it looks textual
    let txt = String::from_utf8_lossy(&bytes);
    let head: String = txt.chars().take(4000).collect();
    println!("{head}");
    Ok(())
}
