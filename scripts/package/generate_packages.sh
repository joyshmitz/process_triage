#!/usr/bin/env bash
# Generate package manifests from release checksums.
# Usage: ./generate_packages.sh <version> <checksums_file> <output_dir>
#
# Required artifacts:
#   - Linux x86_64 + aarch64
#   - macOS x86_64 + aarch64
# Optional artifacts:
#   - Windows x64 (+ optionally arm64) for Winget manifests

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WINGET_GENERATED=0

usage() {
    echo "Usage: $0 <version> <checksums_file> <output_dir>"
    echo ""
    echo "Arguments:"
    echo "  version        Release version (e.g., 1.0.0)"
    echo "  checksums_file Path to checksums.sha256 file"
    echo "  output_dir     Directory to write generated files"
    exit 1
}

log_info() {
    echo "[INFO] $*" >&2
}

log_error() {
    echo "[ERROR] $*" >&2
}

extract_checksum_line() {
    local checksums_file="$1"
    local pattern="$2"

    local line
    line=$(grep -E "$pattern" "$checksums_file" | head -1 || true)

    if [[ -z "$line" ]]; then
        log_error "Could not find checksum for pattern: $pattern"
        return 1
    fi

    echo "$line"
}

extract_sha256() {
    local checksums_file="$1"
    local pattern="$2"
    local line
    line=$(extract_checksum_line "$checksums_file" "$pattern")
    awk '{print $1}' <<< "$line"
}

extract_asset_name() {
    local checksums_file="$1"
    local pattern="$2"
    local line
    line=$(extract_checksum_line "$checksums_file" "$pattern")
    awk '{print $2}' <<< "$line"
}

github_release_url() {
    local version="$1"
    local asset_name="$2"
    echo "https://github.com/Dicklesworthstone/process_triage/releases/download/v${version}/${asset_name}"
}

escape_sed() {
    local input="$1"
    printf '%s' "$input" | sed -e 's/[\/&]/\\&/g'
}

generate_formula() {
    local version="$1"
    local checksums_file="$2"
    local output_file="$3"

    log_info "Generating Homebrew formula for version $version"

    # Extract checksums and asset names for each platform
    local sha_linux_x86_64 sha_linux_aarch64 sha_macos_x86_64 sha_macos_aarch64
    local asset_linux_x86_64 asset_linux_aarch64 asset_macos_x86_64 asset_macos_aarch64
    local url_linux_x86_64 url_linux_aarch64 url_macos_x86_64 url_macos_aarch64

    sha_linux_x86_64=$(extract_sha256 "$checksums_file" "pt-core-linux-x86_64")
    sha_linux_aarch64=$(extract_sha256 "$checksums_file" "pt-core-linux-aarch64")
    sha_macos_x86_64=$(extract_sha256 "$checksums_file" "pt-core-macos-x86_64")
    sha_macos_aarch64=$(extract_sha256 "$checksums_file" "pt-core-macos-aarch64")
    asset_linux_x86_64=$(extract_asset_name "$checksums_file" "pt-core-linux-x86_64")
    asset_linux_aarch64=$(extract_asset_name "$checksums_file" "pt-core-linux-aarch64")
    asset_macos_x86_64=$(extract_asset_name "$checksums_file" "pt-core-macos-x86_64")
    asset_macos_aarch64=$(extract_asset_name "$checksums_file" "pt-core-macos-aarch64")
    url_linux_x86_64=$(github_release_url "$version" "$asset_linux_x86_64")
    url_linux_aarch64=$(github_release_url "$version" "$asset_linux_aarch64")
    url_macos_x86_64=$(github_release_url "$version" "$asset_macos_x86_64")
    url_macos_aarch64=$(github_release_url "$version" "$asset_macos_aarch64")

    log_info "  Linux x86_64:  ${sha_linux_x86_64:0:16}..."
    log_info "  Linux aarch64: ${sha_linux_aarch64:0:16}..."
    log_info "  macOS x86_64:  ${sha_macos_x86_64:0:16}..."
    log_info "  macOS aarch64: ${sha_macos_aarch64:0:16}..."

    # Generate formula from template
    sed -e "s|{{VERSION}}|$(escape_sed "$version")|g" \
        -e "s|{{SHA256_LINUX_X86_64}}|$(escape_sed "$sha_linux_x86_64")|g" \
        -e "s|{{SHA256_LINUX_AARCH64}}|$(escape_sed "$sha_linux_aarch64")|g" \
        -e "s|{{SHA256_MACOS_X86_64}}|$(escape_sed "$sha_macos_x86_64")|g" \
        -e "s|{{SHA256_MACOS_AARCH64}}|$(escape_sed "$sha_macos_aarch64")|g" \
        -e "s|{{URL_LINUX_X86_64}}|$(escape_sed "$url_linux_x86_64")|g" \
        -e "s|{{URL_LINUX_AARCH64}}|$(escape_sed "$url_linux_aarch64")|g" \
        -e "s|{{URL_MACOS_X86_64}}|$(escape_sed "$url_macos_x86_64")|g" \
        -e "s|{{URL_MACOS_AARCH64}}|$(escape_sed "$url_macos_aarch64")|g" \
        "${SCRIPT_DIR}/pt.rb.template" > "$output_file"

    log_info "  Formula written to: $output_file"
}

