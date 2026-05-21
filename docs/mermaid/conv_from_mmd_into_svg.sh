#!/bin/sh
for f in *.mmd; do
  out="${f%.mmd}.svg"
  echo "Converting $f -> $out"

  docker run --rm \
    -u "$(id -u):$(id -g)" \
    -v "$(pwd):/data" \
    minlag/mermaid-cli \
    -i "/data/$f" \
    -o "/data/$out"
done
