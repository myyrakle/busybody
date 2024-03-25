# nosyman
![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.2.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/nosyman/blob/master/LICENSE)

* [English](/README.md)
* [한국어](/README-ko.md)

It is a disk monitoring tool. It only includes a simple function to send a notification to Slack when disk usage exceeds a threshold.
Currently, operation is only guaranteed on Linux.

## Install

Installation is simple. After installation, just add a symbolic link. (for use with sudo)
```
cargo install nodyman
sudo ln -s /home/$USER/.cargo/bin/nosyman /usr/bin/nosyman
```

## How to use

If the installation is successful, you can perform basic settings and load the daemon using the following commands.
```
nosyman --init
```

If the init task ran normally, you can always check the process status through the systemctl status command.
![image](https://github.com/myyrakle/nosyman/assets/16988115/bd0c6bf9-6417-4752-bb8d-d724aefe14ec)

When disk usage exceeds the threshold you set, a notification is sent to Slack. 
![image](https://github.com/myyrakle/nosyman/assets/16988115/8eab612e-a03b-4515-b01c-ac02caca9b32)
