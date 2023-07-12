#!/bin/bash
set -euo pipefail

run_tmp_webserver() {
    local statedir=$1
    shift
    local dir=$1
    shift

    local tmpf=$(mktemp)
    cd ${dir}
    env PYTHONUNBUFFERED=1 setpriv --pdeathsig SIGTERM -- python3 -m http.server 0 &>${tmpf} &
    cd -
    child_pid=$!

    local tmpf_snap=$(mktemp)
    for x in $(seq 60); do
        echo "Waiting for web server ($x/60)..." >&2
        # Snapshot the output
        cp ${tmpf} ${tmpf_snap}
        # If it's non-empty, see whether it matches our regexp
        if test -s ${tmpf_snap}; then       # py3's http.server prints the http:// address also
          sed -e 's,Serving HTTP on 0.0.0.0 port \([0-9]*\)\( (http://[^)]*)\)\? \.\.\.,\1,' < ${tmpf_snap} > ${statedir}/httpd-port
            if ! cmp ${tmpf_snap} ${statedir}/httpd-port 1>/dev/null; then
                # If so, we've successfully extracted the port
                break
            fi
        fi
        sleep 1
    done

    if [ ! -f ${statedir}/httpd-port ]; then
      cat ${statedir}/httpd-output
      echo "can't start up httpd"; exit 1
    fi

    port=$(cat ${state}/httpd-port)
    echo "http://127.0.0.1:${port}" > ${statedir}/httpd-address
    echo "$child_pid" > ${statedir}/httpd-pid
}


build=${1:-latest}
shift

builddir=builds/${build}/x86_64
buildmeta=${builddir}/meta.json

metalimg=$(jq -er .images.metal.path)
instkernel=$(jq -er .images.kernel)
instinitramfs=$(jq -er .images.initramfs)

# OK, we seem to have the images.  Launch our temporary webserver.
statedir=tmp/coreos-installer-test
tftpdir=${statedir}/tftp
pxeconfigdir=${tftpdir}/pxelinux.cfg
mkdir -p "${pxeconfigdir}"

rm -rf "${statedir}" && mkdir -p "${statedir}"
run_tmp_webserver "${statedir}" "${tftpdir}"
port=$(cat ${statedir}/httpd-port)

for x in ${instkernel} ${instinitramfs} ${metalimg}; do
    ln ${builddir}/${x} ${tftpdir}
done
cat > ${pxeconfigdir}/default << EOF
DEFAULT pxeboot
TIMEOUT 20
PROMPT 0
LABEL pxeboot
    KERNEL ${instkernel}
    APPEND ip=dhcp rd.neednet=1 initrd=${instinitramfs} console=tty0 console=ttyS0 coreos.inst=yes coreos.inst.install_dev=vda coreos.inst.image_url=http://192.168.76.1:${port}/${metalimg} coreos.inst.ignition_url=http://192.168.76.1:${port}/config.ign
IPAPPEND 2
EOF

cp --reflink=auto /usr/share/syslinux/{pxelinux.0,ldlinux.c32} ${tftpdir}

qemu-img create -f qcow2 ${statedir}/disk.qcow2

cat > ${statedir}/config.ign << EOF
{
  "ignition": { "version": "2.2.0" },
  "systemd": {
    "units": [{
      "name": "coreos-test-installer.service",
      "enabled": true,
      "contents": "[Unit]\nRequires=dev-virtio\x2dports-completion.device\n[Service]\nType=oneshot\nExecStart=/bin/sh -c '/usr/bin/echo Hello World >/dev/virtio-ports && systemctl poweroff'\n\n[Install]\nWantedBy=multi-user.target"
    }]
  }
}
EOF

completionf=${statedir}/completion.txt
qemu-system-x86_64 -accel kvm -m 8192 \
       -object rng-random,filename=/dev/urandom,id=rng0 -device virtio-rng-pci,rng=rng0 \
       -boot n -option-rom /usr/share/qemu/pxe-rtl8139.rom \
       -device e1000,netdev=mynet0,mac=52:54:00:12:34:56 -netdev user,id=mynet0,net=192.168.76.0/24,dhcpstart=192.168.76.9,tftp=${tftpdir},bootfile=/pxelinux.0 \
       -drive if=virtio,file=${disk} \
       -device virtio-serial -device virtserialport,chardev=completion,name=completion \
       -chardev file,id=completion,path=${completionf}
cat ${completionf}

