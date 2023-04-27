use clap::{Arg, ArgMatches, Command};

fn parse_arguments() -> ArgMatches {
    let uuid_arg = Arg::new("uuid").help("UUID");

    let convert_command = Command::new("convert")
        .about("Convert the provided id to it's short or default UUID representation")
        .arg(&uuid_arg);

    let random_command = Command::new("random")
        .about("Create a random UUID and print all of it's available representations");

    Command::new("ShortGuid CLI")
        .version("0.4.0")
        .author("Markus Mayer <m.mayer@nyris.io>")
        .about("A tool for generating different UUID representations")
        .arg_required_else_help(true)
        .subcommand(convert_command)
        .subcommand(random_command)
        .get_matches()
}

fn main() {}
