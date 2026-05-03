# RSI Launcher persistent store

Reference for the structure and contents of `%APPDATA%/rsilauncher/launcher store.json` — the launcher's encrypted electron-store file. This is the launcher's source-of-truth for installed games, account state, UI preferences, and language catalogues. `sc-installs` reads it as its primary discovery source (see `crates/sc-installs/src/launcher_store.rs`).

This doc exists so future features don't have to re-decrypt and re-explore the file from scratch.

---

## File format

```
[16-byte AES-256 IV] [':' (0x3A)] [PKCS7-padded AES-CBC ciphertext]
```

Decryption is the standard [electron-store](https://github.com/sindresorhus/electron-store) shape:

| Step | Detail |
|---|---|
| Cipher | AES-256-CBC, PKCS7 padding |
| KDF | PBKDF2-HMAC-SHA512, 10 000 iterations, 32-byte output |
| Password | 44-char ASCII base64 string from the launcher's `app.asar` (literal `encryptionKey:"…"` near `pbkdf2`/`safeStorage` references) |
| Salt | `IV.toString('utf8')` — the JS-side **lossy** conversion of the 16 IV bytes; high-bit IV bytes get folded to U+FFFD before re-encoding. Replicate via `String::from_utf8_lossy(iv).into_owned().into_bytes()`. |

The encryption key is hardcoded in the launcher app and could rotate on any launcher update. We extract it at runtime from `<launcher>/resources/app.asar` rather than embed it. See `extract_encryption_key` in [crates/sc-installs/src/launcher_store.rs](../crates/sc-installs/src/launcher_store.rs).

---

## Top-level structure

10 keys observed on RSI Launcher 2.13.3:

| Key | Type | Purpose |
|---|---|---|
| `__internal__` | object | electron-store migration bookkeeping |
| `accessibility` | object | reduce-motion preferences |
| `application` | object | UI/app state (window bounds, sound, last-online) |
| `device` | object | RSI device cookie (`X-RSI-Device`) |
| `discovery` | object | onboarding-overlay state |
| `identity` | object | logged-in CIG account |
| `language` | object | full language picker catalogue |
| `library` | object | **installed/available games and channels — the gold mine** |
| `session` | object | RSI auth session token (`X-Rsi-Token`) |
| `storage` | object | default library folder |

---

## `library` — installed games and channels

Four sub-arrays. All four contain one entry per game (currently `id="SC"`, `name="Star Citizen"`).

### `library.installed[]`

Channels that are present on disk. **This is the canonical source for sc-installs discovery.**

```jsonc
{
  "id": "SC",
  "name": "Star Citizen",
  "channels": [
    {
      "id": "LIVE",                            // matches Channel::install_dir_name()
      "name": "Live Release",                  // human-readable
      "installDir": "StarCitizen",             // subdir under libraryFolder
      "libraryFolder": "C:\\Games\\",          // parent of installDir
      "platformId": "prod",                    // "prod" or "ptu"
      "status": "installed",                   // "installed" | "available"
      "version": 11715810,                     // monotonic Perforce changelist
      "versionLabel": "4.7.2-live.11715810",   // launcher-style version
      "servicesEndpoint": "https://pub-sc-alpha-470-11518367.test1.cloudimperiumgames.com:443",
      "network": null,
      "weight": 10                             // launcher UI sort key
    }
  ],
  /* plus presentational fields: bgImage, bgVideo, logo, logoWide,
     logoAnimation, theme, buyGamePage, discoverPage, isPromoted,
     hasGamePackage, playableOffline, status */
}
```

Full install root path = `libraryFolder + installDir + channel.id` → e.g. `C:\Games\StarCitizen\LIVE`.

### `library.available[]`

Same shape as `installed[]` but for channels the launcher is *aware of* (e.g. PTU when there's a fresh test branch up). Channels here may have `status: "available"` (not on disk) or `status: "installed"` (on disk but also kept in the available list). **No `libraryFolder`** for not-installed channels.

### `library.defaults[]`

The default channel per game — what the big "Launch" button activates.

```jsonc
{
  "gameId": "SC",
  "gameName": "Star Citizen",
  "channelId": "LIVE",
  "channelName": "Live Release",
  "platformId": "prod"
}
```

This is the **best** signal for "which channel does the user actually want to launch", better than `Channel::priority()`'s hardcoded LIVE-first rule.

### `library.settings[]`

Per-channel install preferences. Unlike `installed[]`, this exists for channels the user has *configured* an install for, even if not yet downloaded. Entries pre-download have no `libraryFolder` filled in.

```jsonc
{
  "gameId": "SC",
  "gameName": "Star Citizen",
  "channelId": "LIVE",
  "channelName": "Live Release",
  "platformId": "prod",
  "installDir": "StarCitizen",
  "libraryFolder": "C:\\Games\\"   // only present once the user picked one
}
```

---

## `application`

UI/app state, mostly UX surface but two fields are interesting:

| Field | Type | Meaning |
|---|---|---|
| `version` | string | launcher version (e.g. `"2.13.3"`) — also in `__internal__.migrations.version` |
| `connection.lastTimeOnline` | u64 | Unix ms timestamp of last successful CIG-API contact. Useful as a stale-store sentinel — if old, the store may not reflect recent installs |
| `quitOnWindowClose`, `hideQuitWarning`, `isDiscoverDone` | bool | UX prefs |
| `window.large.bounds`, `window.small.bounds` | object `{x, y, width, height}` | persisted window geometry |
| `soundSystem.{mainVolume, musicVolume, effectsVolume}` | int 0-100 | audio prefs |
| `soundSystem.lastPlayedTracks` | object | `{"SC": "bg5"}` etc. — last bg music track per game |
| `settings.accessibility.reduceMotion` | object | duplicate of top-level `accessibility.options.reduceMotion` |

---

## `device` and `session` — auth state

Both follow the cookie-shape:

```jsonc
{ "cookie": "_rsi_device", "key": "X-RSI-Device", "value": "<token>" }
{ "cookie": "Rsi-Token",   "key": "X-Rsi-Token",  "value": "<bearer>" }
```

`device` carries an `expires` (Unix ms, ~year out) and a `duration` field (`"year"`).

**Treat the `value` strings as PII / secrets.** Never log them. Don't expose them through any sc-installs public API.

---

## `identity` — logged-in account

```jsonc
{
  "username": "user@example.com",                // login email
  "displayName": "Vee",                          // public display name
  "nickname": "VeeLume",                         // RSI handle
  "avatar": "/media/.../avatar/<uuid>.png",      // CDN-relative
  "heapAccountId": "<7+ chars>",                 // Heap analytics ID
  "trackingMetricsId": "<UUID>",                 // tracking ID
  "privileged": false                            // CIG staff flag
}
```

**Treat all of these as PII.** Username is an email. The IDs are tracking handles. Don't log; don't expose without explicit consumer opt-in.

---

## `language.languageCollection[]`

Launcher's language picker source. 10 entries observed (`en, ja, es-419, fr, de, it, pt-br, ko, es-es, zh-cn`). Each:

```jsonc
{
  "code": "de",                  // BCP-47 code
  "codeLabel": "de",             // launcher-internal label
  "label": "Deutsch",            // display name in that language
  "subLabel": null,              // regional disambiguation (e.g. "(Brasil)")
  "altCodes": null,              // array of related codes for es-419 only
  "badges": [
    {
      "id": 2063,
      "membership.id": 58525205,  // numeric IDs vary per language
      "title": "Localization Testing",
      "title.en": "Localization Testing"
    }
  ]
}
```

The `badges[].membership.id` values appear to be the RSI org/role IDs gating access to localisation test channels. English has no badge.

---

## `accessibility`

```jsonc
"options": {
  "reduceMotion": {
    "syncWithSystem": false,
    "backgroundVideoDisabled": true,
    "wipeAnimationsDisabled": false,
    "unessentialAnimationsDisabled": false
  }
}
```

Mirrored in `application.settings.accessibility.reduceMotion`.

---

## `discovery`

```jsonc
"settings": { "isStarted": false, "isDone": true }
```

Whether the launcher's first-run "Discover" overlay has been completed.

---

## `storage`

```jsonc
{ "defaultLibraryFolder": "C:\\Games\\" }
```

Where new installs go by default. Useful for any "suggest install location" UX.

---

## `__internal__`

```jsonc
{ "migrations": { "version": "2.13.3" } }
```

electron-store migration bookkeeping. Don't touch.

---

## Field-by-field — what sc-installs uses today vs. what it could use

| Field | Used | Could use |
|---|---|---|
| `library.installed[].channels[].id` | ✅ channel | |
| `library.installed[].channels[].libraryFolder` + `installDir` | ✅ root path | |
| `library.installed[].channels[].status` | ✅ filter `installed` | |
| `library.installed[].channels[].versionLabel` | ✅ `StoreInstall::version_label` | |
| `library.installed[].channels[].version` | ✅ `StoreInstall::changelist` | |
| `library.installed[].channels[].platformId` | — | tag installs as `prod`/`ptu` for downstream auth-URL routing |
| `library.installed[].channels[].servicesEndpoint` | — | per-channel CIG services URL — useful if we ever want to build authenticated tools |
| `library.defaults[].channelId` | — | **`discover_default()` — better than priority-based primary** |
| `library.settings[].libraryFolder` | — | discover *configured-but-not-yet-installed* channels |
| `library.available[].channels[].versionLabel` | — | "PTU 4.8.0 is downloadable" UX without hitting the launcher API |
| `application.connection.lastTimeOnline` | — | stale-store warning (`> 30 days` → log fallback may be more recent) |
| `application.version` | — | tag store reads with launcher version for telemetry/error reports |
| `storage.defaultLibraryFolder` | — | "where would a fresh install go?" suggestion |
| `identity.username` | — | multi-account UIs (only with explicit opt-in — PII) |
| `device.value`, `session.value` | — | **never expose** |
| `identity.heapAccountId`, `trackingMetricsId` | — | **never expose** |

---

## Caveats

- Snapshot is from RSI Launcher **2.13.3** (April 2026). New keys may appear in future versions; existing keys generally don't move.
- The hardcoded `encryptionKey` in `app.asar` could rotate on any launcher update. The runtime extractor is regex-based (`encryptionKey:"<base64>"`) — robust to minification but fragile to a refactor that names the field differently.
- electron-store may bump its on-disk version one day; the `__internal__.migrations.version` key is the canary.
