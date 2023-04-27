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

fn showcase(shortguid: ShortGuid) {
    println!("Short UUID:            {}", shortguid);
    println!("UUID:                  {}", shortguid.as_uuid());

    let uuid_as_bytes = shortguid.as_bytes();
    let hex_uuid_string = hex::encode(uuid_as_bytes);
    println!("UUID (bytes):          {}", hex_uuid_string);

    let little_endian = shortguid.to_bytes_le();
    let hex_little_endian_string = hex::encode(little_endian);
    println!("UUID (little endian):  {}", hex_little_endian_string);
}

fn main() -> Result<(), shortguid::ParseError> {
    let arg_matches = parse_arguments();

    match arg_matches.subcommand() {
        Some(("convert", sub_matches)) => match sub_matches.get_one::<String>("input_id") {
            Some(input_id) => {
                let shortguid = ShortGuid::try_parse(input_id)?;
                showcase(shortguid);
                Ok(())
            }
            None => unreachable!("The input_id arg is required"),
        },
        Some(("random", _)) => {
            let shortguid = ShortGuid::new_random();
            showcase(shortguid);
            Ok(())
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
