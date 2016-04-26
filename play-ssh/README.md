
# play-ssh

scp

ssh


About libssh2_userauth_publickey_fromfile_ex
http://manpages.ubuntu.com/manpages/saucy/man3/libssh2_userauth_publickey_fromfile_ex.3.html

Path name of the public key file, If libssh2 is built against OpenSSL, this option
       can be set to NULL.


## References

[rust ssh2](https://github.com/alexcrichton/ssh2-rs)

[ssh2.rs document](http://alexcrichton.com/ssh2-rs/ssh2/index.html)

https://github.com/andreasfrom/kindlepaper
https://github.com/rustyhorde/rh/blob/master/src/bootstrap.rs

https://github.com/ohwgiles/sequeljoe/blob/master/src/sshthread.cpp



###
「SCP和SFTP」

这次想说说「SCP和SFTP」。

 

不管SCP还是SFTP，都是SSH的功能之一。都是使用SSH协议来传输文件的。

不用说文件内容，就是登录时的用户信息都是经过SSH加密后才传输的，所以说SCP和SFTP实现了安全的文件传输。

 

SCP和CP命令相似，SFTP和FTP的使用方法也类似。SCP和SFTP的共同之处在于「使用SSH将文件加密才传输的」

使用「WinSCP」或者「FileZilla」之类的客户端，还可以和Windows之间进行文件传输。

 

SCP和SFTP的不同之处，首先就是之前提到的，SCP使用「SCP命令」，SFTP则类似「FTP处理文件」的使用方式。

它们的不同之处还不止如此，还有「SCP比较简单，是轻量级的，SFTP的功能则比较多」。

详细说的话，虽然还有很多不同之处，但2者的最大不同之处在于「SFTP在文件传输过程中中断的话，连接后还可以继续传输，但SCP不行」。

由于各种原因导致的文件传输中断是经常讨论的话题，所以这个区别（这里的区别指SFTP能断点续传，SCP则不能）被认为是最大的区别。