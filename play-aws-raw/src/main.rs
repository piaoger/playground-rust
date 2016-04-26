
#![feature(rustc_private)]
#![feature(convert)]

/// Play ground to try S3 restful api based on crypto, hyper and rust.
/// Authenticating Requests (AWS Signature Version 4)
/// Any new regions after January 30, 2014 will support only Signature Version 4
/// http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-auth-using-authorization-header.html
/// http://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html
/// http://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-streaming.html

extern crate time;
extern crate url;
extern crate crypto;
extern crate serialize;
extern crate hyper;

use std::env;

use std::ascii::AsciiExt;
use std::str;

use std::collections::BTreeMap; 
use std::collections::btree_map::Entry;

use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

use crypto::mac::Mac;
use crypto::digest::Digest;
use crypto::sha2::Sha256;


use url::Host::{Domain, Ipv6};

// Find a new way to to_hex
use serialize::hex::ToHex;

use hyper::Client;
use hyper::header::{Headers, UserAgent};


/// Environment variables used in s3 command line
/*
export AWS_ACCESS_KEY_ID=
export AWS_SECRET_ACCESS_KEY=
*/
fn aws_access_keys() ->(String, String) {
    let aws_access_key_id = match env::var("AWS_ACCESS_KEY_ID") {
        Ok(s)  => s,
        Err(_) => panic!("AWS_ACCESS_KEY_ID is not defined")
    };
    let aws_secret_access_key = match env::var("AWS_SECRET_ACCESS_KEY") {
        Ok(s)  => s,
        Err(_) => panic!("AWS_SECRET_ACCESS_KEY is not defined")
    };
    (aws_access_key_id, aws_secret_access_key)
}


/// SHA-256 produces a 256-bit (32-byte) message digest hash
fn sha256(input: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.input(input);

    let mut digest = [0; 32];
    hasher.result(&mut digest);

    digest
}

/// HMAC-SHA256("<secret>", "<data>")
fn hmac_sha256 (secret : &[u8], data : &[u8]) -> Vec<u8> {
    let mut hmac = crypto::hmac::Hmac::new(crypto::sha2::Sha256::new(), secret);
    hmac.input(data);
    hmac.result().code().to_vec()
}

/// Hex(SHA256Hash(<data>)
fn hex_sha256(val: &str) -> String {
    let digest = sha256(val.as_bytes());
    digest.to_hex().to_string()
}


/// Calculate signing key.
/// Authorization: AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, 
/// SignedHeaders=host;range;x-amz-date,
/// Signature=fe5f80f77d5fa3beca038a248ff027d0445342fe2855ddc963176630326f1024
fn signing_key (secret: &str, region : &str, service : &str, now_ymd : &str) -> Vec<u8> {
    let key   : String  = format!("AWS4{}", secret);
    let nowkey    : Vec<u8> = hmac_sha256( key.as_bytes(), now_ymd.as_bytes());
    let nowregion_key  : Vec<u8> = hmac_sha256( &nowkey, region.as_bytes() );
    let nowregion_service_key : Vec<u8> = hmac_sha256( &nowregion_key, service.as_bytes());
    hmac_sha256(&nowregion_service_key, "aws4_request".as_bytes())
}


/// Append Key Value to a map of headers
fn append_header(headers: &mut BTreeMap<String, Vec<Vec<u8>>>, key: &str, value: &str) {
    let k = key.to_ascii_lowercase().to_string();
    let v = value.as_bytes().to_vec();

    match headers.entry(k) {
        Entry::Vacant(entry) => {
            let mut values = Vec::new();
            values.push(v);
            entry.insert(values);
        },
        Entry::Occupied(entry) => {
            entry.into_mut().push(v);
        }
    };
}

/// Set hyper headers
fn set_headers(headers: &mut Headers, kvs: &mut BTreeMap<String, Vec<Vec<u8>>>) {
    let ua = "rust-playground".to_string();
    headers.set(UserAgent(ua.to_owned()));
    headers.set_raw("".to_string(), vec!["".as_bytes().to_vec()]); // Only one value now.

    for kv in kvs.iter() {
        headers.set_raw(kv.0.to_owned(), kv.1.to_owned());
    }
}

/// Create canonical url (placeholder now)
fn canonical_uri(path: &str) -> String {
    path.to_string()
}

/// Create canonical query string (placeholder now)
fn canonical_query_string(query_string: &str) -> String {
    query_string.to_string()
}

 fn canonical_values(values: &Vec<Vec<u8>>) -> String {
    let mut st = String::new();
    for v in values {
        let s = str::from_utf8(v).unwrap();
        if st.len() > 0 {
            st.push(',')
        }
        if s.starts_with("\""){
            st.push_str(&s);
        } else {
            st.push_str(s.replace("  ", " ").trim());
        }
    }
    st
}

