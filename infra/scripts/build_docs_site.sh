#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
ROOT_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
CONFIG_PATH="$ROOT_DIR/web/docs/mkdocs.yml"
SITE_DIR="$ROOT_DIR/site"

build_locale() {
    locale="$1"
    site_url="$2"
    output_dir="$3"

    BUILD_ONLY_LOCALE="$locale" \
    DOCS_SITE_URL="$site_url" \
    mkdocs build --strict -f "$CONFIG_PATH" -d "$output_dir"
}

rm -rf "$SITE_DIR"
mkdir -p "$SITE_DIR"

build_locale "en" "https://palmscript.dev/docs/" "$SITE_DIR/docs"
build_locale "es" "https://palmscript.dev/es/docs/" "$SITE_DIR/es/docs"
build_locale "pt-BR" "https://palmscript.dev/pt-BR/docs/" "$SITE_DIR/pt-BR/docs"
build_locale "de" "https://palmscript.dev/de/docs/" "$SITE_DIR/de/docs"
build_locale "ja" "https://palmscript.dev/ja/docs/" "$SITE_DIR/ja/docs"
build_locale "fr" "https://palmscript.dev/fr/docs/" "$SITE_DIR/fr/docs"