generate_manifest() {
    local version="$1"
    local checksums_file="$2"
    local output_file="$3"

    log_info "Generating Scoop manifest for version $version"

    # Scoop currently targets WSL2/Linux artifacts.
    local sha_linux_x86_64 asset_linux_x86_64 url_linux_x86_64 autoupdate_url
    sha_linux_x86_64=$(extract_sha256 "$checksums_file" "pt-core-linux-x86_64")
    asset_linux_x86_64=$(extract_asset_name "$checksums_file" "pt-core-linux-x86_64")
    url_linux_x86_64=$(github_release_url "$version" "$asset_linux_x86_64")
    autoupdate_url="${url_linux_x86_64//$version/\$version}"

    log_info "  Linux x86_64: ${sha_linux_x86_64:0:16}..."

    # Generate manifest from template
    sed -e "s|{{VERSION}}|$(escape_sed "$version")|g" \
        -e "s|{{SHA256_LINUX_X86_64}}|$(escape_sed "$sha_linux_x86_64")|g" \
        -e "s|{{URL_LINUX_X86_64}}|$(escape_sed "$url_linux_x86_64")|g" \
        -e "s|{{AUTOUPDATE_URL_LINUX_X86_64}}|$(escape_sed "$autoupdate_url")|g" \
        "${SCRIPT_DIR}/pt.json.template" > "$output_file"

    log_info "  Manifest written to: $output_file"
}

generate_winget_manifests() {
    local version="$1"
    local checksums_file="$2"
    local output_dir="$3"
    local x64_pattern='pt-core-windows-(x86_64|x64)'
    local arm64_pattern='pt-core-windows-(aarch64|arm64)'

    if ! grep -Eq "$x64_pattern" "$checksums_file"; then
        log_info "Skipping Winget manifest generation (no Windows x64 artifact in checksums)"
        return 0
    fi

    local sha_windows_x64 asset_windows_x64 url_windows_x64
    local sha_windows_arm64="" asset_windows_arm64="" url_windows_arm64=""
    local version_file installer_file locale_file

    sha_windows_x64=$(extract_sha256 "$checksums_file" "$x64_pattern")
    asset_windows_x64=$(extract_asset_name "$checksums_file" "$x64_pattern")
    url_windows_x64=$(github_release_url "$version" "$asset_windows_x64")

    if grep -Eq "$arm64_pattern" "$checksums_file"; then
        sha_windows_arm64=$(extract_sha256 "$checksums_file" "$arm64_pattern")
        asset_windows_arm64=$(extract_asset_name "$checksums_file" "$arm64_pattern")
        url_windows_arm64=$(github_release_url "$version" "$asset_windows_arm64")
    fi

    version_file="${output_dir}/pt.winget.yaml"
    installer_file="${output_dir}/pt.winget.installer.yaml"
    locale_file="${output_dir}/pt.winget.locale.en-US.yaml"

    log_info "Generating Winget manifests for version $version"
    log_info "  Windows x64:   ${sha_windows_x64:0:16}..."
    if [[ -n "$sha_windows_arm64" ]]; then
        log_info "  Windows arm64: ${sha_windows_arm64:0:16}..."
    fi

    cat > "$version_file" <<EOF
PackageIdentifier: ProcessTriage.pt
PackageVersion: ${version}
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.6.0
EOF

    cat > "$installer_file" <<EOF
PackageIdentifier: ProcessTriage.pt
PackageVersion: ${version}
InstallerType: zip
NestedInstallerType: portable
NestedInstallerFiles:
  - RelativeFilePath: pt-core.exe
    PortableCommandAlias: pt-core
Installers:
  - Architecture: x64
    InstallerUrl: ${url_windows_x64}
    InstallerSha256: ${sha_windows_x64}
EOF

    if [[ -n "$sha_windows_arm64" ]]; then
        cat >> "$installer_file" <<EOF
  - Architecture: arm64
    InstallerUrl: ${url_windows_arm64}
    InstallerSha256: ${sha_windows_arm64}
EOF
    fi

    cat >> "$installer_file" <<EOF
ManifestType: installer
ManifestVersion: 1.6.0
EOF

    cat > "$locale_file" <<EOF
PackageIdentifier: ProcessTriage.pt
PackageVersion: ${version}
PackageLocale: en-US
Publisher: Process Triage
PublisherUrl: https://github.com/Dicklesworthstone/process_triage
PackageName: Process Triage
PackageUrl: https://github.com/Dicklesworthstone/process_triage
ShortDescription: Bayesian-inspired zombie/abandoned process detection and cleanup
License: MIT
LicenseUrl: https://github.com/Dicklesworthstone/process_triage/blob/v${version}/LICENSE
ManifestType: defaultLocale
ManifestVersion: 1.6.0
EOF

    log_info "  Winget version manifest:  $version_file"
    log_info "  Winget installer manifest: $installer_file"
    log_info "  Winget locale manifest:   $locale_file"
    WINGET_GENERATED=1
}

