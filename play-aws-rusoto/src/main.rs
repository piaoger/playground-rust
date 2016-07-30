

extern crate rusoto;

use rusoto::AwsError;
use rusoto::{ChainProvider, Region};
use rusoto::s3::S3Helper;


/*
export AWS_ACCESS_KEY_ID=
export AWS_SECRET_ACCESS_KEY=

*/


fn s3_list_buckets(s3: &mut S3Helper<ChainProvider>) -> Result<(), AwsError> {
    println!("listing buckets.");
    let response = try!(s3.list_buckets());
    println!("Got list of buckets: {:?}", response);
    for q in response.buckets {
        println!("Existing bucket: {:?}", q.name);
    }

    Ok(())
}

fn s3_create_bucket(s3: &mut S3Helper<ChainProvider>) -> Result<(), AwsError> {
    println!("creating buckets.");
    let response = try!(s3.create_bucket_in_region("create-bucket-xyz", Region::CnNorth1, None));
    println!("create bucket: {:?}", response);

    Ok(())
}

fn all_s3_tests() {

    println!("s3 integration tests starting up.");

    let mut s3 = S3Helper::new(ChainProvider::new(), Region::CnNorth1);

    match s3_create_bucket(&mut s3) {
        Ok(_) => { println!("Everything worked for S3 list buckets."); },
        Err(err) => { println!("Got error in s3 list buckets: {}", err); }
    }

    match s3_list_buckets(&mut s3) {
        Ok(_) => { println!("Everything worked for S3 list buckets."); },
        Err(err) => { println!("Got error in s3 list buckets: {}", err); }
    }
}


fn main() {
    all_s3_tests();
}
