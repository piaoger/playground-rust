extern crate cronparse;
extern crate users;

// cronparse
use cronparse::{CrontabFile, CrontabFileError};
use cronparse::crontab::{CrontabEntry,  UserCrontabEntry};

// rslib
use std::fs;


use std::env;
use std::path::PathBuf;

std::net::Ipv4Addr

// export PATH=$PATH:/home/xyz/bin
// echo $PATH
fn play_env() {

    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from("/home/piaoger/.cargo"));
        let new_path = env::join_paths(paths.iter()).unwrap();
        env::set_var("PATH", &new_path);

        println!("{:?}", paths);
    }
}


// std::process.
// http://doc.rust-lang.org/stable/std/process/struct.Command.html#method.output

// git
// https://github.com/chef/delivery-cli/blob/master/src/delivery/git/mod.rs
// https://www.atlassian.com/git/tutorials/
use std::{ process, str};
fn play_process_command() {
    use std::process::Command;

    let output = Command::new("sh")
                         .arg("-c")
                         .arg("ls -lrt")
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("err code: {}", &output.status.code().unwrap());

    // uses String::from_utf8_lossy(&output.stdout)?
    println!("stdout: {}", str::from_utf8(&output.stdout).unwrap().trim().to_string());
    println!("stderr: {}", str::from_utf8(&output.stderr).unwrap().trim().to_string());
}



// which wget
fn play_fs_which() {
    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();

        println!("PATH = {:?}", paths);

        for  path in paths.iter() {
            let wget = path.join("wget");
            if wget.is_file() && wget.exists() {
                println!("which wget: {:?}", wget);
                break;
            }
        }
    }
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
//   HP-UX Unix
//       – /var/spool/cron/crontabs/ (user cron location /var/spool/cron/crontabs/piaoger)
//   IBM AIX Unix
//       – /var/spool/cron/ (user cron location /var/spool/cron/piaoger)

#[cfg(target_os = "macos")]
const maccrostab :&'static str = "/usr/lib/cron/tabs/";

#[cfg(target_os = "linux")]
const maccrostab :&'static str = "/var/spool/cron/crontabs";

#[cfg(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
const bsdcrostab :&'static str = "/var/cron/tabs/";


extern crate regex;

use regex::Regex;
// use std::{env, fs};
use std::io::Read;

struct Os {
    family: String,
    platform: String,
}

// from :
/// Fingerprint the OS more granularly than Rust to ensure we build
/// the right modules.
fn guess_os() -> Os {
    if cfg!(target_os = "linux") {

    	// /etc/redhat-release
    	 // RHEL, CentOS, Scientific Linux, Fedora
        // Red Hat family
        if let Ok(mut fh) = fs::File::open("/etc/redhat-release") {
            let mut fc = String::new();
            fh.read_to_string(&mut fc).unwrap();

            let regex = Regex::new(r"^([A-Za-z ]+?)(?: AS)? release").unwrap();
            if let Some(cap) = regex.captures(&fc) {
                let platform = match cap.at(1).unwrap() {
                    "Red Hat Enterprise Linux" => "rhel".to_string(),
                    _ => cap.at(1).unwrap().to_string().to_lowercase(),
                };

                return Os {
                    family: "redhat".to_string(),
                    platform: platform,
                };
            }
        }

        // Ubuntu
        else if let Ok(_) = fs::metadata("/etc/lsb-release") {
            return Os {
                family: "debian".to_string(),
                platform: "ubuntu".to_string(),
            };
        }
        // Debian
        else if let Ok(_) = fs::metadata("/etc/debian_version") {
            return Os {
                family: "debian".to_string(),
                platform: "debian".to_string(),
            };
        }

        panic!("Unknown Linux distro");

    } else {
        return Os {
            family: env::consts::FAMILY.to_string(),
            platform: env::consts::OS.to_string(),
        };
    }

    panic!("Unsupported distro");
}

fn play_crontab() {

    // list crontab for all users
    if let Ok(dir) = fs::read_dir("/usr/lib/cron/tabs") {
        for item in dir {
            if let Ok(entry) = item {
                if let Some(name) = entry.file_name().to_str() {
                    if users::get_user_by_name(name).is_some() {
                        println!("[info] - user name {}", name);
                    } else {
                        println!("[warn] - crontab found with no matching user:{}", name);
                    }
                }
            }
        }
    } else {
        println!("[warn] - crontab not found");
    }

    // list all entry in piaoger's crontab on mac osx
    let mut crontab = CrontabFile::<UserCrontabEntry>::new("/usr/lib/cron/tabs/piaoger").unwrap();
    for (_, crontabentry) in crontab.enumerate()  {

        if let Ok(entry) = crontabentry {
            match entry {
                CrontabEntry::User(userentry) => {
                    println!("{} {}", userentry.cmd, userentry.sched);
                },
                _ => {}
            }
        }
    }
}



fn main() {

    println!("current_exe {:?}", env::current_exe().unwrap().parent().unwrap()
        .join("n").join("f1").join("f2").join("f3"));

    let os = guess_os();
    println!("cargo:rustc-cfg=in_os_family=\"{}\"", os.family);
    println!("cargo:rustc-cfg=in_os_platform=\"{}\"", os.platform);

    play_process_command();
    play_env();
    play_fs_which();
    play_crontab();
}