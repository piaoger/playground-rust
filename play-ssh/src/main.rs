

 extern crate ssh2;

use ssh2::Error as SshError;
use ssh2::Session;
use ssh2::Channel;

use std::io::prelude::*;


use std::net::{TcpStream};
use std::error::Error;
use std::path::Path;

/*
export SSH_EC2_INSTANCE=
export SSH_EC2_PEM=
export SSH_EC2_USER=
*/


pub enum AuthedUser {
    KeyUser(String, String),
    PwdUser(String, String),
}

fn remote_exec(sess: &Session, cmd: &String, has_resp: bool) {

fn sudo_yum_install(sess: &Session, cmd: &String, user: &String) {
    let mut channel = sess.channel_session().unwrap();
    // not sudoer, so use sudo -c cmd - user
    //  http://stackoverflow.com/questions/6851669/netssh2-with-sudo
    // let cmd = format!("sudo su -c '{cd  ~/w   ;  ls -lrt }' - piaoger");
    let sudocmd = format!("sudo su -c '{}' - {}", cmd, user);
    channel.exec(&sudocmd).unwrap();
    channel.write_all(b"y\n").unwrap();

        let mut s = String::new();
        match channel.read_to_string(&mut s) {
            Ok(_) => println!("{}", s),
            Err(_) => println!("err"),
    }
}
fn remote_exec(sess: &Session, cmd: &String, has_resp: bool) {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(&cmd).unwrap();
    if has_resp {
        loop {
            {
                let mut r2 : Vec<u8> = (0..500).collect();
                //channel.write_all(&[0x13]).unwrap();
                 let size = channel.read(&mut r2).unwrap();
                 if size <= 0 {
                    break;
                 }
                // remove chargen like following examples
                // 89:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXY
                // println!("{}", String::from_utf8_lossy(&r2 ));
                println!("{}", String::from_utf8_lossy(&r2[0 .. size-1]) );
            }
        }
    } else {
        // ...
    }
}


// we should return both TcpStream and Session,
// if not, calling sess.channel_session() will receive "Error { code: -7, msg: "Unable to send channel-open request" }"
// more information, https://github.com/alexcrichton/ssh2-rs/issues/16
pub fn authuser_start_session(ec2_instance: &String, authedeuser: &AuthedUser) -> Option<(TcpStream, Session)> {
    let tcp_result = TcpStream::connect(format!("{}:{}", &ec2_instance, 22).trim());
    if let Err(e) = tcp_result {
        println!("tcp connection failure: ({}) {:?}", &ec2_instance, e.description());
        return None;
    }

    let tcp = tcp_result.unwrap();
    let mut sess = Session::new().unwrap();
    match sess.handshake(&tcp) {
        Err(e) => {
            println!("handleshake failure: ({}) {}", &ec2_instance, e.description());
            return None;
        },
        _ => {
            ();
        }
    }

    let user_name = match *authedeuser {
        AuthedUser::KeyUser(ref user_name, ref pem_path) => {
                // libssh2 needs the corresponding public key in order to authenticate. If compiled against
                // openssl, it can generate it itself (pass nullptr to libssh2_userauth_publickey_fromfile_ex)
                // from the private key. If compiled against gnutls, this will fail. Until this is resolved,
                // first attempt to load a public key from <path/to/privatekey>.pub - if this file doesn't
                // exist, cross fingers, pass nullptr, and hope we're running against openssl :)
                // QByteArray publicKeyPath = params.sshKeyPath + ".pub";
                // bool useGuessedPublicKey = false;
                // {
                //     // guess the path to public key
                //     QFileInfo guessedPublicKey(publicKeyPath);
                //     if(guessedPublicKey.exists() && guessedPublicKey.isReadable())
                //         useGuessedPublicKey = true;
                // }
                match sess.userauth_pubkey_file(&user_name, None, Path::new(&pem_path), None) {
                    Err(e) => {
                        println!("ERROR: ({}) {}", &ec2_instance, e.description());
                        return None;
                    },
                    _ => { (); }
                }

                user_name
            },
        AuthedUser::PwdUser(ref user_name, ref pwd) => {
                match sess.userauth_password(&user_name, &pwd) {
                    Err(e) => {
                        println!("ERROR: ({}) {}", &ec2_instance, e.description());
                        return None;
                    },
                    _ => { (); }
                }
                user_name
            },
    };

    assert!(sess.authenticated());

   // sess.set_timeout(1000*20);

    Some((tcp, sess))
}

fn authuser_exec_on_serv(ec2_instance: &String, authedeuser: &AuthedUser) {

    let (tcp, sess) = match authuser_start_session(&ec2_instance, &authedeuser) {
        Some(val) => val,
        None => return,
    };

    {

        let cmd = format!("sudo su -c 'echo $USER' - {}", &user_name);
        remote_exec(&sess, &cmd, true);
    }

    {
        // use sudo su -c mode for no sudoers added for this user
        // http://www.cnblogs.com/zhuowei/archive/2009/04/13/1435190.html

        // but still do not know how to deal with read prompt
        // http://alvinalexander.com/linux-unix/shell-script-how-prompt-read-user-input-bash
        let cmd = format!("read -p xy");
        // ..
    }

    {
        // use sudo su -c mode for no sudoers added for this user
        // http://www.cnblogs.com/zhuowei/archive/2009/04/13/1435190.html

        // but still do not know how to deal with read prompt
        // http://alvinalexander.com/linux-unix/shell-script-how-prompt-read-user-input-bash
        let cmd = format!("sudo su -c 'cd  ~/w   ; ls -lrt; ' - {}", &user_name);
        remote_exec(&sess, &cmd, true);
    }

    {
        println!("# disk usage");
        sudo_yum_install(&sess, &"sudo yum install git".to_string(), &user_name)
    }

    {
        println!("# disk usage");
        let cmd = format!("df -lh");
        remote_exec(&sess, &cmd, true);
    }


    {
        // 10 basic examples of Linux ps command
        // http://www.binarytides.com/linux-ps-command/

        // description of ps fields:
        // https://aychin.wordpress.com/2014/03/25/linux-for-dba/

        println!("# process information (node | python)");
        println!("# comm pid pcpu pmem user time args");
        let cmd = format!("ps -e -o comm,pid,pcpu,pmem,user,time,args | grep 'node\\|python' | grep -v \"grep\"");
        remote_exec(&sess, &cmd, true);
    }

    {
        println!("# cpu usage");
        let cmd = format!("ps aux --sort=-pcpu| head -5");
        remote_exec(&sess, &cmd, true);
    }
}

fn exec_on_remote_serv() {

    let ec2_instance = ::std::env::var("SSH_EC2_INSTANCE").unwrap().to_string();
    let user_name = ::std::env::var("SSH_EC2_USER").unwrap().to_string();
    // let user_pwd = ::std::env::var("SSH_EC2_PWD").unwrap().to_string();
    let pem_path = ::std::env::var("SSH_EC2_PEM").unwrap().to_string();

    let pem_path_env = ::std::env::var("SSH_EC2_PEM");
    let user_pwd_env = ::std::env::var("SSH_EC2_PWD");

    authuser_exec_on_serv(&ec2_instance, &AuthedUser::KeyUser(user_name, pem_path));

}
fn main() {


    exec_on_remote_serv();

}

