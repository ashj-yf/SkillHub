# ====================
# Skills Hub CLI Dockerfile
# ====================

# ====================
# 阶段 1: CLI 构建
# ====================
FROM rustlang/rust:nightly-alpine AS cli-builder

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app

# 配置 Rust crates 清华镜像
RUN mkdir -p /root/.cargo && \
    echo '[source.crates-io]' > /root/.cargo/config.toml && \
    echo 'replace-with = "tuna"' >> /root/.cargo/config.toml && \
    echo '' >> /root/.cargo/config.toml && \
    echo '[source.tuna]' >> /root/.cargo/config.toml && \
    echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' >> /root/.cargo/config.toml

# 复制依赖文件
COPY cli/Cargo.toml ./

# 创建空的 src 目录以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

# 复制源代码
COPY cli/src ./src

# 设置构建信息环境变量
ARG VERSION=0.1.0
ARG GIT_COMMIT=unknown
ARG BUILD_DATE

# 构建
RUN touch src/main.rs && VERSION=$VERSION GIT_COMMIT=$GIT_COMMIT BUILD_DATE=$BUILD_DATE cargo build

# ====================
# 阶段 2: 运行镜像
# ====================
FROM alpine:3.19

# 配置 Alpine 阿里云镜像
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 安装运行时依赖
RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

# 复制 CLI 可执行文件
COPY --from=cli-builder /app/target/debug/skillhub /usr/local/bin/skillhub

# 默认命令
ENTRYPOINT ["skillhub"]
CMD ["--help"]