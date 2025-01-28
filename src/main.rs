mod cli;
mod utils;

use anyhow::Context;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use utils::{find_taxon, FunGuildEntry};

fn main() -> anyhow::Result<()> {
    // Get CLI arguments
    let matches = cli::build_app().get_matches_from(env::args_os());

    // Load FunGuild database
    let db: Vec<FunGuildEntry> = utils::json_to_hashmap()?;

    // Parse CLI arguments for taxa input
    let taxon = if let Some(filepath) = matches.get_one::<String>("file") {
        let file =
            File::open(filepath).with_context(|| format!("Failed to open file {}", filepath))?;
        BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("Failed to read lines from file {}", filepath))?
    } else {
        vec![matches
            .get_one::<String>("TAXON")
            .with_context(|| "TAXON argument is required")?
            .to_string()]
    };

    // Additional CLI arguments
    let output = matches.get_one::<String>("out");
    let is_word = matches.get_flag("word");
    let force_output = matches.get_flag("force");

    //  Find FunGuild by taxon
    let result = find_taxon(&taxon, &db, is_word);

    // Convert result to CSV
    let result_as_csv = utils::result_to_csv(&result)?;

    // Handle output: parse result as csv and output it
    if let Some(filename) = output {
        let path = Path::new(filename);
        if path.exists() && !force_output {
            anyhow::bail!(
                "File '{}' already exists. Use --force to overwrite.",
                filename
            );
        }
        if force_output && path.exists() {
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove file at {}", filename))?;
        }
        File::create(path)
            .and_then(|mut file| file.write_all(result_as_csv.as_bytes()))
            .with_context(|| format!("Failed to write {}", filename))?;
    } else {
        io::stdout()
            .lock()
            .write_all(result_as_csv.as_bytes())
            .with_context(|| "Failed to write output to stdout")?;
    }

    Ok(())
}
