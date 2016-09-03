#!/bin/sh
set -xeuo pipefail
inst() {
    yum -y install "$@"
}
inst https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
(cd /etc/yum.repos.d &&
     for x in https://copr.fedorainfracloud.org/coprs/walters/buildtools/repo/epel-7/walters-buildtools-epel-7.repo http://download.devel.redhat.com/rel-eng/RCMTOOLS/rcm-tools-rhel-7-workstation.repo; do
	 curl -L -O ${x}
     done
)
inst https://www.rdoproject.org/repos/rdo-release.rpm
inst yum-utils tmux sudo python-{nova,glance}client
inst gcc redhat-rpm-config make mock fedpkg rpmdistro-gitoverlay
yum-builddep -y glib2 systemd 

cp /etc/skel/.bash* /root
useradd walters
usermod -a -G wheel,mock walters

