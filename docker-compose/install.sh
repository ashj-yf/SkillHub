#!/bin/bash
#
# Skills Hub 一键安装脚本
# 使用方式: bash <(curl -fsSL https://your-domain.com/install.sh) -l zh_CN
#

set -e

# ==================== 配置 ====================
SCRIPT_VERSION="1.0.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/.env"

# 镜像配置
IMAGE_REGISTRY="${IMAGE_REGISTRY:-swr.cn-north-4.myhuaweicloud.com}"
IMAGE_NAMESPACE="${IMAGE_NAMESPACE:-skillhub}"
IMAGE_NAME="${IMAGE_NAME:-skillhub}"

# ==================== 多语言支持 ====================
declare -A MESSAGES_ZH_CN=(
    ["title"]="Skills Hub 安装向导"
    ["version"]="版本"
    ["checking_env"]="环境检测"
    ["docker_not_found"]="错误: 未检测到 Docker，请先安装 Docker"
    ["docker_compose_not_found"]="错误: 未检测到 Docker Compose，请先安装"
    ["env_ok"]="环境检测通过"
    ["network_config"]="网络配置"
    ["service_port"]="服务端口"
    ["domain_or_ip"]="域名/IP (用于 CORS 配置，多个用逗号分隔)"
    ["security_config"]="安全配置"
    ["generating_keys"]="正在生成安全密钥..."
    ["admin_username"]="管理员用户名"
    ["admin_password"]="管理员密码 (留空自动生成)"
    ["storage_config"]="存储配置"
    ["minio_api_port"]="MinIO API 端口"
    ["minio_console_port"]="MinIO Console 端口"
    ["confirm_install"]="确认安装"
    ["confirm_continue"]="是否继续?"
    ["installing"]="正在安装..."
    ["pulling_images"]="正在拉取镜像..."
    ["starting_services"]="正在启动服务..."
    ["checking_health"]="正在检查服务状态..."
    ["install_success"]="安装完成!"
    ["access_info"]="访问信息"
    ["service_url"]="服务地址"
    ["minio_console"]="MinIO 控制台"
    ["admin_account"]="管理员账号"
    ["sensitive_info"]="敏感信息 (请妥善保管)"
    ["jwt_secret"]="JWT 密钥"
    ["db_password"]="数据库密码"
    ["minio_access_key"]="MinIO Access Key"
    ["minio_secret_key"]="MinIO Secret Key"
    ["next_steps"]="后续步骤"
    ["view_logs"]="查看日志"
    ["stop_service"]="停止服务"
    ["restart_service"]="重启服务"
    ["env_exists"]="警告: .env 文件已存在"
    ["overwrite"]="是否覆盖?"
    ["cancelled"]="已取消安装"
    ["default"]="默认"
    ["auto_generated"]="自动生成"
    ["health_check_failed"]="警告: 服务健康检查未通过，请检查日志"
    ["docker_compose_cmd"]="docker compose"
    ["port_in_use"]="警告: 端口 %s 已被占用"
)

declare -A MESSAGES_EN=(
    ["title"]="Skills Hub Installation Wizard"
    ["version"]="Version"
    ["checking_env"]="Checking environment"
    ["docker_not_found"]="Error: Docker not found, please install Docker first"
    ["docker_compose_not_found"]="Error: Docker Compose not found, please install it first"
    ["env_ok"]="Environment check passed"
    ["network_config"]="Network Configuration"
    ["service_port"]="Service port"
    ["domain_or_ip"]="Domain/IP (for CORS, comma-separated for multiple)"
    ["security_config"]="Security Configuration"
    ["generating_keys"]="Generating security keys..."
    ["admin_username"]="Admin username"
    ["admin_password"]="Admin password (leave empty for auto-generation)"
    ["storage_config"]="Storage Configuration"
    ["minio_api_port"]="MinIO API port"
    ["minio_console_port"]="MinIO Console port"
    ["confirm_install"]="Confirm Installation"
    ["confirm_continue"]="Continue?"
    ["installing"]="Installing..."
    ["pulling_images"]="Pulling images..."
    ["starting_services"]="Starting services..."
    ["checking_health"]="Checking service health..."
    ["install_success"]="Installation completed!"
    ["access_info"]="Access Information"
    ["service_url"]="Service URL"
    ["minio_console"]="MinIO Console"
    ["admin_account"]="Admin Account"
    ["sensitive_info"]="Sensitive Information (keep it safe)"
    ["jwt_secret"]="JWT Secret"
    ["db_password"]="Database Password"
    ["minio_access_key"]="MinIO Access Key"
    ["minio_secret_key"]="MinIO Secret Key"
    ["next_steps"]="Next Steps"
    ["view_logs"]="View logs"
    ["stop_service"]="Stop service"
    ["restart_service"]="Restart service"
    ["env_exists"]="Warning: .env file already exists"
    ["overwrite"]="Overwrite?"
    ["cancelled"]="Installation cancelled"
    ["default"]="default"
    ["auto_generated"]="auto-generated"
    ["health_check_failed"]="Warning: Health check failed, please check logs"
    ["docker_compose_cmd"]="docker compose"
    ["port_in_use"]="Warning: Port %s is already in use"
)

