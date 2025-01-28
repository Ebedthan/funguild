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

pub fn json_to_hashmap() -> Vec<FunGuildEntry> {
    static FUNGUILD_DB: &str = include_str!(r"funguild2.json");
    let mut reader = BufReader::new(FUNGUILD_DB.as_bytes());
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let db: Vec<FunGuildEntry> = serde_json::from_str(&buffer).unwrap();
    db
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

pub fn result_to_csv(data: Vec<FunGuildEntry>) -> String {
    let mut writer = Writer::from_writer(vec![]);
    for record in data {
        writer.serialize(record).unwrap();
    }
    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}
