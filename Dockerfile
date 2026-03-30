# ====================
# Skills Hub 合并镜像 Dockerfile (国内加速版)
# 前端 + 后端 + nginx 统一打包
# ====================

# ====================
# 阶段 1: 前端构建
# ====================
FROM node:20-alpine AS frontend-builder

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 配置 npm 淘宝镜像
RUN npm config set registry https://registry.npmmirror.com

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
FROM rustlang/rust:nightly-alpine AS backend-builder

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

RUN apk add --no-cache musl-dev

WORKDIR /app

# 配置 Rust crates 清华镜像
RUN mkdir -p /root/.cargo && \
    echo '[source.crates-io]' > /root/.cargo/config.toml && \
    echo 'replace-with = "tuna"' >> /root/.cargo/config.toml && \
    echo '' >> /root/.cargo/config.toml && \
    echo '[source.tuna]' >> /root/.cargo/config.toml && \
    echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' >> /root/.cargo/config.toml

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
# 阶段 3: CLI 构建
# ====================
FROM rustlang/rust:nightly-alpine AS cli-builder

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

WORKDIR /app

# 配置 Rust crates 清华镜像
RUN mkdir -p /root/.cargo && \
    echo '[source.crates-io]' > /root/.cargo/config.toml && \
    echo 'replace-with = "tuna"' >> /root/.cargo/config.toml && \
    echo '' >> /root/.cargo/config.toml && \
    echo '[source.tuna]' >> /root/.cargo/config.toml && \
    echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' >> /root/.cargo/config.toml

# 复制 CLI 源码
COPY cli/Cargo.toml cli/Cargo.lock* ./

# 创建空的 src 目录以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY cli/src ./src
COPY cli/build.rs ./build.rs

# 构建参数：版本信息
ARG VERSION=0.1.0
ARG GIT_COMMIT=unknown
ARG BUILD_DATE=unknown

# 编译 CLI
ENV VERSION=${VERSION}
ENV GIT_COMMIT=${GIT_COMMIT}
ENV BUILD_DATE=${BUILD_DATE}

RUN touch src/main.rs && cargo build --release

# 输出目录
RUN mkdir -p /cli-bin && \
    cp target/release/skillhub /cli-bin/skillhub

# ====================
# 阶段 4: 运行镜像
# ====================
FROM alpine:3.19

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 安装运行时依赖（添加 postgresql-client 用于执行迁移）
RUN apk add --no-cache ca-certificates tzdata nginx postgresql-client

WORKDIR /app

# 复制后端可执行文件
COPY --from=backend-builder /app/target/debug/skillhub-backend /app/

# 复制数据库迁移文件
COPY backend/migrations /app/migrations

# 复制前端静态文件
COPY --from=frontend-builder /app/web/dist /app/static

# 创建 CLI 下载目录
RUN mkdir -p /app/static/downloads/cli

# 复制 CLI 二进制文件
COPY --from=cli-builder /cli-bin/skillhub /app/static/downloads/cli/skillhub-linux-x86_64

# 复制 nginx 配置
COPY docker/nginx.conf /etc/nginx/http.d/default.conf

# 复制启动脚本
COPY docker/start.sh /app/start.sh
RUN chmod +x /app/start.sh

# 暴露端口（禁止使用 80，使用自定义端口）
EXPOSE 8080

# 启动
CMD ["./start.sh"]