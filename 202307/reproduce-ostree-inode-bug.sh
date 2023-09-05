#!/bin/bash
set -xeuo pipefail

if test ! -w /sysroot; then
  if test -n "${unshared:-}"; then
    mount -o remount,rw /sysroot
  else
    exec unshare -m env unshared=1 $0 "$@"
  fi
fi

verify-ostree() {
	podman run --rm -ti --pull=newer --privileged -v /:/rootfs --net=none quay.io/cgwalters/ostree-ext-dev provisional-repair repair --sysroot /rootfs/sysroot --dry-run
}

if test -f /run/ostree/staged-deployment; then ostree admin undeploy 0; fi
ostree refs --delete ostree/container/image
rpm-ostree rebase ostree-unverified-registry:quay.io/fedora/fedora-coreos:stable
verify-ostree
