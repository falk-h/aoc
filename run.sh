#!/bin/sh
set -eu
cd "$(dirname "$0")" || exit 1
if ! [ $# -ge 2 ]; then
    echo "Usage: $0 DAY PART [FLAGS]"
    exit 1
fi
day="$1"
part="$2"
shift 2
exec cargo run "$@" --bin "day$day-part$part" "inputs/day$day/input.txt"
