#!/bin/sh
while :; do
  nix-shell --run "hc run -i http" https://holochain.love
  CODE=$?
  if [ $CODE != 129 ]; then
    echo "Exit with code: $CODE"
    exit $CODE
  fi
done
