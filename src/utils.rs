use anyhow::Context;
use csv::Writer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FunGuildEntry {
    taxon: String,
    guid: String,
    mb_number: String,
    taxonomic_level: String,
    trophic_mode: String,
    guild: String,
    confidence_ranking: String,
    growth_form: String,
    #[serde(rename = "trait")]
    x_trait: String,
    notes: String,
    citation_source: String,
}

pub fn json_to_hashmap() -> anyhow::Result<Vec<FunGuildEntry>> {
    static FUNGUILD_DB: &str = include_str!(r"funguild2.json");
    serde_json::from_str(FUNGUILD_DB)
        .with_context(|| "Failed to deserialize JSON data into FunGuildEntry")
}

pub fn find_taxon(taxa: &[String], db: &[FunGuildEntry], is_word: bool) -> Vec<FunGuildEntry> {
    taxa.iter()
        .flat_map(|taxon| {
            db.iter().filter(move |entry| {
                if is_word {
                    entry.taxon == *taxon
                } else {
                    entry.taxon.contains(taxon)
                }
            })
        })
        .cloned()
        .collect()
}

pub fn result_to_csv(data: &[FunGuildEntry]) -> anyhow::Result<String> {
    let mut writer = Writer::from_writer(vec![]);
    for record in data {
        writer
            .serialize(record)
            .with_context(|| "Failed to serialize record")?;
    }
    String::from_utf8(writer.into_inner()?).with_context(|| "Failed to convert to String")
}
