use anyhow::Context;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read};

#[derive(Deserialize, Serialize, Debug)]
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
    let mut reader = BufReader::new(FUNGUILD_DB.as_bytes());
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .with_context(|| format!("Failed to read data to string"))?;
    let db: Vec<FunGuildEntry> =
        serde_json::from_str(&buffer).with_context(|| format!("Failed to deserialize string"))?;
    Ok(db)
}

pub fn find_taxon(taxon: String, db: Vec<FunGuildEntry>, is_word: bool) -> Vec<FunGuildEntry> {
    if is_word {
        db.into_iter().filter(|x| x.taxon == taxon).collect()
    } else {
        db.into_iter()
            .filter(|x| x.taxon.contains(&taxon))
            .collect()
    }
}

pub fn result_to_csv(data: Vec<FunGuildEntry>) -> anyhow::Result<String> {
    let mut writer = Writer::from_writer(vec![]);
    for record in data {
        writer
            .serialize(record)
            .with_context(|| format!("Failed to serialize record"))?;
    }
    Ok(String::from_utf8(writer.into_inner()?)
        .with_context(|| format!("Failed to convert to String"))?)
}
