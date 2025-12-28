#!/usr/bin/env node
/**
 * Script to generate latest.json for Tauri updater
 *
 * Usage: node scripts/generate-latest-json.js <version> <release-tag>
 * Example: node scripts/generate-latest-json.js 0.1.0 app-v0.1.0
 */

const fs = require("fs");
const path = require("path");

const version = process.argv[2] || "0.1.0";
const releaseTag = process.argv[3] || `app-v${version}`;
const repo = "Ripwords/rk-configurator";
const baseUrl = `https://github.com/${repo}/releases/download/${releaseTag}`;

// Read signatures from build artifacts
// These are generated during `tauri build` when TAURI_SIGNING_PRIVATE_KEY is set
const signaturesDir = path.join(
  __dirname,
  "../src-tauri/target/release/bundle"
);

function findSignatureFile(platform) {
  const signaturesDir = path.join(
    __dirname,
    "../src-tauri/target/release/bundle"
  );
  if (!fs.existsSync(signaturesDir)) {
    return null;
  }

  // Try to find signature files that match the platform
  const files = fs.readdirSync(signaturesDir);
  const patterns = {
    "darwin-x86_64": /\.app\.tar\.gz\.sig$/,
    "darwin-aarch64": /\.app\.tar\.gz\.sig$/,
    "windows-x86_64": /\.exe\.sig$/,
    "linux-x86_64": /\.AppImage\.sig$/,
  };

  const pattern = patterns[platform];
  if (pattern) {
    const match = files.find((f) => pattern.test(f));
    if (match) {
      return path.join(signaturesDir, match);
    }
  }

  return null;
}

function getSignature(platform) {
  // First try to find signature file by pattern matching
  const sigFile = findSignatureFile(platform);
  if (sigFile && fs.existsSync(sigFile)) {
    return fs.readFileSync(sigFile, "utf-8").trim();
  }

  // Fallback to exact filename match
  const exactFile = path.join(signaturesDir, getSignatureFileName(platform));
  if (fs.existsSync(exactFile)) {
    return fs.readFileSync(exactFile, "utf-8").trim();
  }

  console.warn(`Warning: Signature file not found for ${platform}`);
  console.warn(`  Searched in: ${signaturesDir}`);
  if (fs.existsSync(signaturesDir)) {
    const files = fs.readdirSync(signaturesDir);
    console.warn(`  Available files: ${files.join(", ")}`);
  }
  return "SIGNATURE_NOT_FOUND";
}

function getSignatureFileName(platform) {
  // Try multiple possible signature file names
  // Tauri generates signatures with names matching the asset files
  const possibleNames = {
    "darwin-x86_64": [
      "Royal-Kludge-Configurator.app.tar.gz.sig",
      "Royal-Kludge-Configurator_*.app.tar.gz.sig",
      "*.app.tar.gz.sig",
    ],
    "darwin-aarch64": [
      "Royal-Kludge-Configurator.app.tar.gz.sig",
      "Royal-Kludge-Configurator_*.app.tar.gz.sig",
      "*.app.tar.gz.sig",
    ],
    "windows-x86_64": [
      "Royal-Kludge-Configurator_*_x64-setup.exe.sig",
      "*.exe.sig",
    ],
    "linux-x86_64": [
      "Royal-Kludge-Configurator_*_amd64.AppImage.sig",
      "*.AppImage.sig",
    ],
  };

  // Return the first pattern - we'll search for matching files
  return possibleNames[platform]?.[0] || `${platform}.sig`;
}

function getAssetUrl(platform) {
  // Tauri action generates files with specific naming patterns
  // Adjust these based on your actual build output
  const map = {
    "darwin-x86_64": `${baseUrl}/Royal-Kludge-Configurator_${version}_x64.app.tar.gz`,
    "darwin-aarch64": `${baseUrl}/Royal-Kludge-Configurator_${version}_aarch64.app.tar.gz`,
    "windows-x86_64": `${baseUrl}/Royal-Kludge-Configurator_${version}_x64-setup.exe`,
    "linux-x86_64": `${baseUrl}/Royal-Kludge-Configurator_${version}_amd64.AppImage`,
  };
  return map[platform] || `${baseUrl}/${platform}.zip`;
}

const platforms = {
  "darwin-x86_64": {
    signature: getSignature("darwin-x86_64"),
    url: getAssetUrl("darwin-x86_64"),
  },
  "darwin-aarch64": {
    signature: getSignature("darwin-aarch64"),
    url: getAssetUrl("darwin-aarch64"),
  },
  "windows-x86_64": {
    signature: getSignature("windows-x86_64"),
    url: getAssetUrl("windows-x86_64"),
  },
  "linux-x86_64": {
    signature: getSignature("linux-x86_64"),
    url: getAssetUrl("linux-x86_64"),
  },
};

const latestJson = {
  version,
  notes: `Release notes for version ${version}`,
  pub_date: new Date().toISOString(),
  platforms,
};

const outputPath = path.join(__dirname, "../latest.json");
fs.writeFileSync(outputPath, JSON.stringify(latestJson, null, 2));
console.log(`Generated latest.json at ${outputPath}`);
console.log(`Version: ${version}`);
console.log(`Release tag: ${releaseTag}`);
console.log(`Platforms configured: ${Object.keys(platforms).join(", ")}`);
