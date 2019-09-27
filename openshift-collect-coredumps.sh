#!/bin/bash
# Scrape core dumps out of an OpenShift 4 cluster.  This
# is a Kubernetes native shell script.
set -xeuo pipefail

nodes=$(oc get --no-headers -o name node)
for node in ${nodes}; do
    node=${node##node/}
    oc debug node/${node} -- ls -A /host/var/lib/systemd/coredump > coredumps.txt
    if [ -s coredumps.txt ]; then
        oc debug node/${node} -- tar -c -C  /host/var/lib/systemd/coredump -f - . > coredumps-${node}.tar
    fi
    rm -f coredumps.txt
done
