# 构建阶段
FROM rust:1.76-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# 复制依赖文件
COPY Cargo.toml ./

# 创建空的 src 目录以缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 构建
RUN touch src/main.rs && cargo build --release

# 运行阶段
FROM alpine:3.19

RUN apk add --no-cache ca-certificates tzdata

WORKDIR /app

COPY --from=builder /app/target/release/skillhub-backend /app/

EXPOSE 3000

CMD ["./skillhub-backend"]