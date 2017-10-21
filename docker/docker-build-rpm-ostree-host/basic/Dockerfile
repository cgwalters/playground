FROM fedora:26
RUN yum -y install kernel{,-core,-modules} dracut-config-generic @core \
                   lvm2 cryptsetup audit policycoreutils \
                   rpm-ostree ostree{,-grub2} nss-altfiles \
    && yum -y remove cronie plymouth \
    && semodule -nB \
    && yum clean all \
    && rpm -evh dnf libdnf dnf-{yum,conf} dnf-plugins-core \
              libcomps deltarpm rpm-plugin-systemd-inhibit python3-{dnf,dnf-plugins-core,hawkey,gpg,libcomps,librepo} \
    && rm -rf /var/lib/dnf \
    && rpm -qa|sort
# https://bugzilla.redhat.com/show_bug.cgi?id=1265295
RUN echo 'Storage=persistent' >> /etc/systemd/journald.conf
# Undo the container base changes
RUN systemctl unmask systemd-remount-fs.service dev-hugepages.mount sys-fs-fuse-connections.mount systemd-logind.service getty.target console-getty.service
# FIXME - not starting for some reason
RUN systemctl mask firewalld
# https://ostree.readthedocs.io/en/latest/manual/adapting-existing/
RUN for x in srv home media mnt opt; do mv /${x} /var/${x} && ln -sr /var/${x} /${x}; done \
    && rm /root -rf && ln -sr /var/roothome /root \
    && rm /usr/local -rf && ln -sr /var/usrlocal /usr/local \
    && mkdir -p /sysroot && ln -sr /sysroot/ostree /ostree \
    && rm /tmp -rf && ln -sr /sysroot/tmp /tmp \
    && rm -rf /run/*