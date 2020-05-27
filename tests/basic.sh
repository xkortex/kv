#!/usr/bin/env bash

errcho() {
    (>&2 echo -e "\e[31m$1\e[0m")
}

for val in x travis GOLANG 'h@rdp#4ssW0rd'; do
  for key in hex abc b32 b64; do
      $GOPATH/bin/kv set $key $val
      if [[ "${key}" != $(kv get "${key}") ]]; then
        FAILED=1
        errcho "Failed encoding test: encoding: [${key}: ${val}]"
      fi
  done
done

if [[ -n "$FAILED" ]]; then
  echo "Failed one or more tests"
  exit 1
fi

echo "PASS!"