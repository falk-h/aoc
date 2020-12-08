#!/bin/sh

days=

if [ $# -eq 0 ]; then
    echo "Usage:"
    echo "    $0 DAY_MAX"
    echo "    $0 -o DAY [DAY...]"
    exit 1
fi

if [ "$1" = "-o" ]; then
    shift
    days=$*
else
    days=$(seq 1 "$1")
fi

export TIMEFMT="%uE"
for day in $days; do
    for part in 1 2; do
        printf "Building day %s part %s..." "$day" "$part"
        if ! cargo build --release --bin "day$day-part$part" > /dev/null 2>&1; then
            echo "Failed to build day $day part $part"
            exit 1
        fi
        printf "\e[1;K\e[100;D"
        printf "Day %s part %s took " "$day" "$part"
        zsh -c "time \"target/release/day$day-part$part\" \"inputs/day$day/input.txt\"" > /dev/null
    done
done
