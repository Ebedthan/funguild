mod cli;
mod utils;

use anyhow::Context;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use utils::find_taxon;

fn main() -> anyhow::Result<()> {
    // Get CLI arguments
    let matches = cli::build_app().get_matches_from(env::args_os());
    // Read in FunGuild Data
    let db: Vec<utils::FunGuildEntry> = utils::json_to_hashmap()?;

    // Parse CLI arguments
    let taxon = matches.get_one::<String>("TAXON").unwrap();
    let output = matches.get_one::<String>("out");
    let is_word = matches.get_flag("word");
    let force_output = matches.get_flag("force");

    //  Find FunGuild by taxon
    let result = find_taxon(taxon.to_string(), db, is_word);

    // Parse result as csv and output it
    let result_as_csv = utils::result_to_csv(result)?;
    if let Some(filename) = output {
        if force_output {
            fs::remove_file(Path::new(filename))
                .with_context(|| format!("Failed to remove file at {}", filename))?;
        }
        let mut outfile =
            File::open(filename).with_context(|| format!("Failed to open file at {}", filename))?;
        outfile.write_all(result_as_csv.as_bytes())?;
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(result_as_csv.as_bytes())?;
    }

    Ok(())
}
