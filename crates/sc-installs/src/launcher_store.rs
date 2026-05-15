//! Decrypt the RSI Launcher's `launcher store.json` to enumerate installed
//! channels without depending on launcher-log activity.
//!
//! ## Why this exists
//!
//! The launcher log is a best-effort breadcrumb trail. It can be wiped by
//! a launcher reinstall, rotated by aggressive uninstall scripts, or simply
//! lack entries for a channel the user hasn't touched in months. The
//! `%APPDATA%/rsilauncher/launcher store.json` file, by contrast, is the
//! launcher's authoritative persistent state — every installed channel
//! is listed there with its full path, version label, and status flag,
//! whether or not it has been launched.
//!
//! ## Format
//!
//! The file is the standard [electron-store](https://github.com/sindresorhus/electron-store)
//! shape — an `aes-256-cbc` ciphertext keyed by a PBKDF2-derived 32-byte
//! key. Layout:
//!
//! ```text
//! [16-byte AES IV] [':' (0x3A)] [PKCS7-padded ciphertext]
//! ```
//!
//! Salt is `IV.toString('utf8')` from JS, which is **lossy** — high-bit IV
//! bytes get folded to U+FFFD before being re-encoded. We replicate that
//! quirk via [`String::from_utf8_lossy`].
//!
//! PBKDF2 parameters: `HMAC-SHA512`, 10,000 iterations, 32-byte output.
//!
//! ## Where the key comes from
//!
//! It's a hardcoded literal inside the launcher's `resources/app.asar`,
//! exposed as a JavaScript object property:
//!
//! ```js
//! { encryptionKey: "<44-char base64 string>" }
//! ```
//!
//! Rather than embed that string in this crate (it's CIG's secret, and any
//! launcher update could rotate it), we extract it at runtime by streaming
//! through `app.asar` looking for the `encryptionKey:"<base64>"` pattern.
//! If extraction fails for any reason — launcher not installed, asar shape
//! changed, key rotated to an unrecognised pattern — we return an error
//! and the caller falls back to log parsing.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use aes::Aes256;
use cbc::Decryptor;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use pbkdf2::pbkdf2_hmac;
use regex::bytes::Regex;
use serde::Deserialize;
use sha2::Sha512;

use crate::channel::Channel;
use crate::error::{Error, Result};

/// One channel entry resolved out of `launcher store.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreInstall {
    pub channel: Channel,
    /// Full installation root, e.g. `C:\Games\StarCitizen\LIVE`.
    pub root: PathBuf,
    /// Launcher-style version label, e.g. `"4.7.2-live.11715810"`.
    pub version_label: Option<String>,
    /// Monotonic Perforce changelist (the int after the channel suffix in
    /// `version_label`).
    pub changelist: Option<u64>,
    /// Auth/services platform tag from the launcher: `"prod"` for LIVE/Hotfix,
    /// `"ptu"` for PTU/EPTU/TECH-PREVIEW. Useful for routing requests to the
    /// right CIG services endpoint without parsing the channel id.
    pub platform_id: Option<String>,
}

/// Combined view of what the launcher's persistent store advertises:
/// every installed channel plus the launcher's own default-channel pick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSnapshot {
    pub installs: Vec<StoreInstall>,
    /// Channel from `library.defaults[].channelId` for game `SC`. This is
    /// the channel the launcher's big "Launch" button would target — a
    /// better signal than `Channel::priority()` for "the channel the user
    /// actually wants to play" (which assumes LIVE > everything else).
    /// `None` if the store doesn't carry a default for SC.
    pub default_channel: Option<Channel>,
}

/// Default path to the launcher's persistent store on Windows:
/// `%APPDATA%/rsilauncher/launcher store.json`.
pub fn launcher_store_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata).join("rsilauncher/launcher store.json")
}

