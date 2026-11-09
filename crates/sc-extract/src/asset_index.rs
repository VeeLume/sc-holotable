//! Cooked pointers into a `Data.p4k`; see [`AssetIndex`].

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

use svarog_common::BinaryReader;
use svarog_p4k::P4kArchive;
use svarog_p4k::zip::{CompressionMethod, LocalFileHeader};
use tracing::{debug, warn};

use crate::mesh::{geometry_files, own_material, resolve_material};
use crate::{AssetSource, CharacterDefinition, Error, Result};

const MAGIC: &[u8; 4] = b"SCAI";
const VERSION: u32 = 1;
const HEADER_LEN: usize = 4 + 4 + 5 * 8 + 8;
const RECORD_LEN: usize = 8 + 8 + 4 + 4 + 1;
/// How much of the directory's end goes into the fingerprint.
const TAIL_HASHED: usize = 64 * 1024;
/// Read with a file's header, so most files take one read: the fixed header
/// plus more name and extra field than a live archive has (about 4.2 KB).
const HEADER_READ: usize = 4 + size_of::<LocalFileHeader>() + 4608;
/// Reads on one handle queue up, so concurrent readers get their own.
const MAX_HANDLES: usize = 16;

/// A cooked list of pointers into a `Data.p4k`, so a host that reads a known
/// set of files never loads the archive's file list.
///
/// Opening an archive reads its whole central directory: 468 MB on a live
/// build, a second and hundreds of MB of memory. Reading one file needs only
/// where it is and how it is packed, so [`AssetIndex`] keeps that for the
/// files a host names, about 25 bytes each and no paths.
///
/// An index is built once per game build while the archive is open anyway,
/// saved by the host, and given to [`AssetSource::open_indexed`]. It is never
/// trusted blindly:
///
/// - the archive's tail is checked against it on open; a different archive
///   (or a missing or unreadable index) means a full open instead;
/// - every read compares the name in the file's own header with the pointer;
/// - a path the index does not list makes the source open the whole archive
///   once and answer from that, so an incomplete cook costs time, never a
///   file. A path the cook looked for and did not find is listed as absent:
///   a detail level a mesh does not have is a normal miss.
///
/// ```no_run
/// use sc_extract::{AssetIndex, AssetSource};
///
/// let install = sc_discovery::discover_primary()?;
/// let index_file = std::path::Path::new("assets.index");
///
/// // While cooking, with the whole archive open:
/// let assets = AssetSource::from_install(&install)?;
/// let mut index = AssetIndex::builder(&assets)?;
/// index.add_model("objects/spaceships/ships/aegs/gladius/aegs_gladius.cga");
/// std::fs::write(index_file, index.finish().to_bytes())?;
///
/// // Every time after:
/// let assets = AssetSource::open_indexed(&install.data_p4k(), index_file)?;
/// let bytes = assets.read_data("objects/spaceships/ships/aegs/gladius/aegs_gladius.cga")?;
/// # let _ = bytes;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetIndex {
    fingerprint: Fingerprint,
    /// Sorted by key.
    records: Vec<Record>,
}

/// What identifies one state of an archive, read from its last pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fingerprint {
    len: u64,
    directory_offset: u64,
    directory_size: u64,
    entries: u64,
    tail_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Record {
    key: u64,
    offset: u64,
    compressed: u32,
    uncompressed: u32,
    kind: Kind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The cook looked for this path and the archive does not have it.
    Absent = 0,
    Stored = 1,
    Zstd = 2,
}

impl AssetIndex {
    /// Start an index of `source`, which has to be a live archive.
    pub fn builder(source: &AssetSource) -> Result<AssetIndexBuilder> {
        let (Some(archive), Some(path)) = (source.archive(), source.source_path()) else {
            return Err(Error::FileNotInP4k(
                "<an asset index needs a live archive>".into(),
            ));
        };
        let reader =
            Reader::open(path).map_err(|e| Error::FileNotInP4k(format!("{path:?}: {e}")))?;
        let fingerprint = reader
            .fingerprint()
            .ok_or_else(|| Error::FileNotInP4k(format!("{path:?}: no zip64 directory")))?;
        let entries = archive
            .iter()
            .map(|e| {
                let kind = match e.compression_method {
                    _ if e.is_encrypted => None,
                    CompressionMethod::Store => Some(Kind::Stored),
                    CompressionMethod::Zstd => Some(Kind::Zstd),
                    CompressionMethod::Deflate => None,
                };
                let record = kind
                    .zip(u32::try_from(e.compressed_size).ok())
                    .zip(u32::try_from(e.uncompressed_size).ok())
                    .map(|((kind, compressed), uncompressed)| Record {
                        key: key(e.name.as_bytes()),
                        offset: e.local_header_offset,
                        compressed,
                        uncompressed,
                        kind,
                    });
                (key(e.name.as_bytes()), record)
            })
            .collect();
        Ok(AssetIndexBuilder {
            reader,
            fingerprint,
            entries,
            records: BTreeMap::new(),
            models: HashSet::new(),
        })
    }

