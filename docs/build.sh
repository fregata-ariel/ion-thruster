#!/usr/bin/env bash
# docs/build.sh — 全文書のglossary resolve + langfilter自動化
#
# 使い方:
#   cd docs && bash build.sh          # 全文書ビルド
#   cd docs && bash build.sh physics  # 指定文書のみ
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEFS="$SCRIPT_DIR/defs.json"
OUTPUT="$SCRIPT_DIR/output"
mkdir -p "$OUTPUT"

# ビルド対象の文書リスト
ALL_DOCS=(architecture physics api userguide)

# 引数があれば指定文書のみ
if [[ $# -gt 0 ]]; then
  DOCS=("$@")
else
  DOCS=("${ALL_DOCS[@]}")
fi

build_doc() {
  local name="$1"
  local src="$SCRIPT_DIR/$name/$name.md"

  if [[ ! -f "$src" ]]; then
    echo "  SKIP $name (source not found: $src)"
    return 0
  fi

  # glossary verify
  if ! glossary verify "$src" -f "$DEFS" 2>/dev/null; then
    echo "  FAIL $name: glossary verify failed" >&2
    return 1
  fi

  # ja output
  langfilter filter --lang ja "$src" \
    | glossary resolve --lang ja -f "$DEFS" \
    | sed '/^::: {lang=ja}$/d; /^::: {lang=en}$/d; /^:::$/d' \
    > "$OUTPUT/$name.ja.md"

  # en output
  langfilter filter --lang en "$src" \
    | glossary resolve --lang en -f "$DEFS" \
    | sed '/^::: {lang=ja}$/d; /^::: {lang=en}$/d; /^:::$/d' \
    > "$OUTPUT/$name.en.md"

  local ja_lines en_lines
  ja_lines=$(wc -l < "$OUTPUT/$name.ja.md")
  en_lines=$(wc -l < "$OUTPUT/$name.en.md")
  echo "  OK   $name  →  $name.ja.md (${ja_lines}L) / $name.en.md (${en_lines}L)"
}

echo "=== ion-craft docs build ==="
echo "defs: $DEFS"
echo "output: $OUTPUT"
echo ""

fail=0
for doc in "${DOCS[@]}"; do
  if ! build_doc "$doc"; then
    fail=1
  fi
done

echo ""
if [[ $fail -eq 0 ]]; then
  echo "All done."
else
  echo "Some documents failed." >&2
  exit 1
fi