/// Discover installations by reading the launcher's persistent store.
///
/// Auto-locates the launcher install directory (for the asar key) and the
/// store file. Returns an error if any step fails — the caller is expected
/// to fall back to log parsing.
///
/// Entries with `status != "installed"` are skipped (the launcher tracks
/// "available" channels even when they're not on disk).
pub fn read_launcher_store() -> Result<Vec<StoreInstall>> {
    Ok(read_launcher_snapshot()?.installs)
}

/// Like [`read_launcher_store`] but also returns the launcher's
/// default-channel pick for downstream consumers that want
/// `discover_default()`-style semantics.
pub fn read_launcher_snapshot() -> Result<StoreSnapshot> {
    let asar = locate_app_asar()?;
    read_launcher_snapshot_from(&launcher_store_path(), &asar)
}

/// Like [`read_launcher_store`] but with explicit paths to the store file
/// and the launcher's `app.asar`. Useful for tests and for users with
/// non-default installs.
pub fn read_launcher_store_from(store_path: &Path, asar_path: &Path) -> Result<Vec<StoreInstall>> {
    Ok(read_launcher_snapshot_from(store_path, asar_path)?.installs)
}

/// Like [`read_launcher_snapshot`] but with explicit paths.
pub fn read_launcher_snapshot_from(store_path: &Path, asar_path: &Path) -> Result<StoreSnapshot> {
    let store_bytes = std::fs::read(store_path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            Error::LauncherStoreNotFound(store_path.to_path_buf())
        } else {
            Error::LauncherStoreUnreadable {
                path: store_path.to_path_buf(),
                source,
            }
        }
    })?;
    let key_b64 = extract_encryption_key(asar_path)?;
    let plaintext = decrypt_store(&store_bytes, &key_b64)?;
    parse_store_json(&plaintext)
}

/// Probe well-known launcher install locations for `resources/app.asar`.
fn locate_app_asar() -> Result<PathBuf> {
    let candidates = [
        std::env::var("PROGRAMFILES").ok().map(PathBuf::from),
        std::env::var("ProgramW6432").ok().map(PathBuf::from),
        Some(PathBuf::from(r"C:\Program Files")),
    ];
    for base in candidates.into_iter().flatten() {
        let p = base
            .join("Roberts Space Industries")
            .join("RSI Launcher")
            .join("resources")
            .join("app.asar");
        if p.is_file() {
            return Ok(p);
        }
    }
    Err(Error::LauncherInstallNotFound)
}

/// Stream-scan `app.asar` for the launcher's hardcoded encryption key.
///
/// The asar is large (~430 MB) but we only want one ~60-byte literal, so we
/// read in chunks with overlap and bail at the first match. Typical cold-cache
/// scan time is seconds; warm is sub-second.
///
/// Returns the base64 ASCII bytes (44 chars including the trailing `=`),
/// not the decoded key — `electron-store` uses the literal string as the
/// PBKDF2 password input.
fn extract_encryption_key(asar_path: &Path) -> Result<Vec<u8>> {
    static KEY_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"encryptionKey:\s*["']([A-Za-z0-9+/]{43}=)["']"#)
            .expect("encryption-key regex is valid")
    });

    let file = File::open(asar_path).map_err(|source| Error::LauncherAsarUnreadable {
        path: asar_path.to_path_buf(),
        source,
    })?;
    let mut reader = BufReader::with_capacity(1 << 20, file);

    // 1 MB chunks with a 128-byte overlap so the marker survives a chunk
    // boundary. The marker is ~60 bytes; 128 is comfortable headroom.
    const CHUNK: usize = 1 << 20;
    const OVERLAP: usize = 128;
    let mut buf = vec![0u8; CHUNK + OVERLAP];
    let mut carry = 0usize;

    loop {
        let n = reader
            .read(&mut buf[carry..])
            .map_err(|source| Error::LauncherAsarUnreadable {
                path: asar_path.to_path_buf(),
                source,
            })?;
        if n == 0 {
            break;
        }
        let scan_end = carry + n;
        if let Some(caps) = KEY_RE.captures(&buf[..scan_end]) {
            return Ok(caps.get(1).unwrap().as_bytes().to_vec());
        }
        // Carry the tail forward to handle a marker straddling boundaries.
        if scan_end > OVERLAP {
            buf.copy_within(scan_end - OVERLAP..scan_end, 0);
            carry = OVERLAP;
        } else {
            carry = scan_end;
        }
    }

    Err(Error::EncryptionKeyNotFound {
        path: asar_path.to_path_buf(),
    })
}

