use base64::Engine;
use clap::{Arg, ArgAction, ArgMatches, Command};
use shortguid::ShortGuid;

fn parse_arguments() -> ArgMatches {
    let input_id_arg = Arg::new("input_id").help("User input ID").required(true);

    let short_id_arg = Arg::new("short")
        .short('s')
        .long("short")
        .action(ArgAction::SetTrue);

    let long_id_arg = Arg::new("long")
        .short('l')
        .long("long")
        .action(ArgAction::SetTrue)
        .conflicts_with("short");

    let convert_command = Command::new("convert")
        .about("Convert the provided id to it's short or default UUID representation")
        .arg(&input_id_arg)
        .arg(short_id_arg)
        .arg(long_id_arg);

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

fn print_all_id_variants(shortguid: ShortGuid) {
    let engine = &base64::engine::general_purpose::STANDARD;
    let mut buffer = String::with_capacity(22);
    let uuid_as_bytes = shortguid.as_bytes();
    let hex_uuid_string = hex::encode(uuid_as_bytes);
    let little_endian_short = shortguid.to_bytes_le();
    let le_short_uuid = ShortGuid::from_bytes(&little_endian_short);
    engine.encode_string(uuid_as_bytes, &mut buffer);

    println!("Short UUID:                  {}", shortguid);
    println!("Base 64:                     {}", buffer);
    println!("UUID:                        {}", shortguid.as_uuid());
    println!("                             {}", hex_uuid_string);
    println!("Short UUID (little endian):  {}", le_short_uuid);
    println!("UUID (little endian):        {}", le_short_uuid.as_uuid());
}

fn main() -> Result<(), shortguid::ParseError> {
    let arg_matches = parse_arguments();

    match arg_matches.subcommand() {
        Some(("convert", sub_matches)) => match sub_matches.get_one::<String>("input_id") {
            Some(input_id) => {
                let shortguid = ShortGuid::try_parse(input_id)?;

                match (sub_matches.get_flag("short"), sub_matches.get_flag("long")) {
                    (true, false) => println!("{}", shortguid),
                    (false, true) => println!("{}", shortguid.as_uuid()),
                    _ => print_all_id_variants(shortguid),
                };

                Ok(())
            }
            None => unreachable!("The input_id arg is required"),
        },
        Some(("random", _)) => {
            let shortguid = ShortGuid::new_random();
            print_all_id_variants(shortguid);
            Ok(())
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