# 当前语言
LANG_CODE="zh_CN"

# 获取消息
msg() {
    local key=$1
    case "$LANG_CODE" in
        "en") echo "${MESSAGES_EN[$key]}" ;;
        *)    echo "${MESSAGES_ZH_CN[$key]}" ;;
    esac
}

# ==================== 颜色定义 ====================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ==================== 工具函数 ====================

# 打印步骤标题
print_step() {
    local step=$1
    local title=$2
    echo ""
    echo -e "${CYAN}[$step] ${title}${NC}"
    echo ""
}

# 打印成功信息
print_ok() {
    echo -e "  ${GREEN}✓${NC} $1"
}

# 打印警告
print_warn() {
    echo -e "  ${YELLOW}!${NC} $1"
}

# 打印错误
print_error() {
    echo -e "  ${RED}✗${NC} $1"
}

# 生成随机字符串
generate_random_string() {
    local length=${1:-32}
    openssl rand -base64 48 | tr -d '/+=' | cut -c1-$length 2>/dev/null || \
    cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w $length | head -n 1
}

# 生成密码
generate_password() {
    local length=${1:-16}
    generate_random_string $length
}

# 检查端口是否被占用
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 1
    fi
    return 0
}

# 等待端口可用
wait_for_port() {
    local port=$1
    local max_attempts=30
    local attempt=0
    while ! check_port $port; do
        attempt=$((attempt + 1))
        if [ $attempt -ge $max_attempts ]; then
            return 1
        fi
        sleep 1
    done
    return 0
}

# ==================== 帮助信息 ====================
show_help() {
    cat << EOF
Skills Hub 安装脚本 v${SCRIPT_VERSION}

使用方式:
  $0 [选项]

选项:
  -l, --lang <lang>       语言 (zh_CN/en)，默认 zh_CN
  -p, --port <port>       服务端口，默认 8080
  --no-interactive        非交互模式，使用默认值
  -h, --help              显示帮助信息

示例:
  $0                      # 交互式安装
  $0 -l en                # 使用英文
  $0 -p 9000              # 指定端口
  $0 --no-interactive     # 非交互模式

EOF
    exit 0
}

# ==================== 参数解析 ====================
NO_INTERACTIVE=false
SERVER_PORT=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -l|--lang)
            LANG_CODE="$2"
            shift 2
            ;;
        -p|--port)
            SERVER_PORT="$2"
            shift 2
            ;;
        --no-interactive)
            NO_INTERACTIVE=true
            shift
            ;;
        -h|--help)
            show_help
            ;;
        *)
            shift
            ;;
    esac
done

# ==================== 主流程 ====================

echo ""
echo -e "${BOLD}${CYAN}╔════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${CYAN}║       Skills Hub 安装向导              ║${NC}"
echo -e "${BOLD}${CYAN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "  $(msg "version"): ${SCRIPT_VERSION}"
echo ""

# [1/5] 环境检测
print_step "1/5" "$(msg "checking_env")"

# 检测 Docker
if ! command -v docker &> /dev/null; then
    print_error "$(msg "docker_not_found")"
    echo ""
    echo "  安装 Docker: https://docs.docker.com/get-docker/"
    exit 1
fi
print_ok "Docker $(docker --version | awk '{print $3}' | tr -d ',')"

