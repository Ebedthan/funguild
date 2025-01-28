use std::path::Path;

use clap::{Arg, ArgAction, Command};

pub fn build_app() -> Command {
    Command::new("funguild")
        .about("Get functional guild of a taxon")
        .arg_required_else_help(true)
        .arg(
            Arg::new("TAXON")
                .conflicts_with("file")
                .help("a taxon name"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("takes TAXON from FILE"),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .help("output to FILE")
                .value_name("FILE")
                .value_parser(is_existing),
        )
        .arg(
            Arg::new("word")
                .short('w')
                .long("word")
                .action(ArgAction::SetTrue)
                .help("match only whole words"),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .help("erase output file")
                .action(ArgAction::SetTrue),
        )
}

fn is_existing(s: &str) -> Result<String, String> {
    if !Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("file should not already exists".to_string())
    }
}
