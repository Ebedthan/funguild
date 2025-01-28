mod cli;
mod utils;

use std::env;

use anyhow::Result;
use utils::find_taxon;

fn main() -> Result<()> {
    let matches = cli::build_app().get_matches_from(env::args_os());
    let db: Vec<utils::FunGuildEntry> = utils::json_to_hashmap();

    let taxon = matches.get_one::<String>("TAXON").unwrap();
    //let output = matches.get_one::<String>("out");
    let is_word = matches.get_flag("word");
    let result = find_taxon(taxon.to_string(), db, is_word);
    println!("{:?}", result);
    Ok(())
}
