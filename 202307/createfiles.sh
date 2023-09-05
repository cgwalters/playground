#!/bin/bash
# Create and remove a bunch of random files
set -euo pipefail
td=$(mktemp -d -p /var/tmp)
echo writing to $td
cd $td
for x in $(seq 5000); do
  echo > dummy.$x
  ls -i dummy.$x
done
cd -
rm $td -rf
