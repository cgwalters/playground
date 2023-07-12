#!/usr/bin/bash
set -euo pipefail
d=$(dirname $0)
cd $d
out=testdata.opt
set -x
cargo build
rm "${out}" -rf && mkdir "${out}"
cargo run -- testdata ${out}