/// AES-256-CBC decrypt the store payload using the JS-quirk-compatible
/// PBKDF2 derivation.
fn decrypt_store(store_bytes: &[u8], key_b64: &[u8]) -> Result<Vec<u8>> {
    if store_bytes.len() < 17 || store_bytes[16] != b':' {
        return Err(Error::LauncherStoreMalformed(
            "store file does not start with `[16-byte IV] :`".into(),
        ));
    }
    let iv = &store_bytes[..16];
    let ct = &store_bytes[17..];

    // Match electron-store's salt derivation: `IV.toString('utf8')` from JS,
    // which is lossy — invalid UTF-8 byte sequences become U+FFFD, then the
    // result is re-encoded as UTF-8. We mirror that exactly.
    let salt = String::from_utf8_lossy(iv).into_owned().into_bytes();

    let mut derived = [0u8; 32];
    pbkdf2_hmac::<Sha512>(key_b64, &salt, 10_000, &mut derived);

    let mut buf = ct.to_vec();
    let plaintext = Decryptor::<Aes256>::new(&derived.into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| Error::LauncherStoreMalformed(format!("AES-CBC decrypt failed: {e}")))?;
    let len = plaintext.len();
    buf.truncate(len);
    Ok(buf)
}

// ── JSON shape (only what we read) ──────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct RawStore {
    library: RawLibrary,
}

#[derive(Debug, Deserialize)]
struct RawLibrary {
    #[serde(default)]
    installed: Vec<RawGame>,
    #[serde(default)]
    defaults: Vec<RawDefault>,
}

#[derive(Debug, Deserialize)]
struct RawGame {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    channels: Vec<RawChannel>,
}

