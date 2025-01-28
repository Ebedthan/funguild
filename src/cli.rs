use clap::{Arg, ArgAction, Command};

pub fn build_app() -> Command {
    Command::new("funguild")
        .about("Fetch the functional guild of a taxon based on FunGuild data.")
        .version("1.0.0")
        .arg_required_else_help(true)
        .arg(
            Arg::new("TAXON")
                .help("Specify the taxon name to query")
                .long_help(
                    "Provide the name of a signel taxon to query its functional guild. \
                    Cannot be used with the --file option.",
                )
                .conflicts_with("file")
                .required_unless_present("file"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Read taxon names from a file")
                .long_help(
                    "Specify a file containing taxon names, one per line. \
                    This option cannot be used with the TAXON argument.",
                )
                .value_name("FILE"),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .help("Write the output to a specified file")
                .long_help(
                    "Specify the file to write the ouput results. \
                    If not provided, the results will be printed to the standard output.",
                )
                .value_name("FILE"),
        )
        .arg(
            Arg::new("word")
                .short('w')
                .long("word")
                .help("Match only whole words")
                .long_help(
                    "Enable exact match mode, where only taxon names that match \
                        the provided query exactly will be returned.",
                )
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .help("Overwrite the output file if it exists")
                .long_help(
                    "If the specified output file already exists, this option allows \
                     overwriting the file. Without this flag, the program will abort \
                     if the file exists.",
                )
                .action(ArgAction::SetTrue),
        )
}
