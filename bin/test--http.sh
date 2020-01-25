#!/bin/sh
function main() {
  cd test
  TEST_PORT="$(echo $TEST_URL | cut -f 3 -d ':')"
  PID=$(_pid)
  if [ $PID ]; then
    kill -HUP $PID
  fi
  while [ -z $(_pid) ]; do
    sleep 1
  done
  node http.js
}
function _pid() {
  lsof -i tcp:$TEST_PORT | grep -v PID | awk '{print $2}'
}
main