    /// How many paths it lists, absent ones included.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether it lists nothing.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// The form a host saves.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_LEN + self.records.len() * RECORD_LEN);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&VERSION.to_le_bytes());
        let f = &self.fingerprint;
        for n in [
            f.len,
            f.directory_offset,
            f.directory_size,
            f.entries,
            f.tail_hash,
            self.records.len() as u64,
        ] {
            out.extend_from_slice(&n.to_le_bytes());
        }
        for r in &self.records {
            out.extend_from_slice(&r.key.to_le_bytes());
            out.extend_from_slice(&r.offset.to_le_bytes());
            out.extend_from_slice(&r.compressed.to_le_bytes());
            out.extend_from_slice(&r.uncompressed.to_le_bytes());
            out.push(r.kind as u8);
        }
        out
    }

    /// Read what [`to_bytes`](Self::to_bytes) wrote; `None` when it is not
    /// that.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let header = bytes.get(..HEADER_LEN)?;
        if &header[..4] != MAGIC || u32_at(header, 4) != VERSION {
            return None;
        }
        let fingerprint = Fingerprint {
            len: u64_at(header, 8),
            directory_offset: u64_at(header, 16),
            directory_size: u64_at(header, 24),
            entries: u64_at(header, 32),
            tail_hash: u64_at(header, 40),
        };
        let count = usize::try_from(u64_at(header, 48)).ok()?;
        let body = &bytes[HEADER_LEN..];
        if body.len() != count.checked_mul(RECORD_LEN)? {
            return None;
        }
        let records = body
            .as_chunks::<RECORD_LEN>()
            .0
            .iter()
            .map(|r| {
                Some(Record {
                    key: u64_at(r, 0),
                    offset: u64_at(r, 8),
                    compressed: u32_at(r, 16),
                    uncompressed: u32_at(r, 20),
                    kind: match r[24] {
                        0 => Kind::Absent,
                        1 => Kind::Stored,
                        2 => Kind::Zstd,
                        _ => return None,
                    },
                })
            })
            .collect::<Option<Vec<_>>>()?;
        records
            .windows(2)
            .all(|w| w[0].key < w[1].key)
            .then_some(Self {
                fingerprint,
                records,
            })
    }

    fn get(&self, key: u64) -> Option<&Record> {
        let i = self.records.binary_search_by_key(&key, |r| r.key).ok()?;
        self.records.get(i)
    }
}

/// Collects what an [`AssetIndex`] points at, from a live archive.
pub struct AssetIndexBuilder {
    reader: Reader,
    fingerprint: Fingerprint,
    /// Every archive entry; `None` for one only a whole open can read.
    entries: HashMap<u64, Option<Record>>,
    records: BTreeMap<u64, Record>,
    models: HashSet<u64>,
}

impl AssetIndexBuilder {
    /// List `path`, in any of the forms [`AssetSource::read_data`] takes. If
    /// the archive does not have it, it is listed as absent.
    pub fn add(&mut self, path: &str) {
        let k = key(path.as_bytes());
        match self.entries.get(&k) {
            Some(Some(record)) => {
                self.records.insert(k, *record);
            }
            // Encrypted, or too large: left for the whole archive to answer.
            Some(None) => {}
            None => {
                self.records.insert(
                    k,
                    Record {
                        key: k,
                        offset: 0,
                        compressed: 0,
                        uncompressed: 0,
                        kind: Kind::Absent,
                    },
                );
            }
        }
    }

