#!/bin/sh
# なぜこんなことをしているのか：Dockerの`plantuml/plantuml`イメージは日本語フォントを持っていないため、SVGを生成すると描画が崩れる。そのため、`plantuml/plantuml-server`イメージを使ってローカルでPlantUMLサーバーを立ち上げ、そこに`*.puml`ファイルをPOSTして`*.svg`を出力する。
docker run --rm -u $(id -u):$(id -g) -v .:/data --entrypoint /bin/sh plantuml/plantuml-server -c '
/entrypoint.sh  > /dev/null 2>&1 &
until curl -s -f http://localhost:8080/svg > /dev/null; do
  sleep 1
done
find /data -name "*.puml" | while read -r puml; do
  svg="${puml%.puml}.svg"
  if [ ! -f "$svg" ]; then
    echo "Generating $svg from $puml"
    curl -H "Content-Type: text/plain; charset=UTF-8" -s -f --data-binary @"$puml" http://localhost:8080/svg > "$svg"
  fi
done
exit
'