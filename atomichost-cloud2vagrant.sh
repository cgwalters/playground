#!/bin/bash

# Convert an OpenStack/AWS-oriented image using cloud-init into a
# Vagrant box.
# usage: $0 /path/to/atomichost.qcow2 /path/to/output/atomichost-vagrant.box

set -euo pipefail

src=$1
shift
dest=$1
shift

tmpd=$(mktemp -d)
touch ${tmpd}/.tempdir
mntdir=${tmpd}/mnt
mkdir -p ${mntdir}
cleanup_tmp() {
    if test -d ${mntdir}; then
        guestunmount ${mntdir} || true
    fi
    if test -f ${tmpd}/.tempdir; then
        rm ${tmpd} -rf
    fi
}
trap cleanup_tmp EXIT

echo "Using tempdir ${tmpd}"
set -x

cd ${tmpd}
# Copy the image
cp ${src} box.img
guestmount --pid guestmount.pid -a box.img -m /dev/mapper/atomicos-root ${mntdir}
cat guestmount.pid
# Find the deployment dir

(cd ${mntdir};
 test -d ./ostree/deploy
 osdir=
 for x in .//ostree/deploy/*; do
     if ! test -d ${x}/var; then
         continue
     fi
     osdir=$x
     break
 done
 test -n "${osdir}"
 echo "using osname: $(basename ${osdir})"
 osvar=${osdir}/var
 deploydir=
 for d in ${osdir}/deploy/*.[0-9]; do
     if ! test -d ${d}/usr; then
         continue
     fi
     deploydir=$d
 done
 test -n "${deploydir}"
 echo "Using deployment: ${deploydir}"
)
(cd ${deploydir}
 # delete the cloud-init target
 rm -f ./etc/systemd/system/multi-user.target.wants/cloud*.service
 cp ./usr/lib/systemd/system/cloud-init.service ./etc/systemd/system/vagrant-prep.service
 cat >./etc/systemd/system/vagrant-prep.service <<EOF
[Unit]
Before=network-online.target sshd.service sshd-keygen.service systemd-user-sessions.service
Wants=local-fs.target cloud-init-local.service sshd.service sshd-keygen.service
ConditionPathExists=!/var/lib/vagrant.initialized

[Service]
Type=oneshot
ExecStart=/usr/libexec/vagrant-prep
RemainAfterExit=yes
TimeoutSec=0
EOF
 cat >./usr/libexec/vagrant-prep <<EOF
#!/bin/sh
useradd vagrant
passwd -u root
sed -i 's,Defaults\\s*requiretty,Defaults !requiretty,' /etc/sudoers
echo 'vagrant ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/vagrant-nopasswd
sed -i 's/.*UseDNS.*/UseDNS no/' /etc/ssh/sshd_config
mkdir -m 0700 -p ~vagrant/.ssh
cat > ~vagrant/.ssh/authorized_keys << EOKEYS
ssh-rsa AAAAB3NzaC1yc2EAAAABIwAAAQEA6NF8iallvQVp22WDkTkyrtvp9eWW6A8YVr+kz4TjGYe7gHzIw+niNltGEFHzD8+v1I2YJ6oXevct1YeS0o9HZyN1Q9qgCgzUFtdOKLv6IedplqoPkcmF0aYet2PkEDo3MlTBckFXPITAMzF8dJSIFo9D8HfdOV0IAdx4O7PtixWKn5y2hMNG0zQPyUecp4pzC6kivAIhyfHilFR61RGL+GPXQ2MWZWFYbAGjyiYJnAmCP3NOTd0jMZEnDkbUvxhMmBYSdETk1rRgm+R4LOzFUGaHqHDLKLX+FIPKcF96hrucXzcWyLbIbEgE98OHlnVYCzRdK8jlqm8tehUc9c9WhQ== vagrant insecure public key
EOKEYS
chmod 600 ~vagrant/.ssh/authorized_keys
chown -R vagrant:vagrant ~vagrant/.ssh/
EOF
 chmod 0755 ./usr/libexec/vagrant-prep
)
guestunmount ${mntdir}
ls -al ${mntdir}
pid="$(cat guestmount.pid)"
timeout=10
count=$timeout
while kill -0 "$pid" 2>/dev/null && [ $count -gt 0 ]; do
    sleep 1
    ((count--))
done
if [ $count -eq 0 ]; then
    echo "$0: wait for guestmount to exit failed after $timeout seconds"
    kill -TERM ${pid}
fi
rmdir ${mntdir}
cat >guestfish.cmds <<EOF
rm {}cloud-config.service
cp-a 
selinux-relabel 
EOF
echo 'selinux-relab'
guestfish -a box.img

cd ${tmpd}
cat >${tmpd}/metadata.json <<EOF
{
  "provider": "libvirt",
  "format": "qcow2",
  "virtual_size": 41
}
EOF
cat >${tmpd}/Vagrantfile <<EOF
Vagrant.configure('2') do |config|
        config.vm.provider :libvirt do |libvirt|
                libvirt.driver = 'kvm'
                libvirt.connect_via_ssh = false
                libvirt.username = 'root'
                libvirt.storage_pool_name = 'default'
        end
end
EOF
ls -alh box.img metadata.json Vagrantfile
tar -cz -f ${dest} box.img metadata.json Vagrantfile
