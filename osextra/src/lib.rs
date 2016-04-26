
extern crate regex;

use regex::Regex;

use std::{env, fs};

use std::io::Read;
use std::path::PathBuf;

// str & String in rust
// http://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html
// http://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html

// maybe use str in the future but more complicated.
//  family: &'a str,
pub struct Env {
    family: String,
    os: String,
}

fn redhat_release_os_string(release : &String) -> Option<Env> {
    match Regex::new(r"^([A-Za-z ]+?)(?: AS)? release") {
         Ok(regex) => {
            match regex.captures(&release) {
                Some(captures) => {
                    let os = match captures.at(1) {
                        Some(v) if v == "Red Hat Enterprise Linux" => "rhel".to_string(),
                        Some(v) => v.to_string().to_lowercase(),
                        None => "rhel".to_string(),
                    };

                    Some(Env {
                        family: "redhat".to_string(),
                        os: os,
                    })
                },
                None => None
            }
         },
         Err(_) => None
    }
}

// RHEL, CentOS, Scientific Linux, Fedora
fn check_redhat_family() -> Option<Env> {

    match fs::File::open("/etc/redhat-release") {
        Err(_) => None,
        Ok(mut file) => {
            let mut data = String::new();
            match file.read_to_string(&mut data) {
                Ok(_) => redhat_release_os_string(&data),
                Err(_) => None,
            }
        }

    }
}

fn check_ubuntu() -> Option<Env> {
    match fs::metadata("/etc/lsb-release") {
        Ok(_) => Some(Env {
                family: "debian".to_string(),
                os: "ubuntu".to_string(),
            }),
        Err(_) => None
    }
}

// /etc/debian_version
fn check_debian() -> Option<Env> {
    match fs::metadata("/etc/debian_version") {
        Ok(_) => Some(Env {
                family: "debian".to_string(),
                os: "debian".to_string(),
            }),
        Err(_) => None
    }
}


fn default_env() -> Env {
    Env {
        family: env::consts::FAMILY.to_string(),
        os: env::consts::OS.to_string(),
    }
}


/// from :
/// Fingerprint the OS more granularly
fn env_details() -> Env {

    if cfg!(target_os = "linux") {
        let mut osenv = check_redhat_family();
        osenv = if osenv.is_some() { osenv } else { check_ubuntu() };
        osenv = if osenv.is_some() { osenv } else { check_debian() };

        match osenv {
            Some(v) => v,
            None => default_env(),
        }

    } else if cfg!(target_os = "freebsd") || cfg!(target_os = "netbsd") || cfg!(target_os = "openbsd") {
        Env {
            family: "bsd".to_string(),
            os: env::consts::OS.to_string(),
        }

    } else  {
       default_env()
    }
}


pub fn family() -> String {
    env_details().family
}

pub fn os() -> String {
    env_details().os
}


//  crontab parse for rust
//  https://github.com/kstep/cronparse.rs

// Directory for personal crontab files(http://www.cyberciti.biz/faq/where-is-the-crontab-file/)
// Linux and Unix-like operating system may change the default from /var/spool/cron/ to something else.
// Use the following as a guideline for your OS (assuming that user name is piaoger):
//   Mac OS X
//       – /usr/lib/cron/tabs/ (user cron location /usr/lib/cron/tabs/piaoger)
//   FreeBSD/OpenBSD/NetBSD
//       – /var/cron/tabs/ (user cron location /var/cron/tabs/piaoger)
//   CentOS/Red Hat/RHEL/Fedora/Scientific Linux
//       – /var/spool/cron/ (user cron location /var/spool/cron/piaoger)
//   Debian / Ubuntu Linux
//       – /var/spool/cron/crontabs/ (user cron location /var/spool/cron/crontabs/piaoger)
pub fn crontabs() -> Option<String> {
    let family_os = env_details();
    match (&family_os.family[..], &family_os.os[..]){
        ("unix" , "macos") => Some("/usr/lib/cron/tabs/".to_string()),
        ("debian", _) => Some("/var/spool/cron/crontabs/".to_string()),
        ("redhat", _) => Some("/var/spool/cron/".to_string()),
        ("bsd", _)  => Some("/var/cron/tabs/".to_string()),
        ("windows", _) => None,
        _ => None,
    }
}

// https://github.com/realpython/shutilwhich/blob/master/shutilwhich/lib.py
// mod fsextra::which


#[test]
fn test_family_os() -> () {
    println!("family: {}  os:{}", family(), os());
}

#[test]
fn test_crontabdir() -> () {
    println!("crontab dir: {:?} ", crontabs().unwrap());
}