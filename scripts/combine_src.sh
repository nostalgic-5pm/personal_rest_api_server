#!/usr/bin/env bash
set -euo pipefail

# 出力先
OUTPUT_FILE="scripts/all_code.txt"

# 既存の出力ファイルは上書き
: > "$OUTPUT_FILE"

# crates/v1/src 以下の .rs ファイルをソートして列挙し、順番に結合
find crates/v1/src -type f -name '*.rs' | sort | while read -r FILE; do
  echo "//// File: $FILE" >> "$OUTPUT_FILE"
  cat "$FILE" >> "$OUTPUT_FILE"
  echo -e "\n" >> "$OUTPUT_FILE"
done

echo "✅ ソースコードをまとめて $OUTPUT_FILE に出力しました"
