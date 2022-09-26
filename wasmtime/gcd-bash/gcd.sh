#!/bin/bash

function gcd() {
  # Cast to number; default = 0
  local x=$(($1))
  local y=$(($2))
  # Invoke GCD from module; supress stderr
  local result=$(wasmtime ../gcd.wat --invoke gcd $x $y 2>/dev/null)
  echo "$result"
}

# main
for num in "27 6" "6 27" "42 12"; do
  set -- $num
  echo "gcd($1, $2) = $(gcd "$1" "$2")"
done