#[derive(Debug, Deserialize)]
struct RawChannel {
    id: String,
    #[serde(default)]
    status: Option<String>,
    #[serde(default, rename = "libraryFolder")]
    library_folder: Option<String>,
    #[serde(default, rename = "installDir")]
    install_dir: Option<String>,
    #[serde(default, rename = "versionLabel")]
    version_label: Option<String>,
    #[serde(default)]
    version: Option<u64>,
    #[serde(default, rename = "platformId")]
    platform_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawDefault {
    #[serde(default, rename = "gameId")]
    game_id: Option<String>,
    #[serde(default, rename = "channelId")]
    channel_id: Option<String>,
}

/// Game id used by the launcher for Star Citizen. The `library.*` arrays
/// nest per-game; we only care about this one.
const SC_GAME_ID: &str = "SC";

fn parse_store_json(plaintext: &[u8]) -> Result<StoreSnapshot> {
    let raw: RawStore = serde_json::from_slice(plaintext)
        .map_err(|source| Error::LauncherStoreInvalidJson { source })?;

    let mut installs = Vec::new();
    for game in raw.library.installed {
        if game.id.as_deref() != Some(SC_GAME_ID) {
            continue;
        }
        for ch in game.channels {
            let Some(channel) = Channel::from_str_loose(&ch.id) else {
                continue;
            };
            if ch.status.as_deref() != Some("installed") {
                continue;
            }
            let (Some(library), Some(install_dir)) = (ch.library_folder, ch.install_dir) else {
                continue;
            };
            // Path = libraryFolder + installDir + channel id.
            // libraryFolder usually includes a trailing separator; PathBuf
            // join handles either case.
            let root = PathBuf::from(library)
                .join(install_dir)
                .join(channel.install_dir_name());

            installs.push(StoreInstall {
                channel,
                root,
                version_label: ch.version_label,
                changelist: ch.version,
                platform_id: ch.platform_id,
            });
        }
    }

    let default_channel = raw
        .library
        .defaults
        .into_iter()
        .find(|d| d.game_id.as_deref() == Some(SC_GAME_ID))
        .and_then(|d| d.channel_id)
        .and_then(|id| Channel::from_str_loose(&id));

    Ok(StoreSnapshot {
        installs,
        default_channel,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_store_json_extracts_installed_channels() {
        let json = br#"{
            "library": {
                "installed": [
                    { "id": "SC", "channels": [
                        { "id": "LIVE", "status": "installed",
                          "libraryFolder": "C:\\Games\\", "installDir": "StarCitizen",
                          "versionLabel": "4.7.2-live.11715810", "version": 11715810,
                          "platformId": "prod" },
                        { "id": "PTU", "status": "available",
                          "libraryFolder": "C:\\Games\\", "installDir": "StarCitizen" },
                        { "id": "TECH-PREVIEW", "status": "installed",
                          "libraryFolder": "D:\\Games\\", "installDir": "StarCitizen",
                          "versionLabel": "4.7-tp.11650317", "version": 11650317,
                          "platformId": "ptu" }
                    ] }
                ],
                "defaults": [
                    { "gameId": "SC", "channelId": "LIVE" }
                ]
            }
        }"#;

        let snapshot = parse_store_json(json).unwrap();
        assert_eq!(snapshot.installs.len(), 2);
        assert_eq!(snapshot.default_channel, Some(Channel::Live));

        let live = snapshot
            .installs
            .iter()
            .find(|i| i.channel == Channel::Live)
            .unwrap();
        assert_eq!(live.root, PathBuf::from("C:\\Games\\StarCitizen\\LIVE"));
        assert_eq!(live.version_label.as_deref(), Some("4.7.2-live.11715810"));
        assert_eq!(live.changelist, Some(11715810));
        assert_eq!(live.platform_id.as_deref(), Some("prod"));

        let tp = snapshot
            .installs
            .iter()
            .find(|i| i.channel == Channel::TechPreview)
            .unwrap();
        assert_eq!(
            tp.root,
            PathBuf::from("D:\\Games\\StarCitizen\\TECH-PREVIEW")
        );
        assert_eq!(tp.platform_id.as_deref(), Some("ptu"));
    }

    #[test]
    fn parse_store_json_default_for_other_game_ignored() {
        // The launcher could ship Squadron 42 etc. one day; we only care
        // about SC's default.
        let json = br#"{
            "library": {
                "installed": [],
                "defaults": [
                    { "gameId": "SQ42", "channelId": "LIVE" }
                ]
            }
        }"#;
        let snapshot = parse_store_json(json).unwrap();
        assert_eq!(snapshot.default_channel, None);
    }

    #[test]
    fn parse_store_json_default_unknown_channel_is_none() {
        let json = br#"{
            "library": {
                "installed": [],
                "defaults": [
                    { "gameId": "SC", "channelId": "FUTURE" }
                ]
            }
        }"#;
        let snapshot = parse_store_json(json).unwrap();
        assert_eq!(snapshot.default_channel, None);
    }