    /// List a geometry file and everything reading it can look for: its
    /// streams, each detail level's, and the `.mtl` it names.
    pub fn add_geometry(&mut self, path: &str) {
        for file in geometry_files(path) {
            self.add(&file);
        }
        if let Some(name) = self.read(path).and_then(|g| own_material(&g)) {
            self.add_material(&resolve_material(&name, path));
        }
    }

    /// A listed-to-be file's bytes, the way viewing will read them.
    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.reader
            .record(self.entries.get(&key(path.as_bytes()))?.as_ref()?)
    }

    /// List what a recipe's model path needs: a `.cdf` assembly with its
    /// model, parts and their materials, any other file as
    /// [`add_geometry`](Self::add_geometry). A `.cdf` that cannot be read is
    /// listed as far as it goes, and reads the same way at view time.
    pub fn add_model(&mut self, path: &str) {
        if !self.models.insert(key(path.as_bytes())) {
            return;
        }
        if !path.to_ascii_lowercase().ends_with(".cdf") {
            self.add_geometry(path);
            return;
        }
        self.add(path);
        let Some(cdf) = self
            .read(path)
            .and_then(|bytes| CharacterDefinition::parse(&bytes).ok())
        else {
            return;
        };
        self.add_model(&cdf.model);
        for a in &cdf.attachments {
            if let Some(binding) = &a.binding {
                self.add_model(binding);
            }
            if let Some(material) = &a.material {
                self.add_material(material);
            }
        }
    }

    /// List a `.mtl`, named with or without its extension.
    pub fn add_material(&mut self, path: &str) {
        if path.to_ascii_lowercase().ends_with(".mtl") {
            self.add(path);
        } else {
            self.add(&format!("{path}.mtl"));
        }
    }

    /// The finished index.
    pub fn finish(self) -> AssetIndex {
        AssetIndex {
            fingerprint: self.fingerprint,
            records: self.records.into_values().collect(),
        }
    }
}

/// What an indexed source answers about a path.
pub(crate) enum Lookup {
    Found(Vec<u8>),
    Absent,
    /// Not in the index, or its pointer did not hold up.
    Unlisted,
}

/// An archive neither listed nor mapped, read through an [`AssetIndex`].
pub(crate) struct Indexed {
    reader: Reader,
    index: AssetIndex,
    path: PathBuf,
    /// The whole archive, opened the first time something needs it.
    whole: OnceLock<Option<P4kArchive>>,
}

impl Indexed {
    /// `None` when the index is missing, unreadable or of another archive.
    pub(crate) fn open(p4k: &Path, index_file: &Path) -> Option<Self> {
        let bytes = match std::fs::read(index_file) {
            Ok(bytes) => bytes,
            Err(e) => {
                debug!(?index_file, %e, "no asset index");
                return None;
            }
        };
        let Some(index) = AssetIndex::from_bytes(&bytes) else {
            warn!(?index_file, "asset index is unreadable");
            return None;
        };
        let reader = Reader::open(p4k).ok()?;
        if reader.fingerprint() != Some(index.fingerprint) {
            warn!(?index_file, ?p4k, "asset index is of another archive");
            return None;
        }
        Some(Self {
            reader,
            index,
            path: p4k.to_path_buf(),
            whole: OnceLock::new(),
        })
    }

    pub(crate) fn read(&self, path: &str) -> Lookup {
        let Some(record) = self.index.get(key(path.as_bytes())) else {
            return Lookup::Unlisted;
        };
        if record.kind == Kind::Absent {
            return Lookup::Absent;
        }
        match self.reader.record(record) {
            Some(bytes) => Lookup::Found(bytes),
            None => {
                warn!(path, "asset index pointer did not hold up");
                Lookup::Unlisted
            }
        }
    }

    /// The whole archive, opening it on first use.
    pub(crate) fn whole(&self) -> Option<&P4kArchive> {
        self.whole
            .get_or_init(|| {
                warn!(p4k = ?self.path, "asset index is not enough; opening the whole archive");
                P4kArchive::open(&self.path)
                    .inspect_err(|e| warn!(p4k = ?self.path, %e, "whole open failed"))
                    .ok()
            })
            .as_ref()
    }

