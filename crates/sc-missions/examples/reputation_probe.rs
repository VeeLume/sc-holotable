//! Re-proves the joins the gRPC `ReputationService` needs (LIVE 4.10, rows
//! seen by sc-grpc): org GEIDs resolve to `FactionReputation` records
//! (199998949491 = Covalex, 199999007029 = Recco Battaglia) and carry their
//! record names (the `name` field reads `(Default)` on all of them), the
//! `standings_id` d31bd373-… is the `FactionReputationScope` ladder, and
//! every scope's ranks come out lowest first. Prints every scope's ladder.
//!
//! ```bash
//! cargo run -p sc-missions --release --example reputation_probe
//! ```

use std::str::FromStr;

use sc_extract::{AssetConfig, AssetData, AssetSource, Datacore, Guid, RecordCollection};
use sc_missions::{FactionReputations, ReputationScopes, ReputationStandings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let install = sc_discovery::discover_primary()?;
    println!("{} v{}", install.channel, install.short_version());
    let assets = AssetSource::from_install(&install)?;
    let data = AssetData::extract(&assets, &AssetConfig::standard())?;
    let datacore = Datacore::parse(&assets, &data)?;
    let factions = FactionReputations::build(&datacore);
    let standings = ReputationStandings::build(&datacore);
    let scopes = ReputationScopes::build(&datacore);
    println!(
        "{} factions, {} standings, {} scopes",
        factions.len(),
        standings.len(),
        scopes.len()
    );

    assert!(
        factions
            .values()
            .all(|f| f.record_name.starts_with("FactionReputation"))
    );
    let covalex = factions
        .by_geid(199_998_949_491)
        .ok_or("no GEID 199998949491")?;
    println!("199998949491 = {covalex:?}");
    assert_eq!(covalex.display_name_key.as_str(), "@Covalex_RepUI_Name");
    let battaglia = factions
        .by_geid(199_999_007_029)
        .ok_or("no GEID 199999007029")?;
    assert_eq!(battaglia.record_name, "FactionReputation_Battaglia");

    let mut all: Vec<_> = scopes.iter().collect();
    all.sort_by(|a, b| a.1.record_name.cmp(&b.1.record_name));
    for (guid, scope) in &all {
        let mins: Vec<i64> = scope
            .ranks
            .iter()
            .map(|r| standings.get(r).map_or(i64::MIN, |s| s.min_reputation))
            .collect();
        println!(
            "{guid} {} ({}) ceiling {} initial {} ranks {mins:?}",
            scope.record_name, scope.scope_name, scope.ceiling, scope.initial
        );
        assert!(
            mins.windows(2).all(|w| w[0] <= w[1]),
            "{}",
            scope.record_name
        );
        assert!(scope.ranks.iter().all(|r| standings.get(r).is_some()));
    }

    let faction_scope = Guid::from_str("d31bd373-35ab-4ea4-b0a7-0ccc867ef082")?;
    let scope = scopes
        .get(&faction_scope)
        .ok_or("no FactionReputationScope")?;
    assert_eq!(scope.record_name, "FactionReputationScope");
    assert!(!scope.ranks.is_empty());
    Ok(())
}
