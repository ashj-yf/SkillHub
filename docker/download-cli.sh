#!/bin/bash
# ====================
# CLI 下载脚本
# 从 GitHub Release 下载多平台 CLI 二进制文件
# ====================

set -e

# 构建参数
GITHUB_REPO="${GITHUB_REPO:-JokerYF/skills_hub}"
CLI_VERSION="${CLI_VERSION:-latest}"
GITHUB_TOKEN="${GITHUB_TOKEN:-}"

# 输出目录
OUTPUT_DIR="${OUTPUT_DIR:-/cli-bin}"
TMP_DIR="/tmp/cli-download"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 平台配置数组
declare -A PLATFORMS=(
    ["linux-x64"]="skillhub-linux-x64.tar.gz|skillhub|skillhub-linux-x86_64"
    ["linux-arm64"]="skillhub-linux-arm64.tar.gz|skillhub|skillhub-linux-arm64"
    ["macos-x64"]="skillhub-macos-x64.tar.gz|skillhub|skillhub-macos-x86_64"
    ["macos-arm64"]="skillhub-macos-arm64.tar.gz|skillhub|skillhub-macos-arm64"
    ["windows-x64"]="skillhub-windows-x64.zip|skillhub.exe|skillhub-windows-x86_64.exe"
)

# 获取 GitHub Release 信息
get_release_info() {
    local api_url

    if [ "$CLI_VERSION" = "latest" ]; then
        api_url="https://api.github.com/repos/${GITHUB_REPO}/releases/latest"
    else
        api_url="https://api.github.com/repos/${GITHUB_REPO}/releases/tags/${CLI_VERSION}"
    fi

    log_info "Fetching release info from: $api_url"

    local auth_header=""
    if [ -n "$GITHUB_TOKEN" ]; then
        auth_header="-H \"Authorization: Bearer ${GITHUB_TOKEN}\""
    fi

    # 获取 release 信息
    local response
    if [ -n "$GITHUB_TOKEN" ]; then
        response=$(curl -s -H "Authorization: Bearer ${GITHUB_TOKEN}" "$api_url")
    else
        response=$(curl -s "$api_url")
    fi

    # 检查是否成功
    if echo "$response" | jq -e '.message' > /dev/null 2>&1; then
        local error_msg=$(echo "$response" | jq -r '.message')
        log_error "GitHub API error: $error_msg"
        return 1
    fi

    # 输出版本信息
    local version=$(echo "$response" | jq -r '.tag_name')
    log_info "Release version: $version"

    echo "$response"
}

# 下载并解压 CLI 文件
download_cli() {
    local platform="$1"
    local archive_name="$2"
    local binary_name="$3"
    local output_name="$4"
    local download_url="$5"

    log_info "Downloading CLI for $platform..."
    log_info "  Archive: $archive_name"
    log_info "  Binary: $binary_name -> $output_name"

    # 创建临时目录
    mkdir -p "$TMP_DIR/$platform"

    # 下载文件
    local archive_path="$TMP_DIR/$platform/$archive_name"

    if [ -n "$GITHUB_TOKEN" ]; then
        curl -sL -H "Authorization: Bearer ${GITHUB_TOKEN}" \
            -o "$archive_path" "$download_url"
    else
        curl -sL -o "$archive_path" "$download_url"
    fi

    # 检查下载是否成功
    if [ ! -f "$archive_path" ] || [ ! -s "$archive_path" ]; then
        log_error "Failed to download $archive_name"
        return 1
    fi

    log_info "Downloaded: $archive_path ($(du -h "$archive_path" | cut -f1))"

    # 解压文件
    cd "$TMP_DIR/$platform"

    if [[ "$archive_name" == *.tar.gz ]]; then
        tar -xzf "$archive_name"
    elif [[ "$archive_name" == *.zip ]]; then
        unzip -q "$archive_name"
    else
        log_error "Unknown archive format: $archive_name"
        return 1
    fi

    # 查找并移动二进制文件
    if [ ! -f "$binary_name" ]; then
        log_error "Binary not found: $binary_name"
        ls -la "$TMP_DIR/$platform"
        return 1
    fi

    # 移动到输出目录
    mkdir -p "$OUTPUT_DIR"
    mv "$binary_name" "$OUTPUT_DIR/$output_name"

    log_info "Extracted: $OUTPUT_DIR/$output_name"

    # 清理
    rm -rf "$TMP_DIR/$platform"
}

# 主函数
main() {
    log_info "Starting CLI download process..."
    log_info "Repository: $GITHUB_REPO"
    log_info "Version: $CLI_VERSION"
    log_info "Output directory: $OUTPUT_DIR"

    # 创建输出目录
    mkdir -p "$OUTPUT_DIR"
    mkdir -p "$TMP_DIR"

    # 获取 release 信息
    local release_info
    release_info=$(get_release_info)

    if [ $? -ne 0 ]; then
        log_error "Failed to get release info"
        exit 1
    fi

    # 下载所有平台的 CLI
    local success_count=0
    local fail_count=0

    for platform in "${!PLATFORMS[@]}"; do
        local config="${PLATFORMS[$platform]}"
        local archive_name=$(echo "$config" | cut -d'|' -f1)
        local binary_name=$(echo "$config" | cut -d'|' -f2)
        local output_name=$(echo "$config" | cut -d'|' -f3)

        # 从 release 信息中获取下载 URL
        local download_url=$(echo "$release_info" | jq -r ".assets[] | select(.name == \"$archive_name\") | .url")

        if [ -z "$download_url" ] || [ "$download_url" = "null" ]; then
            log_warn "Asset not found for $platform: $archive_name"
            fail_count=$((fail_count + 1))
            continue
        fi

        # 下载
        if download_cli "$platform" "$archive_name" "$binary_name" "$output_name" "$download_url"; then
            success_count=$((success_count + 1))
        else
            fail_count=$((fail_count + 1))
        fi
    done

    # 清理临时目录
    rm -rf "$TMP_DIR"

    # 输出结果
    log_info "Download completed!"
    log_info "  Success: $success_count"
    log_info "  Failed: $fail_count"

    # 列出下载的文件
    log_info "Downloaded files:"
    ls -lh "$OUTPUT_DIR"

    if [ $fail_count -gt 0 ]; then
        log_warn "Some platforms failed to download"
        exit 1
    fi

    exit 0
}

# 执行
main