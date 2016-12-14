#!/bin/sh
set -xeuo pipefail
inst() {
    yum -y install "$@"
}
inst yum-utils tmux sudo
inst gcc redhat-rpm-config make mock fedpkg
yum-builddep -y glib2 systemd rpm-ostree

cp /etc/skel/.bash* /root
useradd walters
usermod -a -G wheel,mock walters

ln -s /srv/walters/src ~walters/src
chown -R -h walters: ~walters

