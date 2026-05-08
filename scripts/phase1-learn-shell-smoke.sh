#!/usr/bin/env sh
set -eu

printf '%s\n' 'phase1 learn shell smoke test'

cargo test learn --all-targets
cargo test registry --all-targets

printf '%s\n' 'manual in-shell exercise:'
printf '%s\n' '  sh phase1'
printf '%s\n' '  learn status'
printf '%s\n' '  sysinfo'
printf '%s\n' '  security'
printf '%s\n' '  learn import-history'
printf '%s\n' '  learn suggest'
