#!/bin/bash
exec kola qemuexec --auto-cpus --ignition-direct \
     --qemu-size 300G --qemu-image /srv/walters/pubannex/machine-images/rhcos/rhcos-44.81.202004020530-0-qemu.x86_64.qcow2 \
     --qemu-nvme -i rhcos-raid-containers.json -- \
     -device nvme,drive=disk2,serial=2 -drive if=none,id=disk2,file=/tmp/disk,auto-read-only=off