    #[test]
    fn parse_store_json_skips_unknown_channels() {
        let json = br#"{
            "library": { "installed": [
                { "id": "SC", "channels": [
                    { "id": "WEIRDFUTURE", "status": "installed",
                      "libraryFolder": "C:\\", "installDir": "X" }
                ] }
            ]}
        }"#;
        let snapshot = parse_store_json(json).unwrap();
        assert!(snapshot.installs.is_empty());
    }

    #[test]
    fn parse_store_json_skips_non_sc_games() {
        // Future-proofing: if the launcher ever lists Squadron 42 here,
        // we don't want its channels showing up as SC installs.
        let json = br#"{
            "library": { "installed": [
                { "id": "SQ42", "channels": [
                    { "id": "LIVE", "status": "installed",
                      "libraryFolder": "C:\\Games\\", "installDir": "Squadron42" }
                ] }
            ]}
        }"#;
        let snapshot = parse_store_json(json).unwrap();
        assert!(snapshot.installs.is_empty());
    }

    #[test]
    fn parse_store_json_missing_paths_skipped() {
        let json = br#"{
            "library": { "installed": [
                { "id": "SC", "channels": [
                    { "id": "LIVE", "status": "installed" }
                ] }
            ]}
        }"#;
        let snapshot = parse_store_json(json).unwrap();
        assert!(snapshot.installs.is_empty());
    }

    #[test]
    fn parse_store_json_invalid_returns_error() {
        let err = parse_store_json(b"not json").unwrap_err();
        assert!(matches!(err, Error::LauncherStoreInvalidJson { .. }));
    }

    #[test]
    fn extract_key_finds_marker_in_synthetic_payload() {
        // Synthetic asar-shaped blob with the marker buried after some
        // arbitrary content — verifies the chunked scanner. The "key"
        // here is a synthetic base64 string, not the launcher's real one.
        let mut blob = vec![0u8; 3 * (1 << 20)]; // 3 MB of zeros
        let fake_key = b"AAAABBBBCCCCDDDDEEEEFFFFGGGGHHHHIIIIJJJJKKK=";
        let mut marker = Vec::from(b"foo,bar,encryptionKey:\"" as &[u8]);
        marker.extend_from_slice(fake_key);
        marker.extend_from_slice(b"\",baz");
        // Deliberately place the marker straddling a 1 MB chunk boundary.
        let pos = (1 << 20) - 20;
        blob[pos..pos + marker.len()].copy_from_slice(&marker);

        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), &blob).unwrap();
        let key = extract_encryption_key(tmp.path()).unwrap();
        assert_eq!(key, fake_key);
    }

    #[test]
    fn extract_key_missing_marker_errors() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"no key here at all").unwrap();
        let err = extract_encryption_key(tmp.path()).unwrap_err();
        assert!(matches!(err, Error::EncryptionKeyNotFound { .. }));
    }

    #[test]
    fn decrypt_store_round_trip() {
        // Encrypt a tiny JSON blob using the same parameters the launcher
        // uses, then verify our decrypt produces the original.
        use cbc::cipher::BlockEncryptMut;
        type Encryptor = cbc::Encryptor<Aes256>;

        // Any 44-char base64 works here — the round-trip exercises the
        // PBKDF2 + AES-CBC math, not key authenticity.
        let key_b64 = b"AAAABBBBCCCCDDDDEEEEFFFFGGGGHHHHIIIIJJJJKKK=";
        let iv: [u8; 16] = [
            0xde, 0xad, 0xbe, 0xef, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
            0xaa, 0xbb,
        ];
        let salt = String::from_utf8_lossy(&iv).into_owned().into_bytes();
        let mut derived = [0u8; 32];
        pbkdf2_hmac::<Sha512>(key_b64, &salt, 10_000, &mut derived);

        let plaintext = b"{\"library\":{\"installed\":[]}}";
        let ct_len = plaintext.len() + (16 - plaintext.len() % 16);
        let mut buf = vec![0u8; ct_len];
        buf[..plaintext.len()].copy_from_slice(plaintext);
        let ct = Encryptor::new(&derived.into(), &iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut buf, plaintext.len())
            .unwrap();

        // Frame as `[IV] : [ct]`.
        let mut store_bytes = Vec::with_capacity(17 + ct.len());
        store_bytes.extend_from_slice(&iv);
        store_bytes.push(b':');
        store_bytes.extend_from_slice(ct);

        let recovered = decrypt_store(&store_bytes, key_b64).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn decrypt_store_bad_format_errors() {
        let err = decrypt_store(b"too short", b"key").unwrap_err();
        assert!(matches!(err, Error::LauncherStoreMalformed(_)));
    }
}
