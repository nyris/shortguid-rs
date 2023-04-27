use clap::{Arg, ArgMatches, Command};
use shortguid::ShortGuid;

fn parse_arguments() -> ArgMatches {
    let input_id_arg = Arg::new("input_id").help("User input ID").required(true);

    let convert_command = Command::new("convert")
        .about("Convert the provided id to it's short or default UUID representation")
        .arg(&input_id_arg);

    let random_command = Command::new("random")
        .about("Create a random UUID and print all of it's available representations");

    Command::new("ShortGuid CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Markus Mayer <m.mayer@nyris.io>")
        .about("A tool for generating different UUID representations")
        .arg_required_else_help(true)
        .subcommand(convert_command)
        .subcommand(random_command)
        .get_matches()
}

fn main() {
    let arg_matches = parse_arguments();

    match arg_matches.subcommand() {
        Some(("convert", sub_matches)) => match sub_matches.get_one::<String>("input_id") {
            Some(input_id) => {
                todo!()
            }
            None => unreachable!("The input_id arg is required"),
        },
        Some(("random", _)) => {
            todo!()
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
