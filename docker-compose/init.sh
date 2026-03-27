#!/bin/bash
#
# Skills Hub 环境初始化脚本
# 生成 .env 文件，包含随机生成的敏感信息
#
# 使用方式: ./init.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/.env"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 生成随机字符串
generate_random_string() {
    local length=${1:-32}
    openssl rand -base64 48 | tr -d '/+=' | cut -c1-$length
}

# 生成随机密码
generate_password() {
    local length=${1:-16}
    openssl rand -base64 24 | tr -d '/+=' | cut -c1-$length
}

echo -e "${BLUE}=== Skills Hub 环境初始化 ===${NC}"
echo ""

# 检查 .env 是否存在
if [[ -f "$ENV_FILE" ]]; then
    echo -e "${YELLOW}警告: .env 文件已存在${NC}"
    read -p "是否覆盖? (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "已取消"
        exit 0
    fi
fi

echo -e "${BLUE}正在生成随机密钥...${NC}"
echo ""

# 生成随机值
JWT_SECRET=$(generate_random_string 64)
POSTGRES_PASSWORD=$(generate_password 20)
MINIO_ACCESS_KEY=$(generate_random_string 16)
MINIO_SECRET_KEY=$(generate_password 32)
ADMIN_PASSWORD=$(generate_password 16)

# 收集用户输入
echo -e "${BLUE}请配置以下信息 (直接回车使用默认值):${NC}"
echo ""

# 服务器端口
read -p "服务端口 [8080]: " SERVER_PORT
SERVER_PORT=${SERVER_PORT:-8080}

# CORS 允许的源
read -p "CORS 允许的源 (多个用逗号分隔) [http://localhost:3000,http://localhost:5173]: " CORS_ORIGINS
CORS_ORIGINS=${CORS_ORIGINS:-"http://localhost:3000,http://localhost:5173"}

# 管理员用户名
read -p "管理员用户名 [admin]: " ADMIN_USERNAME
ADMIN_USERNAME=${ADMIN_USERNAME:-admin}

# MinIO 端口
read -p "MinIO API 端口 [9000]: " MINIO_PORT
MINIO_PORT=${MINIO_PORT:-9000}

read -p "MinIO Console 端口 [9001]: " MINIO_CONSOLE_PORT
MINIO_CONSOLE_PORT=${MINIO_CONSOLE_PORT:-9001}

echo ""
echo -e "${BLUE}正在生成 .env 文件...${NC}"

# 生成 .env 文件
cat > "$ENV_FILE" << EOF
# Skills Hub 环境变量配置
# 由 init.sh 自动生成于 $(date '+%Y-%m-%d %H:%M:%S')

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
CORS_ALLOWED_ORIGINS=${CORS_ORIGINS}

# ==================== 管理员账号 ====================
ADMIN_USERNAME=${ADMIN_USERNAME}
ADMIN_PASSWORD=${ADMIN_PASSWORD}
EOF

echo -e "${GREEN}✓ .env 文件已生成${NC}"
echo ""
echo -e "${BLUE}=== 生成的敏感信息 ===${NC}"
echo ""
echo -e "JWT Secret:        ${YELLOW}${JWT_SECRET}${NC}"
echo -e "PostgreSQL 密码:   ${YELLOW}${POSTGRES_PASSWORD}${NC}"
echo -e "MinIO Access Key:  ${YELLOW}${MINIO_ACCESS_KEY}${NC}"
echo -e "MinIO Secret Key:  ${YELLOW}${MINIO_SECRET_KEY}${NC}"
echo -e "管理员密码:        ${YELLOW}${ADMIN_PASSWORD}${NC}"
echo ""
echo -e "${YELLOW}请妥善保管以上信息！${NC}"
echo ""
echo -e "${BLUE}下一步:${NC}"
echo "  docker compose up -d    # 启动服务"