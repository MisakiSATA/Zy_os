#!/bin/sh
set -e

echo "runner got: $1" >&2

case "$1" in
  */deps/*)
    echo "mode: test" >&2
    exec bootimage runner "$1"
    ;;
  *)
    echo "mode: run" >&2
    exec bootimage runner "$1" -display curses
    ;;
esac