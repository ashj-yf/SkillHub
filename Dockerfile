# ====================
# Skills Hub 合并镜像 Dockerfile
# 前端 + 后端 + nginx 统一打包
# ====================

# ====================
# 阶段 1: 前端构建
# ====================
FROM node:20-alpine AS frontend-builder

WORKDIR /app/web

# 复制前端依赖文件
COPY web/package.json ./

# 安装依赖
RUN npm install

# 复制前端源代码
COPY web/ ./

# 构建
RUN npm run build

# ====================
# 阶段 2: 后端构建
# ====================
FROM rust:1.94-alpine AS backend-builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# 复制依赖文件
COPY backend/Cargo.toml ./

# 创建空的 src 目录以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

# 复制源代码
COPY backend/src ./src

# 构建 (debug 模式，加快开发迭代)
RUN touch src/main.rs && cargo build

# ====================
# 阶段 3: 运行镜像
# ====================
FROM alpine:3.19

# 安装运行时依赖
RUN apk add --no-cache ca-certificates tzdata nginx

WORKDIR /app

# 复制后端可执行文件
COPY --from=backend-builder /app/target/debug/skillhub-backend /app/

# 复制前端静态文件
COPY --from=frontend-builder /app/web/dist /app/static

# 复制 nginx 配置
COPY docker/nginx.conf /etc/nginx/http.d/default.conf

# 复制启动脚本
COPY docker/start.sh /app/start.sh
RUN chmod +x /app/start.sh

# 暴露端口
EXPOSE 80

# 启动
CMD ["./start.sh"]