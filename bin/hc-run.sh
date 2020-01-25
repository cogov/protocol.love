#!/bin/sh
while :; do
  nix-shell --run "hc run -i http" https://github.com/holochain/holonix/archive/develop.tar.gz
  CODE=$?
  if [ $CODE != 129 ]; then
    echo "Exit with code: $CODE"
    exit $CODE
  fi
done
