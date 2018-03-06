FROM ubuntu:16.04

RUN apt-get -y clean all && \
    sed -i 's%us.archive.ubuntu.com%mirrors.gigenet.com/ubuntuarchive/%' /etc/apt/sources.list && \
    apt-get -y update && \
    apt-get -y upgrade && \
    apt-get -y install software-properties-common python-software-properties && \
    add-apt-repository -y ppa:cwchien/gradle && \
    apt-get -y update && \
    apt-get -y install git default-jdk gradle wget ansible telnet zip netcat-traditional fakeroot gosu awscli samba smbclient iputils-ping curl libicu55 language-pack-en && \
    locale-gen en_US.UTF-8

RUN wget -O /usr/bin/lein https://raw.githubusercontent.com/technomancy/leiningen/stable/bin/lein

RUN chmod +x /usr/bin/lein

RUN apt-get -y install docker.io

RUN curl -o /rustup https://sh.rustup.rs -sSf

RUN sh /rustup -y -v

ENV PATH="/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/:${PATH}"

ENV PATH="/root/.cargo/bin/:${PATH}"

RUN cargo install cargo-watch

RUN chmod -R o+rx /root
