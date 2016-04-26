

use cloudstorage::aws;

use clap::{ArgMatches};
use std::error::Error;
use std::io;

pub fn exec(args: &ArgMatches) -> Result<(), Box<Error>> {

    if args.is_present("accesskey") && args.is_present("secretid")  {
        aws::config();
        Ok(())
    } else {
        Err(Box::new(io::Error::new(io::ErrorKind::Other,
            "accesskey and secretid are not provided"))
        )
    }
}