# 检测 Docker Compose
if docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
elif command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
else
    print_error "$(msg "docker_compose_not_found")"
    echo ""
    echo "  安装 Docker Compose: https://docs.docker.com/compose/install/"
    exit 1
fi
print_ok "Docker Compose ($COMPOSE_CMD)"

print_ok "$(msg "env_ok")"

# [2/5] 网络配置
print_step "2/5" "$(msg "network_config")"

if [ -z "$SERVER_PORT" ]; then
    if [ "$NO_INTERACTIVE" = true ]; then
        SERVER_PORT=8080
    else
        read -p "  $(msg "service_port") [8080]: " SERVER_PORT
        SERVER_PORT=${SERVER_PORT:-8080}
    fi
else
    echo -e "  $(msg "service_port"): $SERVER_PORT"
fi

# 检查端口
if ! check_port $SERVER_PORT; then
    print_warn "$(printf "$(msg "port_in_use")" $SERVER_PORT)"
fi

# CORS 配置
if [ "$NO_INTERACTIVE" = true ]; then
    CORS_ORIGINS="http://localhost:3000,http://localhost:5173"
else
    read -p "  $(msg "domain_or_ip") [http://localhost:3000,http://localhost:5173]: " CORS_ORIGINS
    CORS_ORIGINS=${CORS_ORIGINS:-"http://localhost:3000,http://localhost:5173"}
fi

# [3/5] 安全配置
print_step "3/5" "$(msg "security_config")"
echo -e "  $(msg "generating_keys")"

JWT_SECRET=$(generate_random_string 64)
POSTGRES_PASSWORD=$(generate_password 20)
MINIO_ACCESS_KEY=$(generate_random_string 16)
MINIO_SECRET_KEY=$(generate_password 32)

print_ok "JWT Secret: ${JWT_SECRET:0:8}..."
print_ok "Database Password: ${POSTGRES_PASSWORD:0:4}****"
print_ok "MinIO Access Key: $MINIO_ACCESS_KEY"

# 管理员配置
if [ "$NO_INTERACTIVE" = true ]; then
    ADMIN_USERNAME="admin"
    ADMIN_PASSWORD=$(generate_password 16)
else
    read -p "  $(msg "admin_username") [admin]: " ADMIN_USERNAME
    ADMIN_USERNAME=${ADMIN_USERNAME:-admin}

    read -p "  $(msg "admin_password") [$(msg "auto_generated")]: " ADMIN_PASSWORD
    if [ -z "$ADMIN_PASSWORD" ]; then
        ADMIN_PASSWORD=$(generate_password 16)
    fi
fi

# [4/5] 存储配置
print_step "4/5" "$(msg "storage_config")"

if [ "$NO_INTERACTIVE" = true ]; then
    MINIO_PORT=9000
    MINIO_CONSOLE_PORT=9001
else
    read -p "  $(msg "minio_api_port") [9000]: " MINIO_PORT
    MINIO_PORT=${MINIO_PORT:-9000}

    read -p "  $(msg "minio_console_port") [9001]: " MINIO_CONSOLE_PORT
    MINIO_CONSOLE_PORT=${MINIO_CONSOLE_PORT:-9001}
fi

# [5/5] 确认安装
print_step "5/5" "$(msg "confirm_install")"

echo -e "  $(msg "service_port"): $SERVER_PORT"
echo -e "  MinIO API: $MINIO_PORT, Console: $MINIO_CONSOLE_PORT"
echo -e "  $(msg "admin_username"): $ADMIN_USERNAME"
echo -e "  CORS Origins: $CORS_ORIGINS"
echo ""

if [ "$NO_INTERACTIVE" = true ]; then
    CONFIRM="y"
else
    read -p "  $(msg "confirm_continue") [Y/n]: " CONFIRM
    CONFIRM=${CONFIRM:-y}
fi

if [[ ! "$CONFIRM" =~ ^[yY] ]]; then
    echo ""
    echo -e "  ${YELLOW}$(msg "cancelled")${NC}"
    exit 0
fi

# ==================== 安装 ====================
echo ""
echo -e "${BLUE}$(msg "installing")${NC}"

