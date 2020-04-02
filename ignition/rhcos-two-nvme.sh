#!/bin/bash
set -euo pipefail
qemu-img create -f qcow2 /tmp/disk1 300G
qemu-img create -f qcow2 /tmp/disk2 300G
exec kola qemuexec --auto-cpus \
     --qemu-image /srv/walters/pubannex/machine-images/rhcos/rhcos-4.3.8-x86_64-qemu.x86_64.qcow2 \
     --qemu-nvme --ignition-direct -i "$1" -- \
     -device nvme,drive=i1,serial=i1 -drive if=none,id=i1,file=/tmp/disk1,auto-read-only=off \
     -device nvme,drive=i2,serial=i2 -drive if=none,id=i2,file=/tmp/disk2,auto-read-only=off