/// Calculate canonical headers and signed headers.
fn canonical_request_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> (String, String) {
    let mut canonical = String::new();
    let mut signed = String::new();
    for (key, value) in headers.iter() {
        canonical.push_str(format!("{}:{}\n", 
            key.to_ascii_lowercase(), 
            canonical_values(value)
        ).as_ref());

        if signed.len() > 0 {
            signed.push(';')
        }
        signed.push_str(&key.to_ascii_lowercase());
    }
    (canonical, signed)
}

/// Calculate hashed payload 
fn  hashed_payload_from_string(payload: &str) -> String {
    let  digest = sha256(payload.as_bytes());
    digest.to_hex().to_string()
}

/// get object api
fn api_get_object(url_str : &str, output : &str) {

    let (aws_access_key_id, aws_secret_access_key) = aws_access_keys();

    let uri = url::Url::parse(url_str).unwrap();
    let host = match uri.host() {
            Some(ref v) => {
                match **v {
                 Domain(ref h) => h.clone(),
                 Ipv6(ref h)   => format!("{}", h) 
                }
            },
            None => "".to_string()
        };

    let now = time::now_utc();
    let now_ymd_hmz = now.strftime("%Y%m%dT%H%M%SZ").unwrap().to_string();
    let now_ymd = now.strftime("%Y%m%d").unwrap().to_string();
    // let now_ymd_hmz = "20130524T000000Z".to_string();
    // let now_ymd = "20130524".to_string();
 
    let mut kvs  = BTreeMap::<String, Vec<Vec<u8>>>::new();
    append_header(&mut kvs, "Host", host.as_str());
    append_header(&mut kvs, "range", "bytes=0-256");
    append_header(&mut kvs, "x-amz-date", now_ymd_hmz.as_str());
    append_header(&mut kvs, "x-amz-content-sha256", &hex_sha256(""));

    let http_method = "GET";

    let query = match uri.query {
        Some(ref value) => value.as_str(),
        None => ""
    };

    let (canonical_headers, signed_headers) = canonical_request_headers(&kvs);

    // ----------------------
    // Create a Canonical Request
    let canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
            &http_method,
            canonical_uri(uri.serialize_path().unwrap().as_str()),
            canonical_query_string(query),
            &canonical_headers,
            &signed_headers,
            &hashed_payload_from_string("")
        );

    println!("\ncanonical request");
    println!("----------------");
    println!("{}", canonical_request);
    println!("----------------");

    // ----------------------
    // Create a String to Sign
    let region = "us-east-1";
    let service = "s3";
    let scope = format!("{}/{}/{}/{}",
            now_ymd,
            &region,
            &service,
            "aws4_request"
        );
    let string_to_sign = format!("{}\n{}\n{}\n{}",
            "AWS4-HMAC-SHA256",
            now_ymd_hmz,
            &scope,
            hex_sha256(&canonical_request)
        );

    println!("\nstring to sign");
    println!("----------------");
    println!("{}", string_to_sign);
    println!("----------------");

    // ----------------------
    // Create a String to Sign
    let signing_key = signing_key(&aws_secret_access_key, &region, &service, &now_ymd.as_str());
    let signature = hmac_sha256(&signing_key, string_to_sign.as_bytes()).to_hex().to_string();

    println!("\nsignature");
    println!("----------------");
    println!("{}", signature);
    println!("----------------");


    // -----------------------------
    // build the actual auth header
    // SignedHeaders 
    let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            &aws_access_key_id, 
            &scope, 
            "host;range;x-amz-content-sha256;x-amz-date", // SignedHeaders 
            &signature
        );
    append_header(&mut kvs, "Authorization", &auth_header); //

    println!("\nauth header");
    println!("----------------");
    println!("{}", auth_header);
    println!("----------------");

    // -----------------------------
    // http request
    let mut headers = Headers::new();
    set_headers(&mut headers, &mut kvs);

    let client = Client::new();
    let mut res = client.get(uri.clone())
        .headers(headers)
        .send().unwrap();

    let mut buffer = Vec::new();
    res.read_to_end(&mut buffer).unwrap();

    let mut file = BufWriter::new(File::create(output).unwrap());
    file.write_all(&buffer).unwrap();
    file.flush().unwrap();
}

fn main() {
    api_get_object("https://play-aws.s3.amazonaws.com/play-aws.log", "play-aws-fragment.log");
}
