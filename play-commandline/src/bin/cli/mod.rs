

pub mod config;
pub mod sync;

// std
use std::io::{self, Write};

// deps
use clap::{App, SubCommand};


fn clap_app<'a, 'v, 'ab, 'u, 'h, 'ar>() -> App<'a, 'v, 'ab, 'u, 'h, 'ar> {
    // Create a list of valid arguments and sub-commands
    let matches = App::new("cloudstorage")
                    .about("A commandline to work with data storages")
                    .author("Piaoger Gong <piaoger@gmail.com>")
                    // Get the version from our Cargo.toml using clap's crate_version!() macro
                    .version(env!("CARGO_PKG_VERSION"))
                    .subcommand_required(true)
                    .after_help("For more information about a subcommand, try `cloudstorage <command> --help`")
                    .subcommand(SubCommand::with_name("config")
                        .about("config accesskey and secretid")
                        .version(&"0.1.0")
                        .arg_from_usage("--accesskey 'Copies the default theme into your source folder'")
                        .arg_from_usage("--secretid 'skip confirmation prompts'"))
                    .subcommand(SubCommand::with_name("sync")
                        .about("sync file from s3 to local directory")
                        .version(&"0.1.0")
                        .arg_from_usage("--bucket 's3 bucket'")
                        .arg_from_usage("--key 's3 key'")
                        .arg_from_usage("--dest 'destinate directory'"));

    return matches;
}


macro_rules! app_matches_subcommand {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    // the `expr` designator is used for expressions
    //
    ($matches:ident, $commad:ident) => (
        if let Some(matches) = $matches.subcommand_matches(stringify!($commad)) {
            match $commad::exec(matches) {
                Ok(_) => {
                },
                Err(e) => {
                },
            }
        }
    );
}

pub fn app_exec()  {

    let app_matches = clap_app().get_matches();

    // // Check which subcomamnds ...
    // let res = match app_matches.subcommand() {
    //     ("config", Some(sub_matches))  => config::exec(sub_matches),
    //     ("sync", Some(sub_matches))    => sync::exec(sub_matches),
    //     (_, _)                         => unreachable!()
    // };

    // if let Err(e) = res {
    //     writeln!(&mut io::stderr(), "An error occured:\n{}", e).ok();
    // }
    app_matches_subcommand!(app_matches, config);
    app_matches_subcommand!(app_matches, sync);
}


