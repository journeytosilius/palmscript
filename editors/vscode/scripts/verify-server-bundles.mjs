#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";

const requiredBundles = [
    { platform: "linux-x64", binary: "tradelang-lsp" },
    { platform: "darwin-x64", binary: "tradelang-lsp" },
    { platform: "darwin-arm64", binary: "tradelang-lsp" },
    { platform: "win32-x64", binary: "tradelang-lsp.exe" },
];

const missing = [];
for (const bundle of requiredBundles) {
    const candidate = path.resolve("server", bundle.platform, bundle.binary);
    if (!fs.existsSync(candidate)) {
        missing.push(candidate);
    }
}

if (missing.length > 0) {
    console.error("missing bundled tradelang-lsp binaries:");
    for (const candidate of missing) {
        console.error(`  - ${candidate}`);
    }
    process.exit(1);
}

console.log("all required tradelang-lsp bundles are present");