# 检查 .env 是否存在
if [ -f "$ENV_FILE" ]; then
    print_warn "$(msg "env_exists")"
    if [ "$NO_INTERACTIVE" = false ]; then
        read -p "  $(msg "overwrite") [y/N]: " OVERWRITE
        if [[ ! "$OVERWRITE" =~ ^[yY] ]]; then
            echo -e "  ${YELLOW}$(msg "cancelled")${NC}"
            exit 0
        fi
    fi
fi

# 生成 .env 文件
cat > "$ENV_FILE" << EOF
# Skills Hub 环境变量配置
# 由 install.sh 自动生成于 $(date '+%Y-%m-%d %H:%M:%S')

# ==================== 服务配置 ====================
SERVER_PORT=${SERVER_PORT}

# ==================== 数据库配置 ====================
POSTGRES_PASSWORD=${POSTGRES_PASSWORD}

# ==================== JWT 配置 ====================
JWT_SECRET=${JWT_SECRET}
JWT_EXPIRATION=86400

# ==================== MinIO 配置 ====================
MINIO_ACCESS_KEY=${MINIO_ACCESS_KEY}
MINIO_SECRET_KEY=${MINIO_SECRET_KEY}
MINIO_PORT=${MINIO_PORT}
MINIO_CONSOLE_PORT=${MINIO_CONSOLE_PORT}

# ==================== CORS 配置 ====================
CORS_ORIGINS=${CORS_ORIGINS}

# ==================== 管理员账号 ====================
ADMIN_USERNAME=${ADMIN_USERNAME}
ADMIN_PASSWORD=${ADMIN_PASSWORD}
EOF

print_ok ".env 文件已生成"

# 拉取镜像
echo ""
echo -e "${BLUE}$(msg "pulling_images")${NC}"
$COMPOSE_CMD -f "$SCRIPT_DIR/docker-compose.yml" pull 2>&1 | while read line; do
    echo -e "  $line"
done

# 启动服务
echo ""
echo -e "${BLUE}$(msg "starting_services")${NC}"
$COMPOSE_CMD -f "$SCRIPT_DIR/docker-compose.yml" up -d

# 健康检查
echo ""
echo -e "${BLUE}$(msg "checking_health")${NC}"
sleep 5

# 等待服务启动
max_wait=60
waited=0
while [ $waited -lt $max_wait ]; do
    if curl -s "http://localhost:$SERVER_PORT/health" > /dev/null 2>&1; then
        print_ok "服务健康检查通过"
        break
    fi
    waited=$((waited + 2))
    sleep 2
done

if [ $waited -ge $max_wait ]; then
    print_warn "$(msg "health_check_failed")"
fi

# ==================== 安装完成 ====================
echo ""
echo -e "${GREEN}${BOLD}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}${BOLD}║        $(msg "install_success")              ║${NC}"
echo -e "${GREEN}${BOLD}╚════════════════════════════════════════╝${NC}"

echo ""
echo -e "${CYAN}$(msg "access_info"):${NC}"
echo -e "  $(msg "service_url"):    ${BOLD}http://localhost:${SERVER_PORT}${NC}"
echo -e "  $(msg "minio_console"): ${BOLD}http://localhost:${MINIO_CONSOLE_PORT}${NC}"
echo ""

echo -e "${CYAN}$(msg "admin_account")}:${NC}"
echo -e "  Username: ${BOLD}${ADMIN_USERNAME}${NC}"
echo -e "  Password: ${BOLD}${ADMIN_PASSWORD}${NC}"
echo ""

echo -e "${YELLOW}$(msg "sensitive_info")}:${NC}"
echo -e "  $(msg "jwt_secret")}:     ${JWT_SECRET}"
echo -e "  $(msg "db_password")}:   ${POSTGRES_PASSWORD}"
echo -e "  $(msg "minio_access_key")}:  ${MINIO_ACCESS_KEY}"
echo -e "  $(msg "minio_secret_key")}:  ${MINIO_SECRET_KEY}"
echo ""

echo -e "${CYAN}$(msg "next_steps")}:${NC}"
echo "  $COMPOSE_CMD -f docker-compose.yml logs -f    # $(msg "view_logs")"
echo "  $COMPOSE_CMD -f docker-compose.yml down       # $(msg "stop_service")"
echo "  $COMPOSE_CMD -f docker-compose.yml restart    # $(msg "restart_service")"
echo ""