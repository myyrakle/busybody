# nosyman
![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.1.0-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/nosyman/blob/master/LICENSE)

디스크 모니터링 도구입니다. 디스크 사용량이 임계치를 넘었을때 슬랙으로 알림을 전송하는 간단한 기능만을 포함합니다.
현재는 Linux에서만 동작이 보장됩니다.

## Install

설치 방법은 간단합니다. 설치 후 심볼릭 링크만 달아줍니다. (sudo 사용을 위함)
```
cargo install nodyman
sudo ln -s /home/$USER/.cargo/bin/nosyman /usr/bin/nosyman
```

## How to use

설치가 제대로 되었다면 다음 명령을 통해 기본 설정과 데몬 load를 수행할 수 있습니다.
```
nosyman --init
```

init 작업이 정상적으로 동작했다면 systemctl status 명령을 통해 프로세스 상태를 상시 확인할 수 있습니다.
![image](https://github.com/myyrakle/nosyman/assets/16988115/bd0c6bf9-6417-4752-bb8d-d724aefe14ec)
