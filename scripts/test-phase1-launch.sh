#!/usr/bin/env sh
set -eu

for file in phase1 start_phase1 scripts/configure-phase1.sh scripts/install-phase1-command.sh; do
    echo "sh -n $file"
    sh -n "$file"
done

echo "phase1 help"
sh phase1 help | grep 'sh phase1'
sh phase1 help | grep 'install-phase1-command'

echo "start help"
sh start_phase1 --help | grep './phase1'
sh start_phase1 --help | grep 'sh phase1'

echo "start doctor"
sh start_phase1 --doctor | grep 'Phase1 launch doctor'
sh start_phase1 --doctor | grep 'gina'
sh start_phase1 --doctor | grep 'base1'
sh start_phase1 --doctor | grep 'launcher'

echo "configure dry-run"
sh scripts/configure-phase1.sh --dry-run | grep 'Launch command: sh phase1'
sh scripts/configure-phase1.sh --dry-run | grep 'Executable command: ./phase1'
sh scripts/configure-phase1.sh --dry-run | grep 'gina'
sh scripts/configure-phase1.sh --dry-run | grep 'base1'

echo "gina plugin files"
test -f plugins/gina.wasi
test -f plugins/ai.wasi
grep 'cybersecurity' plugins/gina.wasi
grep './start_phase1' plugins/gina.wasi

echo "Phase1 launch scripts validated"
