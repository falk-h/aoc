#!/bin/sh
tmp="$(mktemp)"
vim -u /dev/null -e +'%s/\([^\n]\)\n\([^\n]\)/\1 \2/g' +"w! $tmp" +'q!' "$1"
grep byr "$tmp" | grep iyr | grep eyr | grep hgt | grep hcl | grep ecl | grep -c pid
rm "$tmp"