validate_formula() {
    local formula_file="$1"

    log_info "Validating Homebrew formula syntax"

    if command -v ruby &>/dev/null; then
        if ruby -c "$formula_file" &>/dev/null; then
            log_info "  Ruby syntax: OK"
        else
            log_error "  Ruby syntax: FAILED"
            ruby -c "$formula_file"
            return 1
        fi
    else
        log_info "  Ruby not available, skipping syntax check"
    fi
}

validate_manifest() {
    local manifest_file="$1"

    log_info "Validating Scoop manifest JSON"

    if command -v jq &>/dev/null; then
        if jq . "$manifest_file" &>/dev/null; then
            log_info "  JSON syntax: OK"
        else
            log_error "  JSON syntax: FAILED"
            jq . "$manifest_file"
            return 1
        fi
    else
        log_info "  jq not available, skipping syntax check"
    fi
}

validate_winget_manifests() {
    local version_file="$1"
    local installer_file="$2"
    local locale_file="$3"

    log_info "Validating Winget manifests"

    local file
    for file in "$version_file" "$installer_file" "$locale_file"; do
        if [[ ! -f "$file" ]]; then
            log_error "  Missing Winget file: $file"
            return 1
        fi
        if grep -q '{{' "$file"; then
            log_error "  Unreplaced template placeholder in: $file"
            return 1
        fi
    done

    grep -q '^PackageIdentifier: ProcessTriage\.pt$' "$version_file" || return 1
    grep -q '^ManifestType: version$' "$version_file" || return 1
    grep -q '^ManifestType: installer$' "$installer_file" || return 1
    grep -q '^ManifestType: defaultLocale$' "$locale_file" || return 1
    grep -q 'Architecture: x64' "$installer_file" || return 1
    grep -q '^PackageLocale: en-US$' "$locale_file" || return 1

    log_info "  Winget manifest structure: OK"
}

main() {
    if [[ $# -lt 3 ]]; then
        usage
    fi

    local version="$1"
    local checksums_file="$2"
    local output_dir="$3"

    # Validate inputs
    if [[ ! -f "$checksums_file" ]]; then
        log_error "Checksums file not found: $checksums_file"
        exit 1
    fi

    # Create output directory
    mkdir -p "$output_dir"

    # Generate package manifests
    local formula_file="${output_dir}/pt.rb"
    local manifest_file="${output_dir}/pt.json"
    local winget_version_file="${output_dir}/pt.winget.yaml"
    local winget_installer_file="${output_dir}/pt.winget.installer.yaml"
    local winget_locale_file="${output_dir}/pt.winget.locale.en-US.yaml"

    generate_formula "$version" "$checksums_file" "$formula_file"
    generate_manifest "$version" "$checksums_file" "$manifest_file"
    generate_winget_manifests "$version" "$checksums_file" "$output_dir"

    # Validate generated files
    validate_formula "$formula_file"
    validate_manifest "$manifest_file"
    if [[ "$WINGET_GENERATED" == "1" ]]; then
        validate_winget_manifests \
            "$winget_version_file" \
            "$winget_installer_file" \
            "$winget_locale_file"
    fi

    log_info "Package generation complete!"
    log_info ""
    log_info "Generated files:"
    log_info "  Homebrew formula: $formula_file"
    log_info "  Scoop manifest:   $manifest_file"
    if [[ "$WINGET_GENERATED" == "1" ]]; then
        log_info "  Winget version:   $winget_version_file"
        log_info "  Winget installer: $winget_installer_file"
        log_info "  Winget locale:    $winget_locale_file"
    fi

    # Output JSON for GitHub Actions
    if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
        {
            echo "formula_path=$formula_file"
            echo "manifest_path=$manifest_file"
            echo "winget_generated=$WINGET_GENERATED"
        } >> "$GITHUB_OUTPUT"
        if [[ "$WINGET_GENERATED" == "1" ]]; then
            {
                echo "winget_version_path=$winget_version_file"
                echo "winget_installer_path=$winget_installer_file"
                echo "winget_locale_path=$winget_locale_file"
            } >> "$GITHUB_OUTPUT"
        fi
    fi
}

main "$@"