    pub(crate) fn whole_is_open(&self) -> bool {
        self.whole.get().is_some_and(Option::is_some)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn listed(&self) -> usize {
        self.index.len()
    }
}

/// An archive read by position. Mapping it would be simpler, but a mapping of
/// a whole 159 GB archive holds 312 MB of commit before a page is touched.
struct Reader {
    files: Vec<File>,
    next: AtomicUsize,
    len: u64,
}

impl Reader {
    fn open(path: &Path) -> std::io::Result<Self> {
        let first = File::open(path)?;
        let len = first.metadata()?.len();
        let handles = std::thread::available_parallelism()
            .map_or(4, |n| n.get())
            .min(MAX_HANDLES);
        let mut files = vec![first];
        for _ in 1..handles {
            files.push(File::open(path)?);
        }
        Ok(Self {
            files,
            next: AtomicUsize::new(0),
            len,
        })
    }

    /// Fill `buffer` from `offset`, as far as the file goes.
    fn read_at(&self, offset: u64, buffer: &mut [u8]) -> std::io::Result<usize> {
        let file = &self.files[self.next.fetch_add(1, Ordering::Relaxed) % self.files.len()];
        let mut done = 0;
        while done < buffer.len() {
            match read_position(file, &mut buffer[done..], offset + done as u64)? {
                0 => break,
                n => done += n,
            }
        }
        Ok(done)
    }

    /// Up to `len` bytes from `offset`.
    fn bytes(&self, offset: u64, len: usize) -> Option<Vec<u8>> {
        let mut buffer = vec![0; len];
        let got = self.read_at(offset, &mut buffer).ok()?;
        buffer.truncate(got);
        Some(buffer)
    }

    fn fingerprint(&self) -> Option<Fingerprint> {
        Fingerprint::of(self.len, |offset, len| self.bytes(offset, len))
    }

