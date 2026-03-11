#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WEB_DIR="$ROOT/web/ide"

npm --prefix "$WEB_DIR" ci
npm --prefix "$WEB_DIR" run build
