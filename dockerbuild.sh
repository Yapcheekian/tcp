#!/bin/bash

set -e

case "$1" in
  run)
    docker-compose run --rm rust_tcp
    ;;
  *)
    echo "\"$1\" is an unknown command"
    ;;
esac