    /// The bytes behind a pointer, if its file carries the right name.
    fn record(&self, r: &Record) -> Option<Vec<u8>> {
        let read = self.bytes(r.offset, HEADER_READ + r.compressed as usize)?;
        let signature = u32::from_le_bytes(read.get(..4)?.try_into().ok()?);
        if signature != LocalFileHeader::SIGNATURE
            && signature != LocalFileHeader::SIGNATURE_EXTENDED
        {
            return None;
        }
        let header: LocalFileHeader = BinaryReader::new(read.get(4..)?).read_struct().ok()?;
        let name_at = 4 + size_of::<LocalFileHeader>();
        let name_len = { header.file_name_length } as usize;
        if key(read.get(name_at..name_at + name_len)?) != r.key {
            return None;
        }
        if r.uncompressed == 0 {
            return Some(Vec::new());
        }
        let data_at = name_at + header.variable_data_size();
        let compressed = r.compressed as usize;
        let late;
        let data = match read.get(data_at..data_at + compressed) {
            Some(data) => data,
            None => {
                late = self.bytes(r.offset + data_at as u64, compressed)?;
                late.as_slice()
            }
        };
        match r.kind {
            Kind::Stored => (data.len() == r.uncompressed as usize).then(|| data.to_vec()),
            Kind::Zstd => zstd::bulk::decompress(data, r.uncompressed as usize).ok(),
            Kind::Absent => None,
        }
    }
}

#[cfg(windows)]
fn read_position(file: &File, buffer: &mut [u8], offset: u64) -> std::io::Result<usize> {
    std::os::windows::fs::FileExt::seek_read(file, buffer, offset)
}

#[cfg(unix)]
fn read_position(file: &File, buffer: &mut [u8], offset: u64) -> std::io::Result<usize> {
    std::os::unix::fs::FileExt::read_at(file, buffer, offset)
}

/// A path's identity: case, slash direction and a `Data\` prefix don't count.
fn key(path: &[u8]) -> u64 {
    let fold = |b: u8| match b {
        b'/' => b'\\',
        b => b.to_ascii_lowercase(),
    };
    let start = path
        .iter()
        .take_while(|b| matches!(b, b'/' | b'\\'))
        .count();
    let path = &path[start..];
    let path = match path.get(..5) {
        Some(p) if p.iter().map(|b| fold(*b)).eq(*b"data\\") => &path[5..],
        _ => path,
    };
    hash(path.iter().map(|b| fold(*b)))
}

/// FNV-1a.
fn hash(bytes: impl Iterator<Item = u8>) -> u64 {
    bytes.fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn u32_at(d: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(d[at..at + 4].try_into().expect("4 bytes"))
}

fn u64_at(d: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(d[at..at + 8].try_into().expect("8 bytes"))
}

impl Fingerprint {
    /// Read from the zip64 end records; the archive may end in zero padding.
    /// `read` gives up to the asked number of bytes at an offset.
    fn of(len: u64, read: impl Fn(u64, usize) -> Option<Vec<u8>>) -> Option<Self> {
        const STEP: u64 = 64 * 1024;
        let mut before = len;
        let end = loop {
            let from = before.saturating_sub(STEP);
            let block = read(from, (before - from) as usize)?;
            if let Some(i) = block.iter().rposition(|b| *b != 0) {
                break from + i as u64 + 1;
            }
            if from == 0 {
                return None;
            }
            before = from;
        };
        let window_at = end.saturating_sub(65_557 + 100);
        let window = read(window_at, (end - window_at) as usize)?;
        let last =
            |within: &[u8], signature: [u8; 4]| within.windows(4).rposition(|w| w == signature);
        let eocd = last(&window, [0x50, 0x4b, 0x05, 0x06])?;
        let search = eocd.saturating_sub(100);
        let locator = search + last(&window[search..eocd], [0x50, 0x4b, 0x06, 0x07])?;
        let record = u64_at(window.get(..locator + 16)?, locator + 8);
        let fields = read(record, 56)?;
        if fields.len() != 56 || fields[..4] != [0x50, 0x4b, 0x06, 0x06] {
            return None;
        }
        let (entries, directory_size, directory_offset) = (
            u64_at(&fields, 32),
            u64_at(&fields, 40),
            u64_at(&fields, 48),
        );
        let directory_end = directory_offset.checked_add(directory_size)?;
        let tail_len = directory_size.min(TAIL_HASHED as u64);
        let tail = read(directory_end.checked_sub(tail_len)?, tail_len as usize)?;
        if tail.len() as u64 != tail_len {
            return None;
        }
        Some(Self {
            len,
            directory_offset,
            directory_size,
            entries,
            tail_hash: hash(tail.iter().copied()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_of(bytes: &[u8]) -> Fingerprint {
        Fingerprint::of(bytes.len() as u64, |at, len| {
            let at = at as usize;
            Some(bytes[at..(at + len).min(bytes.len())].to_vec())
        })
        .unwrap()
    }

    /// An archive in miniature: local files, a directory, the end records.
    struct Fake {
        bytes: Vec<u8>,
        records: Vec<Record>,
    }

    fn fake(files: &[(&str, &[u8], bool)]) -> Fake {
        let mut bytes = Vec::new();
        let mut records = Vec::new();
        for (name, content, packed) in files {
            let data = if *packed {
                zstd::bulk::compress(content, 3).unwrap()
            } else {
                content.to_vec()
            };
            let offset = bytes.len() as u64;
            bytes.extend_from_slice(&LocalFileHeader::SIGNATURE_EXTENDED.to_le_bytes());
            bytes.extend_from_slice(&[0; 22]);
            bytes.extend_from_slice(&(name.len() as u16).to_le_bytes());
            bytes.extend_from_slice(&3u16.to_le_bytes());
            bytes.extend_from_slice(name.as_bytes());
            bytes.extend_from_slice(&[9; 3]);
            bytes.extend_from_slice(&data);
            records.push(Record {
                key: key(name.as_bytes()),
                offset,
                compressed: data.len() as u32,
                uncompressed: content.len() as u32,
                kind: if *packed { Kind::Zstd } else { Kind::Stored },
            });
        }
        records.sort_by_key(|r| r.key);
        let directory_offset = bytes.len() as u64;
        bytes.extend_from_slice(b"pretend this is the central directory");
        let directory_size = bytes.len() as u64 - directory_offset;
        let record_at = bytes.len() as u64;
        bytes.extend_from_slice(&[0x50, 0x4b, 0x06, 0x06]);
        bytes.extend_from_slice(&[0; 28]);
        bytes.extend_from_slice(&(files.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&directory_size.to_le_bytes());
        bytes.extend_from_slice(&directory_offset.to_le_bytes());
        bytes.extend_from_slice(&[0x50, 0x4b, 0x06, 0x07, 0, 0, 0, 0]);
        bytes.extend_from_slice(&record_at.to_le_bytes());
        bytes.extend_from_slice(&[1, 0, 0, 0]);
        bytes.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06]);
        bytes.extend_from_slice(&[0xFF; 18]);
        bytes.extend_from_slice(&[0; 40]);
        Fake { bytes, records }
    }

    fn open(fake: &Fake, records: Vec<Record>) -> (tempfile::TempDir, Option<Indexed>) {
        let dir = tempfile::tempdir().unwrap();
        let (p4k, index_file) = (dir.path().join("Data.p4k"), dir.path().join("index"));
        std::fs::write(&p4k, &fake.bytes).unwrap();
        let index = AssetIndex {
            fingerprint: print_of(&fake.bytes),
            records,
        };
        std::fs::write(&index_file, index.to_bytes()).unwrap();
        let opened = Indexed::open(&p4k, &index_file);
        (dir, opened)
    }

    #[test]
    fn a_path_has_one_key_however_it_is_written() {
        let k = key(br"Data\Objects\Ships\a.cga");
        assert_eq!(k, key(b"objects/ships/a.CGA"));
        assert_eq!(k, key(b"/data/objects/ships/a.cga"));
        assert_ne!(k, key(b"objects/ships/b.cga"));
    }

    #[test]
    fn pointers_read_stored_and_packed_files() {
        let content: &[u8] = b"some bytes of a model, some bytes of a model";
        let f = fake(&[
            (r"Data\a.cga", content, true),
            (r"Data\b.cgam", b"raw", false),
        ]);
        let (_dir, indexed) = open(&f, f.records.clone());
        let indexed = indexed.expect("opens");
        assert!(matches!(indexed.read("a.cga"), Lookup::Found(b) if b == content));
        assert!(matches!(indexed.read("data/B.CGAM"), Lookup::Found(b) if b == b"raw"));
    }

    #[test]
    fn a_listed_absence_is_a_miss_and_an_unlisted_path_is_not() {
        let f = fake(&[(r"Data\a.cga", b"x", false)]);
        let mut records = f.records.clone();
        records.push(Record {
            key: key(b"a_lod1.cgam"),
            offset: 0,
            compressed: 0,
            uncompressed: 0,
            kind: Kind::Absent,
        });
        records.sort_by_key(|r| r.key);
        let (_dir, indexed) = open(&f, records);
        let indexed = indexed.expect("opens");
        assert!(matches!(indexed.read("a_lod1.cgam"), Lookup::Absent));
        assert!(matches!(indexed.read("other.cga"), Lookup::Unlisted));
    }

    #[test]
    fn a_pointer_landing_on_another_file_does_not_hold_up() {
        let f = fake(&[
            (r"Data\a.cga", b"aaa", false),
            (r"Data\b.cga", b"bbb", false),
        ]);
        let mut records = f.records.clone();
        let (a, b) = (records[0].offset, records[1].offset);
        records[0].offset = b;
        records[1].offset = a;
        let (_dir, indexed) = open(&f, records);
        assert!(matches!(
            indexed.expect("opens").read("a.cga"),
            Lookup::Unlisted
        ));
    }

    #[test]
    fn another_archive_or_a_bad_index_does_not_open() {
        let f = fake(&[(r"Data\a.cga", b"x", false)]);
        let other = fake(&[(r"Data\a.cga", b"xy", false)]);
        let dir = tempfile::tempdir().unwrap();
        let (p4k, index_file) = (dir.path().join("Data.p4k"), dir.path().join("index"));
        std::fs::write(&p4k, &other.bytes).unwrap();
        let index = AssetIndex {
            fingerprint: print_of(&f.bytes),
            records: f.records.clone(),
        };
        std::fs::write(&index_file, index.to_bytes()).unwrap();
        assert!(Indexed::open(&p4k, &index_file).is_none());
        std::fs::write(&index_file, b"not an index").unwrap();
        assert!(Indexed::open(&p4k, &index_file).is_none());
        assert!(Indexed::open(&p4k, &dir.path().join("missing")).is_none());
    }

    #[test]
    fn the_saved_form_round_trips() {
        let f = fake(&[(r"Data\a.cga", b"abc", true), (r"Data\b.cga", b"d", false)]);
        let index = AssetIndex {
            fingerprint: print_of(&f.bytes),
            records: f.records,
        };
        assert_eq!(AssetIndex::from_bytes(&index.to_bytes()), Some(index));
    }
}
