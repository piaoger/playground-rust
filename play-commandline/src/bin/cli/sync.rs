
use cloudstorage::aws;

use clap::{ArgMatches};
use std::error::Error;
use std::io;

pub fn exec(args: &ArgMatches) -> Result<(), Box<Error>> {

    if args.is_present("bucket") && args.is_present("key") && args.is_present("dest") {
        aws::s3::sync();
        Ok(())
    } else {
        Err(Box::new(io::Error::new(io::ErrorKind::Other,
            "bucket, key and dir are not provided"))
        )
    }

}